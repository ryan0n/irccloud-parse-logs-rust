extern crate zip;
extern crate time;
extern crate regex;

use std::time::Instant;
use regex::RegexBuilder;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 || args[1].is_empty() || args[2].is_empty(){
        println!("Usage: {} <filename> <search phrase>", args[0]);
        std::process::exit(0);
    }
    parse_irccloud_log_file();
}

fn parse_irccloud_log_file() {
    let args: Vec<_> = std::env::args().collect();
    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&fname).unwrap();
    let search_phrase = &args[2].to_lowercase();
    println!("\nzip file: {}\nsearch_phrase: {}\n\n", &args[1], &args[2]);

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        if !(&*file.name()).ends_with('/') {
            let start = Instant::now();

            let mut content = String::new();
            file.read_to_string(&mut content);

            if start.elapsed().as_secs() > 0 {
                println!("read to string: {}", start.elapsed().as_secs());
            }

            let needle = RegexBuilder::new(search_phrase)
                .case_insensitive(true)
                .build()
                .expect("Invalid Regex");

            if needle.is_match(&content) {
                println!("contains was true: {}", start.elapsed().as_secs());
                let mut file_name = String::from(&*file.name());
                //println!("content: {}", &content);
                let mut file_name = String::from(&*file.name());
                let mut split_file_name = file_name.split("/");
                let vec = split_file_name.collect::<Vec<&str>>();
                let network = &*vec[1].to_string();
                let channel = &*vec[2].to_string();

                let start = Instant::now();
                println!("1network: {}\nchannel: {}, search_phrase: {}\n\n", network, channel, search_phrase);
                println!("1search_phrase: {}\n\n", search_phrase);

                let reader = BufReader::new(file);
                for line in reader.lines() {
                    println!("in line looper: {}", start.elapsed().as_secs());

                    let mut rawline: String = line.unwrap();
                    if needle.is_match(&rawline) {
                        println!("2network: {}\nchannel: {}, search_phrase: {}\nraw line: {}\n\n", network, channel, search_phrase, rawline);
                    }
                }
                println!("loop through each line finished: {}", start.elapsed().as_secs());
            } else {
                if start.elapsed().as_secs() > 0 {
                    println!("contains was false: {}", start.elapsed().as_secs());
                }
            }
            if start.elapsed().as_secs() > 0 {
                println!("whole file: {}", start.elapsed().as_secs());
            }
        }
    }
    std::process::exit(0);
}
