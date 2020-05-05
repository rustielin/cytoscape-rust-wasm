mod utils;

use wasm_bindgen::prelude::*;
use petgraph::graph::{Graph, NodeIndex, NodeIndices};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct CytoGraph {
    graph: Graph::<(), ()>,
    additions: Vec<usize>, // XXX: vec of element IDs (nodes/edges)
    removals: Vec<usize>,
}


#[wasm_bindgen]
impl CytoGraph {
    pub fn new() -> CytoGraph {
        let graph = Graph::<(), ()>::new();
        let additions = Vec::new();
        let removals = Vec::new();

        CytoGraph {
            graph,
            additions,
            removals,
        }
    }

    pub fn tick(&mut self) {
        self.additions.clear();
        self.removals.clear();
    }

    pub fn add_node(&mut self) -> usize {
        let n = self.graph.add_node(());
        self.additions.push(n.index());
        n.index()
    }

    pub fn add_edge(&mut self, src: usize, dst: usize) {
        self.graph.add_edge(NodeIndex::new(src), NodeIndex::new(dst), ());
    }

    pub fn added_nodes(&self) -> *const usize {
        self.additions.as_ptr()
    }

    pub fn removed_nodes(&self) -> *const usize {
        self.removals.as_ptr()
    }

    pub fn additions_count(&self) -> usize {
        self.additions.len()
    }

    pub fn removals_count(&self) -> usize {
        self.removals.len()
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

}

