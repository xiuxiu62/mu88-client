use lazy_static::lazy_static;
use std::{collections::HashMap, process};
use structopt::StructOpt;
use telnet::Telnet;

const ADDRESS: &str = "192.168.99.202";

macro_rules! command_map {
    ($($key:expr => $val:expr),*) => {{
        let mut map = HashMap::new();
        $(map.insert($key, $val);)*

        map
    }};
}

lazy_static! {
    static ref COMMAND_MAP: HashMap<(u8, u8), &'static [u8; 13]> = command_map![
        (1, 1) => b"//F00U01I01Z\r",
        (1, 2) => b"//F00U01I02Y\r",
        (1, 3) => b"//F00U01I03X\r",
        (1, 4) => b"//F00U01I04_\r",
        (1, 5) => b"//F00U01I05^\r",
        (1, 6) => b"//F00U01I06]\r",
        (1, 7) => b"//F00U01I07\\r",
        (1, 8) => b"//F00U01I08S\r",
        (2, 1) => b"//F00U02I01Y\r",
        (2, 2) => b"//F00U02I02Z\r",
        (2, 3) => b"//F00U02I03[\r",
        (2, 4) => b"//F00U02I04\\r",
        (2, 5) => b"//F00U02I05]\r",
        (2, 6) => b"//F00U02I06^\r",
        (2, 7) => b"//F00U02I07_\r",
        (2, 8) => b"//F00U02I08P\r",
        (3, 1) => b"//F00U03I01X\r",
        (3, 2) => b"//F00U03I02[\r",
        (3, 3) => b"//F00U03I03Z\r",
        (3, 4) => b"//F00U03I04]\r",
        (3, 5) => b"//F00U03I05\\r",
        (3, 6) => b"//F00U03I06_\r",
        (3, 7) => b"//F00U03I07^\r",
        (3, 8) => b"//F00U03I08Q\r",
        (4, 1) => b"//F00U04I01_\r",
        (4, 2) => b"//F00U04I02\\r",
        (4, 3) => b"//F00U04I03]\r",
        (4, 4) => b"//F00U04I04Z\r",
        (4, 5) => b"//F00U04I05[\r",
        (4, 6) => b"//F00U04I06X\r",
        (4, 7) => b"//F00U04I07Y\r",
        (4, 8) => b"//F00U04I08V\r",
        (5, 1) => b"//F00U05I01^\r",
        (5, 2) => b"//F00U05I02]\r",
        (5, 3) => b"//F00U05I03\\r",
        (5, 4) => b"//F00U05I04[\r",
        (5, 5) => b"//F00U05I05Z\r",
        (5, 6) => b"//F00U05I06Y\r",
        (5, 7) => b"//F00U05I07X\r",
        (5, 8) => b"//F00U05I08W\r",
        (6, 1) => b"//F00U06I01]\r",
        (6, 2) => b"//F00U06I02^\r",
        (6, 3) => b"//F00U06I03_\r",
        (6, 4) => b"//F00U06I04X\r",
        (6, 5) => b"//F00U06I05Y\r",
        (6, 6) => b"//F00U06I06Z\r",
        (6, 7) => b"//F00U06I07[\r",
        (6, 8) => b"//F00U06I08T\r",
        (7, 1) => b"//F00U07I01\\r",
        (7, 2) => b"//F00U07I02_\r",
        (7, 3) => b"//F00U07I03^\r",
        (7, 4) => b"//F00U07I04Y\r",
        (7, 5) => b"//F00U07I05X\r",
        (7, 6) => b"//F00U07I06[\r",
        (7, 7) => b"//F00U07I07Z\r",
        (7, 8) => b"//F00U07I08U\r",
        (8, 1) => b"//F00U08I01S\r",
        (8, 2) => b"//F00U08I02P\r",
        (8, 3) => b"//F00U08I03Q\r",
        (8, 4) => b"//F00U08I04V\r",
        (8, 5) => b"//F00U08I05W\r",
        (8, 6) => b"//F00U08I06T\r",
        (8, 7) => b"//F00U08I07U\r",
        (8, 8) => b"//F00U08I08Z\r"
    ];
}

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
        generate_binds().iter().for_each(|command| println!("{command}"));
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

fn generate_binds() -> Vec<String> {
    (1..=8)
        .map(|in_port: u8| {
            (1..=8).map(move |out_port: u8| format!(r#"({in_port}, {out_port}) => b"{}","#, create_bind_command(in_port, out_port)))
        })
        .flatten()
        .collect()
}

fn create_bind_command(in_port: u8, out_port: u8) -> String {
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
fn check_all_ports() -> Result<(), Box<dyn std::error::Error>>  {
    (1..=8).try_for_each(|in_port| {
        (1..=8).try_for_each(|out_port| {
            std::thread::sleep(std::time::Duration::from_millis(500));

            execute_bind(in_port, out_port)
        })
    })
}