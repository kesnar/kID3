//id3 algorithm
//#[allow(warnings)]

extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
use rand::Rng;

use ndarray::{Array2, ArrayView1};
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
type Attribute = usize;
type AttrValue = String;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn readcsv() -> Result<Array2<AttrValue>, Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    let array_read: Array2<AttrValue> = reader.deserialize_array2_dynamic()?;
    Ok(array_read)
}

fn check_single_value(arr: &ArrayView1<AttrValue>) -> (bool, String) {
    let value1 = &arr[0];
    for i in arr.iter() {
        if !value1.eq(i) {
            return (false, "".to_string())
        }
    }
    (true, value1.to_string())
}

/*
//random
fn best_attribute(examples: &Array2<AttrValue>) -> usize {
    if examples.ncols() == 1 {
        0
    }else {
        rand::thread_rng().gen_range(0, examples.ncols()-1)
    }
}*/

fn entropy(array: ArrayView1<AttrValue>) -> f64 {
    let mut values = HashSet::<String>::new();
    array.to_vec().retain(|e| values.insert(e.to_string()));
    
    let mut count= Vec::<f64>::new();
    for i in values.iter() {
        count.push(array.iter().filter(|&n| *n == i.to_string()).count() as f64)
    }
    println!("{:?}", count);

    let mut ret = 0.0;
    let total = array.len() as f64;
    for i in count {
        ret -= i / total * (i/total).log2();
    }
    println!("{:?}", ret);
    ret
}

fn best_attribute(examples: &Array2<AttrValue>) -> usize {    
    let target_S = entropy(examples.column(examples.ncols()-1));
    0
}

fn get_subset(examples: Array2<AttrValue>, vi: String, a: usize) -> Array2<AttrValue> {
    let mut helper = vec![];
    let mut i = 0;
    for row in examples.outer_iter() {
        if row[a] == vi {
            i+=1;
            let mut remove = 0;
            for e in row.iter() {
                if remove != a {
                    helper.push(e.to_string());
                }
                remove += 1;
            }
        }
    }

    //UNSAFE! TO CHANGE!
    if let Ok(ret) = Array2::from_shape_vec((i, examples.ncols()-1), helper) {
        ret
    }
    else {
        Array2::from(vec![[]])
    }
}

fn id3(examples: Array2<AttrValue>, mut attributes: Vec<usize>) -> Tree {
    let mut ret;
    let (cond, value) = check_single_value(&examples.column(examples.ncols()-1));
    if cond {
        ret = Tree::Leaf(value);
        return ret
    }

    /*
        if examples is 1D, meaning only target attribute collumn:
        x = bestOfCollumn(Examples(Target_Attribute))
        return ret.makeLeaf(x)
    
    if attributes.len() == 0 {
        return Tree::Leaf("WHAT?".to_string())
    }*/

    let best = best_attribute(&examples);
    /*for i in attributes.iter() {
        println!("before {}", i)
    }*/

    
    //create values set
    let mut values = HashSet::<String>::new();
    //populate values set
    examples.column(best).to_vec().retain(|e| values.insert(e.to_string()));

    ret = Tree::Branch{ label: attributes[best], children: Vec::new() };
    
    attributes.remove(best);
    /*for i in attributes.iter() {
        println!("after {}", i)
    }*/

    for i in values.iter() {
        if let Tree::Branch{ label: _, children: ref mut kids } = ret {
            let subset = get_subset(examples.to_owned(), i.to_string(), best);
            //println!("{:?}", subset);
            if subset.is_empty() {
                /*
                    if examples is 1D, meaning only target attribute collumn:
                    x = bestOfCollumn(Examples(Target_Attribute))
                    return ret.makeLeaf(x)
                */
            }
            else {
                kids.push(Child{ path: i.to_string(), tree: id3(subset, attributes.clone()) });
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
            let mut attributes = Vec::<usize>::new();
            for i in 0..array.ncols()-1 {
                attributes.push(i);
            }
            let x = id3(array, attributes);
            //println!("{:#?}", x);
            //let test = get_subset(array.to_owned(), "rain".to_string(), 1);
            //println!("{:?} {:?}", array, test)
        }
    }
}

