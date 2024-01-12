use std::{fs::read_to_string, collections::HashSet};

fn main() {
    let input = read_to_string("inputs/input7.txt").unwrap();
    let mut graph: Vec<Vec<u8>> = vec![vec![]; 26];
    let mut all_nodes = HashSet::<u8>::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split(" ").collect();
        let c1 = parts[1].as_bytes()[0] - b'A';
        let c2 = parts[7].as_bytes()[0] - b'A';
        graph[c1 as usize].push(c2);
        all_nodes.insert(c1);
        all_nodes.insert(c2);
    }
    // part1(&graph, &all_nodes);
    part2(&graph, &all_nodes);
}

fn part1(graph: &Vec<Vec<u8>>, all_nodes: &HashSet<u8>) {
    let mut indegree = vec![0; 26];
    for (a, bs) in graph.iter().enumerate() {
        for &b in bs {
            indegree[b as usize] += 1;
        }
    }

    let mut visited = HashSet::<u8>::new();
    let mut path: Vec<char> = vec![];

    while path.len() < all_nodes.len() {
        for i in 0..26 {
            if indegree[i] == 0 && all_nodes.contains(&(i as u8)) &&!visited.contains(&(i as u8)) {
                let i = i as u8;
                let c = (i + b'A') as char;
                path.push(c);
                visited.insert(i);
                for &next in graph[i as usize].iter() {
                    indegree[next as usize] -= 1;
                }
                break;
            }
        }
    }
    let ans: String = path.iter().collect();
    println!("{}", ans);
    println!("{}", ans.len());
}


fn part2(graph: &Vec<Vec<u8>>, all_nodes: &HashSet<u8>) {
    let mut indegree = vec![0; 26];
    for (a, bs) in graph.iter().enumerate() {
        for &b in bs {
            indegree[b as usize] += 1;
        }
    }

    let mut visited = HashSet::<u8>::new();
    let mut path: Vec<char> = vec![];

    let mut worker_finish_times: [i32; 5] = [i32::MAX; 5];
    let mut worker_items: [Option<u8>; 5] = [None; 5];
    let mut cur_time = 0;
    while path.len() < all_nodes.len() {
        let mut has_nodes = false;
        
        for i in 0..26 {
            
            if indegree[i] == 0 && all_nodes.contains(&(i as u8)) &&!visited.contains(&(i as u8)) {
                has_nodes = true;
                let i = i as u8;
                let mut found_worker = false;
                for wi in 0..5 {
                    if worker_items[wi].is_none() {
                        worker_items[wi] = Some(i);
                        worker_finish_times[wi] = cur_time + 61 + i as i32;
                        found_worker = true;
                        visited.insert(i);
                        break;
                    }
                }
                if !found_worker {
                    break;
                }
                
            }
        }

        let has_workers = worker_items.iter().any(|c| c.is_none());
        if !has_workers || !has_nodes {
                
            cur_time = *worker_finish_times.iter().min().unwrap();
            for i in 0..5 {
                if worker_finish_times[i] == cur_time {
                    let c = worker_items[i].unwrap();
                    path.push((c + b'A') as char);
                    for &next in graph[c as usize].iter() {
                        indegree[next as usize] -= 1;
                    }
                    worker_items[i] = None;
                    worker_finish_times[i] = i32::MAX;
                }
            }
        }
    }
    let ans: String = path.iter().collect();
    println!("{}", ans);
    println!("{}", ans.len());
    println!("{}", cur_time);
}
