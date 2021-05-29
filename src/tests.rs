use crate::{disjoint_sets, UnionFind};
use std::hash::Hash;

pub fn unions<T: Eq + Hash>(sets: Vec<Vec<T>>) -> Vec<Vec<T>> {
    disjoint_sets::scoped(|mut uf| {
        for mut set in sets {
            if let Some(r) = set.pop() {
                let rep = uf.insert(r);
                for e in set {
                    uf.insert_union(rep, e);
                }
            }
        }

        uf.groups().collect()
    })
}

#[test]
fn unions_works() {
    let sets = vec![
        vec!["A", "B", "C", "D"],
        vec!["E", "F", "G"],
        vec!["B", "F"],
        vec!["H", "I", "J", "K"],
        // vec!["G", "H"],
    ];
    let unioned = unions(sets)
        .into_iter()
        .map(|mut set| {
            set.sort();
            set
        })
        .collect::<Vec<_>>();
    assert!(
        unioned
            == vec![
                vec!["H", "I", "J", "K"],
                vec!["A", "B", "C", "D", "E", "F", "G"]
            ]
            || unioned
                == vec![
                    vec!["A", "B", "C", "D", "E", "F", "G"],
                    vec!["H", "I", "J", "K"],
                ]
    );

    let sets = vec![
        vec!["A", "B", "C", "D"],
        vec!["E", "F", "G"],
        vec!["B", "F"],
        vec!["H", "I", "J", "K"],
        vec!["G", "H"],
    ];
    let unioned = unions(sets)
        .into_iter()
        .map(|mut set| {
            set.sort();
            set
        })
        .collect::<Vec<_>>();
    assert_eq!(
        unioned,
        vec![vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K"]]
    );
}

type Node = usize;
type Weight = usize;

struct Edge {
    dst: Node,
    weight: Weight,
}

type Graph = Vec<Vec<Edge>>;

fn edges_by_weight(graph: &Graph) -> Vec<(Node, Node, Weight)> {
    let mut edges = vec![];

    for (src, dsts) in graph.iter().enumerate() {
        for edge in dsts {
            edges.push((src, edge.dst, edge.weight));
        }
    }

    edges.sort_by_key(|&(_, _, weight)| weight);
    edges
}

fn mst(graph: &Graph) -> Vec<(Node, Node)> {
    disjoint_sets::scoped(|mut uf| {
        let mut result = vec![];
        for (src, dst, _) in edges_by_weight(graph) {
            let src_rep = uf.insert(src);
            let dst_rep = uf.insert(dst);
            if src_rep != dst_rep {
                uf.union(src_rep, dst_rep);
                result.push((src, dst));
            }
        }

        result
    })
}

#[test]
fn mst_works() {
    // Graph to use:
    //
    //  0 ------ 1 ------ 2
    //  |    6   |    5   |
    //  | 8      | 1      | 4
    //  |        |        |
    //  3 ------ 4 ------ 5
    //  |    7   |    2   |
    //  | 3      | 12     | 11
    //  |        |        |
    //  6 ------ 7 ------ 8
    //       9        10
    let graph = vec![
        // Node 0
        vec![Edge { dst: 1, weight: 6 }, Edge { dst: 3, weight: 8 }],
        // Node 1
        vec![Edge { dst: 2, weight: 5 }, Edge { dst: 4, weight: 1 }],
        // Node 2
        vec![Edge { dst: 5, weight: 4 }],
        // Node 3
        vec![Edge { dst: 4, weight: 7 }, Edge { dst: 6, weight: 3 }],
        // Node 4
        vec![Edge { dst: 5, weight: 2 }, Edge { dst: 7, weight: 12 }],
        // Node 5
        vec![Edge { dst: 8, weight: 11 }],
        // Node 6
        vec![Edge { dst: 7, weight: 9 }],
        // Node 7
        vec![Edge { dst: 8, weight: 10 }],
        // Node 8
        vec![],
    ];

    assert_eq! {
        vec![ (1, 4), (4, 5), (3, 6), (2, 5),
              (0, 1), (3, 4), (6, 7), (7, 8), ],
        mst(&graph)
    };
}
