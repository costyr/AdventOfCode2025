use std::fs;

fn main() {
     let content = fs::read_to_string("Day4Input.txt")
        .expect("Should have been able to read the file");

    let mut map: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    println!("{:?}", map);
  

  let nn: Vec<[i32; 2]> = Vec::from([[-1, 0], [-1, -1], [0, -1], [1, -1], [1, 0], [1, 1], [0, 1], [-1, 1]]);

  let mut to_remove: Vec<[usize; 2]> = Vec::from([]);

  let mut total1 = 0;
  let mut total2 = 0;
  loop {

  let mut total = 0;
  for i in 0..map.len() {
    for j in 0..map[i].len()
    {
       let cc = map[i][j];

       if cc != '@'
       {
         print!("{cc}");
         continue;
       }
      
       let mut n_count: i32 = 0;
      for vv in &nn
      {
        let x = j as i32 + vv[0];
        let y= i as i32 + vv[1];

        if x >= 0 && x < map[i].len() as i32 && y >= 0 && y < map.len() as i32 
        {
          if map[y as usize][x as usize] == '@' {
            n_count += 1;
          }
        }
       
      }

      //println!("{i} {j} {n_count}");
       
      if n_count < 4
      {
        total += 1;

        print!("x");

       to_remove.push([j, i]);
      }
      else {
        print!("{cc}"); 
      }


    }
    println!();
  }

  if to_remove.len() == 0 {
    break;
  }

  for bb in &to_remove
  {
    map[bb[1]][bb[0]] = '.';
  }

   to_remove.clear();

   if total1 == 0 {
    total1 = total;
   }

  total2 += total;

}

  println!("{total1}\n{total2}");
}
