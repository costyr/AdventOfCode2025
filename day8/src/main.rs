use std::{collections::HashSet, fs};

fn compute_dist(pt1: (i64, i64, i64), pt2: (i64, i64, i64)) -> i64 {
  let dist = ((pt2.0 - pt1.0).pow(2) + (pt2.1 - pt1.1).pow(2) + (pt2.2 - pt1.2).pow(2)).isqrt();
  dist
}

fn sort_pairs_by_dist(boxes: &Vec<(i64, i64, i64)>) -> Vec<((i64, i64, i64), (i64, i64, i64))> {
  let mut gg: Vec<((i64, i64, i64), (i64, i64, i64))> = Vec::new();

   for i in 0..boxes.len() {
    for j in i + 1..boxes.len() {
      let p1 = boxes[i];
      let p2 = boxes[j];
      gg.push((p1, p2));
    }
  }

  gg.sort_by(| a, b | {
    let dist1 = compute_dist(a.0, a.1);   
    let dist2 = compute_dist(b.0, b.1);

    if dist1 < dist2 {
      return std::cmp::Ordering::Less;
    }
    else {
      return std::cmp::Ordering::Greater;  
    }
  });

  gg
}

fn _find_nearest(boxes: &Vec<(i64, i64, i64)>, connected: &HashSet<String>) -> Vec<(i64, i64, i64)> {
  
  let mut gg: Vec<(i64, i64, i64)>= Vec::new();

  let mut min_dist: i64 = i64::MAX;
  for i in 0..boxes.len() {
    for j in i + 1..boxes.len() {
      let dist = compute_dist(boxes[i], boxes[j]);

      //println!("{:?} {:?} {dist}", boxes[i], boxes[j]);

      let key1 = _pt_tostring(boxes[i]);
      let key2 = _pt_tostring(boxes[j]);

      if (connected.get(&key1) != None) && (connected.get(&key2) != None) {
        continue;
      }

      if dist < min_dist  
      {
        gg.clear();
        min_dist = dist;
        gg.push(boxes[i]);
        gg.push(boxes[j]);
      }
    }  
  }

  println!("{min_dist}");

  gg
}

fn _pt_tostring(pt: (i64, i64, i64)) -> String {
  let mut ss: String = pt.0.to_string();
  ss += "_";
  ss += &pt.1.to_string();
  ss += "_";
  ss += &pt.2.to_string();
  ss
}

fn contains_box(boxes: &Vec<(i64, i64, i64)>, pt: (i64, i64, i64)) -> bool {
  for i in 0..boxes.len() {
    if boxes[i] == pt {
      return true;
    }
  }
  false
}

fn merge_groups(box1: &Vec<(i64, i64, i64)>, box2: &Vec<(i64, i64, i64)>) -> Vec<(i64, i64, i64)> {
  let mut gg: Vec<(i64, i64, i64)> = Vec::new();
  for i in 0..box1.len() {
    for j in 0..box2.len() {
      if box1[i] == box2[j] {

        //println!("Last:  {:?}", box1[i]);

       gg.extend(box1.iter().cloned());

       for k in 0..box2.len() {
         if !contains_box(&gg, box2[k]) {
           gg.push(box2[k]);
         }
       }
       return gg;
      }
    }
  }  
  gg
}

fn compact_groups(a_groups: & mut Vec<Vec<(i64, i64, i64)>>) {
   loop {
      let mut found = false;

      //println!("{:?}", a_groups);

      for i in 0..a_groups.len() {
        for j in i + 1..a_groups.len() {
           let nn = merge_groups(&a_groups[i], &a_groups[j]);

           //println!("{i} {j} : {:?}", nn);

           if !nn.is_empty() {
             a_groups.remove(i);
             if j > i {
               a_groups.remove(j - 1);
             }
             else {
               a_groups.remove(j); 
             }
             a_groups.push(nn);
             found = true;
             break;             
           }
        }

        if found {
          break;
        }
      }

      /*  if groups.len() == 2 {
        break;
      }*/

      if !found {
        break;
      }

     
    }
}

