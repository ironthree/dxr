use std::marker::PhantomData;

use dxr_shared::types::{MethodCall, Value};
use dxr_shared::{DxrError, FromDXR, ToDXR};

#[derive(Debug)]
pub struct Call<P, R>
where
    P: ToDXR,
    R: FromDXR,
{
    method: String,
    params: Vec<P>,
    retype: PhantomData<*const R>,
}

impl<P, R> Call<P, R>
where
    P: ToDXR,
    R: FromDXR,
{
    pub fn new(method: String, params: Vec<P>) -> Call<P, R> {
        Call {
            method,
            params,
            retype: PhantomData::default(),
        }
    }

    pub(crate) fn params_to_dxr(&self) -> Result<MethodCall, DxrError> {
        Ok(MethodCall::new(self.method(), self.params()?))
    }

    fn method(&self) -> String {
        String::from(&self.method)
    }

    fn params(&self) -> Result<Vec<Value>, DxrError> {
        self.params
            .iter()
            .map(|v| ToDXR::to_dxr(v))
            .collect::<Result<Vec<Value>, DxrError>>()
    }
}
