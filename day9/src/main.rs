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

// Checks if a point is inside the polygon using ray casting
fn is_point_inside_polygon(point: Point, vertices: &Vec<Point>) -> bool {
    let (x, y) = point;
    let mut crossings = 0;
    
    for i in 0..vertices.len() {
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % vertices.len()];
        
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        
        let (y_min, y_max) = (min(y1, y2), max(y1, y2));
        
        if y > y_min && y <= y_max {
            // Skip horizontal segments
            if y1 == y2 {
                continue;
            }
            
            // Calculate the x-intersection
            let x_intersect = x1 + (y - y1) * (x2 - x1) / (y2 - y1);
            
            // If the intersection is to the right, it's a crossing
            if x_intersect >= x {
                crossings += 1;
            }
        }
    }
    
    // Odd number of crossings means inside
    crossings % 2 != 0
}

// Checks if the rectangle defined by two opposite corners (p1, p2) is valid
// Checks that all rectangle boundary points are either on polygon boundary OR inside polygon
fn is_valid_rectangle(p1: Point, p2: Point, vertices: &Vec<Point>, boundary_tiles: &HashSet<Point>) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let (min_x, max_x) = (min(x1, x2), max(x1, x2));
    let (min_y, max_y) = (min(y1, y2), max(y1, y2));

     // OPTIMIZATION: Check all 4 corners first for early exit
    let corners = [
        (min_x, min_y),
        (min_x, max_y),
        (max_x, min_y),
        (max_x, max_y),
    ];

    for &corner in &corners {
        if !boundary_tiles.contains(&corner) && !is_point_inside_polygon(corner, vertices) {
            return false; // Early exit if any corner is outside
        }
    }

    // Check all 4 edges - points must be on boundary OR inside polygon
    // Top and bottom edges
    for x in min_x..=max_x {
        let top = (x, min_y);
        let bottom = (x, max_y);
        
        // Accept if on boundary OR inside
        if !boundary_tiles.contains(&top) && !is_point_inside_polygon(top, vertices) {
            return false;
        }
        if !boundary_tiles.contains(&bottom) && !is_point_inside_polygon(bottom, vertices) {
            return false;
        }
    }

    // Left and right edges
    for y in min_y..=max_y {
        let left = (min_x, y);
        let right = (max_x, y);
        
        // Accept if on boundary OR inside
        if !boundary_tiles.contains(&left) && !is_point_inside_polygon(left, vertices) {
            return false;
        }
        if !boundary_tiles.contains(&right) && !is_point_inside_polygon(right, vertices) {
            return false;
        }
    }

    true
}

// Finds the largest valid rectangle area
fn solve_part2(a_vertices: &Vec<Point>) -> i64 {
    let (boundary_tiles, _) = get_boundary_tiles(&a_vertices);
    let n = a_vertices.len();
    let mut max_area = 0i64;
    let mut best_rectangle: (Point, Point) = ((0, 0), (0, 0));

    // Iterate through all pairs of boundary tiles as opposite corners
    for i in 0..n {
        for j in i + 1..n { // Start from i+1 to avoid checking the same pair twice
            let p1 = a_vertices[i];
            let p2 = a_vertices[j];

            let (x1, y1) = p1;
            let (x2, y2) = p2;

            let area = ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1);

            //println!("Checking rectangle: ({}, {}) to ({}, {}) with area {}", x1, y1, x2, y2, area);

            if area > max_area {

              //print_rectangle(&a_vertices, p1, p2);

                if is_valid_rectangle(p1, p2, a_vertices, &boundary_tiles) {
                    max_area = area;
                    best_rectangle = (p1, p2);
                    println!("New max area {} with rectangle: ({}, {}) to ({}, {})", max_area, x1, y1, x2, y2);
                }
            }
        }
    }

    print_scaled_polygon_and_rectangle(a_vertices, Some(best_rectangle.0), Some(best_rectangle.1))  ;
    max_area
}

