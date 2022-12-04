use std::collections::BinaryHeap;
use std::fs::{File, self, read};
use std::io::prelude::*;
use std::str::{self, FromStr};
use std::iter::Iterator;

fn max_k(iterator: impl Iterator<Item = u32>, k: usize) -> Vec<u32>{
    let mut _max = vec![u32::MIN; k];
    return iterator
        .into_iter()
        .fold(_max, 
            |mut _acum, x| {
                let r = _acum
                    .iter()
                    .position(|a| x > a.clone());
                match r {
                    Some(i) => _acum[i] = x,
                    _ => ()
                };
                _acum
            }
        );
}

fn main() {
    let mut data = String::new();
    let mut file = File::open("./input.txt").expect("File not found");
    let _ = file.read_to_string(&mut data).expect("something went wrong");

    let res = max_k(
        data
        .split("\n\n")
        .map(
            |part| part
                .lines()
                .map(|line| u32::from_str(line).expect("cannot parse"))
                .sum::<u32>()
            ), 3)
        .iter()
        .sum::<u32>();
    
    println!("Day 1 Answer: {:?}", &res);
}
