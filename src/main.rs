use std::io::{self, Write};

use clap::Parser;
use redis_cli::{commands::Command, config::Config, redis::Redis};

#[derive(Parser, Debug)]
#[clap(disable_help_flag = true)]
struct Args {
    #[arg(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,

    #[arg(short, long, default_value = "127.0.0.1")]
    hostname: String,

    #[arg(short, long, default_value = "6379")]
    port: String,

    #[arg(short, long)]
    auth: Option<String>,
}

fn main() {
    let args = Args::parse();
    let cfg = Config::new(&args.hostname, &args.port, args.auth);

    let redis = Redis::new(cfg).unwrap_or_else(|e| {
        println!("Failed to initialize Redis Client. Error => {}", e);
        std::process::exit(1);
    });

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    write!(stdout, "CMD> ").unwrap();
    loop {
        let connection = redis.get_connection().unwrap_or_else(|e| {
            println!("Failed to get a connection. Error => {}", e);
            std::process::exit(1);
        });
        stdout.flush().unwrap();
        
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line from STDIN");
        
        let result = Command::new(&input);
        let cmd: Command;

        if let Err(e) = result {
            writeln!(stderr, "{}", e).unwrap();
            std::process::exit(1);
        } else {
            cmd = result.unwrap();
        }
        
        match cmd.execute(connection) {
            Ok(msg) => {
                write!(stdout, "OUT> ").unwrap();
                writeln!(stdout, "{}", msg).unwrap();
                writeln!(stdout, "-------------------------------------------").unwrap();
            }
            Err(e) => {
                writeln!(stderr, "{}", e).unwrap();
            }
        }
        write!(stdout, "CMD> ").unwrap();
    }
}

