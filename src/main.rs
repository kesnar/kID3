/// kesnar-ID3 (kID3) is a simple implementation of the ID3 algorithm in Rust with 3 options for selecting best attribute (random, information gain, gain ratio)
/// Written by kesnar (Panagiotis Famelis) in November 2020
/// Published under CC BY-NC-SA 4.0 (Attribution-NonCommercial-ShareAlike 4.0 International)

extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::process;
use std::fs;

use rand::Rng;

use ndarray::{ArrayBase, Array2, ArrayView1, Axis, Slice};
use ndarray_csv::Array2Reader;

/// The Tree type is an enumeration type with two possible values, Leaf and Branch.
/// The Leaf option denotes a leaf node of the tree and contains the appropriate value of the target attribute for the specific path.
/// The Branch option denotes an intermediate node of the tree and contains:
///     label: The appropriate attribute this node examines
///     children: A vector of Childs containing the various children of the intermediate node
#[derive(Debug)]
enum Tree {
    Leaf(TargetValue),
    Branch {
        label: Attribute, 
        children: Vec<Child>
    }
}

/// The child struct is the type representing the path to another node. It contains two fields.
///     path: The appropriate attribute value this path is examining
///     tree: The Tree that is following after the path.
#[derive(Debug)]
struct Child {
    path: AttrValue,
    tree: Tree
}

/// Attribute is of type usize. kID3 does not take the attributes' names as input, so it uses a 
/// usize for denoting the various attributes, corresponding to the corresponding column number.
type Attribute = usize;
type TargetValue = String;
type AttrValue = String;

impl Tree {
    /// Function to test an example on a tree
    fn test(&self, row: ArrayView1<AttrValue>) -> bool {
        match self {
            Tree::Leaf(val) => if row[row.len()-1] == val.to_string() {true} else {false},
            Tree::Branch{label, children} => {
                let val = &row[*label];
                let mut ret = false;
                // find val in children and test the subtree
                for Child{path, tree} in children.iter() {
                    if path == val {
                        ret = tree.test(row);
                        break;
                    }
                }
                ret
            }
        }
    }
}

/// Function to read the csv file and save the data read to an ndarray::Array2.
fn readcsv(file_path: String) -> Result<Array2<AttrValue>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    let array_read: Array2<AttrValue> = reader.deserialize_array2_dynamic()?;
    Ok(array_read)
}

/// Function to check if an array contains only one type of element. Returns a tuple, whether there
/// is only one value and what that is. In case there are multiple values, returns an empty string.
fn check_single_value(arr: &ArrayView1<AttrValue>) -> (bool, String) {
    let value1 = &arr[0];
    for i in arr.iter() {
        if !value1.eq(i) {
            return (false, "".to_string())
        }
    }
    (true, value1.to_string())
}


/// Function that returns a random number corresponding to one attribute of the examples array.
fn random_attribute(examples: &Array2<AttrValue>) -> usize {
    if examples.ncols() == 1 {
        0
    }else {
        rand::thread_rng().gen_range(0, examples.ncols()-1)
    }
}

/// Function that returns the entropy of an array.
fn entropy(array: ArrayView1<AttrValue>) -> f64 {
    // Collect the values of the array.
    let mut values = HashSet::<String>::new();
    let mut a = array.to_vec();
    a.retain(|e| values.insert(e.to_string()));

    // Counts the occurrences of each value.
    let mut count= vec![0.0; a.len()];
    for i in array.iter() {
        for j in 0..a.len() {
            if i.to_string() == a[j] {
                count[j]+=1.0;
                break;
            }
        }
    }
    
    let mut ret = 0.0;
    let total = array.len() as f64;
    for i in count {
        ret -= i / total * (i/total).log2();
    }
    ret
}

/// Function that returns the attribute with the best information gain
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

        // Collect the values of the array.
        let mut values = HashSet::<String>::new();
        let mut a = col.to_vec();
        a.retain(|e| values.insert(e.to_string()));

        // Creates a subset containing the target values for each attribute value. 
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
        x+=1;
    }
    ret
}

