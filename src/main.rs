mod input_parsing;
mod api;

use tokio;
use crate::input_parsing::input_parsing::{Input, get_matches};
use crate::api::api::request_weather_data;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {

    let user_input: Input = get_matches();
    println!("{:?}", user_input);


    let response = request_weather_data(user_input).await?;
    println!("Response: {:?}", response);
    
    
    Ok(())
}

