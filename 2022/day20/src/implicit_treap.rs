use std::cmp::Ordering;

use slotmap::{new_key_type, Key, SlotMap};

new_key_type! { pub struct NodeKey; }
type Nodes<V> = SlotMap<NodeKey, Node<V>>;
type Anchor = NodeKey;

struct Node<V> {
    value: V,
    priority: u64,
    count: usize,
    children: [Anchor; 2],
    parent: NodeKey,
}

impl<V> Node<V> {
    fn new(value: V) -> Self {
        Node {
            value,
            priority: rand::random(),
            count: 1,
            children: [NodeKey::null(); 2],
            parent: NodeKey::null(),
        }
    }
}

pub struct ImplicitTreap<V> {
    nodes: Nodes<V>,
    root: NodeKey,
}

impl<V> ImplicitTreap<V> {
    pub fn new() -> Self {
        ImplicitTreap {
            nodes: Nodes::default(),
            root: NodeKey::null(),
        }
    }

    pub fn len(&self) -> usize {
        match self.nodes.get(self.root) {
            None => 0,
            Some(node) => node.count,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.get(self.root).is_none()
    }

    #[cfg(test)]
    fn check(&self) {
        // returns the number of nodes in the subtree
        fn aux<V>(
            nodes: &Nodes<V>,
            node_key: NodeKey,
            parent: NodeKey,
            parent_priority: Option<u64>,
        ) -> usize {
            match nodes.get(node_key) {
                None => 0,
                Some(node) => {
                    // check parent back reference
                    assert_eq!(node.parent, parent, "invalid parent for node {node_key:?}",);
                    // check heap invariant
                    if let Some(parent_priority) = parent_priority {
                        assert!(
                            node.priority <= parent_priority,
                            "invalid priority for node {node_key:?}"
                        );
                    }
                    // recurse
                    let mut count = 0;
                    count += aux(nodes, node.children[0], node_key, Some(node.priority));
                    count += 1;
                    count += aux(nodes, node.children[1], node_key, Some(node.priority));
                    assert_eq!(count, node.count, "invalid node count for {node_key:?}");
                    count
                }
            }
        }
        aux(&self.nodes, self.root, NodeKey::null(), None);
    }

    // attach new_node_key to its own parent, using old_node_key to know what child it was
    fn set_parent_child(
        &mut self,
        parent_key: NodeKey,
        old_node_key: NodeKey,
        new_node_key: NodeKey,
    ) {
        match self.nodes.get_mut(parent_key) {
            None => self.root = new_node_key,
            Some(parent) => {
                if parent.children[0] == old_node_key {
                    parent.children[0] = new_node_key;
                } else if parent.children[1] == old_node_key {
                    parent.children[1] = new_node_key;
                } else {
                    unreachable!()
                }
            }
        }
    }

    // move the dir-child of node_key to the top, while keeping the order
    // returns the new top (dir-child)
    fn rotate(&mut self, node_key: NodeKey, dir: usize) -> NodeKey {
        let new_node_key = self.nodes[node_key].children[dir];
        assert!(self.nodes[new_node_key].priority > self.nodes[node_key].priority);

        let parent_key = self.nodes[node_key].parent;

        // detach node
        self.nodes[node_key].parent = NodeKey::null();

        // detach new_node from node
        self.nodes[new_node_key].parent = NodeKey::null();
        self.nodes[node_key].children[dir] = NodeKey::null();
        self.nodes[node_key].count -= self.nodes[new_node_key].count;

        // move grand-child from new_node to node
        let grand_child_key = self.nodes[new_node_key].children[1 - dir];
        if let Some(grand_child) = self.nodes.get_mut(grand_child_key) {
            grand_child.parent = node_key;
            let grand_child_count = grand_child.count;
            self.nodes[new_node_key].children[1 - dir] = NodeKey::null();
            self.nodes[new_node_key].count -= grand_child_count;
            self.nodes[node_key].children[dir] = grand_child_key;
            self.nodes[node_key].count += grand_child_count;
        }

        // attach node to new_node
        self.nodes[node_key].parent = new_node_key;
        self.nodes[new_node_key].children[1 - dir] = node_key;
        self.nodes[new_node_key].count += self.nodes[node_key].count;

        // attach new_node to node's parent
        self.nodes[new_node_key].parent = parent_key;
        self.set_parent_child(parent_key, node_key, new_node_key);

        new_node_key
    }

    pub fn insert(&mut self, index: usize, value: V) -> NodeKey {
        // returns true when rebalancing might be needed
        fn aux<V>(
            treap: &mut ImplicitTreap<V>,
            node_key: NodeKey,
            mut index: usize,
            new_node_key: NodeKey,
        ) -> (NodeKey, bool) {
            let nodes = &mut treap.nodes;
            let current_index = nodes.get(node_key).map_or(0, |node| {
                nodes.get(node.children[0]).map_or(0, |child| child.count)
            });
            if index == current_index {
                if nodes.get(node_key).is_some() {
                    let left_key = nodes[node_key].children[0];
                    let parent_key = nodes[node_key].parent;
                    nodes[new_node_key].parent = parent_key;
                    nodes[new_node_key].count = nodes[node_key].count + 1;
                    nodes[new_node_key].children[0] = left_key;
                    nodes[new_node_key].children[1] = node_key;
                    if let Some(left) = nodes.get_mut(left_key) {
                        left.parent = new_node_key;
                    }
                    nodes[node_key].parent = new_node_key;
                    nodes[node_key].children[0] = NodeKey::null();
                    nodes[node_key].count = 1 + nodes
                        .get(nodes[node_key].children[1])
                        .map_or(0, |child| child.count);
                    treap.set_parent_child(parent_key, node_key, new_node_key);
                    (treap.bubble_down(new_node_key), true)
                } else {
                    (new_node_key, true)
                }
            } else {
                let dir = if index < current_index {
                    0
                } else {
                    // index > current_index
                    index -= current_index + 1;
                    1
                };
                let (new_child_key, should_rebalance) = aux(
                    treap,
                    treap.nodes[node_key].children[dir],
                    index,
                    new_node_key,
                );
                let nodes = &mut treap.nodes;
                nodes[node_key].children[dir] = new_child_key;
                nodes[new_child_key].parent = node_key;
                nodes[node_key].count += 1;
                if !should_rebalance {
                    return (node_key, false);
                }
                let child_key = nodes[node_key].children[dir];
                assert_eq!(child_key, new_child_key);
                if nodes[child_key].priority > nodes[node_key].priority {
                    // bubble up
                    (treap.rotate(node_key, dir), true)
                } else {
                    (node_key, false)
                }
            }
        }
        let new_node_key = self.nodes.insert(Node::new(value));
        (self.root, _) = aux(self, self.root, index, new_node_key);
        new_node_key
    }

    pub fn push(&mut self, value: V) -> NodeKey {
        let index = self.nodes.get(self.root).map_or(0, |node| node.count);
        self.insert(index, value)
    }

    pub fn find(&self, index: usize) -> NodeKey {
        fn aux<V>(nodes: &Nodes<V>, node_key: NodeKey, index: usize) -> NodeKey {
            match nodes.get(node_key) {
                None => NodeKey::null(),
                Some(node) => {
                    let current_index = nodes.get(node.children[0]).map_or(0, |child| child.count);
                    match index.cmp(&current_index) {
                        Ordering::Equal => node_key,
                        Ordering::Less => aux(nodes, node.children[0], index),
                        Ordering::Greater => {
                            aux(nodes, node.children[1], index - current_index - 1)
                        }
                    }
                }
            }
        }
        aux(&self.nodes, self.root, index)
    }

    pub fn node_index(&self, mut node_key: NodeKey) -> usize {
        let mut node = &self.nodes[node_key];
        let left_key = node.children[0];
        // count left children
        let mut ret = self.nodes.get(left_key).map_or(0, |left| left.count);
        // count left uncles
        while let Some(parent) = self.nodes.get(node.parent) {
            if node_key == parent.children[1] {
                // was right-child, need to count…
                let left_key = parent.children[0];
                // …the parent's left siblings
                ret += self.nodes.get(left_key).map_or(0, |left| left.count);
                // …and the parent itself
                ret += 1;
            }
            node_key = node.parent;
            node = parent;
        }
        ret
    }

    // remove the leftmost descendant of node_key
    // preserve the invariants
    fn leftmost(&mut self, node_key: NodeKey) -> NodeKey {
        let left_key = self.nodes[node_key].children[0];
        let left_left_key = self.nodes[left_key].children[0];
        if self.nodes.get(left_left_key).is_some() {
            let ret = self.leftmost(left_key);
            self.nodes[node_key].count -= 1;
            ret
        } else {
            let left_right_key = self.nodes[left_key].children[1];
            self.nodes[node_key].count -= 1;
            self.nodes[node_key].children[0] = left_right_key;
            if let Some(left_right) = self.nodes.get_mut(left_right_key) {
                left_right.parent = node_key;
            }
            left_key
        }
    }

    // move node_key to its proper position downwards
    // return the key of the node which is at its position at the end
    fn bubble_down(&mut self, node_key: NodeKey) -> NodeKey {
        let node = &self.nodes[node_key];
        let mut max_priority = node.priority;
        let mut max_priority_dir = 2;
        if let Some(child) = self.nodes.get(node.children[0]) {
            if child.priority > max_priority {
                max_priority = child.priority;
                max_priority_dir = 0;
            }
        }
        if let Some(child) = self.nodes.get(node.children[1]) {
            if child.priority > max_priority {
                // max_priority = child.priority;
                max_priority_dir = 1;
            }
        }
        if max_priority_dir == 2 {
            // nothing to do
            return node_key;
        }
        // move max_priority_dir-child in the place of node_key
        let ret = self.rotate(node_key, max_priority_dir);
        self.bubble_down(node_key);
        ret
    }

    pub fn remove_node(&mut self, node_key: NodeKey) -> Option<V> {
        match self.nodes.remove(node_key) {
            None => None,
            Some(node) => {
                let parent_key = node.parent;
                // find replacement node, if any
                let left_key = node.children[0];
                let right_key = node.children[1];
                match (self.nodes.get(left_key), self.nodes.get(right_key)) {
                    (None, None) => {
                        self.set_parent_child(parent_key, node_key, NodeKey::null());
                    }
                    (Some(_), None) => {
                        self.nodes[node.children[0]].parent = node.parent;
                        self.set_parent_child(parent_key, node_key, node.children[0]);
                    }
                    (None, Some(_)) => {
                        self.nodes[node.children[1]].parent = node.parent;
                        self.set_parent_child(parent_key, node_key, node.children[1]);
                    }
                    (Some(_), Some(right)) => {
                        let right_left_key = right.children[0];
                        match self.nodes.get(right_left_key) {
                            None => {
                                self.nodes[left_key].parent = right_key;
                                let right = &mut self.nodes[right_key];
                                right.parent = node.parent;
                                right.children[0] = left_key;
                                right.count = node.count - 1;
                                self.set_parent_child(node.parent, node_key, right_key);
                                self.bubble_down(right_key);
                            }
                            Some(_) => {
                                let new_node_key = self.leftmost(right_key);
                                let new_node = &mut self.nodes[new_node_key];
                                new_node.count = node.count - 1;
                                new_node.parent = node.parent;
                                new_node.children[0] = left_key;
                                new_node.children[1] = right_key;
                                self.nodes[left_key].parent = new_node_key;
                                self.nodes[right_key].parent = new_node_key;
                                self.set_parent_child(node.parent, node_key, new_node_key);
                                self.bubble_down(new_node_key);
                            }
                        }
                    }
                };
                // update parents' node counts
                let mut node_key = parent_key;
                while let Some(node) = self.nodes.get_mut(node_key) {
                    node.count -= 1;
                    node_key = node.parent;
                }
                // return value of old node
                Some(node.value)
            }
        }
    }

    pub fn remove_at(&mut self, index: usize) -> Option<V> {
        let node_key = self.find(index);
        assert_eq!(self.node_index(node_key), index, "node_index");
        self.remove_node(node_key)
    }

    pub fn pop(&mut self) -> Option<V> {
        if let Some(node) = self.nodes.get(self.root) {
            self.remove_at(node.count - 1)
        } else {
            None
        }
    }
}

impl<V> Default for ImplicitTreap<V> {
    fn default() -> Self {
        ImplicitTreap::new()
    }
}

impl<V: std::fmt::Display> ImplicitTreap<V> {
    pub fn print_vec(&self) {
        fn aux<V: std::fmt::Display>(nodes: &Nodes<V>, node_key: NodeKey) {
            match nodes.get(node_key) {
                None => (),
                Some(node) => {
                    aux(nodes, node.children[0]);
                    print!("{} [{:?}], ", node.value, node_key);
                    aux(nodes, node.children[1])
                }
            }
        }
        aux(&self.nodes, self.root);
        println!();
    }

