use std::collections::{HashMap, VecDeque};
use std::rc::{Rc, Weak};
use std::{cell::RefCell, collections::BTreeMap, fs, path::Path};

#[derive(Default, Debug)]
struct TrieNode {
    children: BTreeMap<char, Rc<RefCell<TrieNode>>>,
    failure: Weak<RefCell<TrieNode>>,
    is_end: bool,
}

#[derive(Default, Debug)]
struct AhoCorasick {
    root: Rc<RefCell<TrieNode>>,
    occurences: HashMap<usize, usize>,
}

impl AhoCorasick {
    fn new() -> Self {
        Self::default()
    }

    fn obscenity_occurence(&mut self, input: &str) {
        self.build_failures();

        for (index, line) in input.lines().enumerate() {
            self.obsenity_check(line, index);
        }
    }

    fn obsenity_check(&mut self, line: &str, line_index: usize) {
        let mut curr = Rc::clone(&self.root);
        let mut len = 0;
        for (index, c) in line.chars().enumerate() {
            loop {
                if let Some(child_node) = curr.clone().borrow().children.get(&c) {
                    curr = child_node.clone();
                    len += 1;
                    break;
                }

                let failure = curr.clone().borrow().failure.upgrade();
                match failure {
                    Some(failure_node) => curr = failure_node,
                    None => {
                        len = 0;
                        break;
                    }
                }
            }

            let position = index as i32 - len + 2;
            if curr.borrow().is_end {
                self.occurences.insert(line_index, position as usize);
            }
        }
    }

    fn add_word(&mut self, word: &str) {
        let mut current = Rc::clone(&self.root);

        for c in word.chars() {
            current.borrow_mut().children.entry(c).or_default();
            current = Rc::clone(
                Rc::clone(&current)
                    .borrow_mut()
                    .children
                    .entry(c)
                    .or_default(),
            );
        }

        current.borrow_mut().is_end = true;
    }

    fn build_failures(&mut self) {
        let mut q: VecDeque<Rc<RefCell<TrieNode>>> = VecDeque::new();

        q.push_back(Rc::clone(&self.root));
        while let Some(curr) = q.pop_front() {
            let curr = curr.borrow();

            for (c, child_node) in &curr.children {
                q.push_back(Rc::clone(child_node));

                let mut child_node = child_node.borrow_mut();
                let mut failure = curr.failure.upgrade();
                loop {
                    match &failure {
                        Some(trie_node) => {
                            if trie_node.borrow().children.contains_key(c) {
                                child_node.failure = Rc::downgrade(
                                    failure.clone().unwrap().borrow().children.get(c).unwrap(),
                                );
                                break;
                            } else {
                                failure = failure.unwrap().borrow().failure.upgrade();
                            }
                        }
                        None => {
                            child_node.failure = Rc::downgrade(&self.root);
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let file = Path::new("../aho_corasick_input.txt");
    let input = fs::read_to_string(file).expect("Failed to read file");
    let (_, input) = input.split_once('8').unwrap();

    let dict = vec!["dear", "sweetie", "angel", "dream", "baby"];

    let mut ac = AhoCorasick::new();
    for word in dict {
        ac.add_word(word);
    }

    ac.obscenity_occurence(input);

    for res in ac.occurences {
        println!("Line: {} position: {}", res.0, res.1);
    }
}
