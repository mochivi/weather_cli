use clap::{Arg, Command};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Input {
    pub city: String,
    pub detailed: bool,
    pub forecast: bool
}

pub fn get_matches() -> Input {
    let matches = Command::new("Weather CLI")
    .author("Victor H. Mochi")
        .version("0.1")
        .arg(Arg::new("city")
            .index(1)
            .help("Input a city to get the weather")
            .required(true)
        )
        .arg(Arg::new("detailed")
            .short('d')
            .long("detailed")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("forecast")
            .short('f')
            .long("forecast")
            .action(clap::ArgAction::SetTrue)
        )
    .get_matches();
    
    let city = matches.get_one::<String>("city").expect("city is required");
    let detailed = matches.get_one::<bool>("detailed").unwrap();
    let forecast = matches.get_one::<bool>("forecast").unwrap();

    Input {
        city: city.clone(),
        detailed: *detailed,
        forecast: *forecast
    }
}