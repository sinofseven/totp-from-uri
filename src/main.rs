use std::env;
use totp_rs::TOTP;

fn main() {
    let url = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: trial_totp_rs <otpauth-uri>");
        std::process::exit(1);
    });
    let totp = TOTP::from_url(&url).unwrap();

    let token = totp.generate_current().unwrap();
    println!("{}", token);
}
