use std::collections::HashMap;

use crate::chrono::{DateTime, Utc};
use crate::error::DxrError;
use crate::traits::{FromDXR, FromParams};
use crate::types::Value;
use crate::util::*;

// for simple values, just call the impls for singletons / one-tuples

impl FromParams for Value {
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = FromParams::from_params(values)?;
        Ok(value)
    }
}

impl FromParams for i32 {
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = FromParams::from_params(values)?;
        Ok(value)
    }
}

#[cfg(feature = "i8")]
#[cfg_attr(docsrs, doc(cfg(feature = "i8")))]
impl FromParams for i64 {
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = FromParams::from_params(values)?;
        Ok(value)
    }
}

impl FromParams for bool {
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = FromParams::from_params(values)?;
        Ok(value)
    }
}

impl FromParams for String {
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = FromParams::from_params(values)?;
        Ok(value)
    }
}

impl FromParams for f64 {
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = FromParams::from_params(values)?;
        Ok(value)
    }
}

impl FromParams for DateTime<Utc> {
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = FromParams::from_params(values)?;
        Ok(value)
    }
}

impl FromParams for Vec<u8> {
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = FromParams::from_params(values)?;
        Ok(value)
    }
}

// handle optional values twice (not sure if this is a good idea):
// - check whether there *is* a value
// - check whether it is a <nil> value

#[cfg(feature = "nil")]
#[cfg_attr(docsrs, doc(cfg(feature = "nil")))]
impl<T> FromParams for Option<T>
where
    T: FromDXR,
{
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        // one value: convert or return None if it is a <nil/> value
        match values.len() {
            1 => Ok(Option::from_dxr(values.get(0).unwrap())?),
            0 => Ok(None),
            n => Err(DxrError::parameter_mismatch(n, 1)),
        }
    }
}

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
            n => Err(DxrError::parameter_mismatch(n, 0)),
        }
    }
}

// treat maps as a single value of a struct

impl<T> FromParams for HashMap<String, T>
where
    T: FromDXR,
{
    fn from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = FromParams::from_params(values)?;
        Ok(value)
    }
}

// treat tuples as collections of values of different types

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
