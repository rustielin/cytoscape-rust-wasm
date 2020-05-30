# Cytoscape rust wasm

[![Build Status](https://travis-ci.com/rustielin/cytoscape-rust-wasm.svg?branch=master)](https://travis-ci.com/rustielin/cytoscape-rust-wasm)

Utility for writing network simulations in Rust, compiling to wasm, and displaying the graphics with [CytoscapeJS](https://js.cytoscape.org/)

Building with the awesome [wasm-pack](https://github.com/rustwasm/wasm-pack) and [petgraph](https://github.com/petgraph/petgraph) libraries

## Dev

Build with wasm-pack, then serve locally

    $ wasm-pack build
    $ cd www && npm start

## Notes

To minimize traffic between javascript and wasm linear memory, maintain multiple separate data structures representing different aspects of the graph. For now, these are the underlying petgraph `Graph`, cytoscapeJS instance, and the deltas between the two.

We also use petgraph's notion of weights to store metadata about each node and edge, which is used by the network simulation. This can be whether a node/edge is up/down, "transient", etc.
