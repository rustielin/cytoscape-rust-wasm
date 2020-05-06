mod utils;

extern crate web_sys;

use wasm_bindgen::prelude::*;
use petgraph::graph::{Graph, NodeIndex};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct CytoGraph {
    graph: Graph::<(), ()>,
    added_nodes: Vec<usize>,
    removed_nodes: Vec<usize>,
    added_edges: Vec<usize>, // XXX: hack around Rust -> JS type conversion
    removed_edges: Vec<usize>, // just store as vec with even length
    time: u32,
}


#[wasm_bindgen]
impl CytoGraph {
    pub fn new() -> CytoGraph {
        utils::set_panic_hook();
        let graph = Graph::<(), ()>::new();
        let added_nodes = Vec::new();
        let removed_nodes = Vec::new();
        let added_edges = Vec::new();
        let removed_edges = Vec::new();

        CytoGraph {
            graph,
            added_nodes,
            removed_nodes,
            added_edges,
            removed_edges,
            time: 0,
        }
    }

    pub fn tick(&mut self) {
        utils::set_panic_hook();
        self.added_nodes.clear();
        self.removed_nodes.clear();
        self.added_edges.clear();
        self.removed_edges.clear();
        log!("Tick {}", self.time);
        self.time += 1;
    }

    pub fn add_node(&mut self) -> usize {
        let n_idx = self.graph.add_node(());
        let n = n_idx.index();
        self.added_nodes.push(n);

        // make it fully connected for now
        for idx in self.graph.node_indices() {
            let i = idx.index();
            if n == i {
                continue;
            }
            self.add_edge(n, i);
            self.add_edge(i, n);
        }

        n
    }

    pub fn add_edge(&mut self, src: usize, dst: usize) {
        self.graph.add_edge(NodeIndex::new(src), NodeIndex::new(dst), ());
        self.added_edges.push(src);
        self.added_edges.push(dst);
    }

    pub fn get_added_nodes(&self) -> *const usize {
        self.added_nodes.as_ptr()
    }

    pub fn get_removed_nodes(&self) -> *const usize {
        self.removed_nodes.as_ptr()
    }

    pub fn get_added_edges(&self) -> *const usize {
        self.added_edges.as_ptr()
    }

    pub fn get_removed_edges(&self) -> *const usize {
        self.removed_edges.as_ptr()
    }

    pub fn added_nodes_count(&self) -> usize {
        self.added_nodes.len()
    }

    pub fn removed_nodes_count(&self) -> usize {
        self.removed_nodes.len()
    }

    pub fn added_edges_count(&self) -> usize {
        self.added_edges.len()
    }

    pub fn removed_edges_count(&self) -> usize {
        self.removed_edges.len()
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

}

