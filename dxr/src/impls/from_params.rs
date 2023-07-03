use std::collections::HashMap;

use chrono::NaiveDateTime;

use crate::error::DxrError;
use crate::traits::{TryFromParams, TryFromValue};
use crate::values::Value;

use super::utils::*;

// for simple values, just call the impls for singletons / one-tuples

impl TryFromParams for Value {
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = TryFromParams::try_from_params(values)?;
        Ok(value)
    }
}

impl TryFromParams for i32 {
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = TryFromParams::try_from_params(values)?;
        Ok(value)
    }
}

#[cfg(feature = "i8")]
impl TryFromParams for i64 {
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = TryFromParams::try_from_params(values)?;
        Ok(value)
    }
}

impl TryFromParams for bool {
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = TryFromParams::try_from_params(values)?;
        Ok(value)
    }
}

impl TryFromParams for String {
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = TryFromParams::try_from_params(values)?;
        Ok(value)
    }
}

impl TryFromParams for f64 {
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = TryFromParams::try_from_params(values)?;
        Ok(value)
    }
}

impl TryFromParams for NaiveDateTime {
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = TryFromParams::try_from_params(values)?;
        Ok(value)
    }
}

impl TryFromParams for Vec<u8> {
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = TryFromParams::try_from_params(values)?;
        Ok(value)
    }
}

// handle optional values twice (not sure if this is a good idea):
// - check whether there *is* a value
// - check whether it is a <nil> value

#[cfg(feature = "nil")]
impl<T> TryFromParams for Option<T>
where
    T: TryFromValue,
{
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        // one value: convert or return None if it is a <nil/> value
        match values.len() {
            1 => Ok(Option::try_from_value(&values[0])?),
            0 => Ok(None),
            n => Err(DxrError::parameter_mismatch(n, 1)),
        }
    }
}

// use collections as they are without unwrapping them

impl<T> TryFromParams for Vec<T>
where
    T: TryFromValue,
{
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        values.iter().map(|v| T::try_from_value(v)).collect()
    }
}

impl TryFromParams for () {
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        match values.len() {
            0 => Ok(()),
            n => Err(DxrError::parameter_mismatch(n, 0)),
        }
    }
}

// treat maps as a single value of a struct

impl<T> TryFromParams for HashMap<String, T>
where
    T: TryFromValue,
{
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        let (value,): (Self,) = TryFromParams::try_from_params(values)?;
        Ok(value)
    }
}

// treat tuples as collections of values of different types

impl<T> TryFromParams for (T,)
where
    T: TryFromValue,
{
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_1(values)
    }
}

impl<A, B> TryFromParams for (A, B)
where
    A: TryFromValue,
    B: TryFromValue,
{
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_2(values)
    }
}

impl<A, B, C> TryFromParams for (A, B, C)
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
{
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_3(values)
    }
}

impl<A, B, C, D> TryFromParams for (A, B, C, D)
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
{
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_4(values)
    }
}

impl<A, B, C, D, E> TryFromParams for (A, B, C, D, E)
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
    E: TryFromValue,
{
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_5(values)
    }
}

impl<A, B, C, D, E, F> TryFromParams for (A, B, C, D, E, F)
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
    E: TryFromValue,
    F: TryFromValue,
{
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_6(values)
    }
}

impl<A, B, C, D, E, F, G> TryFromParams for (A, B, C, D, E, F, G)
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
    E: TryFromValue,
    F: TryFromValue,
    G: TryFromValue,
{
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_7(values)
    }
}

impl<A, B, C, D, E, F, G, H> TryFromParams for (A, B, C, D, E, F, G, H)
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
    E: TryFromValue,
    F: TryFromValue,
    G: TryFromValue,
    H: TryFromValue,
{
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError> {
        values_to_tuple_8(values)
    }
}

// if needed, implementations for more arguments can be implemented
