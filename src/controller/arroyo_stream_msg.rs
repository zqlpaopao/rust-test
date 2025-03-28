// https://mp.weixin.qq.com/s/KGCW6ly2wZrDr3YdV0nuWw

pub async fn test_arroyo_kafka() {
    make_data_to_kafka().await;
}

/***************************** 制造数据推入kafka  *********************************/
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use serde_derive::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::interval;

#[derive(Deserialize, Serialize, Debug)]
struct WeatherResponse {
    latitude: f64,
    longitude: f64,
    elevation: f64,
    timezone: String,
    timezone_abbreviation: String,
    current_units: CurrentUnits,
    current: Current,
}

#[derive(Deserialize, Serialize, Debug)]
struct WeatherData {
    temperature_2m: f64,
    wind_speed_10m: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct CurrentUnits {
    time: String,
    interval: String,
    temperature_2m: String,
    wind_speed_10m: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct Current {
    time: String,
    interval: u32,
    temperature_2m: f64,
    wind_speed_10m: f64,
}

async fn make_data_to_kafka() {
    // PLAINTEXT://kafka:9092,PLAINTEXT_HOST://localhost:9093
    // 用9093
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", "localhost:9093")
        .create()
        .expect("Producer creation error");

    produce_weather_data(&producer, 40.41, -3.70).await;
}

async fn produce_weather_data(producer: &FutureProducer, lat: f64, lon: f64) {
    let topic = "weather_data";

    let mut interval = interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        match fetcher_weather(lat, lon).await {
            Ok(weather_data) => {
                let data = WeatherData {
                    temperature_2m: weather_data.current.temperature_2m,
                    wind_speed_10m: weather_data.current.wind_speed_10m,
                };

                match serde_json::to_string(&data) {
                    Ok(payload) => {
                        let delivery_status = producer
                            .send(
                                FutureRecord::to(topic)
                                    .payload(&payload)
                                    .key("weather_data_key"),
                                Duration::from_secs(0),
                            )
                            .await;

                        match delivery_status {
                            Ok(delivery) => println!("Sent: {:?}", delivery),
                            Err((e, _)) => println!("Error: {:?}", e),
                        }
                    }
                    Err(err) => {
                        println!("serde json{}", err);
                    }
                }
            }
            Err(err) => {
                println!("error fetching weather data: {}", err);
                break;
            }
        }
    }
}

async fn fetcher_weather(lat: f64, lon: f64) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m",
        lat, lon
    );
    let response = reqwest::get(&url).await?;
    let weather_data = response.json::<WeatherResponse>().await?;
    println!("Weather data: {:?}", weather_data); // Optional: to check the fetched data
    Ok(weather_data)
}
