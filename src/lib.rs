use anyhow::{bail, Error, Ok, Result};
use base64::prelude::*;

use rand::{distributions::Alphanumeric, Rng};

pub fn seed_hash(seed: &str) -> usize {
    let mut hash = 0;
    seed.chars()
        .enumerate()
        .for_each(|(i, c)| hash += (i + 1) * (c as usize));
    hash
}

const CRYPTO: &str = "7JMatMkJ4TitbPJcQIm5ncz3oRS1ybCmvCKBrqxyxO6VopxSOCZOLJXjiMRClhzNAYKZqPGlv6Hbt7C1hBjkofSFEV2GQsHkuNFkazZAzelx0Xvhubv0WcCnHMOuSZFn";
pub fn generator_password(seed: &str, length: usize) -> Result<String, Error> {
    if seed.len() < 4 {
        bail!("seed length is too short! please try again!")
    }
    if length < 6 {
        bail!("password length is too short! please try again!")
    }
    let mut hash = seed_hash(&seed);
    let mut password = String::new();
    let crypto_len = CRYPTO.len();
    // generator_crypto()
    while hash > 0 {
        let index = hash % crypto_len;
        let nthc = CRYPTO
            .chars()
            .nth(index)
            .expect("Error getting char from crypto!");
        password.push(nthc);
        hash /= crypto_len;
    }

    // 将 seed 和 password 拼接
    seed.chars().for_each(|c| password.push(c));

    password = BASE64_STANDARD.encode(password);
    println!("{}", password);
    if password.len() < length {
        let mut rng = rand::thread_rng();
        while password.len() < length {
            let index = rng.gen_range(0..crypto_len);
            password.push(
                CRYPTO
                    .chars()
                    .nth(index as usize)
                    .expect("Error getting string while completing password length."),
            )
        }
    } else {
        password = password.chars().take(length).collect()
    }
    Ok(password)
}

fn generator_crypto() -> String {
    let mut rng = rand::thread_rng();
    let random_string = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(128)
        .collect();
    println!("{}", random_string);
    random_string
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_hash() {
        assert_eq!(seed_hash("abc"), 590);
        // Add more test cases if needed
    }
    #[test]
    fn test_generator_password() {
        let seed = "wechat";
        let length = 15;
        let result = generator_password(seed, length);
        assert!(result.is_ok());
        let password = result.unwrap();
        assert_eq!(password.len(), length);
    }

    #[test]
    fn test_generator_password_short_length() {
        let seed = "wechat";
        let length = 10;
        let result = generator_password(seed, length);
        assert!(result.is_ok());
        let password = result.unwrap();
        assert_eq!(password.len(), length);
    }

    #[test]
    fn test_generator_password_empty_seed() {
        let seed = "";
        let length = 8;
        let result = generator_password(seed, length);
        println!("{:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn test_generator_password_long_length() {
        let seed = "abcdefg";
        let length = 20;
        let result = generator_password(seed, length);
        assert!(result.is_ok());
        let password = result.unwrap();
        assert_eq!(password.len(), length);
    }
}
