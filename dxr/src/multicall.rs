use crate::{DxrError, Member, Struct, TryToParams, TryToValue, Value};

/// Convenience method for constructing arguments for "system.multicall" calls.
///
/// This method is more efficient than manually constructing the array of XML-RPC
/// calls as structs (either by using a [`HashMap`] or by deriving [`TryToValue`]
/// on a custom struct) because this method can rely on crate internals.
pub fn multicall<P>(calls: Vec<(String, P)>) -> Result<Value, DxrError>
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
