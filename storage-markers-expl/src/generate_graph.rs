use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::collections::HashMap;

fn main() {
    let n: usize = 10_000;     // nodes
    let m: usize = 8;          // outgoing edges per node (roughly)
    let seed: u64 = 1;

    let mut rng = StdRng::seed_from_u64(seed);

    // adjacency: node -> outgoing neighbors
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];

    // Preferential attachment target weights ~ indegree+1
    let mut indeg = vec![1usize; n]; // +1 to avoid zero-weight
    let mut weights: Vec<usize> = vec![1usize; 1]; // for nodes currently "active"

    // Start with a small connected core
    let core = 5.min(n);
    for i in 0..core {
        for j in 0..core {
            if i != j {
                g[i].push(j);
                indeg[j] += 1;
            }
        }
    }

    // Maintain weights for nodes [0..i)
    weights = (0..core).map(|u| indeg[u]).collect();

    for i in core..n {
        // Choose m targets among existing nodes with probability proportional to indegree
        let dist = WeightedIndex::new(&weights).unwrap();

        // Use a small set to avoid duplicates per source node
        let mut picked = std::collections::HashSet::new();
        while picked.len() < m.min(i) {
            let t = dist.sample(&mut rng);
            picked.insert(t);
        }

        for &t in picked.iter() {
            g[i].push(t);
            indeg[t] += 1;
            // adding edge increases target's weight
            weights[t] = indeg[t];
        }

        // Optionally add some "reciprocal" edges to create cycles
        if rng.gen_bool(0.15) && !g[i].is_empty() {
            let t = g[i][rng.gen_range(0..g[i].len())];
            g[t].push(i);
            indeg[i] += 1;
        }

        // New node becomes eligible as a target for future nodes
        weights.push(indeg[i]);
    }

    // Emit JSON in the format: {"n0":[...], "n1":[...], ...}
    // (This prints to stdout; redirect to a file.)
    print!("{{");
    for i in 0..n {
        if i > 0 { print!(","); }
        print!("\"n{0}\":[", i);
        for (k, &t) in g[i].iter().enumerate() {
            if k > 0 { print!(","); }
            print!("\"n{}\"", t);
        }
        print!("]");
    }
    println!("}}");
}

