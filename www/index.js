import { memory } from "wasm-cytoscape/wasm_cytoscape_bg";
import { CytoGraph } from "wasm-cytoscape";
import { META_FLAG_IDX, NODE_ID_PREFIX, EDGE_ID_PREFIX } from "./constants";

import cytoscape from "cytoscape";
import dagre from "cytoscape-dagre";

cytoscape.use(dagre);

/**
 * For now, creates if not found
 * @param {*} cy 
 * @param {*} id 
 * @param {*} meta 
 */
function processEleMeta(cy, id, meta, isNode = true, options = {}) {
  console.log(`Processing ${id} with ${JSON.stringify(options)} for ${meta}`);
  var ele = cy.elements().getElementById(id)[0];
  if (!ele) {
    console.log("ADDING")
    ele = cy.add({
      group: isNode ? "nodes" : "edges",
      data: { id, ...options }
    })
  }
  // check if addClass is idempotent
  if (meta & 1 << META_FLAG_IDX.PRESENT) {
    ele.removeClass('hidden')
  } else {
    ele.addClass('hidden')
  }

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

function operateOnNodeMeta(cy, cytograph) {
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
    processEleMeta(cy, id, meta, true);
  }
}

function operateOnEdgeMeta(cy, cytograph) {
  const edgeIdsPtr = cytograph.get_edge_ids();
  const edgeIdsCount = cytograph.edge_ids_count();
  const edgeIds = new Uint32Array(
    memory.buffer,
    edgeIdsPtr,
    edgeIdsCount
  );
  for (var i = 0; i < edgeIdsCount; i++) {
    let wasmEdgeID = edgeIds[i];
    let ends = new Uint32Array(memory.buffer, cytograph.get_edge_ends(wasmEdgeID), 2);
    let source = wasmNodeIDtoCyto(ends[0]);
    let target = wasmNodeIDtoCyto(ends[1]);
    let id = wasmEdgeIDtoCyto(wasmEdgeID);
    let meta = cytograph.get_edge_meta(wasmEdgeID);
    processEleMeta(cy, id, meta, false, {source, target});
  }
}

function operateOnMeta(cy, cytograph) {
  operateOnNodeMeta(cy, cytograph);
  operateOnEdgeMeta(cy, cytograph);
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
    name: "circle",
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
  console.log(`--------------- TICK ---------------`);
  // populateAdditions(cy, cytograph);
  cytograph.tick(); // do it
  console.log(`--------------- END SIMULATION ---------------`);
  operateOnMeta(cy, cytograph); // display
  console.log(`--------------- END VISUALIZATION ---------------`);
  // removeElements(cy, cytograph);
  regroupCy(cy);
}

/**
 * Initialize the backing wasm graph
 * @param {*} cy
 */
function initGraph(cy) {
  const cytograph = CytoGraph.new_full(5);
  operateOnMeta(cy, cytograph);
  regroupCy(cy);

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
      name: "circle",
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
      {
        selector: '.hidden',
        style: {
          'visibility': 'hidden'
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
