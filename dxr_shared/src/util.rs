#![allow(clippy::type_complexity)]

use crate::error::DxrError;
use crate::traits::FromDXR;
use crate::types::Value;

pub fn values_to_tuple_1<T>(values: &[Value]) -> Result<(T,), DxrError>
where
    T: FromDXR,
{
    match values.len() {
        1 => {
            let value = values.get(0).unwrap();

            Ok((T::from_dxr(value)?,))
        },
        n => Err(DxrError::return_mismatch(n, 1)),
    }
}

pub fn values_to_tuple_2<A, B>(values: &[Value]) -> Result<(A, B), DxrError>
where
    A: FromDXR,
    B: FromDXR,
{
    match values.len() {
        2 => {
            let a = values.get(0).unwrap();
            let b = values.get(1).unwrap();

            Ok((A::from_dxr(a)?, B::from_dxr(b)?))
        },
        n => Err(DxrError::return_mismatch(n, 2)),
    }
}

pub fn values_to_tuple_3<A, B, C>(values: &[Value]) -> Result<(A, B, C), DxrError>
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
{
    match values.len() {
        3 => {
            let a = values.get(0).unwrap();
            let b = values.get(1).unwrap();
            let c = values.get(2).unwrap();

            Ok((A::from_dxr(a)?, B::from_dxr(b)?, C::from_dxr(c)?))
        },
        n => Err(DxrError::return_mismatch(n, 3)),
    }
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
            let a = values.get(0).unwrap();
            let b = values.get(1).unwrap();
            let c = values.get(2).unwrap();
            let d = values.get(3).unwrap();

            Ok((A::from_dxr(a)?, B::from_dxr(b)?, C::from_dxr(c)?, D::from_dxr(d)?))
        },
        n => Err(DxrError::return_mismatch(n, 4)),
    }
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
            let a = values.get(0).unwrap();
            let b = values.get(1).unwrap();
            let c = values.get(2).unwrap();
            let d = values.get(3).unwrap();
            let e = values.get(4).unwrap();

            Ok((
                A::from_dxr(a)?,
                B::from_dxr(b)?,
                C::from_dxr(c)?,
                D::from_dxr(d)?,
                E::from_dxr(e)?,
            ))
        },
        n => Err(DxrError::return_mismatch(n, 5)),
    }
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
            let a = values.get(0).unwrap();
            let b = values.get(1).unwrap();
            let c = values.get(2).unwrap();
            let d = values.get(3).unwrap();
            let e = values.get(4).unwrap();
            let f = values.get(5).unwrap();

            Ok((
                A::from_dxr(a)?,
                B::from_dxr(b)?,
                C::from_dxr(c)?,
                D::from_dxr(d)?,
                E::from_dxr(e)?,
                F::from_dxr(f)?,
            ))
        },
        n => Err(DxrError::return_mismatch(n, 6)),
    }
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
            let a = values.get(0).unwrap();
            let b = values.get(1).unwrap();
            let c = values.get(2).unwrap();
            let d = values.get(3).unwrap();
            let e = values.get(4).unwrap();
            let f = values.get(5).unwrap();
            let g = values.get(6).unwrap();

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
        n => Err(DxrError::return_mismatch(n, 7)),
    }
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
            let a = values.get(0).unwrap();
            let b = values.get(1).unwrap();
            let c = values.get(2).unwrap();
            let d = values.get(3).unwrap();
            let e = values.get(4).unwrap();
            let f = values.get(5).unwrap();
            let g = values.get(6).unwrap();
            let h = values.get(7).unwrap();

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
        n => Err(DxrError::return_mismatch(n, 8)),
    }
}

// if needed, implementations for more arguments can be implemented
