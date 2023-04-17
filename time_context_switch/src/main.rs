use std::fs;
use std::env;
use std::time::{Instant};

use nix::sched::{CpuSet, sched_setaffinity};
use nix::unistd::Pid;

fn main() {
    let mut cpu_set = CpuSet::new();
    cpu_set.set(0).unwrap();
    sched_setaffinity(Pid::from_raw(0), &cpu_set).unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!["filename not provided!"];
    }
    let start = Instant::now();
    // initiate the context switch
    let contents = fs::read_to_string(&args[1]).ok().unwrap();
    let duration = start.elapsed();
    // record time elapsed 
    println!("time elapsed during context switch: {:?}", &duration);
  
    println!("contents of the file{:?}: {:?}", &args[1], &contents);
}
