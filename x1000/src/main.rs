use std::fs::File;
use std::io::{BufReader, BufRead};
use std::thread::spawn;
use std::time::Instant;
pub mod grid;
use crate::grid::make_grid;

const NUM_THREADS: usize = 4;

fn main() {
    
    make_grid();
    let start = Instant::now();
    let file = File::open("rsrandom.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut inputs: [String; NUM_THREADS] = Default::default();

    for i in 0..1000 {
        match lines.next() {
            Some(Ok(line)) => inputs[i%NUM_THREADS].push_str(&line),
            _ => break,
        }
    }

    let mut handles = vec![];
    for i in 0..NUM_THREADS {
        let input = inputs[i].clone();
        handles.push(
            spawn(move || {
                input
                    .chars()
                    .map(|c| {
                        match c.to_digit(10) {
                            Some(d) => d as u8,
                            None => 0,
                        }
                })
                    .fold(0u32, |acc, x| acc + (x as u32))
            }))
    };
    let final_result = handles.into_iter().map(|h| h.join().unwrap()).sum::<u32>();
    println!("{}", final_result);
    let end = Instant::now();
    println!("Calculated grid in {:?}...", end.duration_since(start));
}
