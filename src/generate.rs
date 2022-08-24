use std::{env, io, path::PathBuf};

pub(crate) fn generate_bind_file() -> io::Result<()> {
    let mut commands: String = generate_binds()
        .iter()
        .map(|bind| format!("\t\t{bind}\n"))
        .collect::<String>()
        .trim_end()
        .to_owned();
    commands.pop();

    let contents = format!(
        r#"// This file was auto-generated

use lazy_static::lazy_static;
use std::collections::HashMap;

macro_rules! command_map {{
    ($($key:expr => $val:expr),*) => {{{{
        let mut map = HashMap::new();
        $(map.insert($key, $val);)*

        map
    }}}};
}}

lazy_static! {{
    pub static ref COMMAND_MAP: HashMap<(u8, u8), &'static [u8; 13]> = command_map![
{commands}
    ];
}}
    "#
    );

    let mut out_file = source_directory();
    out_file.push("./binds.rs");

    std::fs::write(out_file, contents)
}

fn generate_binds() -> Vec<String> {
    (1..=8)
        .map(|in_port: u8| {
            (1..=8).map(move |out_port: u8| {
                format!(
                    r#"({in_port}, {out_port}) => b"{}","#,
                    crate::create_bind_command(in_port, out_port)
                )
            })
        })
        .flatten()
        .collect()
}

fn source_directory() -> PathBuf {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    directory.push("src");

    directory
}
