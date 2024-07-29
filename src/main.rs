use std::env;
use std::path::Path;
use std::process::exit;

/// USAGE: summer [hash type] [filename] [expected hash]
fn main() {
    let arg: Vec<String> = env::args().collect();
    let filehash: String;

    match arg[1].to_lowercase().as_str() {
        "sha256" => {
            let infile = Path::new(arg[2].as_str());
            filehash = sha256::try_digest(infile).unwrap(); },
        _ => { println!("Error: {} is not a supported algorithm...", arg[1]); exit(2); },
    }
    
    if arg[3].to_lowercase() == filehash {
        println!("The hash of {} matches!", arg[1]);
        exit(0);
    } else {
        println!(
            "The hash of {} does NOT match with the provided hash.\nSide-By-Side:\nOutput: {}\nGiven:  {}",
            arg[2], filehash, arg[3].to_lowercase()
        );
        exit(1);
    }
}
