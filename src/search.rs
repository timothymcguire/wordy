use std::cmp::{max, Ordering};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::fs::{File, read};

// Returns the key on a line
pub fn get_line(mut file: &File, pos: u64) -> String {
    let max_line_length: u64 = 25_000;
    let num_bytes =  file.metadata().unwrap().len();

    let mut middle_pos: u64 = file.seek(SeekFrom::Start(pos))
        .expect("Could not get position!"); // go to middle

    // Get the minimum possible position
    let min_pos: u64 = if middle_pos < max_line_length {
        0
    } else {
        middle_pos - max_line_length
    };

    // Get the beginning position of the line
    file.seek(SeekFrom::Start(min_pos));
    let mut begin_pos: u64 = min_pos;
    // let mut newlines: u64 = 0;
    for (i, byte) in file.bytes().enumerate() {
        let c = byte.unwrap() as char;
        if c == '\n' {
            begin_pos = i as u64 + min_pos + 1;
        }
        if i as u64 + min_pos == middle_pos {
            break;
        }
    }

    // Get the end position of the line
    let mut end_pos: u64 = begin_pos;
    file.seek(SeekFrom::Start(begin_pos));
    for (i, byte) in file.bytes().enumerate() {
        let c = byte.unwrap() as char;
        if c == '\n' {
            end_pos = i as u64 + begin_pos;
            break;
        }
    }

    file.seek(SeekFrom::Start(begin_pos));
    let mut line = String::new();
    for (i, byte) in file.bytes().enumerate() {
        let c = byte.unwrap() as char;
        if i as u64 + begin_pos == end_pos {
            break;
        } else {
            line.push(c);
        }
    }
    line
}


// Search by key and returns line
pub fn search(mut file: &File, key: String) -> Option<String> {
    let min = 0;
    let num_bytes =  file.metadata().unwrap().len();

    let mut left: u64 = 0;
    let mut right: u64 = num_bytes - 1;
    let mut middle: u64 =  (left + right) / 2;

    let mut line = get_line(&mut file, middle);
    let mut new_key = line.split_whitespace().next().expect("Could not get key.").to_string();

    let mut file_buf: Vec<u8> = Vec::with_capacity(num_bytes as usize);
    file.read_to_end(&mut file_buf);

    loop {
        // Get the line and key for comparison
        // let line = get_line(&mut file, middle);
        // let new_key = line.split_whitespace().next().expect("Could not get key.").to_string();

        // Compare the key
        match key.cmp(&new_key) {
            Ordering::Equal => {
                // println!("{} == {}", key, new_key);
                return Option::Some(line)
            },
            Ordering::Greater => {
                // println!("{} > {}", key, new_key);
                left = middle + 1;
            },
            Ordering::Less => {
                // println!("{} < {}", key, new_key);
                right = middle -1;
            },
        }

        if left > right {
            return Option::None
        }

        middle = (left + right) / 2;
        line = get_line(&mut file, middle);
        new_key = line.split_whitespace().next().expect("Could not get key.").to_string();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // 1739

    #[test]
    fn first_line_beginning_test() {
        // First character of the line
        let test_string: String
            = "  1 This software and database is being provided to you, the LICENSEE, by  "
            .to_string();

        let mut file = File::open("dict/index.noun")
            .expect("Cannot open dict/index.noun");

        assert_eq!(get_line(&mut file, 0), test_string);
    }

    #[test]
    fn first_line_middle_test() {
        // Character in the middle
        let test_string: String
            = "  1 This software and database is being provided to you, the LICENSEE, by  "
            .to_string();

        let mut file = File::open("dict/index.noun")
            .expect("Cannot open dict/index.noun");

        assert_eq!(get_line(&mut file, 25), test_string);
    }

    #[test]
    fn first_line_end_test() {
        // Test the newline character at the end
        let test_string: String
            = "  1 This software and database is being provided to you, the LICENSEE, by  "
            .to_string();

        let mut file = File::open("dict/index.noun")
            .expect("Cannot open dict/index.noun");

        assert_eq!(get_line(&mut file, 74), test_string);
    }

    #[test]
    fn second_line_beginning_test() {
        let test_string: String
            = "  2 Princeton University under the following license.  By obtaining, using  "
            .to_string();

        let mut file = File::open("dict/index.noun")
            .expect("Cannot open dict/index.noun");

        assert_eq!(get_line(&mut file, 75), test_string);
    }

    #[test]
    fn second_line_middle_test() {
        let test_string: String
            = "  2 Princeton University under the following license.  By obtaining, using  "
            .to_string();

        let mut file = File::open("dict/index.noun")
            .expect("Cannot open dict/index.noun");

        assert_eq!(get_line(&mut file, 84), test_string);
    }

    #[test]
    fn second_line_end_test() {
        let test_string: String
            = "  2 Princeton University under the following license.  By obtaining, using  "
            .to_string();

        let mut file = File::open("dict/index.noun")
            .expect("Cannot open dict/index.noun");

        assert_eq!(get_line(&mut file, 149), test_string);
    }
}