/// Function that returns the attribute with the best gain ratio
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

        // Collect the values of the array.
        let mut values = HashSet::<String>::new();
        let mut a = col.to_vec();
        a.retain(|e| values.insert(e.to_string()));

        // Creates a subset containing the target values for each attribute value. 
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
        x+=1;
    }
    ret
}

/// Generic function that calls one of the three selection functions.
fn best_attribute(examples: &Array2<AttrValue>, sel: i32) -> usize {
    match sel {
        1 => random_attribute(examples),
        2 => information_gain(examples),
        3 => gain_ratio(examples),
        _ => 0 //to-do: error handling
    }
}

/// Function to get subset of an array, such that the subset contains every row of examples that contain vi in column a
fn get_subset(examples: Array2<AttrValue>, vi: String, a: usize) -> Array2<AttrValue> {
    let mut helper = vec![];
    let mut i = 0;
    for row in examples.outer_iter() {
        //Find the rows that correspond to the value that we are examining
        if row[a] == vi {
            i+=1;
            //Keep these rows without the value we are examining
            let mut remove = 0;
            for e in row.iter() {
                if remove != a {
                    helper.push(e.to_string());
                }
                remove += 1;
            }
        }
    }

    // to-do: error handling
    // UNSAFE! TO CHANGE!
    if let Ok(ret) = Array2::from_shape_vec((i, examples.ncols()-1), helper) {
        ret
    }
    else {
        Array2::from(vec![[]])
    }
}

/// Function that implements the ID3 algorithm
fn id3(examples: Array2<AttrValue>, mut attributes: Vec<usize>, selection: i32) -> Tree {
    let mut ret;
    // Checks whether target attribute has only one value. Terminal case.
    let (cond, value) = check_single_value(&examples.column(examples.ncols()-1));
    if cond {
        ret = Tree::Leaf(value);
        return ret
    }

    let best = best_attribute(&examples, selection);
    
    // Create values set for best attribute.
    let mut values = HashSet::<String>::new();
    // Populate values set.
    examples.column(best).to_vec().retain(|e| values.insert(e.to_string()));

    ret = Tree::Branch{ label: attributes[best], children: Vec::new() };
    
    // Remove best attribute from the attribute vector.
    attributes.remove(best);

    // For each value in best attribute create a child
    for i in values.iter() {
        if let Tree::Branch{ label: _, children: ref mut kids } = ret {
            //Find subset of examples with specific value in attribute and call ID3 on it.
            let subset = get_subset(examples.to_owned(), i.to_string(), best);
            kids.push(Child{ path: i.to_string(), tree: id3(subset, attributes.clone(), selection) });
        }
    }
    ret
}

/// Function to split a 2D array in two.
fn split(examples: Array2<AttrValue>, cut: i32) -> (Array2<AttrValue>, Array2<AttrValue>) {

    (examples.slice_axis(Axis(0), Slice::from(0..cut)).to_owned(), examples.slice_axis(Axis(0), Slice::from(cut+1..)).to_owned())
}

/// Function to split a 2D array in k roughly even parts
fn splitk(examples: Array2<AttrValue>, k: usize) -> Vec<Array2<AttrValue>> {
    let mut ret = Vec::<Array2<AttrValue>>::new();

    let nrows = examples.nrows();
    let s = nrows / k;
    let smod = nrows % k;

    for i in 0..k {
        // Not sure if it is needed anymore
        /*if i == k-1 {
            ret.push(examples.slice_axis(Axis(0), Slice::from(i*s..)).to_owned());
        }*/
        // While i < smod: create sets with s+1
        if i < smod {
            ret.push(examples.slice_axis(Axis(0), Slice::from(i*(s+1)..((i+1)*(s+1)))).to_owned());
        }
        // while i <= smode: create sets with s
        else {
            ret.push(examples.slice_axis(Axis(0), Slice::from(i*s..((i+1)*s))).to_owned());
        }
    }
    ret
}

