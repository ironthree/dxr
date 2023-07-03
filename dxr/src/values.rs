/// Date & time format used by the XML-RPC `dateTime.iso8601` value type
///
/// This string represents the format of the timezone-unaware date & time format used by the XML-RPC
/// `dateTime.iso8601` value type, to be used with [`chrono::NaiveDateTime::parse_from_str`].
pub const XML_RPC_DATE_FORMAT: &str = "%Y%m%dT%H:%M:%S";

mod ser_de;

mod types;
pub use types::*;
