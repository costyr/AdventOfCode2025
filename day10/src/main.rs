use std::fs;
use std::collections::HashSet;

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


fn _find_min2(a_buttons: &Vec<Vec<i32>>, a_cr: &Vec<i32>, a_count: usize, a_min: & mut usize) -> bool {
  
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
       println!("{:?}", a_cr);
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

    _find_min2(a_buttons, &new_cr, a_count + 1, a_min);
  }

  false
}

#[derive(Clone)]
struct Equation {
  vars: Vec<usize>, // button indices appearing with coefficient 1
  rhs: i64,
}

fn build_equations(buttons: &Vec<Vec<i32>>, target: &Vec<i32>) -> Vec<Equation> {
  let m = target.len();
  let mut eqs: Vec<Equation> = Vec::with_capacity(m);
  // For each counter i, equation: sum over buttons j that contain i of x_j = target[i]
  for i in 0..m {
    let mut vars: Vec<usize> = Vec::new();
    for (j, btn) in buttons.iter().enumerate() {
      if btn.contains(&(i as i32)) { vars.push(j); }
    }
    vars.sort_unstable();
    eqs.push(Equation { vars, rhs: target[i] as i64 });
  }
  eqs
}

fn print_initial_equations(eqs: &Vec<Equation>) {
  for eq in eqs {
    let left = eq.vars.iter().map(|v| format!("x{}", v)).collect::<Vec<_>>().join(" + ");
    println!("{} = {}", left, eq.rhs);
  }
}

fn reduce_and_solve(buttons: &Vec<Vec<i32>>, target: &Vec<i32>) -> Option<Vec<i64>> {
  let n = buttons.len();
  let mut eqs = build_equations(buttons, target);
  let mut known: Vec<Option<i64>> = vec![None; n];

  // To prevent generating duplicate derived equations
  let mut seen: HashSet<(Vec<usize>, i64)> = HashSet::new();
  for e in &eqs { seen.insert((e.vars.clone(), e.rhs)); }

  // Print only the initial equations, as requested
  print_initial_equations(&eqs);

  loop {
    // Sort by count of unknowns ascending
    eqs.sort_by(|a, b| {
      let au = a.vars.iter().filter(|&&v| known[v].is_none()).count();
      let bu = b.vars.iter().filter(|&&v| known[v].is_none()).count();
      au.cmp(&bu)
    });

    let mut progress = false;

    // Try to solve any equation with exactly one unknown
    for eq in &eqs {
      let mut unknown_vars: Vec<usize> = Vec::new();
      let mut residual = eq.rhs;
      for &v in &eq.vars {
        if let Some(val) = known[v] { residual -= val; } else { unknown_vars.push(v); }
      }
      if unknown_vars.len() == 1 {
        let v = unknown_vars[0];
        if residual < 0 { return None; }
        known[v] = Some(residual);
        progress = true;
      }
    }

    if known.iter().all(|o| o.is_some()) {
      return Some(known.into_iter().map(|o| o.unwrap()).collect());
    }

    if progress { continue; }

    // If stuck, derive a new equation by subtracting a smaller equation from a larger one
    // when the smaller's var set is a subset of the larger's var set.
    // NewEq: vars = big \ small; rhs = big.rhs - small.rhs.
    let len = eqs.len();
    let mut derived_added = false;
    'outer: for i in 0..len {
      for j in (i+1)..len {
        let (smaller, bigger) = if eqs[i].vars.len() <= eqs[j].vars.len() { (&eqs[i], &eqs[j]) } else { (&eqs[j], &eqs[i]) };
        // Check subset
        let mut it_small = 0usize; let mut it_big = 0usize; let mut is_subset = true;
        while it_small < smaller.vars.len() && it_big < bigger.vars.len() {
          if smaller.vars[it_small] == bigger.vars[it_big] { it_small += 1; it_big += 1; }
          else if smaller.vars[it_small] > bigger.vars[it_big] { it_big += 1; }
          else { is_subset = false; break; }
        }
        if is_subset && it_small == smaller.vars.len() {
          // Build difference set
          let mut diff: Vec<usize> = Vec::new();
          let mut p_small = 0usize;
          for &v in &bigger.vars {
            if p_small < smaller.vars.len() && smaller.vars[p_small] == v { p_small += 1; }
            else { diff.push(v); }
          }
          let rhs = bigger.rhs - smaller.rhs;
          if !diff.is_empty() {
            let key = (diff.clone(), rhs);
            if !seen.contains(&key) {
              seen.insert(key);
              eqs.push(Equation { vars: diff, rhs });
              derived_added = true;
              break 'outer;
            }
          }
        }
      }
    }

    if !derived_added {
      // No progress possible under constraints, but user guarantees solvable uniquely
      return None;
    }
  }
}