/// Function to validate a tree with a set of examples. Returns the accuracy.
fn validate(tree: &Tree, validation: Array2<AttrValue>) -> f64 {

    let mut pass = 0.0;
    for row in validation.outer_iter() {
        if tree.test(row) {
            pass+=1.0;
        }
    }
    pass / validation.nrows() as f64
}


fn main() {
    
    // Collect arguments
    let args: Vec<String> = env::args().collect();
    
    // Check that the number of arguments are correct
    if (args.len() == 6) || (args.len() == 5) {
        match readcsv(args[1].to_string()) {
            Err(e) => {
                // In case there is an error in the data file reading.
                println!("Error {}", e);
                process::exit(1)
            }
            Ok(array) => {
                // Create attributes array based on number of columns in the examples data set.
                let mut attributes = Vec::<usize>::new();
                for i in 0..array.ncols()-1 {
                    attributes.push(i);
                }
                
                // Parse attribute selection.
                let attr_sel = args[3].parse::<i32>().expect("not valid attribute selection");
                if attr_sel > 3 {
                    println!("not valid attribute selection");
                    process::exit(1)
                }

                let nrows = array.nrows() as i32;
                let choice = args[4].parse::<String>().expect("not valid split selection");
                if choice.eq("a") {
                    let tree = id3(array, attributes, attr_sel);
                    fs::write(format!("./{}",args[2]), format!("{:#?}",tree)).expect("Unable to write file");
                } else if choice.eq("b") {
                    let k = args[5].parse::<i32>().expect("not valid split selection") * nrows/100;
                    if (k < nrows-1) && (k > 0) {
                        let (validation, examples) = split(array, k);
                        let tree = id3(examples, attributes, attr_sel);
                        let accuracy = validate(&tree, validation);    
                        fs::write(format!("./{}",args[2]), format!("{:#?}\n\naccuracy: {}",tree, accuracy)).expect("Unable to write file");
                    } else {
                        let tree = id3(array, attributes, attr_sel);
                        fs::write(format!("./{}",args[2]), format!("{:#?}",tree)).expect("Unable to write file");
                    }
                } else if choice.eq("c") {
                    let k = args[5].parse::<usize>().expect("not valid split selection");
                    let mut mean_acc = 0.0;
                    let partition = splitk(array, k);
                    let mut wstr = "".to_string();
                    for i in 0..k {
                        let mut tmp = partition.clone();
                        let validation = tmp.remove(i);

                        // to-do: better
                        let mut pmt = Vec::<String>::new();
                        for a in tmp.iter() {
                            for b in a.iter() {
                                pmt.push(b.to_string());
                            }
                        }
                        if let Ok(examples) = Array2::from_shape_vec((nrows as usize - validation.nrows(), validation.ncols()), pmt) {
                            let tree = id3(examples.clone(), attributes.clone(), attr_sel);
                            let accuracy = validate(&tree, validation);
                            mean_acc +=accuracy;
                            wstr = format!("{}{:#?}\n\naccuracy: {}\n\n-----------------------------\n\n", wstr, tree, accuracy);
                        }
                    }
                    fs::write(format!("./{}",args[2]), format!("{}{}", wstr, mean_acc/k as f64)).expect("Unable to write file")
                }else {
                    // Print message to inform the format of arguments.
                    println!("arg1: data file\narg2: output file\narg3: method for attribute selection (1: random, 2: information gain, 3: gain ratio)\narg4: select validation type (a: no validation, b: percent holdout, c: k-fold cross validation)\narg5: percentage for holdout or number of k for k-fold cross-validation depending on arg4");
                }
            }
        }
    } else {
        // Print message to inform the format of arguments.
        println!("arg1: data file\narg2: output file\narg3: method for attribute selection (1: random, 2: information gain, 3: gain ratio)\narg4: select validation type (a: no validation, b: percent holdout, c: k-fold cross validation)\narg5: percentage for holdout or number of k for k-fold cross-validation depending on arg4");
    }
}