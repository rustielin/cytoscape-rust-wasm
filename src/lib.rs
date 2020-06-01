mod utils;
mod fsms;

extern crate web_sys;
extern crate js_sys;

use petgraph::graph::{Graph, NodeIndex, EdgeIndex};
use petgraph::Direction;
use wasm_bindgen::prelude::*;
use rust_fsm::*;

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

pub struct EndpointBuf(u32, u32);

/// The main data structure for representing a cytoscape graph.
/// CytoGraph uses an internal graph structure `graph` as well as graph element
/// deltas such as `added_nodes` and `removed_nodes` to buffer changes to UI.
///
/// Graph metadata is stored in the internal graph itself. Under the hood, elements should
/// not be removed directly from the graph, but rather flagged as removed with metadata.
#[wasm_bindgen]
pub struct CytoGraph {
    graph: Graph<u8, u8>, // use u8 to store metadata as weight for now
    node_ids: Vec<u32>,
    edge_ids: Vec<u32>,
    endpoint_buf: EndpointBuf,
    // added_nodes: Vec<WasmNode>,
    // removed_nodes: Vec<WasmNode>,
    // added_edges: Vec<WasmEdge>,
    // removed_edges: Vec<WasmEdge>,
    // node_machines: Vec<StateMachine>,
    time: u32,
}

#[wasm_bindgen]
impl CytoGraph {
    pub fn new() -> CytoGraph {
        utils::set_panic_hook();
        let graph = Graph::<u8, u8>::new();
        let node_ids = Vec::new();
        let edge_ids = Vec::new();
        let endpoint_buf = EndpointBuf(0, 0);
        // let added_nodes = Vec::new();
        // let removed_nodes = Vec::new();
        // let added_edges = Vec::new();
        // let removed_edges = Vec::new();
        let time = 0;
        CytoGraph {
            graph,
            node_ids,
            edge_ids,
            endpoint_buf,
            // added_nodes,
            // removed_nodes,
            // added_edges,
            // removed_edges,
            time,
        }
    }

    /// Creates a fully connected CytoGraph of a certain size
    pub fn new_full(size: usize) -> CytoGraph {
        let mut g = CytoGraph::new();
        for _ in 0..size {
            g.add_node();
        }
        let indices: Vec<u32> = g.graph.node_indices().map(|idx| idx.index() as u32).collect();
        for i in indices.clone() {
            for j in indices.clone() {
                if i == j {
                    continue;
                }
                g.add_edge(i, j);
            }
        }
        g
    }

    /// algo exposes cytograph members too
    /// algo(cytograph)
    ///     for each node as state machine
    ///         do thing
    ///             hold node state too, but not pub struct with wasm-bindgen
    /// 
    ///             assign meta
    ///             do deltas
    ///             !!!!!!!!! dont need deltas now that we're doing all meta!!!!!!!!!
    ///                      check the meta deltas, first bit meaning add/del
    ///                         - if included in meta, that's everything
    ///                         - aka no partial meta updates!!!!!!!
    ///                         

    // XXX: want to get all the EdgeIndices adj to a NodeIndex!!!
    // this will do for now
    // fn get_edge_idx_from_node_idx(&self, src: NodeIndex, dst: NodeIndex) {
    //     let meta = self.graph.
    //     self.graph.update_edge(src, dst, weight: E)
    // }

    /// Do something given the current network state for all elements
    /// Returns a u8 depending on its execution state
    /// (0) return, (1) yield, (2) error
    pub fn tick(&mut self) -> u8 {
        // everything starts of with a default (steady) state metadata
        let mut buf_node_meta: Vec<u8> = vec![1; self.graph.node_indices().len()];
        let mut buf_edge_meta: Vec<u8> = vec![1; self.graph.edge_indices().len()];
        for idx in self.graph.node_indices() {
            let node_meta = self.get_node_meta(idx.index() as u32);
            let new_meta = node_meta ^ 1;
            if js_sys::Math::random() < 0.5 {
                // each node does something given state and state machine
                buf_node_meta[idx.index()] = buf_node_meta[idx.index()] & new_meta;
                log!("New meta for node_{} set to {}", idx.index(), new_meta);
                // update its edges too
                for idx2 in self.graph.node_indices() {
                    match self.graph.find_edge(idx, idx2) {
                        Some(edge_idx) => {
                            buf_edge_meta[edge_idx.index()] = buf_edge_meta[edge_idx.index()] & new_meta;
                            log!("New meta for edge {} => {} set to {}", idx.index(), idx2.index(), node_meta & new_meta);
                        },
                        None => ()
                    }
                    match self.graph.find_edge(idx2, idx) {
                        Some(edge_idx2) => {
                            buf_edge_meta[edge_idx2.index()] = buf_edge_meta[edge_idx2.index()] & new_meta;
                            log!("New meta for edge {} => {} set to {}", idx2.index(), idx.index(), node_meta & new_meta);
                        },
                        None => ()
                    }
                }
            } else {
                log!("Meta for node_{} stays at {}", idx.index(), node_meta);
            }
        }

        let mut alive_node_indices: Vec<u32> = Vec::new();
        // apply all the bufs
        for idx in self.graph.node_indices() {
            self.set_node_meta(idx.index() as u32, buf_node_meta[idx.index()]);
            if buf_node_meta[idx.index()] & 1 == 1 { // we're using the first bit as alive or not
                alive_node_indices.push(idx.index() as u32);
            }
        }
        for idx in self.graph.edge_indices() {
            self.set_edge_meta(idx.index() as u32, buf_edge_meta[idx.index()]);
        }

        // select one live node to be the leader
        let leader = alive_node_indices[(js_sys::Math::random() * alive_node_indices.len() as f64) as usize % alive_node_indices.len()];
        self.set_node_meta(leader, buf_node_meta[leader as usize] | 1 << 1);

        self.time += 1;
        1
    }

