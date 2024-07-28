use std::env;
use std::path::Path;
use std::process::exit;

/// USAGE: summer [hash type] [filename] [expected hash]
fn main() {
    let arg: Vec<String> = env::args().collect();
    let filehash: String;

    if arg[1] == "sha256".to_string() {
        let infile = Path::new(arg[2].as_str());
        filehash = sha256::try_digest(infile).unwrap();
    } else { println!("Something went wrong..."); exit(2) }
    
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
