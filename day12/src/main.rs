use std::fs;

struct Config {
  shapes: Vec<Vec<Vec<char>>>,
  regions: Vec<(usize, usize, Vec<usize>)>  // (height, width, [count of shape 0, count of shape 1, ...])
}

fn parse_input(a_content: &String) -> Config {
       let config: Vec<&str> = a_content.split("\r\n\r\n").collect();
       
       let mut ss: Vec<Vec<Vec<char>>> = Vec::new();
       for i in 0..config.len() - 1 {
         let line = config[i];

         let gg: Vec<&str> = line.split(":\r\n").collect();

         let ss0 = gg[1].lines().map(| v| v.chars().collect()).collect();  

         ss.push(ss0);
        }

       let rr = config[config.len() - 1];

       let dd: Vec<(usize, usize, Vec<usize>)> = rr.lines().map(| v | {
        let bb: Vec<&str> = v.split(": ").collect();

        let yy: Vec<usize> =  bb[0].split("x").map(| v | v.parse::<usize>().unwrap()).collect();

        let kk:Vec<usize> = bb[1].split(" ").map(|v | v.parse::<usize>().unwrap()).collect();

        (yy[0], yy[1], kk)
       }).collect();

      Config {shapes: ss, regions: dd}
}

// Rotate a shape 90 degrees clockwise
fn rotate_shape(shape: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = shape.len();
    let cols = shape[0].len();
    let mut rotated = vec![vec!['.'; rows]; cols];
    
    for i in 0..rows {
        for j in 0..cols {
            rotated[j][rows - 1 - i] = shape[i][j];
        }
    }
    rotated
}

// Get all 4 rotations of a shape
fn get_rotations(shape: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut rotations = Vec::new();
    let mut current = shape.clone();
    
    for _ in 0..4 {
        rotations.push(current.clone());
        current = rotate_shape(&current);
    }
    
    rotations
}

// Check if a shape can fit at position (row, col) in the grid
fn can_place_shape(grid: &Vec<Vec<char>>, shape: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let shape_rows = shape.len();
    let shape_cols = shape[0].len();
    let grid_rows = grid.len();
    let grid_cols = grid[0].len();
    
    // Check bounds
    if row + shape_rows > grid_rows || col + shape_cols > grid_cols {
        return false;
    }
    
    // Check if all '#' positions in shape are empty in grid
    for i in 0..shape_rows {
        for j in 0..shape_cols {
            if shape[i][j] == '#' && grid[row + i][col + j] != '.' {
                return false;
            }
        }
    }
    
    true
}

// Place a shape on the grid
fn place_shape(grid: &mut Vec<Vec<char>>, shape: &Vec<Vec<char>>, row: usize, col: usize) {
    let shape_rows = shape.len();
    let shape_cols = shape[0].len();
    
    for i in 0..shape_rows {
        for j in 0..shape_cols {
            if shape[i][j] == '#' {
                grid[row + i][col + j] = '#';
            }
        }
    }
}

// Remove a shape from the grid
fn remove_shape(grid: &mut Vec<Vec<char>>, shape: &Vec<Vec<char>>, row: usize, col: usize) {
    let shape_rows = shape.len();
    let shape_cols = shape[0].len();
    
    for i in 0..shape_rows {
        for j in 0..shape_cols {
            if shape[i][j] == '#' {
                grid[row + i][col + j] = '.';
            }
        }
    }
}

// Try to place all shapes in a region using backtracking
// shape_counts[i] = number of copies of shape i that need to be placed
fn try_place_shapes(
    grid: &mut Vec<Vec<char>>,
    shape_counts: &mut Vec<usize>,
    all_shapes: &Vec<Vec<Vec<char>>>
) -> bool {
    // Base case: all shapes placed successfully (all counts are 0)
    if shape_counts.iter().all(|&count| count == 0) {
        return true;
    }
    
    // Try to place one instance of each shape type
    for shape_idx in 0..shape_counts.len() {
        if shape_counts[shape_idx] == 0 {
            continue; // No more of this shape to place
        }
        
        let shape = &all_shapes[shape_idx];
        let rotations = get_rotations(shape);
        
        for rotation in &rotations {
            // Try all positions in the grid
            for row in 0..grid.len() {
                for col in 0..grid[0].len() {
                    if can_place_shape(grid, rotation, row, col) {
                        place_shape(grid, rotation, row, col);
                        shape_counts[shape_idx] -= 1;
                        
                        // Recursively try to place remaining shapes
                        if try_place_shapes(grid, shape_counts, all_shapes) {
                            return true;
                        }
                        
                        // Backtrack
                        shape_counts[shape_idx] += 1;
                        remove_shape(grid, rotation, row, col);
                    }
                }
            }
        }
        
        // If we tried to place this shape and failed, no point continuing
        return false;
    }
    
    false
}

// Test if all shapes can fit in a region
// shape_counts[i] = number of copies of shape i that need to be placed
fn test_region_fits_shapes(
    region_height: usize,
    region_width: usize,
    shape_counts: &Vec<usize>,
    all_shapes: &Vec<Vec<Vec<char>>>
) -> bool {
    let mut grid = vec![vec!['.'; region_width]; region_height];
    let mut counts = shape_counts.clone();
    try_place_shapes(&mut grid, &mut counts, all_shapes)
}

// Count how many regions fit all their corresponding shapes
fn count_fitting_regions(config: &Config) -> usize {
    let mut count = 0;
    
    for (idx, region) in config.regions.iter().enumerate() {
        let (height, width, shape_counts) = region;
        
        // Verify the shape_counts vector has the right length
        if shape_counts.len() != config.shapes.len() {
            println!("Region {} ({}x{}): Warning - expected {} shape counts but got {}", 
                     idx, height, width, config.shapes.len(), shape_counts.len());
            continue;
        }
        
        // Calculate total cells needed
        let total_cells_needed: usize = shape_counts.iter().enumerate()
            .map(|(shape_idx, &count)| {
                let shape = &config.shapes[shape_idx];
                let shape_cells = shape.iter()
                    .flat_map(|row| row.iter())
                    .filter(|&&c| c == '#')
                    .count();
                count * shape_cells
            })
            .sum();
        
        let region_cells = height * width;
        
        println!("\nRegion {} ({}x{}): needs {} cells, has {} cells", 
                 idx, height, width, total_cells_needed, region_cells);
        println!("  Shape counts: {:?}", shape_counts);
        
        if total_cells_needed > region_cells {
            println!("  Result: IMPOSSIBLE (not enough space)");
            continue;
        }
        
        if test_region_fits_shapes(*height, *width, shape_counts, &config.shapes) {
            count += 1;
            println!("  Result: FITS!");
        } else {
            println!("  Result: Does NOT fit");
        }
    }
    
    count
}

fn main() {
      let content = fs::read_to_string("Day12Input.txt").expect("Should have been able to read the file");

     let cc = parse_input(&content);

     println!("Parsed {} shapes:", cc.shapes.len());
     for (idx, shape) in cc.shapes.iter().enumerate() {
         println!("  Shape {}: {}x{}", idx, shape.len(), shape[0].len());
     }
     
     println!("\nParsed {} regions", cc.regions.len());
     
     let fitting_count = count_fitting_regions(&cc);
     println!("\n===================================");
     println!("Number of regions that fit all shapes: {}", fitting_count);
}
