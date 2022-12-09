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
    fn from_commands(filename: &str) -> Self {
        let root = FSNode::Directory(DirectoryNode::new("/"));
        let root = Rc::new(RefCell::new(root));
        let contents = std::fs::read_to_string(filename).unwrap();
        let mut current_path = vec![Rc::clone(&root)];
        let mut lines = contents.lines().peekable();
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
}

fn small_dirs(filename: &str) -> u32 {
    let fs = FSNode::from_commands(filename);
    fs.sizes_below(100_000)
}

fn puzzle1() {
    assert_eq!(small_dirs("example"), 95437);
    assert_eq!(small_dirs("input"), 2104783);
}
fn main() {
    puzzle1();
}
