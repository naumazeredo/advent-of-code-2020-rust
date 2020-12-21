pub fn run(part : &str, buffer: String) {
    let mut nodes = Vec::new();

    buffer.lines()
        .for_each(|s| {
            let mut iter = s.split_whitespace();
            let node_type = iter.next().unwrap();
            let value = iter.next().unwrap().parse::<i32>().unwrap();
            add_node(node_type, value, &mut nodes);
        });


    match part {
        "1" => part1(&nodes),
        "2" => part2(&mut nodes),
        _ => (),
    }
}

fn part1(nodes: &Vec<Node>) {
    let acc = execute(nodes);
    println!("{}", acc);
}


fn part2(nodes: &mut Vec<Node>) {
    let mut vis = vec![Vis::No; nodes.len() + 1];
    vis[nodes.len()] = Vis::Reaches;

    fn find_error(vis: &mut Vec<Vis>, nodes: &Vec<Node>) -> usize {
        dfs(0, Vis::InPath, vis, &nodes);
        for pos in 1..nodes.len() {
            dfs(pos, Vis::NotInPath, vis, &nodes);
        }

        for pos in 0..nodes.len() {
            match nodes[pos] {
                Node::Nop(x) => {
                    let new_pos = add(pos, x);
                    if vis[pos] == Vis::InPath && vis[new_pos] == Vis::Reaches {
                        return pos;
                    }
                },
                Node::Jmp(_) => {
                    let new_pos = add(pos, 1);
                    if vis[pos] == Vis::InPath && vis[new_pos] == Vis::Reaches {
                        return pos;
                    }
                },
                _ => (),
            }
        }

        panic!();
    }

    let error_pos = find_error(&mut vis, &nodes);
    println!("error {}", error_pos);

    match nodes[error_pos] {
        Node::Nop(x) => nodes[error_pos] = Node::Jmp(x),
        Node::Jmp(x) => nodes[error_pos] = Node::Nop(x),
        _ => panic!(),
    }

    let acc = execute(nodes);
    println!("{}", acc);
}

fn add(a: usize, b: i32) -> usize {
    if b.is_negative() {
        a.checked_sub(b.wrapping_abs() as usize).unwrap()
    } else {
        a.checked_add(b as usize).unwrap()
    }
}

enum Node {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn add_node(node_type: &str, value: i32, nodes: &mut Vec<Node>) {
    match node_type {
        "nop" => nodes.push(Node::Nop(value)),
        "acc" => nodes.push(Node::Acc(value)),
        "jmp" => nodes.push(Node::Jmp(value)),
        _ => panic!(),
    }
}

fn execute(nodes: &Vec<Node>) -> i32 {
    fn execute_single(pos: &mut usize, acc: &mut i32, nodes: &Vec<Node>) {
        match nodes[*pos] {
            Node::Nop(_) => *pos = add(*pos, 1),
            Node::Acc(x) => {
                *acc += x;
                *pos = add(*pos, 1);
            },
            Node::Jmp(x) => *pos = add(*pos, x),
        }
    }

    let mut vis = vec![false; nodes.len()+1];
    vis[nodes.len()] = true;

    let mut pos = 0usize;
    let mut acc = 0i32;
    while !vis[pos] {
        vis[pos] = true;
        execute_single(&mut pos, &mut acc, &nodes);
    }

    acc
}

#[derive(Clone, Copy, PartialEq)]
enum Vis {
    No,
    NotInPath,
    InPath,
    Reaches,
}

fn dfs(pos: usize, vis_state: Vis, vis: &mut Vec<Vis>, nodes: &Vec<Node>) -> Vis {
    if vis[pos] != Vis::No { return vis[pos]; }
    vis[pos] = vis_state;

    match nodes[pos] {
        Node::Nop(_) | Node::Acc(_) => {
            let next = dfs(add(pos, 1), vis_state, vis, nodes);
            if next == Vis::Reaches {
                vis[pos] = Vis::Reaches;
            }
        },
        Node::Jmp(x) => {
            if dfs(add(pos, x), vis_state, vis, nodes) == Vis::Reaches {
                vis[pos] = Vis::Reaches;
            }
        }
    }

    return vis[pos];
}

