use std::process::Command;

use crate::prelude::*;

use super::NetworkPort;

pub fn create_cluster(data_path: &str) -> Result<String> {
     let output = Command::new("pg_ctl")
        .arg("initdb")
        .arg("-D")
        .arg(data_path)
        .arg("-o --encoding=utf8 --locale=C --auth=trust")
        .env("TZ", "UTC")
        .output()?;

     if output.stderr.is_empty() {
        return Ok(String::from_utf8(output.stdout).expect("Failing to convert to utf8"));
     }
     Err(Error::Generic(String::from_utf8(output.stderr).expect("Failing to convert error to utf8")))
}

pub fn start_server(data_path: &str, port: NetworkPort) {
    let output = Command::new("pg_ctl")
        .arg("-D")
        .arg(data_path)
        .arg(f!("-o \"-p {port}\""))
        .arg("start")
        .output();
    println!("{:?}", output);
}
