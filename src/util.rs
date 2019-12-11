use rand::{Rng, thread_rng};

pub type ID = String;

const ALPHABET: &'static [u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012356789_-";

pub fn generate_id() -> ID {
    let mut s = String::new();
    for _ in 0..11 {
        s.push(ALPHABET[thread_rng().gen_range(0, ALPHABET.len())] as char)
    }
    s
}

#[cfg(test)]
mod tests {
    use super::generate_id;

    #[test]
    fn test_generate_id_length() {
        assert!(generate_id().len() == 11)
    }
}
