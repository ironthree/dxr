pub(crate) mod datetime {
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::values::XML_RPC_DATE_FORMAT;

    pub(crate) fn from_str(s: &str) -> Result<NaiveDateTime, String> {
        match NaiveDateTime::parse_from_str(s, XML_RPC_DATE_FORMAT) {
            Ok(date) => Ok(date),
            Err(error) => Err(format!("Invalid date format: {}", error)),
        }
    }

    pub(crate) fn serialize<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = datetime.format(XML_RPC_DATE_FORMAT).to_string();
        serializer.serialize_str(&string)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        from_str(&string).map_err(serde::de::Error::custom)
    }
}

pub(crate) mod boolean {
    use serde::{Deserialize, Deserializer, Serializer};

    pub(crate) fn from_str(s: &str) -> Result<bool, String> {
        match s {
            "1" => Ok(true),
            "0" => Ok(false),
            _ => Err(format!("Unsupported boolean value: {}", s)),
        }
    }

    pub(crate) fn serialize<S>(boolean: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = match boolean {
            true => "1",
            false => "0",
        };
        serializer.serialize_str(string)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        from_str(&string).map_err(serde::de::Error::custom)
    }
}

pub(crate) mod base64 {
    use serde::{Deserialize, Deserializer, Serializer};

    pub(crate) fn from_str(s: &str) -> Result<Vec<u8>, base64::DecodeError> {
        // filter out optional whitespace from input string:
        // some XML-RPC implementations line-wrap base64 encoded strings
        let stripped: String = s.chars().filter(|c| !c.is_ascii_whitespace()).collect();
        crate::base64::decode(stripped)
    }

    pub(crate) fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = crate::base64::encode(bytes);
        serializer.serialize_str(&string)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        from_str(&string).map_err(serde::de::Error::custom)
    }
}

/// This mod contains the deserialization logic for the XML-RPC value types.
///
/// This manual deserialization is necessary to support scalar string values
/// without a `<type>` element, e.g. `<value>foo</value>` instead of
/// `<value><string>foo</string></value>` (which are both valid XML-RPC).
pub(crate) mod value {
    #[cfg(feature = "nil")]
    use serde::de::IgnoredAny;
    use serde::{
        de::{self, Deserializer, Visitor},
        Deserialize,
    };

    use std::fmt;

    use crate::values::Value;

    struct ValueVisitor {}

    impl<'de> Visitor<'de> for ValueVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid XML-RPC scalar value")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            const FIELDS: &[&str] = &[
                "i4",
                "int",
                #[cfg(feature = "i8")]
                "i8",
                "boolean",
                "string",
                "double",
                "dateTime.iso8601",
                "base64",
                "struct",
                "array",
                #[cfg(feature = "nil")]
                "nil",
            ];

            #[derive(Debug)]
            enum Field {
                I4,
                #[cfg(feature = "i8")]
                I8,
                Boolean,
                String,
                Double,
                DateTime,
                Base64,
                Struct,
                Array,
                #[cfg(feature = "nil")]
                Nil,
            }

            impl<'de> Deserialize<'de> for Field {
                fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    struct FieldVisitor;

                    impl Visitor<'_> for FieldVisitor {
                        type Value = Field;

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str("field should be called `value`")
                        }

                        fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: de::Error,
                        {
                            match value {
                                "i4" => Ok(Field::I4),
                                "int" => Ok(Field::I4),
                                #[cfg(feature = "i8")]
                                "i8" => Ok(Field::I8),
                                "boolean" => Ok(Field::Boolean),
                                "string" => Ok(Field::String),
                                "double" => Ok(Field::Double),
                                "dateTime.iso8601" => Ok(Field::DateTime),
                                "base64" => Ok(Field::Base64),
                                "struct" => Ok(Field::Struct),
                                "array" => Ok(Field::Array),
                                #[cfg(feature = "nil")]
                                "nil" => Ok(Field::Nil),
                                "$value" => Ok(Field::String),
                                "$text" => Ok(Field::String),
                                _ => Err(de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }

                    deserializer.deserialize_identifier(FieldVisitor)
                }
            }

            let value = if let Some(key) = map.next_key::<Field>()? {
                match key {
                    Field::I4 => {
                        let value = map.next_value()?;
                        Ok(Value::i4(value))
                    },
                    #[cfg(feature = "i8")]
                    Field::I8 => {
                        let value = map.next_value()?;
                        Ok(Value::i8(value))
                    },
                    Field::Boolean => {
                        let string: String = map.next_value()?;
                        super::boolean::from_str(&string)
                            .map(Value::boolean)
                            .map_err(de::Error::custom)
                    },
                    Field::String => {
                        let value: String = map.next_value()?;
                        Ok(Value::string(value))
                    },
                    Field::Double => {
                        let value = map.next_value()?;
                        Ok(Value::double(value))
                    },
                    Field::DateTime => {
                        let string: String = map.next_value()?;
                        super::datetime::from_str(&string)
                            .map(Value::datetime)
                            .map_err(de::Error::custom)
                    },
                    Field::Base64 => {
                        let string: String = map.next_value()?;
                        super::base64::from_str(&string)
                            .map(Value::base64)
                            .map_err(de::Error::custom)
                    },
                    Field::Struct => {
                        let value = map.next_value()?;
                        Ok(Value::structure(value))
                    },
                    Field::Array => {
                        let value = map.next_value()?;
                        Ok(Value::array(value))
                    },
                    #[cfg(feature = "nil")]
                    Field::Nil => {
                        let _: IgnoredAny = map.next_value()?;
                        Ok(Value::nil())
                    },
                }
            } else {
                // <value></value>
                return Ok(Value::string(String::new()));
            };

            if let Some(key) = map.next_key::<Field>()? {
                Err(de::Error::custom(format!("Unexpected key: {:?}", key)))
            } else {
                value
            }
        }
    }

    impl<'de> Deserialize<'de> for Value {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(ValueVisitor {})
        }
    }
}
