extern crate a_to_b;

use a_to_b::run;

fn main() {
    let a_to_b: (String, String)= run(); 

    let dist = a_to_b.0.parse::<f32>().unwrap() / 1000.;
    let time = a_to_b.1.parse::<u32>().unwrap() / 1000;

    let hours = &time/3600;
    let minutes = (&time%3600)/60;

    println!("Le trajet se fait en {} km", dist);
    println!("Le trajet se fait en {}h{}", hours, minutes);
}
