//id3 algorithm

extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

enum Tree {
    Leaf(TargetValue),
    Branch {
        label: Attribute, 
        children: Vec<Child>
    }
}

struct Child {
    path: AttrValue,
    tree: Tree
}

type TargetValue = String;
type Attribute = String;
type AttrValue = String;

fn readcsv(file_path: OsString) -> Result<csv::Reader<File>, Box<Error>> {
    let file = File::open(file_path)?;
    Ok(csv::ReaderBuilder::new().has_headers(false).from_reader(file))
}

fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {

    match get_first_arg() {
        Err(_) => {
            println!("Error on arguments");
            process::exit(1)
        },
        Ok(file) => {
            match readcsv(file) {
                Err(_) => {
                    println!("Error on reading file");
                    process::exit(1);
                },
                Ok(ref mut rdr) => {
                    for result in rdr.records() {
                        if let Ok(record) = result {
                            println!("{:?}", record);
                            for i in record.iter() {
                                println!("{:?}", i);
                            }
                        }
                        else {
                            println!("Error on format");
                            process::exit(1)
                        }
                    }
                }
            }
        }
    }

}