use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    result::Result,
};

use clap::Parser;

use sha1::Digest;
use sha1::Sha1;

const SHA1_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file_name = args.file;
    let hash = args.hash;

    match brute_force(file_name, hash).unwrap() {
        true => println!("Successful!"),
        false => println!("Unsuccessful"),
    };

    Ok(())
}

pub fn brute_force(file_name: String, hash: String) -> Result<bool, Box<dyn Error>> {
    if file_name.ends_with(".txt") == false {
        return Err(format!("Invalid wordlist, {} doesn't end with .txt", &file_name).into());
    }
    if hash.len() != SHA1_LENGTH {
        return Err("Not a valid 40 digit sha-1 digest".into());
    }

    let wordlist = File::open(&file_name).expect(&format!("error opening {}", &file_name));
    let reader = BufReader::new(&wordlist);

    for line in reader.lines() {
        let line = line.expect("error reading line");
        let word = line.trim();
        match compare_hash(word, &hash) {
            true => {
                println!("Hash cracked. Original word is {}", &word);
                return Ok(true);
            }
            false => {}
        }
    }

    println!("Word not found in {}", file_name);
    Ok(false)
}

fn compare_hash(word: &str, hash: &String) -> bool {
    hash == &hex::encode(Sha1::digest(&word.as_bytes()))
}

/// A sha-1 password cracker
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// File containing list of dictionary words
    #[clap(long, value_parser)]
    pub file: String,

    /// 40-digit digest to be cracked
    #[clap(long, value_parser)]
    pub hash: String,
}

#[cfg(test)]
mod test {
    use super::brute_force;

    const FILE_NAME: &str = "dictionary.txt";

    fn test(hash: &str, expected: bool) {
        let file_name = String::from(FILE_NAME);
        let result = brute_force(file_name, String::from(hash)).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn cracker_works() {
        let hash1 = "1d2f56e6e74d722ac2f6941f29db35b391c83504";
        let hash2 = "a64431388c02ce7fa2ae6a622befa56cf7f21c95";
        let hash3 = "03d67c263c27a453ef65b29e30334727333ccbcd";
        let hash4 = "2f687ad99af02a29d257830e824a7333ed781082";
        let hash5 = "10200aec1d1c507d3cf4d1f4979866aae86f3dd7";

        test(hash1, true);
        test(hash2, true);
        test(hash3, true);
        test(hash4, false);
        test(hash5, false);
    }
}
