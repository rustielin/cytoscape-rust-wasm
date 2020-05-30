mod utils;

extern crate web_sys;

use petgraph::graph::{Graph, NodeIndex, EdgeIndex};
use wasm_bindgen::prelude::*;

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

/// Represents a CytoGraph node in wasm. Each of the members are fetched as bytes
#[derive(Copy, Clone)]
pub struct WasmNode {
    id: u32,
}

/// Represents a CytoGraph edge in wasm. Each of the members are fetched as bytes
pub struct WasmEdge {
    id: u32,
    src: u32,
    dst: u32,
}

/// The main data structure for representing a cytoscape graph.
/// CytoGraph uses an internal graph structure `graph` as well as graph element
/// deltas such as `added_nodes` and `removed_nodes` to buffer changes to UI.
///
/// Graph metadata is stored in the internal graph itself. Under the hood, elements should
/// not be removed directly from the graph, but rather flagged as removed with metadata.
#[wasm_bindgen]
pub struct CytoGraph {
    graph: Graph<u8, u8>, // use u8 to store metadata as weight for now
    added_nodes: Vec<WasmNode>,
    removed_nodes: Vec<WasmNode>,
    added_edges: Vec<WasmEdge>,
    removed_edges: Vec<WasmEdge>,
    time: u32,
}

#[wasm_bindgen]
impl CytoGraph {
    pub fn new() -> CytoGraph {
        let graph = Graph::<u8, u8>::new();
        let added_nodes = Vec::new();
        let removed_nodes = Vec::new();
        let added_edges = Vec::new();
        let removed_edges = Vec::new();
        let time = 0;
        CytoGraph {
            graph,
            added_nodes,
            removed_nodes,
            added_edges,
            removed_edges,
            time,
        }
    }

    /// Creates a fully connected CytoGraph of a certain size
    pub fn new_full(size: usize) -> CytoGraph {
        let mut g = CytoGraph::new();
        for _ in 0..size {
            g.add_node();
        }
        for i in 0..size {
            for j in 0..size {
                if i == j {
                    continue;
                }
                g.add_edge(g.added_nodes[i].id, g.added_nodes[j].id);
            }
        }
        g
    }

    pub fn add_node(&mut self) -> u32 {
        let idx = self.graph.add_node(0);
        let id = idx.index() as u32;
        self.added_nodes.push(WasmNode { id });
        id
    }

    pub fn get_node_meta(&self, idx: u32) -> u8 {
        let w = self.graph.node_weight(NodeIndex::new(idx as usize));
        match w {
            Some(x) => return *x,
            None => return 0,
        }
    }

    pub fn set_node_meta(&mut self, idx: u32, meta: u8) {
        let w = self.graph.node_weight_mut(NodeIndex::new(idx as usize));
        match w {
            Some(x) => *x = *x & 0 | meta,
            None => (),
        }
    }

    pub fn get_added_nodes(&self) -> *const WasmNode {
        self.added_nodes.as_ptr()
    }

    pub fn added_nodes_count(&self) -> u32 {
        self.added_nodes.len() as u32
    }

    /// Number of members in each added node. This is used by Javascript to index
    /// directly into wasm linear memory
    pub fn added_nodes_size(&self) -> u32 {
        1
    }

    pub fn add_edge(&mut self, src: u32, dst: u32) -> u32 {
        let idx = self.graph.add_edge(
            NodeIndex::new(src as usize),
            NodeIndex::new(dst as usize),
            0,
        );
        let id = idx.index() as u32;
        self.added_edges.push(WasmEdge { id, src, dst });
        id
    }

    pub fn get_edge_meta(&self, idx: u32) -> u8 {
        let w = self.graph.edge_weight(EdgeIndex::new(idx as usize));
        match w {
            Some(x) => return *x,
            None => return 0,
        }
    }

    pub fn set_edge_meta(&mut self, idx: u32, meta: u8) {
        let w = self.graph.edge_weight_mut(EdgeIndex::new(idx as usize));
        match w {
            Some(x) => *x = *x & 0 | meta,
            None => (),
        }
    }

    pub fn get_added_edges(&self) -> *const WasmEdge {
        self.added_edges.as_ptr()
    }

    pub fn added_edges_count(&self) -> u32 {
        self.added_edges.len() as u32
    }

    /// Number of members in each added edge. This is used by Javascript to index
    /// directly into wasm linear memory
    pub fn added_edges_size(&self) -> u32 {
        3
    }

    //     pub fn new() -> CytoGraph {
    //         utils::set_panic_hook();
    //         let graph = Graph::<(), ()>::new();
    //         let added_nodes = Vec::new();
    //         let removed_nodes = Vec::new();
    //         let added_edges = Vec::new();
    //         let removed_edges = Vec::new();

    //         CytoGraph {
    //             graph,
    //             added_nodes,
    //             removed_nodes,
    //             added_edges,
    //             removed_edges,
    //             time: 0,
    //         }
    //     }

    //     pub fn tick(&mut self) {
    //         utils::set_panic_hook();
    //         self.added_nodes.clear();
    //         self.removed_nodes.clear();
    //         self.added_edges.clear();
    //         self.removed_edges.clear();
    //         log!("Tick {}", self.time);
    //         self.time += 1;
    //     }

    //     pub fn add_node(&mut self) -> usize {
    //         let n_idx = self.graph.add_node(());
    //         let n = n_idx.index();
    //         self.added_nodes.push(n);

    //         // make it fully connected for now
    //         for idx in self.graph.node_indices() {
    //             let i = idx.index();
    //             if n == i {
    //                 continue;
    //             }
    //             self.add_edge(n, i);
    //             self.add_edge(i, n);
    //         }

    //         n
    //     }

    //     pub fn add_edge(&mut self, src: usize, dst: usize) {
    //         self.graph
    //             .add_edge(NodeIndex::new(src), NodeIndex::new(dst), ());
    //         self.added_edges.push(src);
    //         self.added_edges.push(dst);
    //     }

    //     pub fn get_added_nodes(&self) -> *const usize {
    //         self.added_nodes.as_ptr()
    //     }

    //     pub fn get_removed_nodes(&self) -> *const usize {
    //         self.removed_nodes.as_ptr()
    //     }

    //     pub fn get_added_edges(&self) -> *const usize {
    //         self.added_edges.as_ptr()
    //     }

    //     pub fn get_removed_edges(&self) -> *const usize {
    //         self.removed_edges.as_ptr()
    //     }

    //     pub fn added_nodes_count(&self) -> usize {
    //         self.added_nodes.len()
    //     }

    //     pub fn removed_nodes_count(&self) -> usize {
    //         self.removed_nodes.len()
    //     }

    //     pub fn added_edges_count(&self) -> usize {
    //         self.added_edges.len()
    //     }

    //     pub fn removed_edges_count(&self) -> usize {
    //         self.removed_edges.len()
    //     }

    //     pub fn node_count(&self) -> usize {
    //         self.graph.node_count()
    //     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_inst() {
        let mut cyto = CytoGraph::new();
        let idx = cyto.add_node();
        let added = cyto.get_added_nodes();
        let ptr = unsafe { *added } as WasmNode;
        assert_eq!(ptr.id, idx);
    }
    #[test]
    fn test_get_set_meta() {
        let mut cyto = CytoGraph::new();
        let idx = cyto.add_node();
        let meta = cyto.get_node_meta(idx);
        assert_eq!(meta, 0);

        cyto.set_node_meta(idx, 32);
        let meta = cyto.get_node_meta(idx);
        assert_eq!(meta, 32);
    }
}
