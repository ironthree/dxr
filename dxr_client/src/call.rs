use std::marker::PhantomData;

use dxr::{DxrError, MethodCall, TryFromValue, TryToParams, Value};

/// # XML-RPC method call
///
/// This type describes the data associated with an XML-RPC method call on the client side. This
/// includes the method name, method parameters, and expected return type.
#[derive(Debug)]
pub struct Call<'a, P, R>
where
    P: TryToParams,
    R: TryFromValue,
{
    method: &'a str,
    params: P,
    retype: PhantomData<R>,
}

impl<'a, P, R> Call<'a, P, R>
where
    P: TryToParams,
    R: TryFromValue,
{
    /// Constructor for [`Call`] values from method name and method parameters.
    ///
    /// This method accepts every type of value for the `params` argument if it implements the
    /// [`TryToParams`] trait. This includes:
    ///
    /// - primitives (`i32`, `i64`, `String`, `f64`, `DateTime`, bytes / `Vec<u8`, etc.)
    /// - arrays and slices of values of the same type (i.e. `Vec<T`, `[T]`, `&[T]`)
    /// - tuples up to length 8 of values of possibly different types (i.e. `(i32, bool)`
    ///
    /// For method calls with arguments that have different values, either convert them all to
    /// [`Value`] first and use an array type, or use them directly and pass them as a tuple.
    ///
    /// Note that this method will need type annotations to determine the type `R` of the expected
    /// return value.
    pub fn new(method: &'a str, params: P) -> Call<P, R> {
        Call {
            method,
            params,
            retype: PhantomData,
        }
    }

    /// convert [`Call`] into [`MethodCall`] XML-RPC value
    pub fn as_xml_rpc(&self) -> Result<MethodCall, DxrError> {
        Ok(MethodCall::new(self.method(), self.params()?))
    }

    pub(crate) fn method(&self) -> String {
        String::from(self.method)
    }

    pub(crate) fn params(&self) -> Result<Vec<Value>, DxrError> {
        self.params.try_to_params()
    }
}

#[cfg(feature = "multicall")]
impl<P> Call<'static, P, Vec<Value>>
where
    P: TryToParams,
{
    /// Constructor for [`Call`] values for `system.multicall` calls.
    pub fn multicall(calls: Vec<(String, P)>) -> Result<Call<'static, Value, Vec<Value>>, DxrError> {
        let calls = dxr::into_multicall_params(calls)?;
        Ok(Call::new("system.multicall", calls))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    #[cfg(feature = "multicall")]
    use super::*;

    #[cfg(feature = "multicall")]
    #[test]
    fn to_multicall() {
        let call = Call::multicall(vec![(String::from("add"), (1, 2)), (String::from("sub"), (2, 1))]).unwrap();
        let string = quick_xml::se::to_string(&call.as_xml_rpc().unwrap()).unwrap();

        let expected = "\
<methodCall>
<methodName>system.multicall</methodName>
<params>

<param>
<value>
<array><data>

<value>
<struct>
<member><name>methodName</name><value><string>add</string></value></member>
<member><name>params</name><value><array><data><value><i4>1</i4></value><value><i4>2</i4></value></data></array></value></member>
</struct>
</value>

<value>
<struct>
<member><name>methodName</name><value><string>sub</string></value></member>
<member><name>params</name><value><array><data><value><i4>2</i4></value><value><i4>1</i4></value></data></array></value></member>
</struct>
</value>

</data></array>
</value>
</param>

</params>
</methodCall>".replace('\n', "");

        assert_eq!(string, expected);
    }
}
