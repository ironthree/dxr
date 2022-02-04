use std::collections::HashMap;

use crate::chrono::{DateTime, Utc};
use crate::error::DxrError;
use crate::traits::{FromDXR, FromParams};
use crate::types::Value;
use crate::util::*;

// use collections as they are without unwrapping them

impl<T> FromParams for Vec<T>
where
    T: FromDXR,
{
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        values.iter().map(|v| T::from_dxr(v)).collect()
    }
}

impl FromParams for () {
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        match values.len() {
            0 => Ok(()),
            n => Err(DxrError::return_mismatch(n, 0)),
        }
    }
}

impl<T> FromParams for (T,)
where
    T: FromDXR,
{
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_1(values)
    }
}

impl<A, B> FromParams for (A, B)
where
    A: FromDXR,
    B: FromDXR,
{
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_2(values)
    }
}

impl<A, B, C> FromParams for (A, B, C)
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
{
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_3(values)
    }
}

impl<A, B, C, D> FromParams for (A, B, C, D)
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
    D: FromDXR,
{
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_4(values)
    }
}

impl<A, B, C, D, E> FromParams for (A, B, C, D, E)
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
    D: FromDXR,
    E: FromDXR,
{
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_5(values)
    }
}

impl<A, B, C, D, E, F> FromParams for (A, B, C, D, E, F)
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
    D: FromDXR,
    E: FromDXR,
    F: FromDXR,
{
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_6(values)
    }
}

impl<A, B, C, D, E, F, G> FromParams for (A, B, C, D, E, F, G)
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
    D: FromDXR,
    E: FromDXR,
    F: FromDXR,
    G: FromDXR,
{
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_7(values)
    }
}

impl<A, B, C, D, E, F, G, H> FromParams for (A, B, C, D, E, F, G, H)
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
    D: FromDXR,
    E: FromDXR,
    F: FromDXR,
    G: FromDXR,
    H: FromDXR,
{
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_8(values)
    }
}

// if needed, implementations for more arguments can be implemented
