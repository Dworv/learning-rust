use std::env;
use std::thread::spawn;
use std::sync::{Arc, Mutex};
use std::collections::BTreeMap;
use std::mem::drop;
use std::time::{Instant, Duration};

extern crate reqwest;
use reqwest::blocking::Client;

const NUM_THREADS: usize = 2;

fn main() {
    
    let args = env::args().skip(1).enumerate().collect::<Vec<(usize, String)>>();
    let sites = Arc::new(Mutex::new(args));
    let finished: Arc<Mutex<BTreeMap<usize, (String, Duration)>>> = Arc::new(Mutex::new(BTreeMap::new()));

    let mut handles = Vec::new();
    for _ in 0..NUM_THREADS {
        let sites_arc = sites.clone();
        let finished_arc = finished.clone();
        handles.push(
            spawn(
                move || { 
                    let client = Client::new();
                    loop {
                        let mut snapshot = sites_arc
                            .lock()
                            .unwrap();
                        match snapshot.is_empty() {
                            true => break,
                            false => {
                                let (i, uri) = snapshot.pop().unwrap();
                                drop(snapshot);
                                let start = Instant::now();
                                client.get(&uri).send().unwrap();
                                let latency = start.elapsed();
                                let mut dest = finished_arc.lock().unwrap();
                                dest.insert(i, (uri, latency));
                            }
                        }
                    }
                }
            )
        )
    };

    for handle in handles {
        handle.join().unwrap();
    }
    let binding = finished.clone();
    let results = binding.lock().unwrap();
    println!(
        "{} visted.\nAvg Ping: {:?}ms, Best ping: {}ms", 
        results.len(), 
        results.clone().into_iter().map(|item| {item.1.1.as_millis() as usize}).fold(0usize, |acc, secs| acc + secs) / results.len(),
        results.clone().into_iter().map(|item| {item.1.1.as_millis() as usize}).min().unwrap()
    );
}
