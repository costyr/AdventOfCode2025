use std::fs;
use std::cmp;

fn intersect_intervals(a_start1:i64, a_end1: i64, a_start2: i64, a_end2: i64) -> (i64, i64) {
  if a_start2 > a_end1 || a_start1 > a_end2 {
    (0, 0)
  }
  else {
     let start = cmp::min(a_start1, a_start2);
     let end = cmp::max(a_end1, a_end2);

     (start, end)  
  }
}

fn main() {
    let content = fs::read_to_string("Day5Input.txt")
        .expect("Should have been able to read the file");

    let database: Vec<&str> = content.split("\r\n\r\n").collect();

    let intervals: Vec<_> = database[0].lines().map(|v| {
      let gg: Vec<i64> = v.split("-").map(|c| {
        c.parse::<i64>().unwrap()
      }).collect();

      (gg[0], gg[1])
    }).collect();

    //println!("{:?}", intervals);

    let ingredients: Vec<i64> = database[1].lines().map(|v| v.parse::<i64>().unwrap()).collect();

    let mut count: i32 = 0;
    for i in ingredients {
      for j in &intervals {
        let (start, end) = j;

        if i >= *start && i <= *end {
          count += 1;
          break;
        }
      }
    }

    println!("{count}");

    let mut total2: i64= 0;
    let mut kk = intervals;

    loop {

    let mut gg: Vec<(i64, i64)> = Vec::from([]);

    let last = kk.pop();

    let mut cr = (0, 0);

    if last != None
    {
       cr = last.unwrap();
    }

    let mut found: bool = false;
    for j in kk {

       let (start0, end0) = cr;

       let (start, end) = j;

      let nn = intersect_intervals(start0, end0, 
        start, end);

      if nn == (0, 0)
      {
         gg.push(j);    
      }
      else 
      {
        cr = nn;
        found = true;   
      }

      //println!("({start0}-{end0} + ({start}-{end}) = {:?}", nn);
     
    }

    //println!("{:?}", cr);

    if !found {

      total2 += cr.1 - cr.0 + 1;

      if gg.is_empty()
      {
        break;
      }
    }
    else {
      gg.push(cr);
    }

    kk = gg;

  }

  println!("{total2}");

}