    pub fn add_node(&mut self) -> u32 {
        let idx = self.graph.add_node(1);
        let id = idx.index() as u32;
        self.node_ids.push(id);
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

    // pub fn get_added_nodes(&self) -> *const WasmNode {
    //     self.added_nodes.as_ptr()
    // }

    // pub fn added_nodes_count(&self) -> u32 {
    //     self.added_nodes.len() as u32
    // }

    /// Number of members in each added node. This is used by Javascript to index
    /// directly into wasm linear memory
    // pub fn added_nodes_size(&self) -> u32 {
    //     1
    // }

    pub fn add_edge(&mut self, src: u32, dst: u32) -> u32 {
        let idx = self.graph.update_edge(
            NodeIndex::new(src as usize),
            NodeIndex::new(dst as usize),
            1,
        );
        let id = idx.index() as u32;
        self.edge_ids.push(id);
        id
    }

    /// Hack to return a buf of edge endpoints by index
    pub fn get_edge_ends(&mut self, idx: u32) -> *const EndpointBuf {
        match self.graph.edge_endpoints(EdgeIndex::new(idx as usize)) {
            Some((x, y)) => {
                self.endpoint_buf = EndpointBuf(x.index() as u32, y.index() as u32);
                &self.endpoint_buf
            },
            None => {
                self.endpoint_buf = EndpointBuf(0,0);
                &self.endpoint_buf 
            },
        }
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

    // pub fn get_added_edges(&self) -> *const WasmEdge {
    //     self.added_edges.as_ptr()
    // }

    // pub fn added_edges_count(&self) -> u32 {
    //     self.added_edges.len() as u32
    // }

    /// Number of members in each added edge. This is used by Javascript to index
    /// directly into wasm linear memory
    // pub fn added_edges_size(&self) -> u32 {
    //     3
    // }

    pub fn get_node_ids(&self) -> *const u32 {
        self.node_ids.as_ptr()
    }

    pub fn node_ids_count(&self) -> u32 {
        self.node_ids.len() as u32
    }

    pub fn get_edge_ids(&self) -> *const u32 {
        self.edge_ids.as_ptr()
    }

    pub fn edge_ids_count(&self) -> u32 {
        self.edge_ids.len() as u32
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

    // #[test]
    // fn test_inst() {
    //     let mut cyto = CytoGraph::new();
    //     let idx = cyto.add_node();
    //     let added = cyto.get_added_nodes();
    //     let ptr = unsafe { *added } as WasmNode;
    //     assert_eq!(ptr.id, idx);
    // }
    #[test]
    fn test_get_set_meta() {
        let mut cyto = CytoGraph::new();
        let idx = cyto.add_node();
        let meta = cyto.get_node_meta(idx);
        assert_eq!(meta, 1);

        cyto.set_node_meta(idx, 32);
        let meta = cyto.get_node_meta(idx);
        assert_eq!(meta, 32);
    }

    #[test]
    fn test_imported_fsms() {
        let mut machine: StateMachine<fsms::random_on_off_fsm::RandomOnOff> = StateMachine::new();
        let _ = machine.consume(&fsms::random_on_off_fsm::RandomOnOffInput::Successful);
        assert_eq!(machine.state(), &fsms::random_on_off_fsm::RandomOnOffState::On);

        let _ = machine.consume(&fsms::random_on_off_fsm::RandomOnOffInput::Successful);
        assert_eq!(machine.state(), &fsms::random_on_off_fsm::RandomOnOffState::Off);
    }
}
