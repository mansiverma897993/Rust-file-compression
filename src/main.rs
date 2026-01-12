extern crate flate2;

use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use flate2::write::GzEncoder;
use flate2::Compression;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: <source> <target>");
        return;
    }

    let input = File::open(args().nth(1).unwrap()).unwrap();
    let output = File::create(args().nth(2).unwrap()).unwrap();

    let mut reader = BufReader::new(input);
    let mut encoder = GzEncoder::new(output, Compression::default());

    copy(&mut reader, &mut encoder).unwrap();
    encoder.finish().unwrap();

    println!("Compression completed successfully!");
}
