use std::{env, io, fs};
use std::process::exit;
use sha2::{Sha256, Digest};
use md5::Md5;


/// USAGE: summer [hash type] [filename] [expected hash]
fn main() {
    let arg: Vec<String> = env::args().collect();
    if arg[1].to_lowercase().as_str() == "help" { println!("USAGE: summer [algorithm] [file] [expected hash]"); exit(0) }
    let mut infile = fs::File::open(arg[2].as_str()).unwrap();
    let filehash: String;

    match arg[1].to_lowercase().as_str() {
        "sha256" => { let mut hasher = Sha256::new();
            let _bytes_written = io::copy(&mut infile, &mut hasher).unwrap();
                // Reads from the file for hashing without loading the entire file into RAM
            filehash = format!("{:X}",hasher.finalize()).to_lowercase(); },
        "md5" => { use md5::Digest;
            let mut hasher = Md5::new();
            let _bytes_written = io::copy(&mut infile, &mut hasher).unwrap();
            filehash = format!("{:X}",hasher.finalize()).to_lowercase(); },
        _ => { println!("Error: {} is not a supported algorithm...", arg[1]); exit(2); },
    }
    
    if arg[3].to_lowercase() == filehash {
        println!("The hash of {} matches!", arg[2]);
        exit(0);
    } else {
        println!(
            "The hash of {} does NOT match with the provided hash.\nSide-By-Side:\nOutput: {}\nGiven:  {}",
            arg[2], filehash, arg[3].to_lowercase()
        );
        exit(1);
    }
}
