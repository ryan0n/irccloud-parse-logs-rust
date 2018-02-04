extern crate zip;

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
    let search_phrase = &args[2];
    println!("\nzip file: {}\nsearch_phrase: {}\n\n", &args[1], &args[2]);

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }

        }

        if !(&*file.name()).ends_with('/') {
            let mut file_name = String::from(&*file.name());
            let mut split_file_name = file_name.split("/");
            let vec = split_file_name.collect::<Vec<&str>>();
            let network = &*vec[1].to_string();
            let channel = &*vec[2].to_string();

            let reader = BufReader::new(file);
            for line in reader.lines() {
                let mut rawline: String = line.unwrap();
                if rawline.contains(search_phrase) {
                        println!("network: {}\nchannel: {}\nraw line: {}\n\n", network, channel, rawline);
                }
            }
        }
    }
    std::process::exit(0);
}
