use crate::error::DxrError;
use crate::values::Value;

/// Trait for converting from Rust values to XML-RPC values.
pub trait TryToValue: Sized {
    /// Fallible conversion method from Rust values into XML-RPC values.
    ///
    /// This method is infallible for all trait implementations in this crate, but still returns a
    /// [`Result`] - both for backwards compatibility with older versions of this crate, and for
    /// forwards compatibility (i.e. implementing this trait for a type where this method *can* fail
    /// will not be a breaking change).
    fn try_to_value(&self) -> Result<Value, DxrError>;
}

/// Trait for converting from XML-RPC values to Rust values.
pub trait TryFromValue: Sized {
    /// Fallible conversion method from XML-RPC values into the Rust values.
    ///
    /// If the value contains a type that is not compatible with the target type, the conversion
    /// will fail (missing struct members, type mismatches, or mismatch with the expected length of
    /// an array or tuple).
    fn try_from_value(value: &Value) -> Result<Self, DxrError>;
}

/// Trait for converting from Rust values to XML-RPC method call arguments.
pub trait TryToParams: Sized {
    /// Fallible conversion method from Rust values into XML-RPC method call argument lists.
    ///
    /// For primitive types and maps, calling this method just calls their [`TryToValue`]
    /// implementation in turn. For collections ([`Vec`] and tuples), their values are converted to
    /// [`Value`]s, but they are treated as a list of arguments, not as a single argument that is a
    /// list. To pass a single argument of type `array`, pass a [`Value`] of type array directly.
    fn try_to_params(&self) -> Result<Vec<Value>, DxrError>;
}

/// Trait for converting from XML-RPC method call argument lists to Rust values.
pub trait TryFromParams: Sized {
    /// Fallible conversion method from XML-RPC method call argument lists to Rust values.
    ///
    /// The conversion aims to do the "expected" thing, depending on the input XML-RPC value type
    /// and the targeted Rust type.
    ///
    /// - Tuples with *N* members of potentially heterogeneous types are converted from lists with
    ///   length *N*. This returns an error if the lengths don't match, or if any of the target
    ///   types don't match.
    /// - Simple values are treated the same as singletons / one-tuples, i.e. this returns an error
    ///   if the length of the parameter list is not one.
    /// - Lists of homogeneously typed values are not checked for length, but only converted to the
    ///   target type. This returns an error if any list value does not match the target type.
    fn try_from_params(values: &[Value]) -> Result<Self, DxrError>;
}
