use std::process::Command;
use structopt::StructOpt;
use std::thread;
use std::sync::Arc;

#[derive(StructOpt)]
struct Cli {
    command: String,
    profiles: Vec<String>
}

fn main() {
    let args = Cli::from_args();
    let mut handlers = vec![];
    let command = Arc::new(args.command);
    for p in args.profiles {
        let command = Arc::clone(&command);
        handlers.push(thread::spawn(move || {
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!("aws {} --profile={}", command, p))
                .output()
                .expect("failed to execute process");

            println!("{}", p);
            println!("{}", String::from_utf8(output.stderr).unwrap());
            println!("{}", String::from_utf8(output.stdout).unwrap())
        }));
    }

    for h in handlers {
        h.join().unwrap();
    }
}
