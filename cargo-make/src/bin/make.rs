use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Current command line:  {:?}", args);

    for parameter in args.iter().skip(1) {
        run(parameter);
    }
}

fn run(parameter: &String) {
        // If the command is `cargo build`, run `cargo build --release`
        let status = Command::new("cargo")
            .args(&[parameter])
            .status()
            .expect("failed to execute cargo");

        assert!(status.success());
}
