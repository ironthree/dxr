#![allow(clippy::type_complexity)]

use crate::error::DxrError;
use crate::traits::{TryFromValue, TryToValue};
use crate::values::Value;

pub(crate) fn tuple_to_values_1<T>((v,): &(T,)) -> Result<Vec<Value>, DxrError>
where
    T: TryToValue,
{
    Ok(vec![v.try_to_value()?])
}

pub(crate) fn values_to_tuple_1<T>(values: &[Value]) -> Result<(T,), DxrError>
where
    T: TryFromValue,
{
    match values.len() {
        1 => {
            let value = &values[0];

            Ok((T::try_from_value(value)?,))
        },
        n => Err(DxrError::parameter_mismatch(n, 1)),
    }
}

pub(crate) fn tuple_to_values_2<A, B>((a, b): &(A, B)) -> Result<Vec<Value>, DxrError>
where
    A: TryToValue,
    B: TryToValue,
{
    Ok(vec![a.try_to_value()?, b.try_to_value()?])
}

pub(crate) fn values_to_tuple_2<A, B>(values: &[Value]) -> Result<(A, B), DxrError>
where
    A: TryFromValue,
    B: TryFromValue,
{
    match values.len() {
        2 => {
            let a = &values[0];
            let b = &values[1];

            Ok((A::try_from_value(a)?, B::try_from_value(b)?))
        },
        n => Err(DxrError::parameter_mismatch(n, 2)),
    }
}

pub(crate) fn tuple_to_values_3<A, B, C>((a, b, c): &(A, B, C)) -> Result<Vec<Value>, DxrError>
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
{
    Ok(vec![a.try_to_value()?, b.try_to_value()?, c.try_to_value()?])
}

pub(crate) fn values_to_tuple_3<A, B, C>(values: &[Value]) -> Result<(A, B, C), DxrError>
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
{
    match values.len() {
        3 => {
            let a = &values[0];
            let b = &values[1];
            let c = &values[2];

            Ok((A::try_from_value(a)?, B::try_from_value(b)?, C::try_from_value(c)?))
        },
        n => Err(DxrError::parameter_mismatch(n, 3)),
    }
}

pub(crate) fn tuple_to_values_4<A, B, C, D>((a, b, c, d): &(A, B, C, D)) -> Result<Vec<Value>, DxrError>
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
    D: TryToValue,
{
    Ok(vec![
        a.try_to_value()?,
        b.try_to_value()?,
        c.try_to_value()?,
        d.try_to_value()?,
    ])
}

pub(crate) fn values_to_tuple_4<A, B, C, D>(values: &[Value]) -> Result<(A, B, C, D), DxrError>
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
{
    match values.len() {
        4 => {
            let a = &values[0];
            let b = &values[1];
            let c = &values[2];
            let d = &values[3];

            Ok((
                A::try_from_value(a)?,
                B::try_from_value(b)?,
                C::try_from_value(c)?,
                D::try_from_value(d)?,
            ))
        },
        n => Err(DxrError::parameter_mismatch(n, 4)),
    }
}

pub(crate) fn tuple_to_values_5<A, B, C, D, E>((a, b, c, d, e): &(A, B, C, D, E)) -> Result<Vec<Value>, DxrError>
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
    D: TryToValue,
    E: TryToValue,
{
    Ok(vec![
        a.try_to_value()?,
        b.try_to_value()?,
        c.try_to_value()?,
        d.try_to_value()?,
        e.try_to_value()?,
    ])
}

pub(crate) fn values_to_tuple_5<A, B, C, D, E>(values: &[Value]) -> Result<(A, B, C, D, E), DxrError>
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
    E: TryFromValue,
{
    match values.len() {
        5 => {
            let a = &values[0];
            let b = &values[1];
            let c = &values[2];
            let d = &values[3];
            let e = &values[4];

            Ok((
                A::try_from_value(a)?,
                B::try_from_value(b)?,
                C::try_from_value(c)?,
                D::try_from_value(d)?,
                E::try_from_value(e)?,
            ))
        },
        n => Err(DxrError::parameter_mismatch(n, 5)),
    }
}

