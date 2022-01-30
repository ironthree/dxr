pub const XML_RPC_DATE_FORMAT: &str = "%Y%m%dT%H:%M:%S";

mod ser_de;

mod impls;
pub use impls::*;

mod types;
pub use types::*;

pub trait FromValue<T> {
    fn from_value(value: &Value) -> Result<T, ()>;
}

#[cfg(test)]
mod tests;
