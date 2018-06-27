extern crate reqwest;
extern crate serde_json;

use std::io;
use serde_json::{Value, Error};

pub struct Trajet {
    pub start_city: String,
    pub end_city: String,
    duration: u32,
    distance: u32,
}

impl Trajet {
    pub fn new() -> Trajet {
        println!("Give a first city");
        let mut start_city = String::new();
        io::stdin().read_line(&mut start_city).expect("Fail to read input");

        println!("Give a second city");
        let mut end_city = String::new();
        io::stdin().read_line(&mut end_city).expect("Fail to read input");

        Trajet{start_city: start_city, end_city: end_city, duration: 0, distance: 0}
    }
}

pub struct Coordinates {
    pub lat: String,
    pub lon: String,
}

pub fn get_body(url: &str) -> String {
    let res = reqwest::get(url);
    let mut res = match res {
        Ok(response) => response,
        Err(e) => {
            panic!("There was an error making an HTTP GET: {}", e)
        }
    };

    let body = res.text();
    let body = match body {
        Ok(content) => content,
        Err(error) => {
            panic!("There was a problem getting the HTTP GET content")
        },
    };

    body
}

pub fn get_coordinates(json: &str) -> Coordinates {
    let v: Value = serde_json::from_str(&json[..]).unwrap();
    let u = &v[0];
    let lat = &u["lat"];
    let lon = &u["lon"];
    Coordinates {lat: lat.to_string(), lon: lon.to_string(),}
}
