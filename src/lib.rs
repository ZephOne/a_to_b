use std::io;

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
