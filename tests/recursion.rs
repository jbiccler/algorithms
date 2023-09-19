use algos::algos::recursion::*;

#[test]
fn test_recusive_sum() {
    assert_eq!(recursive_sum(1), 1);
    assert_eq!(recursive_sum(0), 0);
    assert_eq!(recursive_sum(99), 99 * 100 / 2);
}
#[test]
fn test_maze_solve() {
    let maze = vec![
        "xxxxxxxxxx x",
        "x        x x",
        "x        x x",
        "x xxxxxxxx x",
        "x          x",
        "x xxxxxxxxxx",
    ];
    let result = vec![
        (0, 10),
        (1, 10),
        (2, 10),
        (3, 10),
        (4, 10),
        (4, 9),
        (4, 8),
        (4, 7),
        (4, 6),
        (4, 5),
        (4, 4),
        (4, 3),
        (4, 2),
        (4, 1),
        (5, 1),
    ];
    assert_eq!(solve_maze(maze, 'x', (0, 10), (5, 1)), result);
}
