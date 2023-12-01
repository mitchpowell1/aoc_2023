use std::env;
use std::fs;

const SESSION_COOKIE_ENV_VAR: &str = "AOC_SESSION_COOKIE";

pub fn setup_day(day: u8) {
    let session_cookie = env::var(SESSION_COOKIE_ENV_VAR)
        .expect("The `AOC_SESSION_COOKIE_VAR` environment variable must be set to complete setup");
    let out = std::process::Command::new("curl")
        .arg("--cookie")
        .arg(format!("session={session_cookie}"))
        .arg(format!("https://adventofcode.com/2023/day/{day}/input"))
        .output()
        .expect("Couldn't complete request for input")
        .stdout;

    fs::write(format!("inputs/day_{day}"), out).expect("Unable to write input contents to file");
}
