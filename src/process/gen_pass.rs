use anyhow::Result;
use rand::Rng;
use zxcvbn::zxcvbn;
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"+-!@#$%^&*()_+<>/?_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> Result<()> {
    let mut rng = rand::rng();
    let mut password = String::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
    }
    if lower {
        chars.extend_from_slice(LOWER);
    }
    if number {
        chars.extend_from_slice(NUMBER);
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
    }

    for _ in 0..length {
        let idx = rng.random_range(0..chars.len());
        password.push(chars[idx] as char);
    }

    println!("{}", password);
    // output password strength in stderr
    let estimate = zxcvbn(&password, &[]);
    eprintln!("password strength: {}", estimate.score());
    Ok(())
}
