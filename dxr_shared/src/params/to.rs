use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::error::DxrError;
use crate::traits::{ToDXR, ToParams};
use crate::types::Value;
use crate::util::*;

// for simple values, use ToDXR to convert them

impl ToParams for Value {
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

impl ToParams for &Value {
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

impl ToParams for i32 {
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

#[cfg(feature = "i8")]
impl ToParams for i64 {
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

impl ToParams for bool {
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

impl ToParams for String {
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

impl ToParams for &str {
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

impl ToParams for f64 {
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

impl ToParams for DateTime<Utc> {
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

impl ToParams for Vec<u8> {
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

impl<const N: usize> ToParams for [u8; N] {
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

impl ToParams for &[u8] {
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

#[cfg(feature = "nil")]
impl<T> ToParams for Option<T>
where
    T: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

#[cfg(feature = "nil")]
impl<T> ToParams for &Option<T>
where
    T: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

// use collections as they are without wrapping them in another Vec

impl<T> ToParams for Vec<T>
where
    T: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        self.iter().map(|v| v.to_dxr()).collect()
    }
}

impl<T, const N: usize> ToParams for [T; N]
where
    T: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        self.iter().map(|v| v.to_dxr()).collect()
    }
}

impl<T> ToParams for &[T]
where
    T: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        self.iter().map(|v| v.to_dxr()).collect()
    }
}

// treat maps as a single value of a struct

impl<T> ToParams for HashMap<String, T>
where
    T: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

impl<T> ToParams for HashMap<&str, T>
where
    T: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

// treat tuples as collections of values of different types

impl<T> ToParams for (T,)
where
    T: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        Ok(vec![self.to_dxr()?])
    }
}

impl<A, B> ToParams for (A, B)
where
    A: ToDXR,
    B: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_2(self)
    }
}

impl<A, B, C> ToParams for (A, B, C)
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_3(self)
    }
}

impl<A, B, C, D> ToParams for (A, B, C, D)
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
    D: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_4(self)
    }
}

impl<A, B, C, D, E> ToParams for (A, B, C, D, E)
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
    D: ToDXR,
    E: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_5(self)
    }
}

impl<A, B, C, D, E, F> ToParams for (A, B, C, D, E, F)
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
    D: ToDXR,
    E: ToDXR,
    F: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_6(self)
    }
}

impl<A, B, C, D, E, F, G> ToParams for (A, B, C, D, E, F, G)
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
    D: ToDXR,
    E: ToDXR,
    F: ToDXR,
    G: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_7(self)
    }
}

impl<A, B, C, D, E, F, G, H> ToParams for (A, B, C, D, E, F, G, H)
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
    D: ToDXR,
    E: ToDXR,
    F: ToDXR,
    G: ToDXR,
    H: ToDXR,
{
    fn to_params(&self) -> Result<Vec<Value>, DxrError> {
        tuple_to_values_8(self)
    }
}

// if needed, implementations for more arguments can be implemented
