use generate::generate_bind_file;
use std::process;
use structopt::StructOpt;
use telnet::Telnet;

mod binds;
mod generate;

pub use binds::COMMAND_MAP;

const ADDRESS: &str = "192.168.99.202";

#[derive(Debug, StructOpt)]
#[structopt(name = "mu88-cli", about = "A telnet client for the mu88 KM switch")]
struct Options {
    #[structopt(short, long, about = "Generate port bind commands")]
    generate: bool,
    #[structopt(long = "in")]
    in_port: Option<u8>,
    #[structopt(long = "out")]
    out_port: Option<u8>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();
    if options.generate {
        generate_bind_file()?;
        process::exit(0);
    }

    let (in_port, out_port) = match (options.in_port, options.out_port) {
        (Some(in_port), Some(out_port)) => (in_port, out_port),
        _ => {
            eprintln!("Both ports required");
            process::exit(1);
        }
    };

    execute_bind(in_port, out_port)
}

fn execute_bind(in_port: u8, out_port: u8) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Telnet::connect((ADDRESS, 23), 256)?;
    let command = *match COMMAND_MAP.get(&(in_port, out_port)) {
        Some(command) => command,
        None => return Err(format!("Invalid ports: ({in_port}, {out_port})").into()),
    };

    client.write(command)?;

    Ok(())
}

pub fn create_bind_command(in_port: u8, out_port: u8) -> String {
    let body = format!("//F00U0{}I0{}", in_port, out_port);

    create_command(body)
}

fn create_command(body: String) -> String {
    let checksum = checksum(body.as_bytes()) as char;

    format!(r#"{body}{checksum}\r"#)
}

fn checksum(body: &[u8]) -> u8 {
    body.iter().fold(0, |acc, byte| acc ^ byte)
}

#[test]
fn check_all_ports() -> Result<(), Box<dyn std::error::Error>> {
    (1..=8).try_for_each(|in_port| {
        (1..=8).try_for_each(|out_port| {
            std::thread::sleep(std::time::Duration::from_millis(500));

            execute_bind(in_port, out_port)
        })
    })
}
