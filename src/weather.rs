use colored::*;
use reqwest::StatusCode;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub wind: Wind,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub pressure: u32,
    pub humidity: u8,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
}

pub fn fetch_weather(city: &str, country_code: &str) -> Result<WeatherResponse, reqwest::Error> {
    let api_key = env::var("API_KEY").expect("API_KEY not found in .env file");

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(url)?.error_for_status()?;
    response.json::<WeatherResponse>()
}

pub fn display_weather(response: &WeatherResponse) {
    let description = &response.weather[0].description;

    let weather_text = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C, 
        > Humidity: {:.1}%, 
        > Pressure: {:.1} hPa, 
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temperature_emoji(response.main.temp),
        response.main.temp,
        response.main.humidity,
        response.main.pressure,
        response.wind.speed,
    );

    let weather_text_colored = get_colored_weather_text(description, &weather_text);
    println!("{}", weather_text_colored);
}

fn get_colored_weather_text(description: &str, weather_text: &str) -> colored::ColoredString {
    match description {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    }
}

fn get_temperature_emoji(temp: f64) -> &'static str {
    match temp {
        t if t < 0.0 => "❄️",
        t if t < 10.0 => "☁️",
        t if t < 20.0 => "⛅",
        t if t < 30.0 => "🌤️",
        _ => "🔥",
    }
}

pub fn handle_error(error: &reqwest::Error) {
    if let Some(status) = error.status() {
        match status {
            StatusCode::NOT_FOUND => eprintln!("The city is not found"),
            StatusCode::UNAUTHORIZED => eprintln!("Invalid API key"),
            _ => eprintln!("Error {}: {}", status, error),
        }
    } else {
        eprintln!("Unknown error: {}", error);
    }
}
