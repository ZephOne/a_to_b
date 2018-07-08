extern crate reqwest;
extern crate serde_json;

use std::io;
use serde_json::Value;

pub mod graphhopper;

pub fn run() -> (String, String) {
    let trajet = init();

    let city1_url = format!("https://nominatim.openstreetmap.org/search?city={}&format=json", trajet.city1);
    let city2_url = format!("https://nominatim.openstreetmap.org/search?city={}&format=json", trajet.city2);

    let city1_response_body = get_body(&city1_url).unwrap();
    let city2_response_body = get_body(&city2_url).unwrap();

    let city1_coordinates = get_coordinates(&city1_response_body[..]);
    let city2_coordinates = get_coordinates(&city2_response_body[..]);

    let travel_distance = get_distance(&city1_coordinates, &city2_coordinates);
    let travel_time = get_time(&city1_coordinates, &city2_coordinates);

    (travel_distance, travel_time)
}

pub struct Trajet {
    pub city1: String,
    pub city2: String,
}

pub struct Coordinates {
    pub lat: String,
    pub lon: String,
}

pub fn init() -> Trajet {
    let mut city1 = String::from("");
    let mut city2 = String::from("");

    println!("Give a first city: ");
    io::stdin().read_line(&mut city1).expect("Can't read first city");
    city1.pop();
    println!("Give a second city: ");
    io::stdin().read_line(&mut city2).expect("Can't read first city");
    city2.pop();

    Trajet {city1, city2}
}

pub fn get_body(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url)?.text()?;

    Ok(body)
}

pub fn get_coordinates(json: &str) -> Coordinates {
    let v: Value = serde_json::from_str(&json[..]).unwrap();
    let u = &v[0];
    let lat = String::from(u["lat"].as_str().expect("Can't get latitude"));
    let lon = String::from(u["lon"].as_str().expect("Can't get longitude"));
    Coordinates {lat, lon}
}

pub fn get_distance<'a>(coord1: &'a Coordinates, coord2: &'a Coordinates) -> String {
    let api_key = graphhopper::get_api_key();
    // Api de GraphHopper
    let base_url = format!("https://graphhopper.com/api/1/route?point={},{}&point={},{}&vehicle=car&locale=fr&key={}",&coord1.lat, &coord1.lon, &coord2.lat, &coord2.lon, api_key);

    let graphhopper_response = get_body(&base_url[..]).unwrap();
    let v: Value = serde_json::from_str(&graphhopper_response[..]).unwrap();

    let u = &v["paths"][0];
    let distance = &u["distance"];

    distance.to_string()
}

pub fn get_time<'a>(coord1: &'a Coordinates, coord2: &'a Coordinates) -> String {
    let api_key = graphhopper::get_api_key();
    // Api de GraphHopper
    let base_url = format!("https://graphhopper.com/api/1/route?point={},{}&point={},{}&vehicle=car&locale=fr&key={}",&coord1.lat, &coord1.lon, &coord2.lat, &coord2.lon, api_key);

    let graphhopper_response = get_body(&base_url[..]).unwrap();
    let v: Value = serde_json::from_str(&graphhopper_response[..]).unwrap();

    let u = &v["paths"][0];
    let time = &u["time"];

    time.to_string()
}