    pub fn print_tree(&self) {
        fn aux<V: std::fmt::Display>(nodes: &Nodes<V>, node_key: NodeKey, depth: usize) {
            let prefix = "    ".repeat(depth);
            match nodes.get(node_key) {
                None => println!("{}-", prefix),
                Some(node) => {
                    aux(nodes, node.children[1], depth + 1);
                    println!(
                        "{}- {} (priority={}, count={}) [{:?} <- {:?}]",
                        prefix, node.value, node.priority, node.count, node.parent, node_key
                    );
                    aux(nodes, node.children[0], depth + 1);
                }
            }
        }
        aux(&self.nodes, self.root, 0);
    }
}

// non-consuming iterator
enum ExplorationState {
    Unexplored,
    LeftYielded,
}

pub struct IterRef<'a, V> {
    treap: &'a ImplicitTreap<V>,
    stack: Vec<(ExplorationState, NodeKey)>,
}

impl<'a, V> IterRef<'a, V> {
    fn new(treap: &'a ImplicitTreap<V>) -> Self {
        if treap.nodes.get(treap.root).is_some() {
            IterRef {
                treap,
                stack: vec![(ExplorationState::Unexplored, treap.root)],
            }
        } else {
            IterRef {
                treap,
                stack: vec![],
            }
        }
    }
}

