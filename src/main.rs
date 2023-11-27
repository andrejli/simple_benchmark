
use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::*;
use bytes::Bytes;
mod hashtools;
mod datastruct;


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
    for _x in 1..1000 {
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
    let mut counter = 1;
    for n in buffer.iter() { 
     
        let stringval: String =  n.to_string();  // changing to String
   
        let srtval = stringval.as_str(); // changing to &str
        // println!(" {} {} {}", n, n, srtval );
        let stringval2 : String = stringval.clone();
        let bytes_value: bytes::Bytes = Bytes::from(stringval2);
        let _calculated_hash = hashtools::md5hash(srtval);
        let _calculated_hash2 = hashtools::md5bytes(bytes_value);
        println!("{:06}  {:?}  {:?}", counter,  _calculated_hash, _calculated_hash2);  // control print
        counter += 1;
    }

    // stop timer
    let timer_stop = SystemTime::now();
    let stop = timer_stop.duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!(" {:?} ", stop);

    let result = stop - start;
    println!("1milion hashes calculated in {} seconds", result);

    // Struct example
    let mut sardinka = datastruct::Ryba {
        active: true,
        druh: String::from("sardinka"),
        dlzka: 15, //cm
        vaha: 70, // gramov
    };

    println!("{}", sardinka.druh);
    println!("{:?}", sardinka.active);
    let a: bool = sardinka.get_active(); // ziskany atribut sardinky pomocou gettera
    println!("{:?}", a);

    // Use setter on sardinka and change flag to false
    sardinka.set_active(false);
    let a: bool = sardinka.get_active(); // ziskany atribut sardinky pomocou gettera
    println!("{:?}", a);

    // Play with vectors
    let v: [i32; 4] = [12 ,34 ,56 ,78 ];
    for i in v {
        print!("{} ", i); // macro print! writes values without new line
    }
    let mut victor: Vec<i32> = v.to_vec();
    println!(" {:?}", victor);

    let v2: [i32; 3] = [1,2,3];
    for i in v2 {
        victor.push(i);  // appends value to vector
    }

    println!(" {:?}", victor);
    let v2vec:  Vec<i32> = v2.to_vec();

    victor.extend(v2vec);  // extends vec with another vector
    println!(" {:?}", victor);
    println!(" {} Vector Length", victor.len());

    for i in 0..victor.len() {
        victor[i] = 0;
    }
    println!(" {:?}", victor);
    println!(" {} Vector Length", victor.len());

    for i in (0..victor.len()-1).rev() {
        victor.remove(i);
        println!("{} {} REMOVED !", i, victor[i]);
    }
    println!(" {:?}", victor);
    println!(" {} Vector Length", victor.len());
}

