// file1.rs

// This exercise is simply to read a file.

// I AM NOT DONE

use std::fs;

fn read_the_file() -> String {
    fs::read_to_string("exercises/io/assets/file1.txt").unwrap()
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test () {
        let text_from_file = read_the_file();

        assert!(guess_string == secret_message, "You have not correctly entered the secret message!");
    }
}