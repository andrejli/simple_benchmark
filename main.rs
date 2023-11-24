
use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::*;
mod hashtools;

/// Generate random number and return number as 32-bit integer
/// Function has no input value
/// return:: 32bit integer
fn generate_random_value() -> i32 {
    let mut rng = rand::thread_rng();
    let result = rng.gen_range(1..1000000);  // ::<i32>();
    return result
}






fn main() {

    // Generate milion vqalues to array
    let mut buffer: Vec<i32> = Vec::new();  // dynamic array is called vector
    for _x in 1..1000000 {
        let value: i32 = generate_random_value();
        buffer.push(value);  // push appends value
    }
    // println!(" {:?} ", buffer);

    // start timer
    let timer_start = SystemTime::now();
    let start = timer_start.duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!(" {:?} ", start);
    
    // calculate milion hashes
    // let n : i32 = 0; 
    for n in buffer.iter() { 
     
    let stringval: String =  n.to_string();  // changing to String
    let srtval = stringval.as_str(); // changing to &str
    // println!(" {} {} {}", n, n, srtval );

    let _calculated_hash = hashtools::md5hash(srtval);
    // println!(" {:?} ", calculated_hash);  // control print

    }

    // stop timer
    let timer_stop = SystemTime::now();
    let stop = timer_stop.duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!(" {:?} ", stop);

    let result = stop - start;
    println!("1milion hashes calculated in {} seconds", result);

}