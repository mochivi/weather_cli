use crate::input_parsing::input_parsing::Input;

use super::structs::WeatherResponse;
use anyhow::anyhow;
use reqwest;
use std::fs;

pub async fn request_weather_data(input: &Input) -> Result<WeatherResponse, anyhow::Error> {
    let request_url: String = create_request_url(&input);
    let response: reqwest::Response = reqwest::get(request_url).await?;

    if !response.status().is_success() {
        return Err(anyhow!("Request failed with status code: {}", response.status()));
    }

    let weather_response: WeatherResponse = response.json().await?;
    Ok(weather_response)
}

pub fn create_request_url(input: &Input) -> String {
    let api_key: String = read_api_key();
    let req_url: String = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=yes", api_key, &input.city);
    return req_url;
}

fn read_api_key() -> String {
    let contents = fs::read_to_string("api_key.txt")
        .expect(" Api key file could not be read");
    return contents.trim().to_string();
}