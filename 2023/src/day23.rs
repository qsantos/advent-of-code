use std::collections::{HashMap, HashSet};

type Coord = (usize, usize);

pub fn part1(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let grid: Vec<&[u8]> = data.lines().map(|line| line.as_bytes()).collect();

    fn aux(
        grid: &[&[u8]],
        (x, y): Coord,
        visited: &mut HashSet<Coord>,
    ) -> Option<usize> {
        let rows = grid.len();
        let cols = grid[0].len();
        if (x, y) == (rows - 1, cols - 2) {
            return Some(0);
        }
        if visited.contains(&(x, y)) {
            return None;
        }
        visited.insert((x, y));
        let mut candidates = Vec::new();
        if x > 0 {
            let c = grid[x - 1][y];
            // NOTE: no ^ in example or input
            if c == b'.' || c == b'^' {
                if let Some(v) = aux(grid, (x - 1, y), visited) {
                    candidates.push(v + 1)
                }
            }
        }
        if x < rows - 1 {
            let c = grid[x + 1][y];
            if c == b'.' || c == b'v' {
                if let Some(v) = aux(grid, (x + 1, y), visited) {
                    candidates.push(v + 1)
                }
            }
        }
        if y > 0 {
            let c = grid[x][y - 1];
            // NOTE: no < in example or input
            if c == b'.' || c == b'<' {
                if let Some(v) = aux(grid, (x, y - 1), visited) {
                    candidates.push(v + 1)
                }
            }
        }
        if y < cols - 1 {
            let c = grid[x][y + 1];
            if c == b'.' || c == b'>' {
                if let Some(v) = aux(grid, (x, y + 1), visited) {
                    candidates.push(v + 1)
                }
            }
        }
        visited.remove(&(x, y));
        candidates.into_iter().max()
    }

    let mut visited = HashSet::new();
    aux(&grid, (0, 1), &mut visited).unwrap()
}

pub fn part2(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let grid: Vec<&[u8]> = data.lines().map(|line| line.as_bytes()).collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let start = (0, 1);
    let end = (rows - 1, cols - 2);

    // look for intersections
    let mut intersections = HashSet::new();
    intersections.insert(start);
    intersections.insert(end);
    for (i, row) in grid.iter().enumerate() {
        for j in 0..cols - 2 {
            if row[j..=j + 2] == [b'>', b'.', b'>'] {
                intersections.insert((i, j + 1));
            }
        }
    }
    for j in 0..cols {
        for i in 0..rows - 2 {
            if (grid[i][j], grid[i + 1][j], grid[i + 2][j]) == (b'v', b'.', b'v') {
                intersections.insert((i + 1, j));
            }
        }
    }

    // resolve distance between intersections
    fn aux(
        grid: &[&[u8]],
        (x, y): Coord,
        d: usize,
        visited: &mut HashSet<Coord>,
        intersections: &HashSet<Coord>,
        edges: &mut HashMap<Coord, usize>,
    ) {
        let rows = grid.len();
        let cols = grid[0].len();
        if visited.contains(&(x, y)) {
            return;
        }
        if d != 0 && intersections.contains(&(x, y)) {
            edges.insert((x, y), d);
            return;
        }
        visited.insert((x, y));
        if x > 0 && grid[x - 1][y] != b'#' {
            aux(grid, (x - 1, y), d + 1, visited, intersections, edges);
        }
        if x < rows - 1 && grid[x + 1][y] != b'#' {
            aux(grid, (x + 1, y), d + 1, visited, intersections, edges);
        }
        if y > 0 && grid[x][y - 1] != b'#' {
            aux(grid, (x, y - 1), d + 1, visited, intersections, edges);
        }
        if y < cols - 1 && grid[x][y + 1] != b'#' {
            aux(grid, (x, y + 1), d + 1, visited, intersections, edges);
        }
        visited.remove(&(x, y));
    }
    let mut visited = HashSet::new();
    let mut graph = HashMap::new();
    for intersection in intersections.iter() {
        let mut edges = HashMap::new();
        aux(
            &grid,
            *intersection,
            0,
            &mut visited,
            &intersections,
            &mut edges,
        );
        graph.insert(*intersection, edges);
    }

    // backtracking on simplified graph
    fn aux2(
        graph: &HashMap<Coord, HashMap<Coord, usize>>,
        visited: &mut HashSet<Coord>,
        cur: Coord,
        end: Coord,
    ) -> Option<usize> {
        if cur == end {
            return Some(0);
        }
        if visited.contains(&cur) {
            return None;
        }
        visited.insert(cur);
        let ret = graph[&cur]
            .iter()
            .flat_map(|(neighbor, distance)| {
                aux2(graph, visited, *neighbor, end).map(|d| distance + d)
            })
            .max();
        visited.remove(&cur);
        ret
    }

    aux2(&graph, &mut visited, start, end).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day23.txt");
    const INPUT: &str = include_str!("../inputs/day23.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 94);
        assert_eq!(part1(INPUT), 2182);

        assert_eq!(part2(EXAMPLE), 154);
        assert_eq!(part2(INPUT), 6670);
    }
}
