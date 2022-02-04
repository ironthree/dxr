use crate::error::DxrError;
use crate::types::Value;

/// conversion trait from XML-RPC values to Rust types
pub trait FromDXR: Sized {
    /// fallible conversion method from an XML-RPC value into the target type
    ///
    /// If the value contains a type that is not compatible with the target type, the conversion
    /// will fail.
    fn from_dxr(value: &Value) -> Result<Self, DxrError>;
}

/// conversion trait from Rust types to XML-RPC values
pub trait ToDXR: Sized {
    /// fallible conversion method from types into XML-RPC values
    ///
    /// The resulting XML-RPC value will automatically have a compatible type, so this conversion
    /// can only fail if strings cannot un-escaped from XML correctly.
    fn to_dxr(&self) -> Result<Value, DxrError>;
}

/// conversion trait from an XML-RPC call argument list to Rust types
pub trait FromParams: Sized {
    /// conversion method from XML-RPC method call argument lists to Rust types
    ///
    /// The conversion aims to do the "expected" thing, depending on the input XML-RPC value type
    /// and the targeted Rust type.
    ///
    /// TODO: FIXME more docs how lists are handled
    ///
    /// This trait can be used for argument conversion in XML-RPC servers.
    fn from_params(values: &[Value]) -> Result<Self, DxrError>;
}

/// conversion trait from Rust types to XML-RPC method call argument lists
pub trait ToParams: Sized {
    /// conversion method from types into XML-RPC method call argument lists
    ///
    /// For primitive types and maps, calling this method just calls their [`ToDXR`] implementation
    /// in turn. For collections ([`Vec`] and tuples), their values are converted to [`Value`]s,
    /// but they are treated as a list of arguments, not as a single argument that is a list.
    ///
    /// This trait can be used for argument conversion in XML-RPC clients.
    fn to_params(&self) -> Result<Vec<Value>, DxrError>;
}
