use std::fs;

fn has_beam(a_beams: & mut Vec<(i32, i32, i64)>, a_beam: (i32, i32)) -> i32 {

  for i in 0..a_beams.len() {
    if a_beams[i].0 == a_beam.0 && a_beams[i].1 == a_beam.1 {
      return i as i32;
    }
  }

  -1
}

fn main() {
     let content = fs::read_to_string("Day7TestInput.txt")
        .expect("Should have been able to read the file");

    let map: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let mut beams: Vec<_> = Vec::new();

    for i in 0..map.len() {
      for j in 0..map[i].len() {
        if map[i][j] == 'S'
        {
          beams.push((j as i32, i as i32, 1 as i64));
          break;    
        }
      }
    }

    let mut count: i32 = 0;
    loop {

      let mut new_beams: Vec<_> = Vec::new();
      for k in 0..beams.len() {
        beams[k].1 += 1;

        if (beams[k].1 as usize) >= map.len() {
          continue;
        }

        if map[beams[k].1 as usize][beams[k].0 as usize] == '^' {
          count += 1;

          let x0 = beams[k].0 - 1;
          let x1 = beams[k].0 + 1;

          if x0 >= 0
          {
            let bb = has_beam(& mut new_beams, (x0, beams[k].1));
            if bb == -1 {
              new_beams.push((x0, beams[k].1, beams[k].2));
              
            }
            else {
              new_beams[bb as usize].2 +=  beams[k].2;
            }
          }

          if x1 < map[0].len() as i32
          {
            let bb = has_beam(& mut new_beams, (x1, beams[k].1));
            
            if bb == -1 {
              new_beams.push((x1, beams[k].1, beams[k].2));
             
            }
             else {
              new_beams[bb as usize].2 += beams[k].2;
            }
          }
        }
        else {
          new_beams.push(beams[k]);
        } 
      }

       println!("{:?}", new_beams);

      let mut ccc = 0;
      for h in 0..new_beams.len() {
        ccc += new_beams[h].2;
      }

      println!("{ccc}");

      //count2 = ccc;

      if new_beams.is_empty() {
        break;
      }

      beams = new_beams;
    }

   println!("{count}");
}
