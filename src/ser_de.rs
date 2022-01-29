pub mod datetime {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::XML_RPC_DATE_FORMAT;

    pub fn serialize<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let string = datetime.format(XML_RPC_DATE_FORMAT).to_string();
        serializer.serialize_str(&string)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        match Utc.datetime_from_str(&string, XML_RPC_DATE_FORMAT) {
            Ok(date) => Ok(date),
            Err(error) => Err(error).map_err(serde::de::Error::custom),
        }
    }
}

pub mod boolean {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(boolean: &bool, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let string = match boolean {
            true => "1",
            false => "0",
        };
        serializer.serialize_str(string)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
        where
            D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        match string.as_str() {
            "1" => Ok(true),
            "0" => Ok(false),
            x => Err(serde::de::Error::custom(format!("Unsupported boolean value: {}", x))),
        }
    }
}

pub mod base64 {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let string = base64::encode(bytes);
        serializer.serialize_str(&string)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        match base64::decode(&string) {
            Ok(value) => Ok(value),
            Err(error) => Err(serde::de::Error::custom(error.to_string())),
        }
    }
}
