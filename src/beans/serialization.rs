use crate::beans::bean::BeanRoast;
use chrono::naive::NaiveDate;
use chrono::offset::Local;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

pub fn decode_roast<'de, D>(deserializer: D) -> Result<BeanRoast, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    println!("{:?}", s);
    Ok(BeanRoast::Light(0))
}

pub fn encode_roast<S>(roast: &BeanRoast, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let str = match roast {
        BeanRoast::Light(t) => format!("light-roast temp={}", t),
        BeanRoast::Medium(t) => format!("medium-roast temp={}", t),
        BeanRoast::Dark(t) => format!("dark-roast temp={}", t),
    };
    serializer.serialize_str(str.as_str())
}

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
