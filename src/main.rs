use scheesim_lexparse::ScheesimSchema;
use std::io::read_to_string;
use std::fs::File;

fn main() {   
    let ss = read_to_string(File::open("examples/test.scheenet").unwrap()).unwrap();

    let schema = ScheesimSchema::from(ss.trim());

    println!("{:?}", schema);
}
