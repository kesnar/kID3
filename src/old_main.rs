//id3 algorithm

extern crate csv;

use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

type NodesBox<T> = Option<Vec<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    label: T,
    children: NodesBox<T>
}

impl <T> Node<T> {
    fn new(s: T) -> Node<T> {
        Node{label: s, children: None}
    }

    fn add_children(&mut self, node: Node<T>) {
        match self.children{
            Some(ref mut v) => v.push(node),
            None => {
                let mut v = Vec::new();
                v.push(node);
                self.children = Some(v)
            }
        }
    }
}

#[derive(Debug)]
struct Coo<T> {
    array: Vec<T>,
    values: Vec<T>,
}

impl <T> Coo<T> 
where T: std::fmt::Debug + Ord + Copy + PartialEq {
    fn new(arr: &[T]) -> Coo<T> {
        let mut uniques = BTreeSet::new();
        let mut v = arr.to_vec();
        v.retain(|e| uniques.insert(*e));
        Coo {
            array: arr.to_vec(),
            values: v,
        }
    }
    fn calc_ig<S>(&self, reference: &Coo<S>) -> f64 
    where S: std::fmt::Debug + Ord + Copy + PartialEq {
        let mut tr = vec![vec![0; reference.values.len()]; self.values.len()];

        /*for k in &self.values {
            let helper = self.array.iter().filter(|&n| *n == *k);
            tr[i] = helper.count() as f64;
            i += 1;
        }*/


        let it = self.array.iter().zip(reference.array.iter()).enumerate().collect::<Vec<_>>();

        for (_, (x,y)) in it {
            println!("{:?}", (x,y));
            let mut i = 0;
            for k in &self.values {
                let mut j = 0;
                for l in &reference.values {
                    if (x == k) && (y == l) {
                        println!("{:?}, {:?}", k, l);
                        tr[i][j] += 1;
                    }
                    j += 1;
                }
                i += 1;
            }
        }
        println!("{:?}", tr);
           
           /* let helper = 
            if x == yes {
                tr[yes]++
                if y == yes

            }
            if y == no {

            }

            let mut i = 0;
            for k in &self.values {
                if y == k {
                    println!("{:?} {:?}", y, i);
                    tr[i] = self.array.iter().filter(|&n| *n == *k).count() as f64;
                    i += 1;
                }
            }

        }*/
        0.0
    }
}

fn calc_entropy(arr: &[bool]) -> f64 {
//where T: std::fmt::Debug {
//where T: std::iter::IntoIterator {
    let mut tr = 0.0;
    let mut fa = 0.0;
    let length = arr.len() as f64;

    for i in arr.iter() {
        match i {
            true => tr += 1.0,
            false =>fa += 1.0
        }
    }
    0.0 - (tr/length) * (tr/length).log2() - (fa/length) * (fa/length).log2()
}

/*fn vec_to_set(vec: Vec<T>) -> HashSet<T>
where T: std::clone::Clone {
    HashSet::from_iter(vec)
}*/

/*fn readcsv() -> Result<(), Box<Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
        for i in record.iter() {
            println!("{:?}", i);
        }
    }
    Ok(())
}*/

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
                    let mut i = 0 ;
                    for result in rdr.records() {
                        if let Ok(record) = result {
                            if i != 0 {
    							//fill vectors

                                /*println!("{:?}", record);
                                for i in record.iter() {
                                    println!("{:?}", i);
                                }*/
                            } else {
                                //create vectors
                            }
                        }
                        else {
                            println!("Error on format");
                            process::exit(1)
                        }
                        i += 1;
                    }
                }
            }
        }
    }


    /*if let Err(err) = readcsv() {
        println!("{}", err);
        process::exit(1);
    }*/
}


/*
    let mut root = Node::new(1);

    let child2 = Node::new(2);
    root.add_children(child2);
    let child3 = Node::new(3);
    root.add_children(child3);
    let child5 = Node::new(5);
    let mut child4 = Node::new(4);
    child4.add_children(child5);
    root.add_children(child4);
    
    println!("{:#?}", root);

    let id = [1,2,3,4,5,6,7,8,9,10,11,12,13,14];
    println!("{:?} {}", id, id.len());

    let fever = [false, true, true, true, true, false, true, true, false, true, false, false, false, true];
    println!("{:?} {}", fever, fever.len());

    let cough = [false, true, true, false, true, true, false, false, true, true, true, true, true, true];
    println!("{:?} {}", cough, cough.len());

    let breath = [false, true, false, true, true, false, true, true, true, false, false, true, true, false];
    println!("{:?} {}", breath, breath.len());

    let infected = [false, true, false, true, true, false, true, true, true, true, false, true, false, false];
    println!("{:?} {}", infected, infected.len());
    
    //let test = vec_to_set(infected.to_vec());
    //println!("{:?}", test);

    let inf = Coo::new(&infected);
    println!("{:?}", inf);
    let fev = Coo::new(&fever);
    fev.calc_ig(&inf);

    let infected_entropy = calc_entropy(&infected);
    println!("{}", infected_entropy);

    let a = vec![1; 10];
    let b = vec![2; 10];
    
    let it = a.iter().zip(b.iter());

    for (i,(x,y)) in it.enumerate() {
        println!("{} {} {:#?}", i, x, y);
    }

    //let (x,y) =calc_sub_entropy(&infected, breath);
}
*/