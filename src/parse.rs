use std::fs::File;
use std::io::{prelude::*, BufReader};
use chrono::NaiveDateTime;

use crate::model::WeightLine;
use crate::model::TIMESTAMP_FORMAT;

const SEPARATOR :&str = ",";

pub fn parse_file(file :&File) -> Vec<WeightLine> {
    let reader = BufReader::new(file);

    let mut result = Vec::new();
    let mut line_counter = 0;
    for line_result in reader.lines() {
        if line_result.is_err() {
            print!("could not read line: ");
            println!("{}", line_result.unwrap_err());
            continue;
        }
        let line = line_result.unwrap();
        let parse_result = parse_line(&line, &line_counter);
        let line_error = parse_result.1;
        if !line_error {
            result.push(parse_result.0);
        }
        line_counter += 1;
    }
    return result;
}

fn parse_line(line :&String, line_counter:&u32) -> (WeightLine, bool) {
    //println!("{}", line);
    let mut weight_line = WeightLine::new();
    let mut line_error = false;
    let mut index :usize = 0;
    let mut err_msg :&str = "";

    let mut parse_result = parse_timestamp(line, & mut weight_line, &index);
    if parse_result.is_ok() {
        index = parse_result.unwrap();
    } else {
        line_error = true;
        err_msg = parse_result.err().unwrap();
    }

    if !line_error {
        parse_result = parse_float(line, &mut weight_line.weight, &index, "could not parse: weight float");
        //if parse_result.is_ok() {
        //    index = parse_result.unwrap();
        //} else {
        //    index += 1;
        //}
        if parse_result.is_err() {
            // never mind
        }
    }

    if line_error && *line_counter > 2 {
        println!("error parsing line: {}", err_msg);
        print!("    ");
        println!("{}", line);
    }
    return (weight_line, line_error);
}

fn parse_timestamp(line :&String, weight_line :&mut WeightLine, start_index :&usize) -> Result<usize, &'static str> {
    let line_remainer = &line[*start_index +1 .. line.len()]; // +1 for quote
    let find_result = line_remainer.find(SEPARATOR);
    let err_msg = "could not parse: timestamp";
    return if find_result.is_some() {
        let index = find_result.unwrap();
        if index > 0 {
            let timestamp_str: &str = &line_remainer[0 .. index -1];
            let parse_result = NaiveDateTime::parse_from_str(timestamp_str, TIMESTAMP_FORMAT);
            if parse_result.is_ok() {
                weight_line.timestamp = parse_result.unwrap();
                Result::Ok(*start_index +1 + index)
            } else {
                Result::Err(err_msg)
            }
        } else {
            Result::Err(err_msg)
        }
    } else {
        Result::Err(err_msg)
    }
}

fn parse_float(line :&String, target :&mut f32, start_index :&usize, err_msg :&'static str) -> Result<usize, &'static str> {
    let line_remainer = &line[*start_index +1 .. line.len()];
    let find_result = line_remainer.find(SEPARATOR);
    return if find_result.is_some() {
        let index = find_result.unwrap();
        if index > 0 {
            let mut int_str = &line_remainer[0..index];
            let decimal_result = int_str.find(",");
            if decimal_result.is_some() {
                let decimal_index = decimal_result.unwrap();
                if decimal_index > 0 {
                    int_str = &line_remainer[0..decimal_index];
                }
            }
            let parse_result = int_str.parse::<f32>();
            if parse_result.is_ok() {
                *target = parse_result.unwrap();
                Result::Ok(*start_index +1 + index)
            } else {
                Result::Err(err_msg)
            }
        } else {
            Result::Err(err_msg)
        }
    } else {
        Result::Err(err_msg)
    }
}
