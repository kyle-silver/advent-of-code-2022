const INPUT: &str = include_str!("res/07.txt");
const TOTAL_FS_SIZE: usize = 70_000_000;
const REQUIRED_SPACE: usize = 30_000_000;

#[derive(Debug)]
struct Dir<'a> {
    _name: &'a str,
    size: usize,
    children: Vec<Box<Node<'a>>>,
}

impl<'a> Dir<'a> {
    fn add_child(&mut self, node: Node<'a>) {
        let size = node.size();
        self.size += size;
        self.children.push(Box::new(node));
    }
}

#[derive(Debug)]
struct File<'a> {
    _name: &'a str,
    size: usize,
}

#[derive(Debug)]
enum Node<'a> {
    Dir(Dir<'a>),
    File(File<'a>),
}

impl Node<'_> {
    fn size(&self) -> usize {
        match self {
            Node::Dir(d) => d.size,
            Node::File(f) => f.size,
        }
    }
}

#[derive(Debug)]
enum Cmd<'a> {
    Cd(&'a str),
    Ls,
    FileDescriptor(usize, &'a str),
    DirName(&'a str),
}

impl<'a> Cmd<'a> {
    fn parse(line: &'a str) -> Self {
        let tokens: Vec<_> = line.split_ascii_whitespace().collect();
        let cmd = tokens[1];
        match tokens[0] {
            "$" => match cmd {
                "ls" => return Cmd::Ls,
                "cd" => {
                    let dir = tokens[2];
                    return Cmd::Cd(dir);
                }
                _ => panic!(),
            },
            _ => match tokens[0].parse::<usize>() {
                Ok(size) => Cmd::FileDescriptor(size, tokens[1]),
                Err(_) => Cmd::DirName(tokens[1]),
            },
        }
    }
}

impl<'a> Node<'a> {
    fn parse(lines: &mut impl Iterator<Item = &'a str>, name: &'a str) -> Node<'a> {
        let mut dir = Dir {
            _name: name,
            size: 0,
            children: Vec::new(),
        };
        while let Some(line) = lines.next() {
            match Cmd::parse(line) {
                Cmd::Cd("..") => return Node::Dir(dir),
                Cmd::Cd(child) => {
                    let child = Node::parse(lines, child);
                    dir.add_child(child);
                }
                Cmd::FileDescriptor(size, name) => {
                    let file = Node::File(File { size, _name: name });
                    dir.add_child(file);
                }
                _ => {}
            }
        }
        Node::Dir(dir)
    }

    fn space_available(&self) -> usize {
        TOTAL_FS_SIZE - self.size()
    }
}

fn sum_under_threshold(node: &Node, threshold: usize) -> usize {
    match node {
        Node::File(_) => 0,
        Node::Dir(dir) => {
            let mut acc = if dir.size <= threshold { dir.size } else { 0 };
            for child in &dir.children {
                acc += sum_under_threshold(child.as_ref(), threshold);
            }
            return acc;
        }
    }
}

fn find_deletion_candidates<'a, 'b>(
    node: &'b Node<'a>,
    space_available: usize,
    candidates: &mut Vec<&'b Dir<'a>>,
) {
    match node {
        Node::File(_) => {}
        Node::Dir(d) => {
            if space_available + d.size > REQUIRED_SPACE {
                candidates.push(d);
            }
            for child in &d.children {
                find_deletion_candidates(child, space_available, candidates)
            }
        }
    }
}

#[test]
fn day7() {
    // parse the tree
    let mut lines = INPUT.lines();
    let tree = Node::parse(&mut lines, "/");

    // part 1
    let part1 = sum_under_threshold(&tree, 100_000);
    println!("Day 7, part 1: {part1}");

    // part 2
    let mut candidates = Vec::new();
    let space_available = tree.space_available();
    find_deletion_candidates(&tree, space_available, &mut candidates);
    let part2 = candidates.iter().map(|dir| dir.size).min().unwrap();
    println!("Day 7, part 2: {part2}");
}
