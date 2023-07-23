use std::collections::HashMap;

use crate::{Array, DxrError, Fault, Member, Struct, TryFromValue, TryToParams, TryToValue, Value};

/// Convenience method for constructing arguments for "system.multicall" calls.
///
/// This method is more efficient than manually constructing the array of XML-RPC
/// calls as structs (either by using a [`HashMap`] or by deriving [`TryToValue`]
/// on a custom struct) because this method can rely on crate internals.
pub fn into_multicall_params<P>(calls: Vec<(String, P)>) -> Result<Value, DxrError>
where
    P: TryToParams,
{
    let params: Vec<Value> = calls
        .into_iter()
        .map(|(n, p)| {
            let members = vec![
                Member::new(String::from("methodName"), Value::string(n)),
                Member::new(String::from("params"), p.try_to_params()?.try_to_value()?),
            ];
            Ok(Value::structure(Struct::new(members)))
        })
        .collect::<Result<Vec<Value>, DxrError>>()?;

    params.try_to_value()
}

/// Convenience method for reconstructing method calls from "system.multicall" arguments.
#[allow(clippy::type_complexity)]
pub fn from_multicall_params(mut values: Vec<Value>) -> Result<Vec<Result<(String, Vec<Value>), DxrError>>, DxrError> {
    // system.multicall calls take an array of arguments as single argument
    let value = match values.pop() {
        Some(value) => value,
        None => return Err(DxrError::parameter_mismatch(0, 1)),
    };

    // check if there are more than one arguments
    if !values.is_empty() {
        return Err(DxrError::parameter_mismatch(values.len() + 1, 1));
    }

    // extract vector of argument values
    let params = <Vec<Value>>::try_from_value(&value)?;

    let calls: Vec<Result<(String, Vec<Value>), DxrError>> = params
        .into_iter()
        .map(|v| {
            let mut members: HashMap<String, Value> = HashMap::try_from_value(&v)?;

            if members.len() != 2 {
                return Err(DxrError::parameter_mismatch(members.len(), 2));
            }

            let name = match members.remove("methodName") {
                Some(name) => name,
                None => return Err(DxrError::missing_field("system.multicall", "methodName")),
            };

            let params = match members.remove("params") {
                Some(params) => params,
                None => return Err(DxrError::missing_field("system.multicall", "params")),
            };

            Ok((String::try_from_value(&name)?, <Vec<Value>>::try_from_value(&params)?))
        })
        .collect();

    Ok(calls)
}

/// Convenience method for constructing return values for "system.multicall" calls.
pub fn into_multicall_response(results: Vec<Result<Value, Fault>>) -> Value {
    let values: Vec<Value> = results
        .into_iter()
        .map(|r| match r {
            Ok(value) => Value::array(Array::new(vec![value])),
            Err(fault) => {
                let members = vec![
                    Member::new(String::from("faultCode"), Value::i4(fault.code())),
                    Member::new(String::from("faultString"), Value::string(fault.string().to_owned())),
                ];
                Value::structure(Struct::new(members))
            },
        })
        .collect();

    Value::array(Array::new(values))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    use crate::values::MethodCall;
    use crate::xml::deserialize_xml;

    #[test]
    fn from_multicall() {
        let string = "\
<methodCall>
    <methodName>system.multicall</methodName>
    <params>
        <param>
            <value>
                <array>
                    <data>
                        <value>
                            <struct>
                                <member>
                                    <name>methodName</name>
                                    <value>event</value>
                                </member>
                                <member>
                                    <name>params</name>
                                    <value>
                                        <array>
                                            <data>
                                                <value>foo</value>
                                                <value>bar</value>
                                                <value>baz</value>
                                                <value><boolean>1</boolean></value>
                                            </data>
                                        </array>
                                    </value>
                                </member>
                            </struct>
                        </value>
                        <value>
                            <struct>
                                <member>
                                    <name>methodName</name>
                                    <value>event</value>
                                </member>
                                <member>
                                    <name>params</name>
                                    <value>
                                        <array>
                                            <data>
                                                <value>another</value>
                                                <value>call</value>
                                                <value>hi</value>
                                                <value><boolean>1</boolean></value>
                                            </data>
                                        </array>
                                    </value>
                                </member>
                            </struct>
                        </value>
                    </data>
                </array>
            </value>
        </param>
    </params>
</methodCall>";

        let call: MethodCall = deserialize_xml(string).unwrap();

        let params = from_multicall_params(call.params()).unwrap();

        let expected: Vec<Result<(String, Vec<Value>), DxrError>> = vec![
            Ok((
                String::from("event"),
                vec![
                    Value::string(String::from("foo")),
                    Value::string(String::from("bar")),
                    Value::string(String::from("baz")),
                    Value::boolean(true),
                ],
            )),
            Ok((
                String::from("event"),
                vec![
                    Value::string(String::from("another")),
                    Value::string(String::from("call")),
                    Value::string(String::from("hi")),
                    Value::boolean(true),
                ],
            )),
        ];

        assert_eq!(params, expected);
    }
}