fn _to_group_lens(a_groups: &Vec<Vec<(i64, i64, i64)>>) -> Vec<i64> {
  let mut pp: Vec<i64> = Vec::new();

  for i in 0..a_groups.len() {

    if a_groups[i].len() == 2 {
      //println!("{:?}", a_groups[i]);
    }

    pp.push(a_groups[i].len() as i64);
  }

  pp
}

fn find_group(a_groups: &Vec<Vec<(i64, i64, i64)>>, group: (i64, i64, i64)) ->Option<usize> {
  for i in 0..a_groups.len() {
    for j in 0..a_groups[i].len() {
      if a_groups[i][j] == group {
        return Some(i);
      }
    }
  }

  None
}

fn solve_part2(a_sorted_pairs: &Vec<((i64, i64, i64), (i64, i64, i64))>, a_total_junction_boxes: usize) {
   let mut groups: Vec<Vec<(i64, i64, i64)>> = Vec::new();

    loop {
      for i in 0..a_sorted_pairs.len() {

        let first = a_sorted_pairs[i].0;
        let second = a_sorted_pairs[i].1;

        //println!("{:?} {:?}", first, second);

        let o1 = find_group(&groups, first);
        let o2 = find_group(&groups, second);

        if o1 == None && o2 == None {

          let mut ff: Vec<(i64, i64, i64)> = Vec::new();

          ff.push(first);
          ff.push(second);

          groups.push(ff);
        } 
        else if o1 == None && o2 != None {
          groups[o2.unwrap()].push(first);      
        }
        else if o1 != None && o2 == None {
          groups[o1.unwrap()].push(second);    
        }
        else {

          if o1.unwrap() == o2.unwrap() {
            continue;
          }

          for k in 0..groups[o2.unwrap()].len() {

            let nn = groups[o2.unwrap()][k];

            groups[o1.unwrap()].push(nn);   
          }

          groups.remove(o2.unwrap());

          continue;
        } 

         if groups.len() == 1 && groups[0].len() == a_total_junction_boxes
         {
          let total = first.0 * second.0;
          println!("{total}");
          break;
         }
      }

      if groups.len() == 1
      {
        break;
      }
  }
}

fn main() {
     let content = fs::read_to_string("Day8Input.txt")
        .expect("Should have been able to read the file");

    let junction_boxes: Vec<(i64, i64, i64)> = content.lines().map(|v| {
      let cc:Vec<i64> = v.split(",").map(|b| b.parse::<i64>().unwrap()).collect();   
      (cc[0], cc[1], cc[2])
    }).collect();

    //println!("{:?}", junction_boxes);

    let mut groups: Vec<Vec<(i64, i64, i64)>> = Vec::new();

    let sorted_pairs = sort_pairs_by_dist(&junction_boxes);

    //println!("{:?}", sorted_pairs);

    let count = 1000;
    for pp in 0..count {
    
      let mut ff: Vec<(i64, i64, i64)> = Vec::new();

      ff.push(sorted_pairs[pp].0);
      ff.push(sorted_pairs[pp].1);

      //println!("{:?}", ff);

       let mut found = false;
      for k in 0..ff.len() {

        let bb = ff[k];
        
        for i in 0..groups.len() {

          found = contains_box(&groups[i], bb);

          if found {
            if k == 0 {
              if !contains_box(&groups[i], ff[1]) {
                groups[i].push(ff[1]);
              }
            }
            else {
              if !contains_box(&groups[i], ff[0]) {
                groups[i].push(ff[0]);
              }
            }
            break;
          }

        }

        if found {
          break;
        }
    }

     if !found {

          groups.push(ff);
        }

        //println!("{:?}", to_group_lens(&groups));

    }

     compact_groups(& mut groups);

    let mut ooo: Vec<i64> = Vec::new();
    for gr in groups {
      ooo.push(gr.len() as i64);
    }

    ooo.sort_by(|a, b| 
      { if a > b {
      return std::cmp::Ordering::Less;
    }
    else {
      return std::cmp::Ordering::Greater;  
    }});

     let mut total: i64 = 1;
    for i in 0..3 {
      total *= ooo[i];
    }

    //println!("{:?}", ooo);
    println!("{total}");

    solve_part2(&sorted_pairs, junction_boxes.len());

}
