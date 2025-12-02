use std::fs;

fn main() {
     let content = fs::read_to_string("Day1Input.txt")
        .expect("Should have been able to read the file");
}
