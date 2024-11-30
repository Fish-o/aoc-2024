use std::collections::HashMap;

use petgraph::{
    graph::{NodeIndex, UnGraph},
    visit::NodeIndexable,
    Graph,
};

use super::matrix::{Matrix, Pos};

impl<E> Matrix<E> {
    pub fn gen_graph<NF>(
        &self,
        start: &Vec<Pos>,
        nf: NF,
    ) -> (HashMap<Pos, NodeIndex>, Graph<Pos, usize>)
    where
        NF: Fn(&Self, &Pos) -> Vec<(Pos, usize)>,
    {
        let mut g = Graph::<Pos, usize>::new();
        let mut stack = start.clone();
        let mut index_lookup = HashMap::new();
        start.iter().for_each(|v| {
            index_lookup
                .entry(v.clone())
                .or_insert_with(|| g.add_node(v.clone()));
        });

        while let Some(pos) = stack.pop() {
            let n = nf(self, &pos);
            let c_i = index_lookup.get(&pos).unwrap().clone();
            for (neighbour, weight) in n {
                let n_i = index_lookup
                    .entry(neighbour.clone())
                    .or_insert_with(|| {
                        stack.push(neighbour.clone());
                        g.add_node(neighbour.clone())
                    })
                    .clone();
                if g.edges_connecting(c_i, n_i).any(|e| *e.weight() == weight) {
                    continue;
                };
                g.add_edge(c_i, n_i, weight);
            }
        }

        (index_lookup, g)
    }
}
