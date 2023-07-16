use std::collections::HashMap;

use chrono::NaiveDateTime;

use crate::error::DxrError;
use crate::traits::{TryToParams, TryToValue};
use crate::values::Value;

use super::utils::*;

// for simple values, use TryToValue to convert them

impl TryToParams for Value {
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

impl TryToParams for &Value {
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

impl TryToParams for i32 {
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

#[cfg(feature = "i8")]
impl TryToParams for i64 {
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

impl TryToParams for bool {
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

impl TryToParams for String {
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

impl TryToParams for &str {
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

impl TryToParams for f64 {
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

impl TryToParams for NaiveDateTime {
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

impl TryToParams for Vec<u8> {
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

impl<const N: usize> TryToParams for [u8; N] {
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

impl TryToParams for &[u8] {
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

#[cfg(feature = "nil")]
impl<T> TryToParams for Option<T>
where
    T: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

#[cfg(feature = "nil")]
impl<T> TryToParams for &Option<T>
where
    T: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

// use collections as they are without wrapping them in another Vec

impl<T> TryToParams for Vec<T>
where
    T: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        self.iter().map(|v| v.try_to_value()).collect()
    }
}

impl<T, const N: usize> TryToParams for [T; N]
where
    T: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        self.iter().map(|v| v.try_to_value()).collect()
    }
}

impl<T> TryToParams for &[T]
where
    T: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        self.iter().map(|v| v.try_to_value()).collect()
    }
}

// treat maps as a single value of a struct

impl<T> TryToParams for HashMap<String, T>
where
    T: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

impl<T> TryToParams for HashMap<&str, T>
where
    T: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.try_to_value()?])
    }
}

// treat tuples as collections of values of different types

impl TryToParams for () {
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(Vec::new())
    }
}

impl<T> TryToParams for (T,)
where
    T: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_1(self)
    }
}

impl<A, B> TryToParams for (A, B)
where
    A: TryToValue,
    B: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_2(self)
    }
}

impl<A, B, C> TryToParams for (A, B, C)
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_3(self)
    }
}

impl<A, B, C, D> TryToParams for (A, B, C, D)
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
    D: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_4(self)
    }
}

impl<A, B, C, D, E> TryToParams for (A, B, C, D, E)
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
    D: TryToValue,
    E: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_5(self)
    }
}

impl<A, B, C, D, E, F> TryToParams for (A, B, C, D, E, F)
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
    D: TryToValue,
    E: TryToValue,
    F: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_6(self)
    }
}

impl<A, B, C, D, E, F, G> TryToParams for (A, B, C, D, E, F, G)
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
    D: TryToValue,
    E: TryToValue,
    F: TryToValue,
    G: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_7(self)
    }
}

impl<A, B, C, D, E, F, G, H> TryToParams for (A, B, C, D, E, F, G, H)
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
    D: TryToValue,
    E: TryToValue,
    F: TryToValue,
    G: TryToValue,
    H: TryToValue,
{
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_8(self)
    }
}

// if needed, implementations for more arguments can be implemented
