#![allow(clippy::type_complexity)]

use crate::error::DxrError;
use crate::types::structs::Value;
use crate::types::traits::{FromDXR, ToDXR};

pub fn tuple_to_values_1<T>((v,): &(T,)) -> Result<Vec<Value>, DxrError>
where
    T: ToDXR,
{
    Ok(vec![v.to_dxr()?])
}

pub fn values_to_tuple_1<T>(values: &[Value]) -> Result<(T,), DxrError>
where
    T: FromDXR,
{
    match values.len() {
        1 => {
            let value = &values[0];

            Ok((T::from_dxr(value)?,))
        },
        n => Err(DxrError::parameter_mismatch(n, 1)),
    }
}

pub fn tuple_to_values_2<A, B>((a, b): &(A, B)) -> Result<Vec<Value>, DxrError>
where
    A: ToDXR,
    B: ToDXR,
{
    Ok(vec![a.to_dxr()?, b.to_dxr()?])
}

pub fn values_to_tuple_2<A, B>(values: &[Value]) -> Result<(A, B), DxrError>
where
    A: FromDXR,
    B: FromDXR,
{
    match values.len() {
        2 => {
            let a = &values[0];
            let b = &values[1];

            Ok((A::from_dxr(a)?, B::from_dxr(b)?))
        },
        n => Err(DxrError::parameter_mismatch(n, 2)),
    }
}

pub fn tuple_to_values_3<A, B, C>((a, b, c): &(A, B, C)) -> Result<Vec<Value>, DxrError>
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
{
    Ok(vec![a.to_dxr()?, b.to_dxr()?, c.to_dxr()?])
}

pub fn values_to_tuple_3<A, B, C>(values: &[Value]) -> Result<(A, B, C), DxrError>
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
{
    match values.len() {
        3 => {
            let a = &values[0];
            let b = &values[1];
            let c = &values[2];

            Ok((A::from_dxr(a)?, B::from_dxr(b)?, C::from_dxr(c)?))
        },
        n => Err(DxrError::parameter_mismatch(n, 3)),
    }
}

pub fn tuple_to_values_4<A, B, C, D>((a, b, c, d): &(A, B, C, D)) -> Result<Vec<Value>, DxrError>
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
    D: ToDXR,
{
    Ok(vec![a.to_dxr()?, b.to_dxr()?, c.to_dxr()?, d.to_dxr()?])
}

pub fn values_to_tuple_4<A, B, C, D>(values: &[Value]) -> Result<(A, B, C, D), DxrError>
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
    D: FromDXR,
{
    match values.len() {
        4 => {
            let a = &values[0];
            let b = &values[1];
            let c = &values[2];
            let d = &values[3];

            Ok((A::from_dxr(a)?, B::from_dxr(b)?, C::from_dxr(c)?, D::from_dxr(d)?))
        },
        n => Err(DxrError::parameter_mismatch(n, 4)),
    }
}

pub fn tuple_to_values_5<A, B, C, D, E>((a, b, c, d, e): &(A, B, C, D, E)) -> Result<Vec<Value>, DxrError>
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
    D: ToDXR,
    E: ToDXR,
{
    Ok(vec![a.to_dxr()?, b.to_dxr()?, c.to_dxr()?, d.to_dxr()?, e.to_dxr()?])
}

pub fn values_to_tuple_5<A, B, C, D, E>(values: &[Value]) -> Result<(A, B, C, D, E), DxrError>
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
    D: FromDXR,
    E: FromDXR,
{
    match values.len() {
        5 => {
            let a = &values[0];
            let b = &values[1];
            let c = &values[2];
            let d = &values[3];
            let e = &values[4];

            Ok((
                A::from_dxr(a)?,
                B::from_dxr(b)?,
                C::from_dxr(c)?,
                D::from_dxr(d)?,
                E::from_dxr(e)?,
            ))
        },
        n => Err(DxrError::parameter_mismatch(n, 5)),
    }
}

pub fn tuple_to_values_6<A, B, C, D, E, F>((a, b, c, d, e, f): &(A, B, C, D, E, F)) -> Result<Vec<Value>, DxrError>
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
    D: ToDXR,
    E: ToDXR,
    F: ToDXR,
{
    Ok(vec![
        a.to_dxr()?,
        b.to_dxr()?,
        c.to_dxr()?,
        d.to_dxr()?,
        e.to_dxr()?,
        f.to_dxr()?,
    ])
}

pub fn values_to_tuple_6<A, B, C, D, E, F>(values: &[Value]) -> Result<(A, B, C, D, E, F), DxrError>
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
    D: FromDXR,
    E: FromDXR,
    F: FromDXR,
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
                A::from_dxr(a)?,
                B::from_dxr(b)?,
                C::from_dxr(c)?,
                D::from_dxr(d)?,
                E::from_dxr(e)?,
                F::from_dxr(f)?,
            ))
        },
        n => Err(DxrError::parameter_mismatch(n, 6)),
    }
}

pub fn tuple_to_values_7<A, B, C, D, E, F, G>(
    (a, b, c, d, e, f, g): &(A, B, C, D, E, F, G),
) -> Result<Vec<Value>, DxrError>
where
    A: ToDXR,
    B: ToDXR,
    C: ToDXR,
    D: ToDXR,
    E: ToDXR,
    F: ToDXR,
    G: ToDXR,
{
    Ok(vec![
        a.to_dxr()?,
        b.to_dxr()?,
        c.to_dxr()?,
        d.to_dxr()?,
        e.to_dxr()?,
        f.to_dxr()?,
        g.to_dxr()?,
    ])
}

pub fn values_to_tuple_7<A, B, C, D, E, F, G>(values: &[Value]) -> Result<(A, B, C, D, E, F, G), DxrError>
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
    D: FromDXR,
    E: FromDXR,
    F: FromDXR,
    G: FromDXR,
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
                A::from_dxr(a)?,
                B::from_dxr(b)?,
                C::from_dxr(c)?,
                D::from_dxr(d)?,
                E::from_dxr(e)?,
                F::from_dxr(f)?,
                G::from_dxr(g)?,
            ))
        },
        n => Err(DxrError::parameter_mismatch(n, 7)),
    }
}

pub fn tuple_to_values_8<A, B, C, D, E, F, G, H>(
    (a, b, c, d, e, f, g, h): &(A, B, C, D, E, F, G, H),
) -> Result<Vec<Value>, DxrError>
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
    Ok(vec![
        a.to_dxr()?,
        b.to_dxr()?,
        c.to_dxr()?,
        d.to_dxr()?,
        e.to_dxr()?,
        f.to_dxr()?,
        g.to_dxr()?,
        h.to_dxr()?,
    ])
}

pub fn values_to_tuple_8<A, B, C, D, E, F, G, H>(values: &[Value]) -> Result<(A, B, C, D, E, F, G, H), DxrError>
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
                A::from_dxr(a)?,
                B::from_dxr(b)?,
                C::from_dxr(c)?,
                D::from_dxr(d)?,
                E::from_dxr(e)?,
                F::from_dxr(f)?,
                G::from_dxr(g)?,
                H::from_dxr(h)?,
            ))
        },
        n => Err(DxrError::parameter_mismatch(n, 8)),
    }
}

// if needed, implementations for more arguments can be implemented
