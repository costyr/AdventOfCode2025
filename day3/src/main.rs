use std::fs;

fn find_max(a_voltages : &[u8], a_index: usize, a_power: i64, a_level: i32,  a_max: &mut i64) {
  
  //println!("{a_power} {a_level} {a_max}");

  if a_level == 12
  {
    if a_power > *a_max {
      *a_max = a_power;
    }
  
    //println!("{a_power}");
    //println!("{a_power} {a_level} {a_max}");
    return;
  }

  let mut i: usize = a_index;
  let mut max: i64 = 0; 
  while i < a_voltages.len() {

    let b: String = (a_voltages[i] as char).to_string();

    let cc: i64 = b.parse::<i64>().unwrap();

    if cc > max {
      max = cc;
    }
    else {
       i += 1;
       continue; 
    }

    let power = a_power * 10 + cc;

    find_max(a_voltages, i + 1, power, a_level + 1, a_max);

    i += 1;
  }

}

fn max_power(a_voltages : &[u8]) -> i64{
  let mut max: i64 = 0;
  find_max(a_voltages, 0, 0, 0, &mut max);

  println!("{max}");

  max  
}

fn main() {
     let content = fs::read_to_string("Day3Input.txt")
        .expect("Should have been able to read the file");

    let banks = content.split("\r\n");

    let mut total: i32 = 0;
    let mut total2: i64 = 0;
    for bank in banks {

      println!("{bank}");

      let voltages = bank.as_bytes();

      total2 += max_power(voltages);

      let mut i = 0;
      let mut max = 0;
      while i < voltages.len() {

        let mut j = i + 1;
        while j < voltages.len()
        {
          let b: String = (voltages[i] as char).to_string();
          let a: String = (voltages[j] as char).to_string();

          //println!("{a} {b}");

          let power = b.parse::<i32>().unwrap() * 10 + a.parse::<i32>().unwrap();

          if power > max
          {
            max = power;
          }

          j += 1;
        }
        i += 1;
      }

      println!("{max}");

      total += max;
    }

    println!("{total} {total2}");
}
