use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use chrono::NaiveDateTime;

use crate::error::DxrError;
use crate::traits::TryToValue;
use crate::values::{Array, Member, Struct, Value};

use super::utils::*;

impl<T> TryToValue for &T
where
    T: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        TryToValue::try_to_value(*self)
    }
}

impl TryToValue for Value {
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(self.clone())
    }
}

impl TryToValue for i32 {
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::i4(*self))
    }
}

#[cfg(feature = "i8")]
impl TryToValue for i64 {
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::i8(*self))
    }
}

impl TryToValue for bool {
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::boolean(*self))
    }
}

impl TryToValue for String {
    fn try_to_value(&self) -> Result<Value, DxrError> {
        TryToValue::try_to_value(&self.as_str())
    }
}

impl TryToValue for &str {
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::string(String::from(*self)))
    }
}

impl TryToValue for f64 {
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::double(*self))
    }
}

impl TryToValue for NaiveDateTime {
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::datetime(*self))
    }
}

impl TryToValue for Vec<u8> {
    fn try_to_value(&self) -> Result<Value, DxrError> {
        TryToValue::try_to_value(&self.as_slice())
    }
}

impl<const N: usize> TryToValue for [u8; N] {
    fn try_to_value(&self) -> Result<Value, DxrError> {
        TryToValue::try_to_value(&self.as_slice())
    }
}

impl TryToValue for &[u8] {
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::base64(self.to_vec()))
    }
}

#[cfg(feature = "nil")]
impl<T> TryToValue for Option<T>
where
    T: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        if let Some(value) = self {
            T::try_to_value(value)
        } else {
            Ok(Value::nil())
        }
    }
}

impl<'a, T> TryToValue for Cow<'a, T>
where
    T: TryToValue + Clone,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        match self {
            Cow::Owned(owned) => TryToValue::try_to_value(owned),
            Cow::Borrowed(borrowed) => TryToValue::try_to_value(*borrowed),
        }
    }
}

impl<T> TryToValue for Box<T>
where
    T: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        TryToValue::try_to_value(self.as_ref())
    }
}

impl<T> TryToValue for Rc<T>
where
    T: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        TryToValue::try_to_value(self.as_ref())
    }
}

impl<T> TryToValue for Arc<T>
where
    T: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        TryToValue::try_to_value(self.as_ref())
    }
}

impl<T> TryToValue for Vec<T>
where
    T: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        TryToValue::try_to_value(&self.as_slice())
    }
}

impl<T, const N: usize> TryToValue for [T; N]
where
    T: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        TryToValue::try_to_value(&self.as_slice())
    }
}

impl<T> TryToValue for &[T]
where
    T: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        let values = self
            .iter()
            .map(|value| T::try_to_value(value))
            .collect::<Result<Vec<Value>, DxrError>>();

        Ok(Value::array(Array::new(values?)))
    }
}

impl<T> TryToValue for HashMap<String, T>
where
    T: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        let members = self
            .iter()
            .map(|(k, v)| T::try_to_value(v).map(|v| Member::new(k.to_owned(), v)))
            .collect::<Result<Vec<Member>, DxrError>>();

        Ok(Value::structure(Struct::new(members?)))
    }
}

impl<T> TryToValue for HashMap<&str, T>
where
    T: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        let members = self
            .iter()
            .map(|(k, v)| T::try_to_value(v).map(|v| Member::new((*k).to_owned(), v)))
            .collect::<Result<Vec<Member>, DxrError>>();

        Ok(Value::structure(Struct::new(members?)))
    }
}

impl<T> TryToValue for (T,)
where
    T: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::array(Array::new(tuple_to_values_1(self)?)))
    }
}

impl<A, B> TryToValue for (A, B)
where
    A: TryToValue,
    B: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::array(Array::new(tuple_to_values_2(self)?)))
    }
}

impl<A, B, C> TryToValue for (A, B, C)
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::array(Array::new(tuple_to_values_3(self)?)))
    }
}

impl<A, B, C, D> TryToValue for (A, B, C, D)
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
    D: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::array(Array::new(tuple_to_values_4(self)?)))
    }
}

impl<A, B, C, D, E> TryToValue for (A, B, C, D, E)
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
    D: TryToValue,
    E: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::array(Array::new(tuple_to_values_5(self)?)))
    }
}

impl<A, B, C, D, E, F> TryToValue for (A, B, C, D, E, F)
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
    D: TryToValue,
    E: TryToValue,
    F: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::array(Array::new(tuple_to_values_6(self)?)))
    }
}

impl<A, B, C, D, E, F, G> TryToValue for (A, B, C, D, E, F, G)
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
    D: TryToValue,
    E: TryToValue,
    F: TryToValue,
    G: TryToValue,
{
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::array(Array::new(tuple_to_values_7(self)?)))
    }
}

impl<A, B, C, D, E, F, G, H> TryToValue for (A, B, C, D, E, F, G, H)
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
    fn try_to_value(&self) -> Result<Value, DxrError> {
        Ok(Value::array(Array::new(tuple_to_values_8(self)?)))
    }
}

// if needed, implementations for more arguments can be implemented
