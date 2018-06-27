extern crate a_to_b;

use a_to_b::{get_body, get_coordinates};

fn main() {

    let city1_response_body = a_to_b::get_body("https://nominatim.openstreetmap.org/search?city=proussy&format=json");
    println!("{}", city1_response_body);

    let city1_coordinates = a_to_b::get_coordinates(&city1_response_body[..]);

    let lat = &city1_coordinates.lat;
    let lon = &city1_coordinates.lon;

    println!("La latitude de proussy est {}", lat);
    println!("La longitude de proussy est {}", lon);
}
