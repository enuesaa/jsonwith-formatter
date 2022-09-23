use clap::Parser;

use jsonwith_formatter::json::serializer::Serializer;
use std::fs;

#[derive(Parser)]
#[clap(disable_help_flag = true)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    let json_string = read(&args.path);
    let serializer = Serializer::new(&json_string);

    let value = serializer.values.clone();
    for mut i in value {
        println!("{}\t {:?}", i.path.to_string(), i.value);
    }
}

pub fn read(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Err(reason) => panic!("failed to open file {}: {}", filename, reason),
        Ok(file) => file,
    }
}