impl<'a, V> Iterator for IterRef<'a, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((state, node_key)) = self.stack.pop() {
            match state {
                ExplorationState::Unexplored => {
                    self.stack.push((ExplorationState::LeftYielded, node_key));
                    let node = &self.treap.nodes[node_key];
                    if self.treap.nodes.get(node.children[0]).is_some() {
                        self.stack
                            .push((ExplorationState::Unexplored, node.children[0]));
                    }
                    self.next()
                }
                ExplorationState::LeftYielded => {
                    let node = &self.treap.nodes[node_key];
                    if self.treap.nodes.get(node.children[1]).is_some() {
                        self.stack
                            .push((ExplorationState::Unexplored, node.children[1]));
                    }
                    Some(&node.value)
                }
            }
        } else {
            None
        }
    }
}

impl<'a, V> IntoIterator for &'a ImplicitTreap<V> {
    type IntoIter = IterRef<'a, V>;
    type Item = &'a V;
    fn into_iter(self) -> Self::IntoIter {
        IterRef::new(self)
    }
}

impl<V> ImplicitTreap<V> {
    pub fn iter(&self) -> IterRef<V> {
        self.into_iter()
    }
}

impl<V: std::fmt::Display> std::ops::Index<usize> for ImplicitTreap<V> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        let node_index = self.find(index);
        &self.nodes[node_index].value
    }
}

