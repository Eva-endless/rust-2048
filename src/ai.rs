#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy)]
pub struct Grid {
    cells: [[Option<u32>; 4]; 4],
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            cells: [[None; 4]; 4],
        }
    }

    pub fn from_tiles(tiles: &[(i32, i32, u32)]) -> Self {
        let mut grid = Grid::new();
        for &(x, y, score) in tiles {
            if x >= 0 && x < 4 && y >= 0 && y < 4 {
                grid.cells[y as usize][x as usize] = Some(score);
            }
        }
        grid
    }

    fn get_empty_cells(&self) -> Vec<(usize, usize)> {
        let mut empty = Vec::new();
        for y in 0..4 {
            for x in 0..4 {
                if self.cells[y][x].is_none() {
                    empty.push((x, y));
                }
            }
        }
        empty
    }

    fn move_left(&mut self) -> bool {
        let mut moved = false;
        for y in 0..4 {
            let original_cells = self.cells[y];
            
            let row: Vec<u32> = self.cells[y].iter().filter_map(|&x| x).collect();
            
            let mut merged = Vec::new();
            let mut i = 0;
            while i < row.len() {
                if i + 1 < row.len() && row[i] == row[i + 1] {
                    merged.push(row[i] * 2);
                    i += 2;
                } else {
                    merged.push(row[i]);
                    i += 1;
                }
            }
            
            while merged.len() < 4 {
                merged.push(0);
            }
            
            for x in 0..4 {
                self.cells[y][x] = if merged[x] > 0 { Some(merged[x]) } else { None };
            }
            
            if self.cells[y] != original_cells {
                moved = true;
            }
        }
        moved
    }

    fn move_right(&mut self) -> bool {
        let mut moved = false;
        for y in 0..4 {
            let original_cells = self.cells[y];
            
            let row: Vec<u32> = self.cells[y].iter().filter_map(|&x| x).collect();
            
            let mut merged = Vec::new();
            let mut i = row.len();
            while i > 0 {
                i -= 1;
                if i > 0 && row[i] == row[i - 1] {
                    merged.push(row[i] * 2);
                    i -= 1;
                } else {
                    merged.push(row[i]);
                }
            }
            merged.reverse();
            
            while merged.len() < 4 {
                merged.push(0);
            }
            
            for x in 0..4 {
                self.cells[y][x] = if merged[x] > 0 { Some(merged[x]) } else { None };
            }
            
            if self.cells[y] != original_cells {
                moved = true;
            }
        }
        moved
    }

    fn move_up(&mut self) -> bool {
        let mut moved = false;
        for x in 0..4 {
            let original_cells: [Option<u32>; 4] = [
                self.cells[0][x], self.cells[1][x], 
                self.cells[2][x], self.cells[3][x]
            ];
            
            let col: Vec<u32> = (0..4).filter_map(|y| self.cells[y][x]).collect();
            
            let mut merged = Vec::new();
            let mut i = 0;
            while i < col.len() {
                if i + 1 < col.len() && col[i] == col[i + 1] {
                    merged.push(col[i] * 2);
                    i += 2;
                } else {
                    merged.push(col[i]);
                    i += 1;
                }
            }
            
            while merged.len() < 4 {
                merged.push(0);
            }
            
            for y in 0..4 {
                self.cells[y][x] = if merged[y] > 0 { Some(merged[y]) } else { None };
            }
            
            let new_cells: [Option<u32>; 4] = [
                self.cells[0][x], self.cells[1][x], 
                self.cells[2][x], self.cells[3][x]
            ];
            
            if new_cells != original_cells {
                moved = true;
            }
        }
        moved
    }

    fn move_down(&mut self) -> bool {
        let mut moved = false;
        for x in 0..4 {
            let original_cells: [Option<u32>; 4] = [
                self.cells[0][x], self.cells[1][x], 
                self.cells[2][x], self.cells[3][x]
            ];
            
            let col: Vec<u32> = (0..4).filter_map(|y| self.cells[y][x]).collect();
            
            let mut merged = Vec::new();
            let mut i = col.len();
            while i > 0 {
                i -= 1;
                if i > 0 && col[i] == col[i - 1] {
                    merged.push(col[i] * 2);
                    i -= 1;
                } else {
                    merged.push(col[i]);
                }
            }
            merged.reverse();
            
            while merged.len() < 4 {
                merged.push(0);
            }
            
            for y in 0..4 {
                self.cells[y][x] = if merged[y] > 0 { Some(merged[y]) } else { None };
            }
            
            let new_cells: [Option<u32>; 4] = [
                self.cells[0][x], self.cells[1][x], 
                self.cells[2][x], self.cells[3][x]
            ];
            
            if new_cells != original_cells {
                moved = true;
            }
        }
        moved
    }

    pub fn apply_move(&mut self, dir: Direction) -> bool {
        match dir {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
        }
    }

    pub fn can_move(&self) -> bool {
        if self.get_empty_cells().len() > 0 {
            return true;
        }
        
        for y in 0..4 {
            for x in 0..3 {
                if self.cells[y][x] == self.cells[y][x + 1] {
                    return true;
                }
            }
        }
        
        for x in 0..4 {
            for y in 0..3 {
                if self.cells[y][x] == self.cells[y + 1][x] {
                    return true;
                }
            }
        }
        
        false
    }

    pub fn evaluate(&self) -> f64 {
        let mut score = 0.0;
        
        let empty_cells = self.get_empty_cells().len() as f64;
        score += empty_cells * 2.7;
        
        let monotonicity = self.calculate_monotonicity();
        score += monotonicity * 0.5;
        
        let corner_bonus = self.calculate_corner_bonus();
        score += corner_bonus * 1.0;
        
        let smoothness = self.calculate_smoothness();
        score -= smoothness * 0.1;
        
        score
    }

    fn calculate_monotonicity(&self) -> f64 {
        let mut total = 0.0;
        
        for y in 0..4 {
            let mut inc = 0;
            let mut dec = 0;
            for x in 0..3 {
                let left = self.cells[y][x].unwrap_or(0) as f64;
                let right = self.cells[y][x + 1].unwrap_or(0) as f64;
                if left > 0.0 && right > 0.0 {
                    if left <= right {
                        inc += 1;
                    } else {
                        dec += 1;
                    }
                }
            }
            total += (inc as f64 - dec as f64).abs();
        }
        
        for x in 0..4 {
            let mut inc = 0;
            let mut dec = 0;
            for y in 0..3 {
                let up = self.cells[y][x].unwrap_or(0) as f64;
                let down = self.cells[y + 1][x].unwrap_or(0) as f64;
                if up > 0.0 && down > 0.0 {
                    if up <= down {
                        inc += 1;
                    } else {
                        dec += 1;
                    }
                }
            }
            total += (inc as f64 - dec as f64).abs();
        }
        
        total
    }

    fn calculate_corner_bonus(&self) -> f64 {
        let max_tile = self.get_max_tile();
        if max_tile == 0 {
            return 0.0;
        }
        
        let corners = [
            self.cells[0][0],
            self.cells[0][3],
            self.cells[3][0],
            self.cells[3][3],
        ];
        
        for &corner in corners.iter() {
            if corner == Some(max_tile) {
                return max_tile as f64;
            }
        }
        
        0.0
    }

    fn get_max_tile(&self) -> u32 {
        let mut max = 0;
        for y in 0..4 {
            for x in 0..4 {
                if let Some(val) = self.cells[y][x] {
                    if val > max {
                        max = val;
                    }
                }
            }
        }
        max
    }

    fn calculate_smoothness(&self) -> f64 {
        let mut total = 0.0;
        
        for y in 0..4 {
            for x in 0..3 {
                let left = self.cells[y][x].unwrap_or(0) as f64;
                let right = self.cells[y][x + 1].unwrap_or(0) as f64;
                if left > 0.0 && right > 0.0 {
                    total += (left - right).abs();
                }
            }
        }
        
        for x in 0..4 {
            for y in 0..3 {
                let up = self.cells[y][x].unwrap_or(0) as f64;
                let down = self.cells[y + 1][x].unwrap_or(0) as f64;
                if up > 0.0 && down > 0.0 {
                    total += (up - down).abs();
                }
            }
        }
        
        total
    }
}

