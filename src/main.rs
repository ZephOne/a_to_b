extern crate reqwest;

fn get_json(url: &str) -> String {
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

fn main() {

    let city1_json = get_json("https://nominatim.openstreetmap.org/search?city=proussy&format=json");

    println!("{}", city1_json);
}
