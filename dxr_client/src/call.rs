use std::marker::PhantomData;

use dxr_shared::{DxrError, MethodCall, TryFromValue, TryToParams, Value};

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
    /// constructor for [`Call`] values from method name and method parameters
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
            retype: PhantomData::default(),
        }
    }

    /// convert [`Call`] into [`MethodCall`] XML-RPC value
    pub fn as_xml_rpc(&self) -> Result<MethodCall, DxrError> {
        Ok(MethodCall::new(self.method(), self.params()?))
    }

    fn method(&self) -> String {
        String::from(self.method)
    }

    fn params(&self) -> Result<Vec<Value>, DxrError> {
        self.params.try_to_params()
    }
}
