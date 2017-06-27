//! This module contains custom deserialization for non iso8601 dates given by random.org.
//! If there is no incorrect date - it will use default deserialization of `chrono` crate.
 
use std::fmt;

use serde::de;
use chrono::NaiveDateTime;


struct NaiveDateTimeFromMyFormatVisitor;

pub fn deserialize<'de, D>(d: D) -> Result<NaiveDateTime, D::Error>
    where D: de::Deserializer<'de>
{
    Ok(d.deserialize_str(NaiveDateTimeFromMyFormatVisitor)?)
}

impl<'de> de::Visitor<'de> for NaiveDateTimeFromMyFormatVisitor {
    type Value = NaiveDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a datetime string")
    }

    fn visit_str<E>(self, value: &str) -> Result<NaiveDateTime, E>
        where E: de::Error {

        match NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%SZ") {
            Ok(d) => Ok(d),
            Err(_) => NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%SZ")
                                    .map_err(|e| E::custom(format!("Parse error {} for {}",
                                                                   e,
                                                                   value))),
        }
    }
}
