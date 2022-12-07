const INPUT: &str = include_str!("res/07.txt");

#[derive(Debug)]
struct Dir<'a> {
    name: &'a str,
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
    name: &'a str,
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
            name,
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
                    let file = Node::File(File { size, name });
                    dir.add_child(file);
                }
                _ => {}
            }
        }
        Node::Dir(dir)
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

#[test]
fn part1() {
    let mut lines = INPUT.lines();
    let tree = Node::parse(&mut lines, "/");
    println!("{tree:#?}");
    println!("{}", sum_under_threshold(&tree, 100_000))
}
