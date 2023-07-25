//! Module with helper functions to replace the "simple" API that was deprecated with
//! base64 v0.21.

use base64::engine::general_purpose::STANDARD;
use base64::{DecodeError, Engine};

pub(crate) fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
    STANDARD.decode(input)
}

pub(crate) fn encode<T: AsRef<[u8]>>(input: T) -> String {
    STANDARD.encode(input)
}
