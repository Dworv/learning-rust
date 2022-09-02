use std::{fs::{OpenOptions, write}, io::Write, ops::Range};
use rand::{thread_rng, Rng};
use std::time::{Instant};

pub fn make_grid() {
    let start = Instant::now();
    write("rsrandom.txt", "".as_bytes()).unwrap();
    let mut file = OpenOptions::new()
        .append(true)
        .open("rsrandom.txt")
        .unwrap();
    for _ in 0..1000 {
        let mut rng = thread_rng();
        let nums = (0..1000)
            .map(|_| rng.gen_range::<u8, Range<u8>>(0..10).to_string())
            .collect::<Vec<String>>()
            .join("")
            + "\n";
        file.write(nums.as_bytes()).unwrap();
    }
    let end = Instant::now();
    println!("Created new grid in {:?}...", end.duration_since(start));
}