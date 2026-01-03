use rand::Rng;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs,
    path::Path,
};
pub fn create_tests() -> HashMap<usize, u8> {
    let mut result = HashMap::new();
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_path = project_root.join("test");
    let mut rng = rand::rng();
    for n in 1..=50 {
        let count: u8 = rng.random_range(10..=30);
        let size: usize = 30;
        let file = format!("Test{}.txt", n);
        let test = IslandCountTest::create(count, size);
        let _ = fs::write(test_path.join(file), format!("{test}"));
        result.insert(n, test.island_count);
    }
    result
}

struct IslandCountTest {
    grid: Vec<Vec<bool>>,
    island_count: u8,
}
impl IslandCountTest {
    fn create(island_count: u8, size: usize) -> Self {
        let mut rng = rand::rng();
        let mut grid = vec![vec![false; size]; size];
        let mut seeds = HashSet::new();
        let min_distance = 2; // minimum spacing between islands

        while seeds.len() < island_count.into() {
            let row = rng.random_range(0..size);
            let col = rng.random_range(0..size);

            let too_close = seeds.iter().any(|&(r, c)| {
                let dr = r as isize - row as isize;
                let dc = c as isize - col as isize;
                dr.abs() <= min_distance && dc.abs() <= min_distance
            });

            if !too_close {
                seeds.insert((row, col));
                grid[row][col] = true;
            }
        }
        for seed in seeds {
            add_island_bfs(&mut grid, (seed.0, seed.1));
        }

        Self { grid, island_count }
    }
}

impl Display for IslandCountTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in &self.grid {
            for cell in row {
                match cell {
                    true => s.push('1'),
                    false => s.push('0'),
                }
            }
            s.push('\n');
        }
        f.write_str(&s)
    }
}

fn add_island_bfs(grid: &mut [Vec<bool>], pos: (usize, usize)) {
    let mut rng = rand::rng();
    let cols = grid[0].len();
    let rows = grid.len();
    let size: usize = rng.random_range(1..=30);
    let mut queue = VecDeque::new();

    let mut island = HashSet::new();

    queue.push_back(pos);

    'bigloop: while let Some((r, c)) = queue.pop_front() {
        if island.len() == size {
            break;
        }
        if island.contains(&(r, c)) {
            continue;
        }
        let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

        let mut surroundings = Vec::new();
        for direction in directions {
            let cr = r as isize + direction.0;
            let cc = c as isize + direction.1;

            if cr >= 0 && cr < rows as isize && cc >= 0 && cc < cols as isize {
                if !grid[cr as usize][cc as usize] && rng.random_bool(0.7) {
                    surroundings.push((cr as usize, cc as usize));
                } else if grid[cr as usize][cc as usize]
                    && !island.contains(&(cr as usize, cc as usize))
                {
                    continue 'bigloop;
                }
            }
        }
        queue.extend(surroundings);
        grid[r][c] = true;
        island.insert((r, c));
    }
}
