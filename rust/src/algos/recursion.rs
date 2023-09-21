pub fn recursive_sum(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return n + recursive_sum(n - 1);
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn walk_maze(
    maze: &Vec<&str>,
    wall: &char,
    curr: &(usize, usize),
    end: &(usize, usize),
    seen: &mut Vec<Vec<bool>>,
    path: &mut Vec<(usize, usize)>,
) -> bool {
    // Base cases
    // Off the map
    if curr.1 >= maze[0].len() || curr.0 >= maze.len() {
        return false;
    }
    // At a wall
    if maze[curr.0].chars().nth(curr.1).unwrap() == *wall {
        return false;
    }
    // At the End
    if curr.0 == end.0 && curr.1 == end.1 {
        path.push(*curr);
        return true;
    }
    // Already visited
    if seen[curr.0][curr.1] {
        return false;
    }
    path.push(*curr);
    seen[curr.0][curr.1] = true;

    // recurse
    for (dx, dy) in DIRS {
        if walk_maze(
            maze,
            wall,
            &((curr.0 as i32 + dx) as usize, (curr.1 as i32 + dy) as usize),
            end,
            seen,
            path,
        ) {
            return true;
        }
    }

    path.pop();
    false
}
pub fn solve_maze(
    maze: Vec<&str>,
    wall: char,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    // set up seen array
    let mut seen = vec![vec![false; maze[0].len()]; maze.len()];
    let mut path = vec![];
    walk_maze(&maze, &wall, &start, &end, &mut seen, &mut path);
    path
}
