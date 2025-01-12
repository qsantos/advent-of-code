use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct FileNode {
    _name: String,
    size: u32,
}

impl FileNode {
    fn new(name: &str, size: u32) -> Self {
        FileNode {
            _name: name.to_owned(),
            size,
        }
    }
}

#[derive(Debug)]
struct DirectoryNode {
    _name: String,
    children: HashMap<String, Rc<RefCell<FSNode>>>,
}

impl DirectoryNode {
    fn new(name: &str) -> Self {
        DirectoryNode {
            _name: name.to_owned(),
            children: HashMap::new(),
        }
    }
}

#[derive(Debug)]
enum FSNode {
    File(FileNode),
    Directory(DirectoryNode),
}

impl FSNode {
    fn from_commands(input: &str) -> Self {
        let root = FSNode::Directory(DirectoryNode::new("/"));
        let root = Rc::new(RefCell::new(root));
        let mut current_path = vec![Rc::clone(&root)];
        let mut lines = input.lines().peekable();
        while let Some(command) = lines.next() {
            if command == "$ cd /" {
                current_path.clear();
                current_path.push(Rc::clone(&root));
            } else if command == "$ cd .." {
                drop(current_path.pop());
            } else if let Some(name) = command.strip_prefix("$ cd ") {
                let node_ref;
                {
                    let current_node = current_path.last().unwrap();
                    if let FSNode::Directory(parent) = &*current_node.borrow() {
                        let target = &parent.children[name];
                        if let FSNode::Directory(_) = &*target.borrow() {
                            node_ref = Rc::clone(target);
                        } else {
                            panic!("The node '{}' is not a directory!", name);
                        }
                    } else {
                        panic!("The current node is not a directory!");
                    }
                }
                current_path.push(node_ref);
            } else if command == "$ ls" {
                while let Some(&entry) = lines
                    .peek()
                    .filter(|line| !line.is_empty() && !line.starts_with('$'))
                {
                    lines.next();
                    let parts: Vec<_> = entry.split(' ').collect();
                    assert_eq!(parts.len(), 2);
                    let size_or_dir = parts[0];
                    let name = parts[1];
                    let node = if size_or_dir == "dir" {
                        FSNode::Directory(DirectoryNode::new(name))
                    } else {
                        let size = size_or_dir.parse().unwrap();
                        FSNode::File(FileNode::new(name, size))
                    };
                    let mut parent = current_path.last().unwrap().borrow_mut();
                    if let FSNode::Directory(parent) = &mut *parent {
                        parent
                            .children
                            .insert(name.to_owned(), Rc::new(RefCell::new(node)));
                    } else {
                        panic!("Current node should be a directory!");
                    }
                }
            } else {
                unreachable!();
            }
        }
        current_path.clear();
        Rc::try_unwrap(root).unwrap().into_inner()
    }

    fn sizes_below(&self, threshold: u32) -> u32 {
        fn aux(node: &FSNode, threshold: u32) -> (u32, u32) {
            match node {
                FSNode::File(f) => (f.size, 0),
                FSNode::Directory(dir) => {
                    let (size, above) = dir
                        .children
                        .values()
                        .map(|n| aux(&n.borrow(), threshold))
                        .reduce(|acc, item| (acc.0 + item.0, acc.1 + item.1))
                        .unwrap_or((0, 0));
                    if size <= threshold {
                        (size, above + size)
                    } else {
                        (size, above)
                    }
                }
            }
        }

        let (_, above) = aux(self, threshold);
        above
    }

    fn total_size(&self) -> u32 {
        match self {
            FSNode::File(f) => f.size,
            FSNode::Directory(d) => d.children.values().map(|n| n.borrow().total_size()).sum(),
        }
    }

    fn smallest_above(&self, threshold: u32) -> Option<u32> {
        fn aux(node: &FSNode, threshold: u32) -> (u32, Option<u32>) {
            match node {
                FSNode::File(f) => (f.size, None),
                FSNode::Directory(d) => {
                    let (size, smallest) = d
                        .children
                        .values()
                        .map(|n| aux(&n.borrow(), threshold))
                        .reduce(|acc, item| {
                            (
                                acc.0 + item.0,
                                if let (Some(a), Some(b)) = (acc.1, item.1) {
                                    Some(a.min(b))
                                } else {
                                    acc.1.or(item.1)
                                },
                            )
                        })
                        .unwrap_or((0, None));
                    if smallest.is_some() || size < threshold {
                        (size, smallest)
                    } else {
                        (size, Some(size))
                    }
                }
            }
        }
        let (_, smallest) = aux(self, threshold);
        smallest
    }
}

pub fn part1(input: &str) -> u32 {
    let fs = FSNode::from_commands(input);
    fs.sizes_below(100_000)
}

pub fn part2(input: &str) -> u32 {
    let total_space = 70_000_000;
    let needed_space = 30_000_000;
    let fs = FSNode::from_commands(input);
    let missing_space = needed_space - (total_space - fs.total_size());
    fs.smallest_above(missing_space).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day7.txt");
    const INPUT: &str = include_str!("../inputs/day7.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 95437);
        assert_eq!(part1(INPUT), 2104783);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 24933642);
        assert_eq!(part2(INPUT), 5883165);
    }
}
