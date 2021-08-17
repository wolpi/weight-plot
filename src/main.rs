mod model;
mod parse;
mod plot;

use crate::model::WeightLine;
use crate::model::PlotType;

use std::env;
use std::fs::File;
use chrono::Datelike;
use std::ops::Add;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("which file to open ???");
        return;
    }
    let path = &args[1];
    let file_result = File::open(&path);
    if file_result.is_err() {
        println!("could not open file: '{}'!!!", &path);
        println!("{}", file_result.unwrap_err());
        return;
    }
    let file = file_result.unwrap();
    let mut data :Vec<WeightLine> = parse::parse_file(&file);
    data.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    //for line in data {  println!("time: {}, weight: {}", line.timestamp, line.weight); }

    run_with_lifetime(&data);
}

fn run_with_lifetime<'d>(data :&'d Vec<WeightLine>) {
    let mut data_per_year :HashMap<i32, Vec<&'d WeightLine>> = HashMap::new();
    let mut data_per_year_and_month :HashMap<i32, HashMap<u32, Vec<&'d WeightLine>>> = HashMap::new();
    fill_maps(&data, &mut data_per_year, &mut data_per_year_and_month);

    plot_wrapper_complete(&data);
    plot_wrapper_year(&data_per_year);
    plot_wrapper_month(&data_per_year_and_month);
}

fn fill_maps<'d>(
        data :&'d Vec<WeightLine>,
        data_per_year :&mut HashMap<i32, Vec<&'d WeightLine>>,
        data_per_year_and_month :&mut HashMap<i32, HashMap<u32, Vec<&'d WeightLine>>>) {
    for line in data {
        let year = line.timestamp.year();
        let month = line.timestamp.month();

        let data_of_this_year :&mut Vec<&'d WeightLine>;
        if !data_per_year.contains_key(&year) {
            let tmp = Vec::new();
            data_per_year.insert(year, tmp);
        }
        data_of_this_year = data_per_year.get_mut(&year).unwrap();

        let data_of_this_year_and_month :&mut HashMap<u32, Vec<&'d WeightLine>>;
        if !data_per_year_and_month.contains_key(&year) {
            let tmp= HashMap::new();
            data_per_year_and_month.insert(year, tmp);
        }
        data_of_this_year_and_month = data_per_year_and_month.get_mut(&year).unwrap();

        let data_of_this_month :&mut Vec<&'d WeightLine>;
        if !data_of_this_year_and_month.contains_key(&month) {
            let tmp = Vec::new();
            data_of_this_year_and_month.insert(month, tmp);
        }
        data_of_this_month = data_of_this_year_and_month.get_mut(&month).unwrap();

        data_of_this_year.push(line);
        data_of_this_month.push(line);
    }
}

fn plot_wrapper_complete(data :&Vec<WeightLine>) {
    let title = "complete";
    let mut data_refs = Vec::new();
    for line in data {
        data_refs.push(line);
    }
    plot_wrapper(&title, &data_refs, PlotType::FULL);
}

fn plot_wrapper_year(data_per_year :&HashMap<i32, Vec<&WeightLine>>) {
    let mut keys :Vec<&i32> = data_per_year.keys().collect();
    keys.sort();
    for year in keys {
        let data= data_per_year.get(&year).unwrap();
        let title = year.to_string() + "_full";
        plot_wrapper(&title, data, PlotType::YEAR);
    }
}

fn plot_wrapper_month(data_per_year_and_month :&HashMap<i32, HashMap<u32, Vec<&WeightLine>>>) {
    let mut keys_year :Vec<&i32> = data_per_year_and_month.keys().collect();
    keys_year.sort();
    for year in keys_year {
        let data_per_month = data_per_year_and_month.get(&year).unwrap();
        let mut keys_month :Vec<&u32> = data_per_month.keys().collect();
        keys_month.sort();
        for month in keys_month {
            let data = data_per_month.get(&month).unwrap();
            let title = year.to_string() + "_" + (if *month < 10 { "0" } else { "" }) + month.to_string().as_str();
            plot_wrapper(&title, data, PlotType::MONTH);
        }
    }
}

fn plot_wrapper<'d>(title :&str, data :&Vec<&'d WeightLine>, plot_type :PlotType) {
    let path = build_path(&title);
    println!("creating file {}", path);
    let plot_result = plot::plot(data, path.as_str(), title, plot_type);
    if plot_result.is_err() {
        println!("error creating plot!!!");
        println!("{}", plot_result.err().unwrap());
    }
}

fn build_path(title :&str) -> String {
    let path = String::from(title);
    path.add(".png")
}
