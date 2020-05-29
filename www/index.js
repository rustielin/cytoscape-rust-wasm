import { memory } from "wasm-cytoscape/wasm_cytoscape_bg";
import { CytoGraph } from "wasm-cytoscape";

import cytoscape from "cytoscape";
import dagre from "cytoscape-dagre";

cytoscape.use(dagre);

/**
 *  Get the newly added elements in the backing wasm graph and populate them into cytoscape
 */
function populateAdditions(cy, cytograph) {
  const addedNodesPtr = cytograph.get_added_nodes();
  const addedNodesCount = cytograph.added_nodes_count();
  const addedNodes = new Uint32Array(
    memory.buffer,
    addedNodesPtr,
    addedNodesCount
  );
  for (var i = 0; i < addedNodes.length; i++) {
    cy.add({
      group: "nodes",
      data: { id: addedNodes[i] },
    });
  }

  const addedEdgesPtr = cytograph.get_added_edges();
  const addedEdgesCount = cytograph.added_edges_count();
  const addedEdgesRaw = new Uint32Array(
    memory.buffer,
    addedEdgesPtr,
    addedEdgesCount
  );
  for (var i = 0; i < addedEdgesRaw.length; i += 2) {
    cy.add({
      group: "edges",
      data: { source: addedEdgesRaw[i], target: addedEdgesRaw[i + 1] },
    });
  }
}

function removeElements(cy, cytograph) {
  const removedNodesPtr = cytograph.get_removed_nodes();
  const removedNodesCount = cytograph.removed_nodes_count();
  const removedNodes = new Uint32Array(
    memory.buffer,
    removedNodesPtr,
    removedNodesCount
  );
  for (var i = 0; i < removedNodes.length; i++) {
    var el = cy.nodes(`node[id = "${removedNodes[i]}"]`);
    cy.remove(el);
  }

  const removedEdgesPtr = cytograph.get_removed_edges();
  const removedEdgesCount = cytograph.removed_edges_count();
  const removedEdgesRaw = new Uint32Array(
    memory.buffer,
    removedEdgesPtr,
    removedEdgesCount
  );
  for (var i = 0; i < removedEdgesRaw.length; i += 2) {
    var el = cy.edges(
      `edge[source = "${removedEdgesRaw[i]}][target = "${
        removedEdgesRaw[i + 1]
      }"]`
    );
    cy.remove(el);
  }
}

function regroupCy(cy) {
  var layout = cy.layout({
    name: "dagre",
    animationDuration: 300,
  });
  layout.run();
}

/**
 * When the client ticks, do all the buffered changes (e.g. additions and removals)
 * and then finally tick the underlying cytograph in wasm
 * @param {*} cy
 * @param {*} cytograph
 */
function onTick(cy, cytograph) {
  populateAdditions(cy, cytograph);
  removeElements(cy, cytograph);
  cytograph.tick();
}

/**
 * Initialize the backing wasm graph
 * @param {*} cy
 */
function initGraph(cy) {
  const cytograph = CytoGraph.new();
  var src = cytograph.add_node();
  var dst = cytograph.add_node();

  var ptr = cytograph.get_added_nodes();
  const nodes = new Uint32Array(
    memory.buffer,
    ptr,
    cytograph.added_nodes_count() * cytograph.added_nodes_size()
  );
  for (var i = 0; i < nodes.length; i += cytograph.added_nodes_size()) {
    console.log(`Node ID ${nodes[i]}`);
  }

  console.log(`Meta ${cytograph.get_node_meta(src)}`);
  cytograph.set_node_meta(src, 69);
  console.log(`Meta ${cytograph.get_node_meta(src)}`);

  var edge = cytograph.add_edge(src, dst);
  var edge = cytograph.add_edge(dst, src);
  var edge = cytograph.add_edge(src, dst);
  var edge = cytograph.add_edge(src, dst);
  var edge = cytograph.add_edge(src, dst);

  const edges = new Uint32Array(
    memory.buffer,
    cytograph.get_added_edges(),
    cytograph.added_edges_count() * cytograph.added_edges_size()
  );
  for (var i = 0; i < edges.length; i += cytograph.added_edges_size()) {
    console.log(`Edge ID ${edges[i]} from ${edges[i + 1]} => ${edges[i + 2]}`);
  }
  // document.getElementById("addNodeButton").onclick = () => cytograph.add_node();
  // document.getElementById("tickTimeButton").onclick = () =>
  //   onTick(cy, cytograph);
  // document.getElementById("regroupButton").onclick = () => regroupCy(cy);
}

/**
 *  Initialize Cytoscape graphics
 */
function initCy() {
  var cy = (window.cy = cytoscape({
    container: document.getElementById("cy"),

    boxSelectionEnabled: false,
    autounselectify: true,

    layout: {
      name: "dagre",
    },

    style: [
      {
        selector: "node",
        style: {
          "background-color": "#11479e",
        },
      },
      {
        selector: "edge",
        style: {
          width: 4,
          "target-arrow-shape": "triangle",
          "line-color": "#9dbaea",
          "target-arrow-color": "#9dbaea",
          "curve-style": "bezier",
        },
      },
      {
        selector: "node[id]",
        style: {
          label: "data(id)",
        },
      },
    ],
  }));

  // init everything else
  initGraph(cy);
}

if (document.readyState !== "loading") {
  initCy();
} else {
  window.addEventListener("DOMContentLoaded", initCy);
}
