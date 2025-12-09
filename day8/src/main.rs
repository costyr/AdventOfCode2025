use std::fs;

fn main() {
    let content = fs::read_to_string("Day9TestInput.txt")
        .expect("Should have been able to read the file");

    let map: Vec<(i32, i32)> = content.lines().map(|line| { 
      let gg: Vec<i64> = line.split(",").map(|b| b.parse::<i32>().unwrap()).collect();
      (gg[0], gg[1])
    }).collect();

    println!("{:?}", map);
}
