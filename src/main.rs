use std::env;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let cookie = read_cookie();
    let year = "2021";
    let day = "1";
    let input = get_input(year, day, &cookie);
    println!("{}", input);
}

fn read_cookie() -> String {
    return fs::read_to_string("cookie.txt").expect("Failed to read cookie.txt");
}

fn get_input(year:&str, day:&str, cookie:&str) -> String {
    let input_path = get_input_path(year, day);
    println!("{}", input_path.display());
    match fs::read_to_string(&input_path) {
        Err(_reason) => return fetch_input_from_site(year, day, &input_path, cookie),
        Ok(value) => return value,
    };
}

fn get_input_path(year:&str, day:&str) -> PathBuf {
    let mut path = env::current_dir().expect("Couldn't read current dir."); 
    path.push("input");
    let mut yearday = String::from(year);
    yearday.push_str("_");
    yearday.push_str(day);
    path.push(yearday);
    path.set_extension("txt");
    path
}

fn fetch_input_from_site(year:&str, day:&str, inputPath:&PathBuf, cookie:&str) -> String {
    let mut url = String::from("https://www.adventofcode.com/");
    url.push_str(year);
    url.push_str("/day/");
    url.push_str(day);
    url.push_str("/input");

    /*let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::SET, cookie.parse().unwrap());
    let client;
    match reqwest::blocking::Client::builder().cookie_store(true).default_headers(headers).build() {
        Err(_) => panic!(),
        Ok(value) => client = value,
    }*/

    let client = reqwest::blocking::Client::new();
    let response;
    match client.get(&url).header("Cookie", cookie).send() {
        Err(reason) => panic!("{}", reason),
        Ok(value) => response = value.text(),
    }
    match response {
        Err(reason) => panic!("{}", reason),
        Ok(value) => value
    }
}