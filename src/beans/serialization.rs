use chrono::naive::NaiveDate;
use serde::{Deserialize, Deserializer, Serializer};
use std::str::FromStr;

pub fn decode_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(NaiveDate::from_str(String::deserialize(deserializer)?.as_str()).unwrap())
}

pub fn encode_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(date.to_string().as_str())
}
