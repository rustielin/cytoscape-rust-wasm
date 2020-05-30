/**
 * Metadata is encoded as a uint from wasm linear memory. We can read it per bit
 * as a flag for configurable front-end styles.
 * 
 * TODO: probably give it a callback also to apply the style
 */
export const META_FLAG_IDX = {
    HIGHLIGHTED: 0
}

/**
 * Because petgraph NodeIndex and EdgeIndex can overlap, and because we use those
 * as indices for Cytoscape, we need to assign each a unique id using prefix
 */
export const NODE_ID_PREFIX = "node_";
export const EDGE_ID_PREFIX = "edge_";
