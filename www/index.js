import { memory } from "wasm-cytoscape/wasm_cytoscape_bg";
import { CytoGraph } from "wasm-cytoscape";
import { META_FLAG_IDX, NODE_ID_PREFIX, EDGE_ID_PREFIX } from "./constants";

import cytoscape from "cytoscape";
import dagre from "cytoscape-dagre";

cytoscape.use(dagre);

/**
 * 
 * @param {*} cy 
 * @param {*} id 
 * @param {*} meta 
 */
function processEleMeta(cy, id, meta) {
  const ele = cy.elements().getElementById(id)[0];
  if (!ele) {
    return;
  }
  var clear
  if (meta & 1 << META_FLAG_IDX.HIGHLIGHTED) {
    ele.addClass('highlighted');
  } else {
    ele.removeClass('highlighted');
  }
}

function wasmNodeIDtoCyto(id) {
  return NODE_ID_PREFIX + id;
}

function wasmEdgeIDtoCyto(id) {
  return EDGE_ID_PREFIX + id;
}

function operateOnMeta(cy, cytograph) {
  const nodeIdsPtr = cytograph.get_node_ids();
  const nodeIdsCount = cytograph.node_ids_count();
  const nodeIds = new Uint32Array(
    memory.buffer,
    nodeIdsPtr,
    nodeIdsCount
  );
  for (var i = 0; i < nodeIdsCount; i++) {
    let id = wasmNodeIDtoCyto(nodeIds[i]);
    let meta = cytograph.get_node_meta(nodeIds[i]);
    console.log(`Processing ${id} for ${meta}`);
    processEleMeta(cy, id, meta);
  }

  const edgeIdsPtr = cytograph.get_edge_ids();
  const edgeIdsCount = cytograph.edge_ids_count();
  const edgeIds = new Uint32Array(
    memory.buffer,
    edgeIdsPtr,
    edgeIdsCount
  );
  for (var i = 0; i < edgeIdsCount; i++) {
    let id = wasmEdgeIDtoCyto(edgeIds[i]);
    let meta = cytograph.get_edge_meta(edgeIds[i]);
    processEleMeta(cy, id, meta);
  }
}

/**
 *  Get the newly added elements in the backing wasm graph and populate them into cytoscape
 */
function populateAdditions(cy, cytograph) {
  const addedNodesPtr = cytograph.get_added_nodes();
  const addedNodesCount = cytograph.added_nodes_count();
  const addedNodesSize = cytograph.added_nodes_size();
  const addedNodes = new Uint32Array(
    memory.buffer,
    addedNodesPtr,
    addedNodesCount * addedNodesSize
  );
  for (var i = 0; i < addedNodes.length; i += addedNodesSize) {
    let meta = cytograph.get_node_meta(addedNodes[i]);
    let id = wasmNodeIDtoCyto(addedNodes[i]);
    console.log("META", meta)
    cy.add({
      group: "nodes",
      data: { id },
    });
    console.log("Added node", id)
    processEleMeta(cy, id, meta);
  }

  const addedEdgesPtr = cytograph.get_added_edges();
  const addedEdgesCount = cytograph.added_edges_count();
  const addedEdgesSize = cytograph.added_edges_size();
  const addedEdges = new Uint32Array(
    memory.buffer,
    addedEdgesPtr,
    addedEdgesCount * addedEdgesSize
  );
  for (var i = 0; i < addedEdges.length; i += addedEdgesSize) {
    let meta = cytograph.get_edge_meta(addedEdges[i]);
    let id = wasmEdgeIDtoCyto(addedEdges[i]);
    let source = wasmNodeIDtoCyto(addedEdges[i + 1]);
    let target = wasmNodeIDtoCyto(addedEdges[i + 2]);
    cy.add({
      group: "edges",
      data: { id, source, target },
    });
    console.log(`Added edge ${source} => ${target}`)
    processEleMeta(cy, id, meta);
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
  operateOnMeta(cy, cytograph);
  // removeElements(cy, cytograph);
  cytograph.tick();
  regroupCy(cy);
}

/**
 * Initialize the backing wasm graph
 * @param {*} cy
 */
function initGraph(cy) {
  const cytograph = CytoGraph.new_full(5);
  onTick(cy, cytograph);

  document.getElementById("tickTimeButton").onclick = () => onTick(cy, cytograph);
  document.getElementById("regroupButton").onclick = () => regroupCy(cy);
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
      {
        selector: '.highlighted',
        style: {
          'background-color': '#75b5aa',
          'line-color': '#75b5aa',
          'target-arrow-color': '#75b5aa',
          'transition-property': 'background-color, line-color, target-arrow-color',
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
