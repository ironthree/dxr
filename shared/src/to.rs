use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::error::DxrError;
use crate::traits::ToDXR;
use crate::types::{Array, Member, Struct, Value};

impl ToDXR for Value {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(self.clone())
    }
}

impl ToDXR for &Value {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::clone(self))
    }
}

impl ToDXR for i32 {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::i4(*self))
    }
}

#[cfg(feature = "i8")]
impl ToDXR for i64 {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::i8(*self))
    }
}

impl ToDXR for bool {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::boolean(*self))
    }
}

impl ToDXR for String {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        ToDXR::to_dxr(&self.as_str())
    }
}

impl ToDXR for &str {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Value::string_escape(self)
    }
}

impl ToDXR for f64 {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::double(*self))
    }
}

impl ToDXR for DateTime<Utc> {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::datetime(*self))
    }
}

impl ToDXR for Vec<u8> {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        ToDXR::to_dxr(&self.as_slice())
    }
}

impl<const N: usize> ToDXR for [u8; N] {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        ToDXR::to_dxr(&self.as_slice())
    }
}

impl ToDXR for &[u8] {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::base64(self.to_vec()))
    }
}

impl<T> ToDXR for Option<T>
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        if let Some(value) = self {
            T::to_dxr(value)
        } else {
            Ok(Value::nil())
        }
    }
}

impl<T> ToDXR for &Option<T>
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        if let Some(value) = self {
            T::to_dxr(value)
        } else {
            Ok(Value::nil())
        }
    }
}

impl<T> ToDXR for Vec<T>
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        ToDXR::to_dxr(&self.as_slice())
    }
}

impl<T, const N: usize> ToDXR for [T; N]
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        ToDXR::to_dxr(&self.as_slice())
    }
}

impl<T> ToDXR for &[T]
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let values = self
            .iter()
            .map(|value| T::to_dxr(value))
            .collect::<Result<Vec<Value>, DxrError>>();

        Ok(Value::array(Array::new(values?)))
    }
}

impl<T> ToDXR for HashMap<String, T>
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let members = self
            .iter()
            .map(|(k, v)| T::to_dxr(v).map(|v| Member::new(k.to_owned(), v)))
            .collect::<Result<Vec<Member>, DxrError>>();

        Ok(Value::structure(Struct::new(members?)))
    }
}

impl<T> ToDXR for HashMap<&str, T>
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let members = self
            .iter()
            .map(|(k, v)| T::to_dxr(v).map(|v| Member::new((*k).to_owned(), v)))
            .collect::<Result<Vec<Member>, DxrError>>();

        Ok(Value::structure(Struct::new(members?)))
    }
}

impl<T> ToDXR for (T,)
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::array(Array::new(vec![self.to_dxr()?])))
    }
}

impl<A, B> ToDXR for (A, B)
where
    A: ToDXR,
    B: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let (a, b) = self;

        Ok(Value::array(Array::new(vec![a.to_dxr()?, b.to_dxr()?])))
    }
}

impl<A, B, C> ToDXR for (A, B, C)
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let (a, b, c) = self;

        Ok(Value::array(Array::new(vec![a.to_dxr()?, b.to_dxr()?, c.to_dxr()?])))
    }
}

impl<A, B, C, D> ToDXR for (A, B, C, D)
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
    D: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let (a, b, c, d) = self;

        Ok(Value::array(Array::new(vec![
            a.to_dxr()?,
            b.to_dxr()?,
            c.to_dxr()?,
            d.to_dxr()?,
        ])))
    }
}

impl<A, B, C, D, E> ToDXR for (A, B, C, D, E)
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
    D: ToDXR,
    E: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let (a, b, c, d, e) = self;

        Ok(Value::array(Array::new(vec![
            a.to_dxr()?,
            b.to_dxr()?,
            c.to_dxr()?,
            d.to_dxr()?,
            e.to_dxr()?,
        ])))
    }
}

impl<A, B, C, D, E, F> ToDXR for (A, B, C, D, E, F)
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
    D: ToDXR,
    E: ToDXR,
    F: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let (a, b, c, d, e, f) = self;

        Ok(Value::array(Array::new(vec![
            a.to_dxr()?,
            b.to_dxr()?,
            c.to_dxr()?,
            d.to_dxr()?,
            e.to_dxr()?,
            f.to_dxr()?,
        ])))
    }
}

impl<A, B, C, D, E, F, G> ToDXR for (A, B, C, D, E, F, G)
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
    D: ToDXR,
    E: ToDXR,
    F: ToDXR,
    G: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let (a, b, c, d, e, f, g) = self;

        Ok(Value::array(Array::new(vec![
            a.to_dxr()?,
            b.to_dxr()?,
            c.to_dxr()?,
            d.to_dxr()?,
            e.to_dxr()?,
            f.to_dxr()?,
            g.to_dxr()?,
        ])))
    }
}

impl<A, B, C, D, E, F, G, H> ToDXR for (A, B, C, D, E, F, G, H)
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
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let (a, b, c, d, e, f, g, h) = self;

        Ok(Value::array(Array::new(vec![
            a.to_dxr()?,
            b.to_dxr()?,
            c.to_dxr()?,
            d.to_dxr()?,
            e.to_dxr()?,
            f.to_dxr()?,
            g.to_dxr()?,
            h.to_dxr()?,
        ])))
    }
}

// if needed, implementations for more arguments can be implemented
