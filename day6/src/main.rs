use std::fs;

fn main() {
    let content = fs::read_to_string("Day6Input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = content.lines().collect();

    //println!("{:?}", lines);

     let mut numbers_as_strings: Vec<String> = Vec::new();

     for i in 0..lines.len() - 1 {
       let bb = lines[i].as_bytes();

       //println!("{:?}", bb);

       if numbers_as_strings.is_empty()
       {
         for _j in 0..bb.len() {
           let pp: String = String::new(); 
           numbers_as_strings.push(pp);
         }
       }

       for j in 0..bb.len() {
         numbers_as_strings[j].push(bb[j] as char);
       }
     }

    //println!("{:?}", numbers_as_strings);

    let mut numbers: Vec<Vec<i64>> = Vec::new();
    for i in 0..lines.len() - 1 {

      //println!("{i} {:?}", lines[i]);

      let gg: Vec<&str> = lines[i].trim().split(" ").collect();

      let mut bb: Vec<i64> = Vec::new();
      
      for j in 0..gg.len() {
        if gg[j].is_empty() {
          continue;
        }

        bb.push(gg[j].parse::<i64>().unwrap());
      };

      numbers.push(bb);
    }

    let kk: Vec<&str> = lines[lines.len() - 1].split(" ").collect();
    
    let mut ops: Vec<char> = Vec::new();
    for i in 0..kk.len() {

      if kk[i].is_empty() {
        continue;
      }

    
      ops.push(kk[i].as_bytes()[0] as char);
    };

    //println!("{:?}", numbers); 
    //println!("{:?}", ops);

    let mut total: Vec<i64> = Vec::new();

    for op in &ops {
      if *op == '*'
      {
        total.push(1);
      }
      else {
        total.push(0);
      }
    }

    for nn in numbers {
      for i in 0..nn.len() {
        if ops[i] == '*'
        {
          total[i] *= nn[i];
        }
        else {
           total[i] += nn[i]; 
        }
      }
    }

    let mut total2: i64 = 0;
    for hh in total {
      total2 += hh;
    }

    println!("{total2}");


     let mut  xx = 0;
    let mut sum: i64 = 0;
    let mut mmm: i64 = 1;
    let mut total3:i64 = 0;
    for mm in numbers_as_strings {
      let vv = mm.trim();

      //println!("{vv}");

      if vv.is_empty()
      {
         if ops[xx] == '*'
        {
          total3 += mmm;

          //println!("{mmm}");  
        }
        else {
          total3 += sum;

           //println!("{sum}");  
        }

        xx += 1;

        sum = 0;
        mmm = 1;
      }
      else {
        let rr = vv.parse::<i64>().unwrap();

        if ops[xx] == '*'
        {
          mmm *= rr;
        }
        else {
          sum += rr;  
        }
      } 
    } 

     if ops[xx] == '*'
        {
          total3 += mmm;

          //println!("{mmm}");  
        }
        else {
          total3 += sum;

           //println!("{sum}");  
        } 

    println!("{total3}");
}
