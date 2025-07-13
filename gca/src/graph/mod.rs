pub mod viz;

use petgraph::graph::{DiGraph, NodeIndex};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Step {
    pub desc: String,
    pub cost: f32,
    pub p_success: f32,
}

/// Build graph from explicit edge list
pub fn build_graph(steps: Vec<(usize, usize, Step)>) -> DiGraph<Step, ()> {
    let mut g: DiGraph<Step, ()> = DiGraph::new();
    let mut idx_map: std::collections::HashMap<usize, NodeIndex> = Default::default();

    for (s, t, ref step) in &steps {
        idx_map.entry(*s).or_insert_with(|| g.add_node(step.clone()));
        idx_map.entry(*t).or_insert_with(|| g.add_node(step.clone()));
    }
    for (s, t, step) in steps {
        let s_idx = idx_map[&s];
        let t_idx = idx_map[&t];
        g.add_edge(s_idx, t_idx, ());
        g[t_idx] = step; // записываем данные целевого состояния
    }
    g
}

/// Build a simple linear graph from ordered text steps produced by LLM.
pub fn build_from_steps(text_steps: &[String]) -> DiGraph<Step, ()> {
    let mut g: DiGraph<Step, ()> = DiGraph::new();
    let mut prev: Option<NodeIndex> = None;
    for s in text_steps {
        let node = g.add_node(Step { desc: s.clone(), cost: 1.0, p_success: 0.6 });
        if let Some(p) = prev { g.add_edge(p, node, ()); }
        prev = Some(node);
    }
    g
}
