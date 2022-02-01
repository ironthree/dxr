use std::marker::PhantomData;

use dxr_shared::types::{MethodCall, Value};
use dxr_shared::{DxrError, FromDXR, ToParams};

#[derive(Debug)]
pub struct Call<P, R>
where
    P: ToParams,
    R: FromDXR,
{
    method: String,
    params: P,
    retype: PhantomData<*const R>,
}

impl<P, R> Call<P, R>
where
    P: ToParams,
    R: FromDXR,
{
    pub fn new(method: String, params: P) -> Call<P, R> {
        Call {
            method,
            params,
            retype: PhantomData::default(),
        }
    }

    pub(crate) fn as_xml_rpc(&self) -> Result<MethodCall, DxrError> {
        Ok(MethodCall::new(self.method(), self.params()?))
    }

    fn method(&self) -> String {
        String::from(&self.method)
    }

    fn params(&self) -> Result<Vec<Value>, DxrError> {
        self.params.to_params()
    }
}