pub fn get_best_move(grid: &Grid) -> Option<Direction> {
    let directions = [Direction::Left, Direction::Right, Direction::Up, Direction::Down];
    let mut best_dir: Option<Direction> = None;
    let mut best_score = f64::NEG_INFINITY;
    
    for &dir in directions.iter() {
        let mut test_grid = *grid;
        if test_grid.apply_move(dir) {
            let score = test_grid.evaluate();
            if score > best_score {
                best_score = score;
                best_dir = Some(dir);
            }
        }
    }
    
    best_dir
}

pub fn get_best_move_expectimax(grid: &Grid, depth: usize) -> Option<Direction> {
    let directions = [Direction::Left, Direction::Right, Direction::Up, Direction::Down];
    let mut best_dir: Option<Direction> = None;
    let mut best_score = f64::NEG_INFINITY;
    
    for &dir in directions.iter() {
        let mut test_grid = *grid;
        if test_grid.apply_move(dir) {
            let score = expectimax(&test_grid, depth - 1, false);
            if score > best_score {
                best_score = score;
                best_dir = Some(dir);
            }
        }
    }
    
    best_dir
}

fn expectimax(grid: &Grid, depth: usize, is_max: bool) -> f64 {
    if depth == 0 || !grid.can_move() {
        return grid.evaluate();
    }
    
    if is_max {
        let directions = [Direction::Left, Direction::Right, Direction::Up, Direction::Down];
        let mut max_score = f64::NEG_INFINITY;
        
        for &dir in directions.iter() {
            let mut test_grid = *grid;
            if test_grid.apply_move(dir) {
                let score = expectimax(&test_grid, depth - 1, false);
                max_score = max_score.max(score);
            }
        }
        
        max_score
    } else {
        let empty_cells = grid.get_empty_cells();
        if empty_cells.is_empty() {
            return grid.evaluate();
        }
        
        let mut total_score = 0.0;
        
        for &(x, y) in empty_cells.iter() {
            let mut grid2 = *grid;
            grid2.cells[y][x] = Some(2);
            let score2 = expectimax(&grid2, depth - 1, true);
            
            let mut grid4 = *grid;
            grid4.cells[y][x] = Some(4);
            let score4 = expectimax(&grid4, depth - 1, true);
            
            total_score += (score2 * 0.9 + score4 * 0.1) / empty_cells.len() as f64;
        }
        
        total_score
    }
}