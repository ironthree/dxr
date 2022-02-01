#![allow(clippy::unwrap_used)]

use chrono::{SubsecRound, Utc};
use quick_xml::{de::from_str, se::to_string};

use crate::types::*;
use crate::XML_RPC_DATE_FORMAT;

mod arrays;
mod call;
mod response;
mod structs;
mod types;
mod values;
