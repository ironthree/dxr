use crate::{FromDXR, ToDXR};
use dxr_shared::types::{MethodCall, Value};
use dxr_shared::DxrError;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Call<P, R>
where
    P: ToDXR,
    R: FromDXR,
{
    method: String,
    params: Vec<P>,
    retype: PhantomData<R>,
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

    pub(crate) fn retval_from_dxr(&self, returned: &Value) -> Result<R, DxrError> {
        R::from_dxr(returned)
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
