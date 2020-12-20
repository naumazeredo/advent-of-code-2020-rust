use std::collections::HashMap;

type AdjList = Vec<Vec<(usize, usize)>>;

pub fn run(part : &str, buffer: String) {
    match part {
        "1" => part1(buffer),
        "2" => part2(buffer),
        _ => (),
    }
}

fn part1(buffer: String) {
    let mut mapping  = HashMap::new();
    let mut adj_list = AdjList::new();

    let start_id = add_mapping("shiny gold".to_owned(), &mut mapping, &mut adj_list);

    buffer.lines()
        .for_each(|line| {
            let mut iter = line.split("bags contain");
            let parent = iter.next().unwrap().trim();
            let parent_id = add_mapping(parent.to_owned(), &mut mapping, &mut adj_list);

            let s: String = iter.collect();
            s.split(",")
                .for_each(|desc| {
                    let child: String = desc
                        .split_whitespace().skip(1).take(2)
                        .collect::<Vec<&str>>()
                        .join(" ");
                    let child_id = add_mapping(child, &mut mapping, &mut adj_list);
                    add_edge(child_id, parent_id, 0, &mut adj_list);
                });
        });

    // BFS (with stack)
    {
        let mut ans = -1;
        let mut vis = vec![false; adj_list.len()];
        let mut elems: Vec<usize> = vec![start_id];
        while !elems.is_empty() {
            ans += 1;
            let u = elems.pop().unwrap();

            for (v, _) in adj_list[u].iter() {
                if !vis[*v] {
                    vis[*v] = true;
                    elems.push(*v);
                }
            }
        }

        println!("BFS: {}", ans);
    }

    // DFS
    {
        let mut vis = vec![false; adj_list.len()];
        fn dfs(u: usize, vis: &mut Vec<bool>, adj_list: &AdjList) -> u32 {
            if vis[u] { return 0; }
            vis[u] = true;
            let mut cnt = 1;
            for (v, _) in adj_list[u].iter() {
                cnt += dfs(*v, vis, adj_list);
            }
            return cnt;
        }

        let ans = dfs(start_id, &mut vis, &adj_list) - 1;

        println!("DFS: {}", ans);
    }
}

fn part2(buffer: String) {
    // Get input
    let mut mapping   = HashMap::new();
    let mut adj_list  = AdjList::new();

    let start_id = add_mapping("shiny gold".to_owned(), &mut mapping, &mut adj_list);

    buffer.lines()
        .for_each(|line| {
            let mut iter = line.split("bags contain");
            let parent = iter.next().unwrap().trim();
            let parent_id = add_mapping(parent.to_owned(), &mut mapping, &mut adj_list);

            let s: String = iter.collect();
            s.split(",")
                .for_each(|desc| {
                    let mut iter = desc.split_whitespace();

                    let weight = match iter.next().unwrap().parse::<usize>() {
                        Ok(v) => v,
                        Err(_) => return,
                    };

                    let child: String = iter
                        .take(2)
                        .collect::<Vec<&str>>()
                        .join(" ");
                    let child_id = add_mapping(child, &mut mapping, &mut adj_list);
                    add_edge(parent_id, child_id, weight, &mut adj_list);
                });
        });

    let mut vis  = vec![false; adj_list.len()];
    let mut size = vec![0usize; adj_list.len()];
    fn dfs(u: usize, vis: &mut Vec<bool>, size: &mut Vec<usize>, adj_list: &AdjList) -> usize {
        if vis[u] { return size[u]; }
        vis[u] = true;
        size[u] = 1;
        for (v, w) in adj_list[u].iter() {
            let sv = dfs(*v, vis, size, adj_list);
            size[u] += w * sv;
        }
        return size[u];
    }

    let ans = dfs(start_id, &mut vis, &mut size, &adj_list) - 1;

    println!("{}", ans);
}

fn add_mapping(
    name: String,
    mapping: &mut HashMap<String, usize>,
    adj_list: &mut AdjList) -> usize {

    if let Some(&id) = mapping.get(&name) {
        id
    } else {
        let id = adj_list.len();
        mapping.insert(name, id);
        adj_list.push(Vec::<(usize, usize)>::new());
        id
    }
}

fn add_edge(from_id: usize, to_id: usize, weight: usize, adj_list: &mut AdjList) {
    adj_list[from_id].push((to_id, weight));
}
