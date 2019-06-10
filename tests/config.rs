use assert_cmd::prelude::*;
use std::io::prelude::*;
use std::process::{Child, Command, Stdio};

#[test]
fn it_prompts_for_email_and_api_key() {
    let cmd = config();
    let mut stdout = cmd.stdout.unwrap();
    let mut stdin = cmd.stdin.unwrap();

    let mut buffer = [0; 10];
    stdout.read(&mut buffer).expect("could not read fd");
    println!("out {:?}", buffer);

    // println!("{}", read!("{}", stdout));
    write!(stdin, "email\n").unwrap();
    write!(stdin, "api_key\n").unwrap();
}

fn config() -> Child {
    let mut wrangler = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    wrangler
        .arg("config")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
}