pub(crate) fn tuple_to_values_6<A, B, C, D, E, F>(
    (a, b, c, d, e, f): &(A, B, C, D, E, F),
) -> Result<Vec<Value>, DxrError>
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
    D: TryToValue,
    E: TryToValue,
    F: TryToValue,
{
    Ok(vec![
        a.try_to_value()?,
        b.try_to_value()?,
        c.try_to_value()?,
        d.try_to_value()?,
        e.try_to_value()?,
        f.try_to_value()?,
    ])
}

pub(crate) fn values_to_tuple_6<A, B, C, D, E, F>(values: &[Value]) -> Result<(A, B, C, D, E, F), DxrError>
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
    E: TryFromValue,
    F: TryFromValue,
{
    match values.len() {
        6 => {
            let a = &values[0];
            let b = &values[1];
            let c = &values[2];
            let d = &values[3];
            let e = &values[4];
            let f = &values[5];

            Ok((
                A::try_from_value(a)?,
                B::try_from_value(b)?,
                C::try_from_value(c)?,
                D::try_from_value(d)?,
                E::try_from_value(e)?,
                F::try_from_value(f)?,
            ))
        },
        n => Err(DxrError::parameter_mismatch(n, 6)),
    }
}

pub(crate) fn tuple_to_values_7<A, B, C, D, E, F, G>(
    (a, b, c, d, e, f, g): &(A, B, C, D, E, F, G),
) -> Result<Vec<Value>, DxrError>
where
    A: TryToValue,
    B: TryToValue,
    C: TryToValue,
    D: TryToValue,
    E: TryToValue,
    F: TryToValue,
    G: TryToValue,
{
    Ok(vec![
        a.try_to_value()?,
        b.try_to_value()?,
        c.try_to_value()?,
        d.try_to_value()?,
        e.try_to_value()?,
        f.try_to_value()?,
        g.try_to_value()?,
    ])
}

pub(crate) fn values_to_tuple_7<A, B, C, D, E, F, G>(values: &[Value]) -> Result<(A, B, C, D, E, F, G), DxrError>
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
    E: TryFromValue,
    F: TryFromValue,
    G: TryFromValue,
{
    match values.len() {
        7 => {
            let a = &values[0];
            let b = &values[1];
            let c = &values[2];
            let d = &values[3];
            let e = &values[4];
            let f = &values[5];
            let g = &values[6];

            Ok((
                A::try_from_value(a)?,
                B::try_from_value(b)?,
                C::try_from_value(c)?,
                D::try_from_value(d)?,
                E::try_from_value(e)?,
                F::try_from_value(f)?,
                G::try_from_value(g)?,
            ))
        },
        n => Err(DxrError::parameter_mismatch(n, 7)),
    }
}

pub(crate) fn tuple_to_values_8<A, B, C, D, E, F, G, H>(
    (a, b, c, d, e, f, g, h): &(A, B, C, D, E, F, G, H),
) -> Result<Vec<Value>, DxrError>
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
    Ok(vec![
        a.try_to_value()?,
        b.try_to_value()?,
        c.try_to_value()?,
        d.try_to_value()?,
        e.try_to_value()?,
        f.try_to_value()?,
        g.try_to_value()?,
        h.try_to_value()?,
    ])
}

pub(crate) fn values_to_tuple_8<A, B, C, D, E, F, G, H>(values: &[Value]) -> Result<(A, B, C, D, E, F, G, H), DxrError>
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
    match values.len() {
        8 => {
            let a = &values[0];
            let b = &values[1];
            let c = &values[2];
            let d = &values[3];
            let e = &values[4];
            let f = &values[5];
            let g = &values[6];
            let h = &values[7];

            Ok((
                A::try_from_value(a)?,
                B::try_from_value(b)?,
                C::try_from_value(c)?,
                D::try_from_value(d)?,
                E::try_from_value(e)?,
                F::try_from_value(f)?,
                G::try_from_value(g)?,
                H::try_from_value(h)?,
            ))
        },
        n => Err(DxrError::parameter_mismatch(n, 8)),
    }
}

// if needed, implementations for more arguments can be implemented
