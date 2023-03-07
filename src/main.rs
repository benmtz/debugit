use std::env::{set_var, args};
use std::process::Command;

fn main() {
    set_var("GIT_TRACE", "1");
    set_var("GIT_CURL_VERBOSE", "1");

    let received_args = args().collect::<Vec<String>>();
    Command::new("git").args(&received_args[1..]).spawn().expect("Command failed");
}
