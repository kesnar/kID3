//id3 algorithm
#[allow(warnings)]

extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use ndarray::{Axis, stack, Array, Array2, ArrayView1};
use ndarray_csv::Array2Reader;

#[derive(Debug)]
enum Tree {
    Leaf(TargetValue),
    Branch {
        label: Attribute, 
        children: Vec<Child>
    }
}

#[derive(Debug)]
struct Child {
    path: AttrValue,
    tree: Tree
}

type TargetValue = String;
type Attribute = String;
type AttrValue = String;

fn readcsv() -> Result<Array2<String>, Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    let array_read: Array2<String> = reader.deserialize_array2_dynamic()?;
    Ok(array_read)
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn checkSingleValue(arr: &ArrayView1<String>) -> (bool, String) {
    let value1 = &arr[0];
    for i in arr.iter() {
        if !value1.eq(i) {
            return (false, "".to_string())
        }
    }
    (true, value1.to_string())
}

fn bestIG(examples: &Array2<String>) -> usize {
    1
}
fn get_subset(examples: Array2<String>, vi: String, A: usize) -> Array2<String> {
    let ret = Array::from_vec(Vec::with_capacity(examples.ncols()));
    for row in examples.outer_iter() {
        if row[A] == vi {
            stack(Axis(0), &[ret.view().clone(), row.view ().clone()]);
            println!("{:?}", row);
        }
        
    }
    ret
}

fn ID3(examples: Array2<String>) -> Tree {
    let (cond, value) = checkSingleValue(&examples.column(examples.ncols()-1));
    let mut ret;
    if cond {
        ret = Tree::Leaf(value);
        return ret
    }

    /*
        if examples is 1D, meaning only target attribute collumn:
        x = bestOfCollumn(Examples(Target_Attribute))
        return ret.makeLeaf(x)
    */

    let best_attribute = bestIG(&examples);
    let mut values = HashSet::<String>::new();
    examples.column(best_attribute).to_vec().retain(|e| values.insert(e.to_string()));

    ret = Tree::Branch{ label: format!("A{}", best_attribute), children: Vec::new() };
    for i in values.iter() {
        if let Tree::Branch{ label: _, children: ref mut kids } = ret {
            let subset = get_subset(examples.to_owned(), i.to_string(), best_attribute);
            if subset.is_empty() {
                /*
                    if examples is 1D, meaning only target attribute collumn:
                    x = bestOfCollumn(Examples(Target_Attribute))
                    return ret.makeLeaf(x)
                */
            }
            else {
                kids.push(Child{ path: i.to_string(), tree: ID3(subset)});
            }
        }
    }
    ret
}

fn main() {

    match readcsv() {
        Err(e) => {
            println!("Error {}", e);
            process::exit(1)
        }
        Ok(array) => {
            //let x = ID3(array);
            //println!("{:?}", x);
            //let y = ID3(array);
            let test = get_subset(array.to_owned(), "rain".to_string(), 1);
            //println!("{:?} {:?}", array, test)
        }
    }
}

