use std::fs;

fn main() {
     let content = fs::read_to_string("Day1Input.txt")
        .expect("Should have been able to read the file");

    let insts = content.split("\r\n").map(|v| {
      let tt= &v[0..1];
      let bb = &v[1..];
      ( tt, bb.parse::<i32>().unwrap() )
    });

    let mut password: i32 = 50;
    let mut count: i32 = 0;
    let mut count2: i32 = 0;
    for inst in insts {
      let (i, c) = inst;

       count2 += c / 100;

       let r = c % 100;

      if i == "L" {

        let d = password - r;

        if password != 0 && d <= 0 {
          count2 += 1;
        }

        password -= c;

        password %= 100;

        if password < 0
        {
          password = 100 + password;
        }
      }
      else  {

         let d = password + r;

        if d > 99 {
          count2 += 1;
        }

        password += c;

        password %= 100;

        if password > 99 {
          password = password - 100;
        }
      }

      if password == 0 {
        count += 1;
      }

       println!("{i} {c} -> {password} {count2}");
      
    }

    println!("{count}\n{count2}");
}
