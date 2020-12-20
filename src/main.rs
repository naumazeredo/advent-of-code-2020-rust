use std::fs::File;
use std::io::{self, Read};
use std::vec::Vec;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() != 3 { panic!("Tell me the day!"); }

    let mut filename = String::from("./data/day");
    filename.push_str(args[1].as_str());

    println!("filename: {}", filename);
    let file = File::open(filename).unwrap();

    let mut buffer = String::new();
    io::BufReader::new(file).read_to_string(&mut buffer).unwrap();

    match args[1].as_str() {
        "1" => day1::run(args[2].as_str(), buffer),
        "2" => day2::run(args[2].as_str(), buffer),
        "3" => day3::run(args[2].as_str(), buffer),
        "4" => day4::run(args[2].as_str(), buffer),
        "5" => day5::run(args[2].as_str(), buffer),
        "6" => day6::run(args[2].as_str(), buffer),
        _ => (),
    }
}
