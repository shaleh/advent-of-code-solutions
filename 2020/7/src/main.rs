use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

use petgraph::algo::all_simple_paths;
use petgraph::graph::{NodeIndex, DiGraph};
use petgraph::dot::{Dot, Config};

fn read_input() -> Result<Vec<String>, std::io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.lines().collect::<Result<Vec<_>, _>>()
}

// Walk the input, turn into a graph with connection indicating bags inside.
// Add the number of bags as decoration on the edges.
// no other bags means no connections on the graph
//
// What bags can hold a specific bag is determined by computing paths.
fn process_input(result: &Vec<String>) -> (HashMap<String, NodeIndex>, DiGraph::<String, ()>) {
    let mut graph: DiGraph<String, ()> = DiGraph::new();
    let mut nodes = HashMap::new();

    for line in result {
        let parts = line.split("contain").map(|x| x.trim()).collect::<Vec<_>>();
        let container_bag = parts[0].split(" bags").collect::<Vec<_>>();
        let bag_color = String::from(container_bag[0]);
        let node = match nodes.get(&bag_color) {
            Some(x) => *x,
            None => {
                let new_node = graph.add_node(bag_color.clone());
                nodes.insert(bag_color, new_node);
                new_node
            }
        };
        let contained = parts[1].split(", ").collect::<Vec<_>>();
        if &contained[0][0..3] != "no " {
            for bag in contained {
                let pieces = bag.split(" ").collect::<Vec<_>>();
                let bag_color = String::from(pieces[1..3].join(" "));
                let bag_node = match nodes.get(&bag_color) {
                    Some(x) => *x,
                    None => {
                        let new_node = graph.add_node(bag_color.clone());
                        nodes.insert(bag_color, new_node);
                        new_node
                    }
                };
                graph.add_edge(node, bag_node, ());
            }
        }
    }

    (nodes, graph)
}

fn main() {
    let result = read_input().expect("Failed to read input");
    let (nodes, graph) = process_input(&result);
    // dbg!(&graph);
    // dbg!(&nodes);
    println!("Nodes: {}", nodes.len());
    // dbg!(idx_shiny_gold);
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    // let mut search = Bfs::new(&graph, idx_shiny_gold);
    // //let mut connected = HashSet::new();
    // while let Some(nx) = search.next(&graph) {
    //     // we can access `graph` mutably here still
    //     println!("{:?}", nx);
    // }

    let mut paths: HashSet<_> = HashSet::new();

    let idx_shiny_gold = nodes["shiny gold"];
    for key in nodes.keys() {
        if key == "shiny gold" {
            continue;
        }
        //`println!("trying {}", key);
        let idx = nodes[key];
        let set: HashSet<Vec<_>> = all_simple_paths(&graph, idx_shiny_gold, idx, 0, None).map(|v: Vec<_>| v.into_iter().map(|i| i.index()).collect()).collect();
        for path in set {
            for item in path {
                paths.insert(item);
            }
        }
    }

    println!("{:?}", paths);
    println!("{}", paths.len());
}
