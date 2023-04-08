use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Current command line:  {:?}", args);
    
    let status = Command::new(args.get(1).unwrap())
        .args(args.get(2..).unwrap())
        .status()
        .expect("failed to execute cargo");

    assert!(status.success());
}
