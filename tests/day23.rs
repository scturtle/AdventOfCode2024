use ahash::{AHashMap, AHashSet};
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::{tag, take},
    character::complete::line_ending,
    multi::separated_list1 as sep1,
    sequence::separated_pair,
};
use std::iter::once;

fn bron_kerbosch(n: usize, adj: &[AHashSet<usize>]) -> Vec<AHashSet<usize>> {
    let mut maximal_cliques = Vec::new();
    let mut stack = Vec::new();
    let r = AHashSet::new();
    let p = (0..n).collect::<AHashSet<_>>();
    let x = AHashSet::new();
    stack.push((r, p, x));
    while let Some((r, mut p, mut x)) = stack.pop() {
        if p.is_empty() && x.is_empty() {
            if !r.is_empty() {
                maximal_cliques.push(r);
            }
            continue;
        }
        if p.is_empty() {
            continue;
        }
        let pivot_candidates = &p | &x;
        let pivot = pivot_candidates
            .iter()
            .max_by_key(|&&u| adj[u].intersection(&p).count())
            .unwrap();
        let p_without_pivot_neighbors: AHashSet<usize> =
            p.difference(&adj[*pivot]).cloned().collect();
        for v in p_without_pivot_neighbors.iter() {
            let new_r = {
                let mut next_r = r.clone();
                next_r.insert(*v);
                next_r
            };
            let new_p = p.intersection(&adj[*v]).cloned().collect();
            let new_x = x.intersection(&adj[*v]).cloned().collect();
            stack.push((new_r, new_p, new_x));
            p.remove(v);
            x.insert(*v);
        }
    }
    maximal_cliques
}

fn parse(s: &str) -> IResult<&str, Vec<(&str, &str)>> {
    sep1(
        line_ending,
        separated_pair(take(2usize), tag("-"), take(2usize)),
    )
    .parse(s)
}

#[test]
fn day23() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(23).unwrap();
    let (_, pairs) = parse(&txt).unwrap();

    // part one
    let conns: AHashSet<_> = pairs
        .iter()
        .flat_map(|&(a, b)| once((a, b)).chain(once((b, a))))
        .collect();
    let computers = pairs
        .iter()
        .flat_map(|&(a, b)| once(a).chain(once(b)))
        .unique()
        .collect_vec();
    let mut ans1 = 0;
    for (a, b, c) in computers.iter().tuple_combinations() {
        if !a.starts_with('t') && !b.starts_with('t') && !c.starts_with('t') {
            continue;
        }
        if conns.contains(&(a, b)) && conns.contains(&(b, c)) && conns.contains(&(a, c)) {
            ans1 += 1;
        }
    }
    dbg!(ans1);

    // part two
    let id_to_node: Vec<&str> = computers;
    let node_to_id: AHashMap<&str, usize> = id_to_node
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect();

    let mut adj: Vec<AHashSet<usize>> = vec![AHashSet::new(); id_to_node.len()];
    for (u_str, v_str) in pairs {
        let u_id = *node_to_id.get(&u_str).unwrap();
        let v_id = *node_to_id.get(&v_str).unwrap();
        adj[u_id].insert(v_id);
        adj[v_id].insert(u_id);
    }

    let maximal_cliques = bron_kerbosch(id_to_node.len(), &adj);
    let ans2 = maximal_cliques
        .into_iter()
        .max_by_key(|x| x.len())
        .unwrap()
        .into_iter()
        .map(|i| id_to_node[i])
        .sorted()
        .collect_vec()
        .join(",");
    dbg!(ans2);

    // # /// script
    // # dependencies = ["networkx"]
    // # ///
    // import networkx as nx
    // from networkx.algorithms.clique import find_cliques
    // G = nx.read_edgelist('23.txt', delimiter='-')
    // lst = max(find_cliques(G), key=len)
    // print(",".join(sorted(lst)))
}
