use std::num::ParseFloatError;

use serde::de::Visitor;
use serde_derive::Deserialize;

fn get_time_from_string(value: String) -> Result<f32, ParseFloatError> {
    let split: Vec<Result<f32, ParseFloatError>> = value.split(":").map(|x| x.parse::<f32>()).collect();
    let hour = split[0].clone()?;
    let minute = split[1].clone()?;
    let second = split[2].clone()?;
    Ok(hour + (minute/60.) + (second/3600.))
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = ApiTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string as 00:00:00")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        let val = get_time_from_string(String::from(v)).map_err(|op| E::custom(op.to_string()))?;
        Ok(ApiTime{time: val})
    }
}

#[derive(Debug)]
pub struct ApiTime {
    pub time: f32
}

impl<'de> serde::Deserialize<'de> for ApiTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_str(StringVisitor)
    }
}

#[derive(Deserialize, Debug)]
pub struct ApiTimeResults {
    pub first_light: ApiTime,
    pub sunrise: ApiTime,
    pub sunset: ApiTime,
    pub last_light: ApiTime,
}

#[derive(Deserialize, Debug)]
pub struct ApiData {
    pub results: ApiTimeResults
}

pub fn get_api_data(lat: f32, lon: f32) -> Result<ApiData, Box<dyn std::error::Error>> {
    let request_url = format!("https://api.sunrisesunset.io/json?lat={}&lng={}&time_format=24", lat, lon);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(request_url)
        .send()?;
    let res: ApiData = response.json()?;
    Ok(res)
}
