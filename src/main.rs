extern crate zip;
extern crate time;
extern crate regex;

use regex::RegexBuilder;
use std::fs;
use std::io::prelude::*;

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
    println!("\nzip file: {}\nsearch phrase: {}\n\n", &args[1], &args[2]);

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        if !(&*file.name()).ends_with('/') {

            let mut content = String::new();
            file.read_to_string(&mut content)
                .expect("Could not read file");

            let needle = RegexBuilder::new(search_phrase)
                .case_insensitive(true)
                .build()
                .expect("Invalid Regex");

            if needle.is_match(&content) {
                let mut file_name = String::from(&*file.name());
                let mut split_file_name = file_name.split("/");
                let vec = split_file_name.collect::<Vec<&str>>();
                let network = &*vec[1].to_string();
                let channel = &*vec[2].to_string();

                for line in content.lines() {
                    if needle.is_match(&line) {
                        println!("network: {}\nchannel: {}\nsearch phrase: {}\nraw line: {}\n\n", network, channel, search_phrase, line);
                    }
                }
            }
        }
    }
    std::process::exit(0);
}
