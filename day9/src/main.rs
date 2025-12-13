use std::{collections::HashSet, fs};
use std::cmp::{max, min};

type Point = (i64, i64);

// Fills in all boundary points (red and green connecting segments)
fn get_boundary_tiles(vertices: &Vec<Point>) -> (HashSet<Point>, i64) {
    let n = vertices.len();
    let mut boundary_tiles = HashSet::new();
    let mut boundary_count = 0;

    for i in 0..n {
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % n];

        let (x1, y1) = p1;
        let (x2, y2) = p2;

        if x1 == x2 { // Vertical segment
            for y in min(y1, y2)..=max(y1, y2) {
                boundary_tiles.insert((x1, y));
            }
            boundary_count += (y1 - y2).abs();
        } else { // Horizontal segment
            for x in min(x1, x2)..=max(x1, x2) {
                boundary_tiles.insert((x, y1));
            }
            boundary_count += (x1 - x2).abs();
        }
    }
    // The number of *segments* is boundary_count, 
    // the number of *unique points* is boundary_tiles.len().
    // We use boundary_count (the sum of segment lengths) for Pick's Theorem.
    (boundary_tiles, boundary_count)
}

// Calculates the area A using the Shoelace Formula
fn shoelace_area(vertices: &Vec<Point>) -> i64 {
    let n = vertices.len();
    let mut area = 0i64;
    for i in 0..n {
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % n];
        area += p1.0 * p2.1 - p2.0 * p1.1; // x1*y2 - x2*y1
    }
    area.abs() / 2
}

// Calculates the set of all red/green tiles (boundary + interior)
fn get_valid_tiles(vertices: &Vec<Point>) -> HashSet<Point> {
    let (boundary_set, boundary_count) = get_boundary_tiles(vertices);
    let area = shoelace_area(vertices);

    // Pick's Theorem: A = I + B/2 - 1 => I = A - B/2 + 1
    // A: Area from Shoelace. B: sum of segment lengths. I: number of interior points.
    let interior_count = area - boundary_count / 2 + 1;
    let _total_tiles = interior_count + boundary_count;

    // Use a bounding box to find the interior tiles to add to the boundary set
    let (min_x, max_x, min_y, max_y) = vertices.iter().fold(
        (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
        |acc, &(x, y)| {
            (min(acc.0, x), max(acc.1, x), min(acc.2, y), max(acc.3, y))
        },
    );

    let mut valid_tiles = boundary_set.clone();
    
    // We use a simplified ray casting (Winding Number) check for the interior.
    // We iterate over the bounding box and check which points are inside.
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = (x, y);

            if valid_tiles.contains(&p) {
                continue; // Already a boundary tile
            }

            // Simplified ray casting for interior check (odd/even rule)
            let mut crossings = 0;
            for i in 0..vertices.len() {
                let p1 = vertices[i];
                let p2 = vertices[(i + 1) % vertices.len()];

                let (x1, y1) = p1;
                let (x2, y2) = p2;

                let (y_min, y_max) = (min(y1, y2), max(y1, y2));

                if y > y_min && y <= y_max {
                    let _is_crossing = false;
                    
                    // Horizontal segment check (must not be on boundary, which is checked earlier)
                    if y1 == y2 {
                        continue;
                    }
                    
                    // Calculate the x-intersection of the horizontal ray (y) with the segment (p1, p2)
                    let x_intersect = x1 + (y - y1) * (x2 - x1) / (y2 - y1);
                    
                    // If the intersection is to the right of the point (x), it's a crossing
                    if x_intersect > x {
                        crossings += 1;
                    }
                }
            }
            
            // Odd number of crossings means the point is inside
            if crossings % 2 != 0 {
                valid_tiles.insert(p);
            }
        }
    }

    valid_tiles
}

// Checks if the rectangle defined by two opposite corners (p1, p2) is valid
// Checks if all corners are boundary tiles and all edge points are valid tiles
fn is_valid_rectangle(p1: Point, p2: Point, boundary_tiles: &HashSet<Point>, valid_tiles: &HashSet<Point>) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    // All 4 corners must be boundary tiles
    let corners = [
        (x1, y1),
        (x1, y2),
        (x2, y1),
        (x2, y2)
    ];

    if !corners.iter().all(|corner| boundary_tiles.contains(corner)) {
        return false;
    }

    // Check all 4 edges are within valid tiles
    let (min_x, max_x) = (min(x1, x2), max(x1, x2));
    let (min_y, max_y) = (min(y1, y2), max(y1, y2));

    // Check top and bottom edges
    for x in min_x..=max_x {
        if !valid_tiles.contains(&(x, min_y)) || !valid_tiles.contains(&(x, max_y)) {
            return false;
        }
    }

    // Check left and right edges
    for y in min_y..=max_y {
        if !valid_tiles.contains(&(min_x, y)) || !valid_tiles.contains(&(max_x, y)) {
            return false;
        }
    }

    true
}

// Finds the largest valid rectangle area
fn solve_part2(a_vertices: &Vec<Point>) -> i64 {
    let (boundary_tiles, _) = get_boundary_tiles(&a_vertices);
    let valid_tiles = get_valid_tiles(&a_vertices);
    let n = a_vertices.len();
    let mut max_area = 0i64;

    // Iterate through all pairs of boundary tiles as opposite corners
    for i in 0..n {
        for j in i + 1..n { // Start from i+1 to avoid checking the same pair twice
            let p1 = a_vertices[i];
            let p2 = a_vertices[j];

            let (x1, y1) = p1;
            let (x2, y2) = p2;

            let area = (x1 - x2 + 1).abs() * (y1 - y2 + 1).abs();

            if area > max_area {
                if is_valid_rectangle(p1, p2, &boundary_tiles, &valid_tiles) {
                    max_area = area;
                }
            }
        }
    }

    max_area
}

fn main() {
    let content = fs::read_to_string("Day9TestInput.txt")
        .expect("Should have been able to read the file");

    let vertices: Vec<Point> = content.lines().map(|line| { 
      let gg: Vec<i64> = line.split(",").map(|b| b.parse::<i64>().unwrap()).collect();
      (gg[0], gg[1])
    }).collect();

    println!("{:?}", vertices);

    let mut max_area = 0;
    for i in 0..vertices.len() {
      for j in i + 1..vertices.len(){
        let area = (vertices[i].1 - vertices[j].1 + 1).abs() * (vertices[i].0 - vertices[j].0 + 1).abs();
        if area > max_area {
          max_area = area;
        }
      }
    }

    println!("{max_area}");

    let mm = solve_part2(&vertices);

    println!("{mm}");
}