// ---------------- Row Echelon Form (REF) solver with minimal natural solution sum ----------------
fn build_matrix(buttons: &Vec<Vec<i32>>, m: usize, n: usize) -> Vec<Vec<f64>> {
  let mut a = vec![vec![0.0f64; n]; m];
  for (j, btn) in buttons.iter().enumerate() {
    for &idx in btn { a[idx as usize][j] = 1.0; }
  }
  a
}

fn rref_f64(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<(Vec<Vec<f64>>, Vec<f64>, Vec<Option<usize>>)> {
  let m = a.len();
  if m == 0 { return Some((a, b, vec![])); }
  let n = a[0].len();
  let mut row = 0usize;
  let mut pivot_cols: Vec<Option<usize>> = vec![None; m];
  let eps = 1e-12f64;
  for col in 0..n {
    if row >= m { break; }
    // Find pivot row
    let mut pr = None;
    for r in row..m { if a[r][col].abs() > eps { pr = Some(r); break; } }
    if pr.is_none() { continue; }
    let r0 = pr.unwrap();
    if r0 != row { a.swap(r0, row); b.swap(r0, row); }
    // Normalize pivot to 1
    let piv = a[row][col];
    if piv.abs() < eps { return None; }
    for c in col..n { a[row][c] /= piv; }
    b[row] /= piv;
    // Eliminate in all other rows
    for r in 0..m {
      if r == row { continue; }
      let factor = a[r][col];
      if factor.abs() <= eps { continue; }
      for c in col..n { a[r][c] -= factor * a[row][c]; }
      b[r] -= factor * b[row];
    }
    pivot_cols[row] = Some(col);
    row += 1;
  }
  // Check consistency: any zero row with non-zero RHS is inconsistent
  for r in 0..m {
    let all_zero = (0..n).all(|c| a[r][c].abs() <= eps);
    if all_zero && b[r].abs() > eps { return None; }
  }
  Some((a, b, pivot_cols))
}

fn solve_by_ref_min_sum(buttons: &Vec<Vec<i32>>, target: &Vec<i32>) -> Option<Vec<i64>> {
  let m = target.len();
  let n = buttons.len();
  if m == 0 || n == 0 { return Some(vec![]); }

  let a = build_matrix(buttons, m, n);
  let b: Vec<f64> = target.iter().map(|&t| t as f64).collect();
  let (r, y, pivmap) = rref_f64(a, b)?;
  let eps = 1e-9f64;

  // Determine pivot columns and free columns
  let mut is_pivot_col = vec![false; n];
  for &pc in &pivmap { if let Some(c) = pc { is_pivot_col[c] = true; } }
  let free_cols: Vec<usize> = (0..n).filter(|&c| !is_pivot_col[c]).collect();
  let pivot_rows: Vec<usize> = (0..m).filter(|&ridx| pivmap[ridx].is_some()).collect();

  // Upper bounds for free vars from non-negativity of pivot variables
  let mut ub: Vec<usize> = vec![usize::MAX; free_cols.len()];
  for &ridx in pivot_rows.iter() {
    let rhs = y[ridx];
    for (k, &fc) in free_cols.iter().enumerate() {
      let coeff = r[ridx][fc];
      if coeff > eps {
        let bound = (rhs / coeff).floor();
        if bound.is_finite() && bound >= 0.0 {
          let bb = bound as i64;
          if bb >= 0 { ub[k] = ub[k].min(bb as usize); }
        }
      }
    }
  }
  for k in 0..ub.len() { if ub[k] == usize::MAX { ub[k] = 0; } }

  // DFS over free variables within bounds; compute pivot values, check feasibility; minimize sum
  let mut best_sum: Option<i64> = None;
  let mut best_sol: Vec<i64> = vec![0; n];

  fn dfs_assign(
    idx: usize,
    free_cols: &Vec<usize>,
    r: &Vec<Vec<f64>>, y: &Vec<f64>, pivmap: &Vec<Option<usize>>,
    buttons: &Vec<Vec<i32>>, target: &Vec<i32>,
    current_free: &mut Vec<i64>,
    current_sum: i64,
    best_sum: &mut Option<i64>, best_sol: &mut Vec<i64>,
  ) {
    // Pruning
    if best_sum.is_some() && current_sum >= best_sum.unwrap() {
        return;
    }

    if idx == free_cols.len() {
      // Build full solution: first set free vars, then compute pivots
      let n = best_sol.len();
      let m = target.len();
      let mut x: Vec<f64> = vec![0.0; n];
      for (k, &fc) in free_cols.iter().enumerate() { x[fc] = current_free[k] as f64; }
      
      let mut pivots_sum = 0.0;
      let mut pivots_ok = true;

      for rrow in 0..pivmap.len() {
        if let Some(pc) = pivmap[rrow] {
          let mut val = y[rrow];
          for (k, &fc) in free_cols.iter().enumerate() {
            val -= r[rrow][fc] * (current_free[k] as f64);
          }
          // Must be non-negative integer
          if val < -1e-9 { pivots_ok = false; break; }
          let rounded = val.round();
          if (val - rounded).abs() > 1e-6 { pivots_ok = false; break; }
          x[pc] = rounded.max(0.0);
          pivots_sum += x[pc];
        }
      }

      if !pivots_ok { return; }

      let total_f64 = current_sum as f64 + pivots_sum;
      if best_sum.is_some() && total_f64 >= best_sum.unwrap() as f64 {
          return;
      }

      // Verify A x == b exactly and natural numbers
      let xi: Vec<i64> = x.iter().map(|&v| v as i64).collect();
      if xi.iter().any(|&vv| vv < 0) { return; }
      let mut ok = true;
      for i in 0..m {
        let mut sum = 0i64;
        for (j, btn) in buttons.iter().enumerate() {
          if btn.contains(&(i as i32)) { sum += xi[j]; }
        }
        if sum != target[i] as i64 { ok = false; break; }
      }
      if !ok { return; }
      let total: i64 = xi.iter().sum();
      if best_sum.map_or(true, |bs| total < bs) {
        *best_sum = Some(total);
        *best_sol = xi;
      }
      return;
    }

    // Iterate through possible values for the current free variable
    // We don't have a tight upper bound, so we just iterate upwards, relying on pruning.
    // A practical limit is needed to prevent infinite loops.
    // The sum of target values is a loose upper bound on the total sum.
    let sum_target: i64 = target.iter().map(|&v| v as i64).sum();
    let search_limit = best_sum.unwrap_or(sum_target + 1);

    for val in 0..search_limit {
        current_free[idx] = val as i64;
        dfs_assign(
            idx + 1, free_cols, r, y, pivmap, buttons, target,
            current_free, current_sum + val, best_sum, best_sol,
        );
    }
  }

  let mut current_free: Vec<i64> = vec![0; free_cols.len()];
  dfs_assign(0, &free_cols, &r, &y, &pivmap, buttons, target, &mut current_free, 0, &mut best_sum, &mut best_sol);
  if best_sum.is_some() { Some(best_sol) } else { None }
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

    let mut total2: i64 = 0;
    for (i, m) in machines.iter().enumerate() {
      println!("\n=== Machine {} ===", i + 1);
      if let Some(sol) = reduce_and_solve(&m.1, &m.2) {
        let sum: i64 = sol.iter().sum();
        println!("Solution x = {:?}; sum = {}", sol, sum);
        total2 += sum;
      } else {
        println!("No solution via reduction; trying REF minimal sum...");
        if let Some(sol2) = solve_by_ref_min_sum(&m.1, &m.2) {
          let sum2: i64 = sol2.iter().sum();
          println!("REF solution x = {:?}; sum = {}", sol2, sum2);
          total2 += sum2;
         } else {
          println!("No solution!");
          break;
        }
      }
    }

    println!("\nTotal presses: {}", total2);

}
