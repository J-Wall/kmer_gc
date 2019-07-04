#[macro_use]
extern crate clap;
use clap::App;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).get_matches();

    let nbins = value_t!(m, "nbins", usize).unwrap();
    let k = value_t!(m, "k", usize).unwrap();
    let mut hist: Vec<Vec<u32>> = vec![vec![0; k + 1]; nbins];

    let file = File::open(m.value_of("INPUT").unwrap())
        .expect("Error reading file");

    let lines = BufReader::new(file).lines();

    for (gc, count) in lines.filter_map(|l| process_line(l.unwrap())) {
        if count < nbins {
            hist[count][gc] +=1;
        }
    }

    for row in hist {
        let line = row.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(m.value_of("sep").unwrap());
        println!("{}", line);
    }
}

fn process_line(l: String) -> Option<(usize, usize)> {
    if l.starts_with("#") {
        return None
    }

    let mut vals = l.split_whitespace();
    let kmer = vals.next().unwrap().to_ascii_uppercase();
    let count: usize = vals.next().unwrap().parse().expect("Invalid file format");
    let gc: usize = kmer.chars().filter(|x| (x == &'C') || (x == &'G')).count();

    Some((gc, count))
}