impl<V: std::fmt::Display> std::ops::IndexMut<usize> for ImplicitTreap<V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let node_index = self.find(index);
        &mut self.nodes[node_index].value
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};

    #[test]
    fn test() {
        let mut treap = super::ImplicitTreap::new();
        treap.print_tree();
        treap.print_vec();

        for i in 1..10 {
            println!("Inserting {i}");
            treap.push(i);
            treap.print_tree();
            treap.print_vec();
            treap.check();
        }

        while let Some(x) = treap.pop() {
            println!("Removed {x:?}");
            treap.print_tree();
            treap.print_vec();
            treap.check();
        }
    }

    #[test]
    fn big_test() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let mut treap = super::ImplicitTreap::new();
        let mut expected = Vec::new();

        // add some
        for _ in 0..1000 {
            let x: u64 = rng.gen();
            treap.push(x);
            treap.check();
            expected.push(x);
        }
        let actual: Vec<_> = treap.iter().copied().collect();
        assert_eq!(actual, expected);

        // add some more
        for _ in 0..100 {
            let i: usize = rng.gen_range(0..expected.len());
            let x: u64 = rng.gen();
            treap.insert(i, x);
            treap.check();
            expected.insert(i, x);
        }
        let actual: Vec<_> = treap.iter().copied().collect();
        assert_eq!(actual, expected);

        // remove some
        for _ in 0..100 {
            let i = rng.gen_range(0..expected.len() - 1);
            treap.remove_at(i);
            treap.check();
            expected.remove(i);
        }
        let actual: Vec<_> = treap.iter().copied().collect();
        assert_eq!(actual, expected);
    }
}
