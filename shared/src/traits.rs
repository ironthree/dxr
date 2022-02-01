use crate::error::DxrError;
use crate::types::Value;

/// conversion trait from XML-RPC values to primitives, `Option`, `HashMap`, and user-defined types
pub trait FromDXR: Sized {
    /// fallible conversion method from an XML-RPC value into the target type
    ///
    /// If the value contains a type that is not compatible with the target type, the conversion
    /// will fail.
    fn from_dxr(value: &Value) -> Result<Self, DxrError>;
}

/// conversion trait from primitives, `Option`, `HashMap`, and user-defined types to XML-RPC values
pub trait ToDXR: Sized {
    /// conversion method from types into XML-RPC values
    ///
    /// The resulting XML-RPC value will automatically have a compatible type, so this conversion
    /// can only fail if strings cannot un-escaped from XML correctly.
    fn to_dxr(&self) -> Result<Value, DxrError>;
}
