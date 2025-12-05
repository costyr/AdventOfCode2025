use std::fs;

fn main() {
    let content = fs::read_to_string("Day6Input.txt")
        .expect("Should have been able to read the file");
}
