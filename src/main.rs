extern crate a_to_b;
extern crate curl;

use a_to_b::Trajet;
use curl::easy::Easy;
use std::io::{ stdout, Write };

fn main() {
    println!("Welcome to A to B, the distance giver program!\n");

    let trajet = Trajet::new();

    let mut start_city = trajet.start_city.to_lowercase();
    let mut end_city = trajet.end_city.to_lowercase();

    // let mut city_url = format!("https://nominatim.openstreetmap.org/search?q={}&format=json", &start_city);
    
    let start_city_url = "https://nominatim.openstreetmap.org/search?q=proussy&format=json";
    let mut easy_start_city = Easy::new();
    let user_agent = "AToB";
    easy_start_city.useragent(&user_agent);
    easy_start_city.url(&start_city_url).unwrap();
    easy_start_city.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy_start_city.perform().unwrap();
    let json_start_city = easy_start_city.response_code().unwrap();

    println!("{}", json_start_city);

    let end_city_url = "https://nominatim.openstreetmap.org/search?q=toulouse&format=json";
    let mut easy_end_city = Easy::new();
    let user_agent = "AToB";
    easy_end_city.useragent(&user_agent);
    easy_end_city.url(&end_city_url).unwrap();
    easy_end_city.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy_end_city.perform().unwrap();
    let json_end_city = easy_end_city.response_code().unwrap();

    println!("{}", json_end_city);
}