// Prints a scaled-down version of the polygon and rectangle in a Full HD grid (1920x1080)
fn print_scaled_polygon_and_rectangle(vertices: &Vec<Point>, rect_p1: Option<Point>, rect_p2: Option<Point>) {
    const GRID_WIDTH: usize = 192;  // 1920 / 10 for reasonable console display
    const GRID_HEIGHT: usize = 54;  // 1080 / 20 for reasonable console display
    
    let (boundary_tiles, _) = get_boundary_tiles(vertices);
    
    // Find bounding box of the polygon
    let (min_x, max_x, min_y, max_y) = vertices.iter().fold(
        (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
        |acc, &(x, y)| {
            (min(acc.0, x), max(acc.1, x), min(acc.2, y), max(acc.3, y))
        },
    );
    
    let poly_width = (max_x - min_x + 1) as f64;
    let poly_height = (max_y - min_y + 1) as f64;
    
    // Calculate scaling factors to fit the polygon in the grid
    let scale_x = (GRID_WIDTH as f64) / poly_width;
    let scale_y = (GRID_HEIGHT as f64) / poly_height;
    let scale = scale_x.min(scale_y) * 0.9; // Use 90% to leave some margin
    
    // Function to scale a point
    let scale_point = |p: Point| -> (usize, usize) {
        let scaled_x = ((p.0 - min_x) as f64 * scale) as usize;
        let scaled_y = ((p.1 - min_y) as f64 * scale) as usize;
        (scaled_x, scaled_y)
    };
    
    // Create grid
    let mut grid = vec![vec![' '; GRID_WIDTH]; GRID_HEIGHT];
    
    // Draw polygon boundary
    for &point in boundary_tiles.iter() {
        let (sx, sy) = scale_point(point);
        if sx < GRID_WIDTH && sy < GRID_HEIGHT {
            grid[sy][sx] = '#';
        }
    }
    
    // Draw polygon interior using point-in-polygon test
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if grid[y][x] == ' ' {
                // Map grid coordinates back to original coordinates
                let orig_x = min_x + (x as f64 / scale) as i64;
                let orig_y = min_y + (y as f64 / scale) as i64;
                
                if is_point_inside_polygon((orig_x, orig_y), vertices) {
                    grid[y][x] = '.';
                }
            }
        }
    }
    
    // Draw rectangle if provided
    if let (Some(p1), Some(p2)) = (rect_p1, rect_p2) {
        let (rx1, ry1) = p1;
        let (rx2, ry2) = p2;
        let (rect_min_x, rect_max_x) = (min(rx1, rx2), max(rx1, rx2));
        let (rect_min_y, rect_max_y) = (min(ry1, ry2), max(ry1, ry2));
        
        // Draw rectangle boundary
        for x in rect_min_x..=rect_max_x {
            for y in rect_min_y..=rect_max_y {
                let (sx, sy) = scale_point((x, y));
                if sx < GRID_WIDTH && sy < GRID_HEIGHT {
                    let on_edge = x == rect_min_x || x == rect_max_x || y == rect_min_y || y == rect_max_y;
                    if on_edge {
                        grid[sy][sx] = 'O'; // Rectangle boundary
                    } else if grid[sy][sx] != 'O' {
                        grid[sy][sx] = '*'; // Rectangle interior
                    }
                }
            }
        }
    }
    
    // Print the grid
    println!("\n=== Scaled Polygon Visualization ({}x{}) ===", GRID_WIDTH, GRID_HEIGHT);
    println!("# = Polygon boundary");
    println!(". = Polygon interior");
    if rect_p1.is_some() {
        println!("O = Rectangle boundary");
        println!("* = Rectangle interior");
    }
    println!("Scale factor: {:.2}x", scale);
    println!("Original bounds: ({}, {}) to ({}, {})", min_x, min_y, max_x, max_y);
    println!();
    
    for row in grid.iter() {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
    println!("===========================================\n");
}

/// Checks if a rectangle is valid by ensuring no polygon boundary points are in its interior.
/// Points on the rectangle's own border are not considered "inside".
fn is_valid_rectangle_no_inner_boundary_points(p1: Point, p2: Point, boundary_tiles: &HashSet<Point>) -> bool {
    // Define the rectangle's boundaries.
    let (min_x, max_x) = (min(p1.0, p2.0), max(p1.0, p2.0));
    let (min_y, max_y) = (min(p1.1, p2.1), max(p1.1, p2.1));

    // Iterate through all points on the polygon's boundary.
    for &boundary_point in boundary_tiles {
        // Check if the boundary point is strictly inside the rectangle.
        let (vx, vy) = boundary_point;
        if vx > min_x && vx < max_x && vy > min_y && vy < max_y {
            // A boundary point was found inside the rectangle, so it's invalid.
            return false;
        }
    }

    // No boundary points were found inside the rectangle. It's valid.
    true
}

/// Solves part 2 by finding the largest rectangle that contains no other polygon boundary points.
fn solve_part2_new(a_vertices: &Vec<Point>) -> i64 {
    let n = a_vertices.len();
    let mut max_area = 0i64;
    
    // Get all points on the polygon's boundary once.
    let (boundary_tiles, _) = get_boundary_tiles(a_vertices);

    // Iterate through all pairs of vertices to form candidate rectangles.
    for i in 0..n {
        for j in i + 1..n {
            let p1 = a_vertices[i];
            let p2 = a_vertices[j];

            // Check if this rectangle is valid according to the new rule.
            if is_valid_rectangle_no_inner_boundary_points(p1, p2, &boundary_tiles) {
                let (x1, y1) = p1;
                let (x2, y2) = p2;
                let area = ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1);

                if area > max_area {
                    max_area = area;
                }
            }
        }
    }
    max_area
}

fn main() {
    let content = fs::read_to_string("Day9Input.txt")
        .expect("Should have been able to read the file");

    let vertices: Vec<Point> = content.lines().map(|line| { 
      let gg: Vec<i64> = line.split(",").map(|b| b.parse::<i64>().unwrap()).collect();
      (gg[0], gg[1])
    }).collect();

    println!("{:?}", vertices);

    //print_tiles(&vertices);

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

    let mm = solve_part2_new(&vertices);

    println!("{mm}");

}
