use rand::seq::{IndexedRandom, SliceRandom};

const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()_+[]{}|;:,.<>?";

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if lowercase {
        chars.extend_from_slice(LOWERCASE);
        password.push(
            *LOWERCASE
                .choose(&mut rng)
                .ok_or_else(|| anyhow::anyhow!("Failed to choose a character"))?,
        );
    }

    if uppercase {
        chars.extend_from_slice(UPPERCASE);
        password.push(
            *UPPERCASE
                .choose(&mut rng)
                .ok_or_else(|| anyhow::anyhow!("Failed to choose a character"))?,
        );
    }

    if numbers {
        chars.extend_from_slice(NUMBERS);
        password.push(
            *NUMBERS
                .choose(&mut rng)
                .ok_or_else(|| anyhow::anyhow!("Failed to choose a character"))?,
        );
    }

    if symbols {
        chars.extend_from_slice(SYMBOLS);
        password.push(
            *SYMBOLS
                .choose(&mut rng)
                .ok_or_else(|| anyhow::anyhow!("Failed to choose a character"))?,
        );
    }

    for _ in 0..length - password.len() as u8 {
        let c = chars
            .choose(&mut rng)
            .ok_or_else(|| anyhow::anyhow!("Failed to choose a character"))?;
        password.push(*c);
    }

    password.shuffle(&mut rng);

    let password_str = String::from_utf8(password)?;
    println!("{}", password_str);

    let estimate = zxcvbn::zxcvbn(password_str.as_str(), &[]);

    eprintln!("Password strength: {}", estimate.score());

    Ok(())
}
