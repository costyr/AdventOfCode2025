use std::fs;

type Config = (Vec<i32>, Vec<Vec<i32>>, Vec<i32>);

fn _find_min(a_buttons: &Vec<Vec<i32>>, a_index: usize, a_cr: &Vec<i32>, a_count: usize, a_min: & mut usize, a_target: &Vec<i32>) {
  
  if a_index >= a_buttons.len() {
    return;
  }

  if a_count > *a_min {
    return;
  }

  let mut found: bool = true;
  for i in 0..a_target.len() {
    if a_target[i] != a_cr[i] {
      found = false;
      break;
    }
  }

  if found {
   if a_count < *a_min {
     *a_min = a_count; 
   }
   return;
  }

  for i in a_index..a_buttons.len() {

    let mut new_cr = a_cr.clone();
    for j in 0..a_buttons[i].len() {
      let bb = a_buttons[i][j];

      if new_cr[bb as usize] == 1 {
        new_cr[bb as usize] = 0;
      }
      else {
        new_cr[bb as usize] = 1;
      }
    }

    _find_min(&a_buttons, a_index + 1, &new_cr, a_count + 1, a_min, a_target);

  }
}


fn find_min2(a_buttons: &Vec<Vec<i32>>, a_cr: &Vec<i32>, a_count: usize, a_min: & mut usize) -> bool {
  
  if a_count > *a_min {
    return false;
  }

  let mut found: bool = true;
  for k in 0..a_cr.len() {

    if a_cr[k] != 0 {
      found = false;
    }

     if a_cr[k] < 0
     {
       return false;
     }
  }

  if found {
    if a_count < *a_min {
      *a_min = a_count;
    }

    println!("{:?} {:?}", a_cr, *a_min);

    return true;
  }

  for i in 0..a_buttons.len() {
      let mut new_cr = a_cr.clone();
    for j in 0..a_buttons[i].len() {
      let bb = a_buttons[i][j];

      new_cr[bb as usize] -= 1;
    }

    find_min2(a_buttons, &new_cr, a_count + 1, a_min);
  }

  false
}

fn main() {
     let content = fs::read_to_string("Day10Input.txt")
        .expect("Should have been able to read the file");

     let machines: Vec<Config> = content.lines().map(| line | {
       let cc: Vec<_> = line.split(" ").collect();

       let tt = cc[0].trim_matches('[').trim_end_matches(']').as_bytes();

       let mut oo: Vec<i32> = Vec::new();
       for i in 0..tt.len() {
         let nn = tt[i] as char;
         if nn == '.' {
            oo.push(0);
         }
         else {
            oo.push(1);
         } 
       } 


       let mut bb: Vec<Vec<i32>> = Vec::new();
       for i in 1..cc.len() - 1 {
         let hh: Vec<i32> = cc[i].trim_matches('(').trim_end_matches(')').split(",").map(| v | v.parse::<i32>().unwrap()).collect();
         bb.push(hh);
       }

       let gg: Vec<i32> = cc[cc.len() - 1].trim_matches('{').trim_end_matches('}').split(",").map(| v | v.parse::<i32>().unwrap()).collect();

       (oo, bb, gg)

    }).collect();

    println!("{:?}", machines);

    /*let mut total: i32 = 0;
    for i in 0..machines.len() {
      let mut min = i32::MAX as usize;

      let mut cr: Vec<i32> = Vec::new();

      for _i in 0..machines[i].0.len() {
        cr.push(0);
      }

      find_min(&machines[i].1, 0, &cr, 0, & mut min, &machines[i].0);

      println!("{:?} {:?} {min}", machines[i].0, cr);

      total += min as i32;
    }

    println!("{total}");*/

    let mut total2: i32 = 0;
    for i in 0..machines.len() {
      let mut min = i32::MAX as usize;

      let cr: Vec<i32> = machines[i].2.clone();

      find_min2(&machines[i].1, &cr, 0, & mut min);

      println!("{:?} {:?} {min}", machines[i].2, cr);

      total2 += min as i32;
    }

    println!("{total2}");

}
