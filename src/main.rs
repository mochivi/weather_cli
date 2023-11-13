mod input_parsing;
mod api;

use api::structs::WeatherResponse;
use tokio;
use crate::input_parsing::input_parsing::{Input, get_matches};
use crate::api::api::request_weather_data;

fn simple_data_display(weather_response: &WeatherResponse) {
    weather_response.location.simple_display();  
    weather_response.current.simple_display();
}    

fn detailed_data_display(weather_response: &WeatherResponse) {
    weather_response.location.detailed_display();
    weather_response.current.detailed_display();
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {

    let user_input: Input = get_matches();

    let response = request_weather_data(&user_input).await?;

    // Simple or detailed data display
    if !user_input.detailed {
        simple_data_display(&response);
    } else {
        detailed_data_display(&response);
    }

    Ok(())
}

