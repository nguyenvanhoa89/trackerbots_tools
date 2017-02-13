#![feature(question_mark)]

extern crate byteorder;
#[macro_use] extern crate clap;
extern crate iq_converter;

use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use clap::{Arg, App};
use iq_converter::IqConverter;

arg_enum!{
    #[derive(Debug)]
    enum Encoding {
        LittleEndian,
        BigEndian,
        Text
    }
}

fn main() {
    let matches =
        App::new("HackRF: I/Q Converter")
            .version("0.1")
            .author("Michael Chesser")
            .about("Converts raw I/Q samples from the HackRF to complex numbers")
            .arg(Arg::with_name("input")
                .help("The name of the input file")
                .required(true)
                .index(1))
            .arg(Arg::with_name("output")
                .help("The name of the output file")
                .required(true)
                .index(2))
            .arg(Arg::with_name("encoding")
                .help("The encoding to use for the output file.")
                .short("e")
                .long("encoding")
                .possible_values(&Encoding::variants())
                .default_value("LittleEndian")
                .takes_value(true))
            .get_matches();

    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let encoding = value_t!(matches.value_of("encoding"), Encoding).unwrap();

    if let Err(e) = run(input, output, encoding) {
        println!("{}", e);
    }
}

fn run(input_filename: &str, output_filename: &str, encoding: Encoding) -> io::Result<()> {
    let mut input = File::open(input_filename)?;
    let mut data = vec![];
    input.read_to_end(&mut data)?;

    let mut output = BufWriter::new(File::create(output_filename)?);
    for (i, q) in IqConverter::new(data) {
        match encoding {
            Encoding::LittleEndian => {
                output.write_f32::<LittleEndian>(i)?;
                output.write_f32::<LittleEndian>(q)?;
            }

            Encoding::BigEndian => {
                output.write_f32::<BigEndian>(i)?;
                output.write_f32::<BigEndian>(q)?;
            }

            Encoding::Text => {
                writeln!(output, "{},{}", i, q)?;
            }
        }
    }

    Ok(())
}
