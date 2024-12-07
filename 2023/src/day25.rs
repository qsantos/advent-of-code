use std::collections::{HashMap, HashSet};

type Edge<'a> = (&'a str, &'a str);

fn insert_edge<'a>(edges: &mut HashMap<&'a str, Vec<&'a str>>, a: &'a str, b: &'a str) {
    edges
        .entry(a)
        .and_modify(|e| e.push(b))
        .or_insert_with(|| vec![b]);
}

fn randrange(state: &mut u64, max: u64) -> u64 {
    // basic Linear Congruential Generator
    *state = state.wrapping_mul(6364136223846793005) + 1;
    *state % max
}

fn resolve_merged_node<'a>(merged_nodes: &HashMap<&'a str, &'a str>, mut node: &'a str) -> &'a str {
    while let Some(p) = merged_nodes.get(&node) {
        node = p;
    }
    node
}

fn min_cut<'a>(
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
    mut edges: Vec<Edge<'a>>,
    size: usize,
) -> Vec<Edge<'a>> {
    let mut state = 42;

    // merged_nodes will serve as a union_find to quickly merge components
    let mut merged_nodes: HashMap<&str, &str> = HashMap::new();
    // keep track of removed edges to restore them after an attempt
    let mut removed_edges = Vec::new();
    loop {
        // contract |V| - 2 pairs of nodes
        for _ in 0..graph.len() - 2 {
            loop {
                let r = randrange(&mut state, edges.len() as u64);
                let edge = edges.swap_remove(r as usize);
                removed_edges.push(edge);
                let (a, b) = edge;
                let pa = resolve_merged_node(&merged_nodes, a);
                let pb = resolve_merged_node(&merged_nodes, b);
                if pa != pb {
                    // merge a into b
                    merged_nodes.insert(pa, pb);
                    break;
                }
                // a and b have been merged together, find another edge
            }
        }
        // check how many edges are remaining
        let edge_count = edges
            .iter()
            .filter(|(a, b)| {
                let pa = resolve_merged_node(&merged_nodes, a);
                let pb = resolve_merged_node(&merged_nodes, b);
                pa != pb
            })
            .count();
        if edge_count == size {
            // clean up the edges
            edges.retain(|(a, b)| {
                let pa = resolve_merged_node(&merged_nodes, a);
                let pb = resolve_merged_node(&merged_nodes, b);
                pa != pb
            });
            return edges;
        }
        // reset the graph
        edges.append(&mut removed_edges);
        merged_nodes.clear();
    }
}

pub fn part1(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut graph = HashMap::new();
    let mut edges = Vec::new();
    for line in data.lines() {
        let (node, neighbors) = line.split_once(": ").unwrap();
        for neighbor in neighbors.split(' ') {
            edges.push((node, neighbor));
            insert_edge(&mut graph, node, neighbor);
            insert_edge(&mut graph, neighbor, node);
        }
    }

    let edges = min_cut(&graph, edges, 3);

    // find size of the components
    let mut q = Vec::new();
    q.push(graph.keys().next().unwrap());
    let mut visited = HashSet::new();
    while let Some(cur) = q.pop() {
        if visited.contains(cur) {
            continue;
        }
        visited.insert(cur);
        for n in graph.get(cur).unwrap() {
            if edges.contains(&(cur, n)) || edges.contains(&(n, cur)) {
                continue;
            }
            q.push(n);
        }
    }
    assert_ne!(visited.len(), graph.len());
    let a = visited.len();
    let b = graph.len() - a;
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day25.txt");
    const INPUT: &str = include_str!("../inputs/day25.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 54);
        assert_eq!(part1(INPUT), 555856);
    }
}
