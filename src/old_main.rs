//id3 algorithm

extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use std::collections::HashSet;
use std::env;
use std::erro
use std:
use std::fs::File;
use std::process;
use std::fs;

use rand::Rng;

use ndarray::{ArrayBase, Array2, ArrayView1, Axis};
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

/*fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}*/

fn readcsv(file_path: String) -> Result<Array2<AttrValue>, Box<dyn Error>> {
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


//random
fn random_attribute(examples: &Array2<AttrValue>) -> usize {
    if examples.ncols() == 1 {
        0
    }else {
        rand::thread_rng().gen_range(0, examples.ncols()-1)
    }
}

fn entropy(array: ArrayView1<AttrValue>) -> f64 {
    let mut values = HashSet::<String>::new();
    let mut a = array.to_vec();
    a.retain(|e| values.insert(e.to_string()));

    let mut count= vec![0.0;a.len()];
    for i in array.iter() {
        for j in 0..a.len() {
            if i.to_string() == a[j] {
                count[j]+=1.0;
                break;
            }
        }
    }
    
    /*
    //UNOPTIMIZED
    for i in values.iter() {
        count.push(array.iter().filter(|&n| *n == i.to_string()).count() as f64)
    }
    */

    //println!("{:?}", count);
    let mut ret = 0.0;
    let total = array.len() as f64;
    for i in count {
        ret -= i / total * (i/total).log2();
    }
    //println!("{:?}", ret);
    ret
}

//information gain
fn information_gain(examples: &Array2<AttrValue>) -> usize {    
    let target_col = examples.column(examples.ncols()-1);
    let target_s = entropy(target_col);

    let nrows = examples.nrows() as f64;
    let ncols = examples.ncols();
    
    let mut x = 0;
    let mut max_gain = 0.0;
    let mut ret = 0;
    for col in examples.axis_iter(Axis(1)) {
        if x == ncols -1 {
            break
        }

        let mut values = HashSet::<String>::new();
        let mut a = col.to_vec();
        a.retain(|e| values.insert(e.to_string()));
        let mut subsets = vec![Vec::<String>::new();a.len()];
        for i in 0..col.len() {
            for j in 0..a.len() {
                //Notice!
                // index i is common on col and target_col
                // index j is common on a and subsets

                if col[i] == a[j] {
                    subsets[j]. push(target_col[i].clone());
                    break;
                }
            }
        }

        let mut inf_gain = target_s;
        let mut split_inf = 0.0;
        for subset in subsets.iter() {
            inf_gain -= subset.len() as f64 / nrows * entropy(ArrayBase::from(subset));
            split_inf -= subset.len() as f64 / nrows * (subset.len() as f64 / nrows).log2();
        }
        let gain_ratio = inf_gain / split_inf;

        if gain_ratio > max_gain {
            max_gain = gain_ratio;
            ret = x;
        }
        //println!("{:?} {:?}", inf_gain, ret);
        x+=1;
    }
    ret
}
//gain ratio
fn gain_ratio(examples: &Array2<AttrValue>) -> usize {    
    let target_col = examples.column(examples.ncols()-1);
    let target_s = entropy(target_col);

    let nrows = examples.nrows() as f64;
    let ncols = examples.ncols();
    
    let mut x = 0;
    let mut max_gain = 0.0;
    let mut ret = 0;
    for col in examples.axis_iter(Axis(1)) {
        if x == ncols -1 {
            break
        }

        let mut values = HashSet::<String>::new();
        let mut a = col.to_vec();
        a.retain(|e| values.insert(e.to_string()));
        let mut subsets = vec![Vec::<String>::new();a.len()];
        for i in 0..col.len() {
            for j in 0..a.len() {
                //Notice!
                // index i is common on col and target_col
                // index j is common on a and subsets

                if col[i] == a[j] {
                    subsets[j]. push(target_col[i].clone());
                    break;
                }
            }
        }

        let mut inf_gain = target_s;

        for subset in subsets.iter() {
            inf_gain -= subset.len() as f64 / nrows * entropy(ArrayBase::from(subset));
        }
        if inf_gain > max_gain {
            max_gain = inf_gain;
            ret = x;
        }
        //println!("{:?} {:?}", inf_gain, ret);
        x+=1;
    }
    ret
}

fn best_attribute(examples: &Array2<AttrValue>, sel: i32) -> usize {
    match sel {
        1 => random_attribute(examples),
        2 => information_gain(examples),
        3 => gain_ratio(examples),
        _ => 0
    }
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

fn id3(examples: Array2<AttrValue>, mut attributes: Vec<usize>, selection: i32) -> Tree {
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

    let best = best_attribute(&examples, selection);
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
                kids.push(Child{ path: i.to_string(), tree: id3(subset, attributes.clone(), selection) });
            }
        }
    }
    ret
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 {
        match readcsv(args[1].to_string()) {
            Err(e) => {
                println!("Error {}", e);
                process::exit(1)
            }
            Ok(array) => {
                let mut attributes = Vec::<usize>::new();
                for i in 0..array.ncols()-1 {
                    attributes.push(i);
                }
                let tree = id3(array, attributes, args[3].parse::<i32>().expect("not valid attribute selection"));
                fs::write(format!("./{}",args[2]), format!("{:#?}",tree)).expect("Unable to write file");
                //println!("{:#?}", x);
                //let test = get_subset(array.to_owned(), "rain".to_string(), 1);
                //println!("{:?} {:?}", array, test)
            }
        }
    } else {
        println!("args1: data file\nargs2: output file\nargs3: method for attribute selection(1:random, 2:information gain, 3:gain ratio)");
    }
} test = get_subset(array.to_owned(), "rain".to_string(), 1);
                //println!("{:?} {:?}", array, test)
            }
        }
    } else {
        println!("args1: data file\nargs2: output file\nargs3: method for attribute selection(1:random, 2:information gain, 3:gain ratio)");
    }
}