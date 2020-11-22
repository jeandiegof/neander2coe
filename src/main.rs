extern crate clap;
use clap::{App, Arg};
use std::fs;
use std::io::prelude::*;

fn main() {
    let matches = App::new("Neander COE Generator")
        .version("0.1")
        .author("Jean Fontena <diegofontena@gmail.com>")
        .about("Generates a COE file from a Neander program.")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Neander program.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Resulting COE file.")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let input_filename = matches.value_of("input").unwrap();
    let output_filename = matches.value_of("output").unwrap();

    let input_file_contents =
        fs::read_to_string(input_filename).expect("Failed to read from file.");

    let instructions = input_file_contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let tokens = line.split(' ').collect::<Vec<&str>>();
            (
                *tokens.get(0).expect("Error parsing file."),
                *tokens.get(1).unwrap_or(&""),
            )
        })
        .collect::<Vec<(&str, &str)>>();

    let mut output_file = fs::File::create(output_filename).expect("Failed to create output file.");
    output_file
        .write_all(b"memory_initialization_radix=16;\n")
        .expect("Failed to write to file.");
    output_file
        .write_all(b"memory_initialization_vector=\n")
        .expect("Failed to write to file.");

    instructions.into_iter().for_each(|instruction| {
        let decoded_instruction = decode_instruction(instruction);
        output_file
            .write_all(decoded_instruction.as_bytes())
            .expect("Failed to write to file.");
        output_file
            .write_all(b",\n")
            .expect("Failed to write to file.");
    });
}

fn decode_instruction((first_value, second_value): (&str, &str)) -> String {
    match (first_value.to_uppercase().as_str(), second_value) {
        ("NOP", _) => "00".to_string(),
        ("STA", address) => format!("10,\n{}", address),
        ("LDA", address) => format!("20,\n{}", address),
        ("ADD", value) => format!("30,\n{}", value),
        ("OR", address) => format!("40,\n{}", address),
        ("AND", address) => format!("50,\n{}", address),
        ("MULT", address) => format!("C0,\n{}", address),
        ("SUB", address) => format!("D0,\n{}", address),
        ("NOT", _) => "60".to_string(),
        ("JMP", address) => format!("80,\n{}", address),
        ("JN", address) => format!("90,\n{}", address),
        ("JZ", address) => format!("A0,\n{}", address),
        ("HLT", _) => "F0".to_string(),
        _ => panic!("Unexpected instruction {:?} {:?}", first_value, second_value),
    }
}
