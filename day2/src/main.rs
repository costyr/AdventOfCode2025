use std::fs;
extern crate is_even;
use is_even::IsEven;

fn is_valid_id(a_id: i64) -> bool {
  
  let ss = a_id.to_string();

  if !(ss.len() as i32).is_even() {
    return true;
  }

  let first = (&ss[0..ss.len()/2]).parse::<i64>().unwrap();
  
  let second = (&ss[ss.len()/2..]).parse::<i64>().unwrap();

  if first == second {

    println!("{a_id}");

    return false;
  }
    
  true
}

fn is_valid_id2(a_id: i64) -> bool {
  
  //println!("{a_id}");

  let ss = a_id.to_string();

  let ten:i64 = 10; 

  let mut i: i32 = 1;
  while i <= (ss.len() as i32) / 2 {

    let dd = ten.pow(i as u32);

    let gg = a_id % dd;

    if gg < dd / ten {
      i += 1;
      continue;
    }

    let mut nn = a_id / dd;

    //println!("{i} {nn} {gg}");

    let mut found: bool = true;
    while nn > 0 {
       let jj = nn % dd;

       if jj != gg {
         found = false;
         break;
       }

       // println!("{jj}");

        nn = nn / dd;
    }

    if found
    {
      println!("{a_id}");
      return false;
    }

    i += 1;
  }
   
  true
}

fn count_invalid(a_start: i64, a_end: i64) -> (i64, i64) {
  
  println!("{a_start}-{a_end}");

  let mut current = a_start;

  let mut total: i64 = 0;
  let mut total2: i64 = 0;
  while current <= a_end
  {
    if !is_valid_id(current) {
      total += current; 
    }

    if !is_valid_id2(current) {
      total2 += current; 
    }

    current += 1;
  } 

  (total, total2)
}

fn main() {
    let content = fs::read_to_string("Day2Input.txt")
        .expect("Should have been able to read the file");

    let ids = content.split(",").map(|v| {
      let tt: Vec<&str> = v.split("-").collect();
      ( tt[0].parse::<i64>().unwrap(), tt[1].parse::<i64>().unwrap() )
    });

    //is_valid_id2(60606);

    let mut total: i64 = 0;
    let mut total2: i64 = 0;
    for id in ids {
      let (start, end) = id;

      let (t, t2) = count_invalid(start, end);

      total += t;
      total2 += t2;
    }

    println!("{total}\n{total2}");
}
