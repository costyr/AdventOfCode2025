use std::fs;
use std::collections::HashMap;

fn find_all_paths(a_devices: &HashMap<String, Vec<String>>, a_cr: &String, a_max: & mut i32) {
  
  if a_cr == "out" {
    *a_max += 1;
    return;
  }

  let connections = a_devices.get(a_cr);

  if connections != None {
    for i in 0..connections.unwrap().len() {
      find_all_paths(a_devices, &connections.unwrap()[i], a_max);
    }
  }
}

fn find_all_paths2(a_cache: & mut HashMap<String, i64>, a_devices: &HashMap<String, Vec<String>>, a_cr: &String, a_end: &String, a_max: & mut i32, path: &Vec<String>) -> i64 {
  
  //println!("\n\n------- {:?} {:?} -----------",a_cr, path);

  if a_cr == a_end {

    //println!("{:?}", path);

    /*let mut has_dac = false;
    let mut has_fft = false; 
    for j in 0..path.len() {
      if path[j] == "dac" {
        has_dac = true;
      }

      if path[j] == "fft" {
        has_fft = true;
      }
    }

    if has_dac && has_fft {
      *a_max += 1;
      println!("{:?}", *a_max);
      return 1;
    }*/
    *a_max += 1;
    //println!("{:?}", *a_max);

    return 1;
  }

  let uu = a_cache.get(a_cr);

  if uu != None {
    //println!("{:?} {:?} {:?}", a_cr, uu, path);

    /*   let mut has_dac = false;
    let mut has_fft = false; 
    for j in 0..path.len() {
      if path[j] == "dac" {
        has_dac = true;
      }

      if path[j] == "fft" {
        has_fft = true;
      }
    }

    if has_dac && has_fft {
      *a_max += 1;
      println!("{:?}", *a_max);
      return 1;
    }*/

    return *uu.unwrap();
  }

  let connections = a_devices.get(a_cr);

   let mut new_count: i64 = 0;

  if connections != None {
    for i in 0..connections.unwrap().len() {

      //let cr = &connections.unwrap()[i];

      let mut new_path = path.clone(); 

      new_path.push(connections.unwrap()[i].clone());

      let count = find_all_paths2(a_cache, a_devices, &connections.unwrap()[i], a_end, a_max, &new_path);

      new_count += count;
    }

     if a_cache.get(a_cr) == None {
        //println!("{:?} {:?}", a_cr, new_count);
        a_cache.insert(a_cr.clone(), new_count);  
      }
      else {
         //println!("{:?} {:?}", a_cr, new_count);
      }
  }

  //println!("####### {:?} {:?} {:?} #######",a_cr, path, new_count);

  new_count
}

fn find_all_paths3(a_devices: &HashMap<String, Vec<String>>, a_cr: &String, a_end: &String)->i64 {
  let mut cache: HashMap<String, i64> = HashMap::new();

   let mut max:  i32 = 0;

   let mut path: Vec<String> = Vec::new();

  return find_all_paths2(& mut cache, a_devices, a_cr, a_end, & mut max, &path);
}

fn main() {
     let content = fs::read_to_string("Day11Input.txt").expect("Should have been able to read the file");

     let devices: HashMap<String, Vec<String>> = content.lines().map(| line | {
       let cc: Vec<_> = line.split(": ").map(|v | v.to_string()).collect();

       let bb: Vec<String> = cc[1].split(" ").map(| v | v.to_string()).collect();

       (cc[0].clone(), bb)
       }).collect::<HashMap<String, Vec<String>>>();


     println!("{:?}", devices);

     let count0 = find_all_paths3(&devices, &"you".to_string(), &"out".to_string());

     let count1 = find_all_paths3(&devices, &"svr".to_string(), &"fft".to_string());

     let count2 = find_all_paths3(&devices, &"fft".to_string(), &"dac".to_string());

     let count3 = find_all_paths3(&devices, &"dac".to_string(), &"out".to_string());

     println!("{count0}\n{:?}", count1 * count2 * count3);   
}
