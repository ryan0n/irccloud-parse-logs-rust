extern crate zip;

use std::fs;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&fname).unwrap();

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

            let mut buffer = String::new();
            file.read_to_string(&mut buffer).expect("could not read file");
            if buffer.contains("donald trump") {
                println!("{}: {}", sanitize_filename(file.name()).as_path().display(), buffer);
            }
        }
    }
    std::process::exit(0);
}

fn sanitize_filename(filename: &str) -> std::path::PathBuf {
    let no_null_filename = match filename.find('\0') {
        Some(index) => &filename[0..index],
        None => filename,
    };

    std::path::Path::new(no_null_filename)
        .components()
        .filter(|component| match *component {
            std::path::Component::Normal(..) => true,
            _ => false,
        })
        .fold(std::path::PathBuf::new(), |mut path, ref cur| {
            path.push(cur.as_os_str());
            path
        })
}
