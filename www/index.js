import { memory } from 'wasm-cytoscape/wasm_cytoscape_bg';
import { CytoGraph } from "wasm-cytoscape";


import cytoscape from 'cytoscape';
import dagre from 'cytoscape-dagre';

cytoscape.use( dagre );


/**
 *  Get the newly added elements in the backing wasm graph and populate them into cytoscape
 */
function populateAdditions(cy, cytograph) {
  const addedNodesPtr = cytograph.get_added_nodes();
  const addedNodesCount = cytograph.added_nodes_count();
  const addedNodes = new Uint32Array(memory.buffer, addedNodesPtr, addedNodesCount);
  for (var i = 0; i < addedNodes.length; i++) {
    cy.add({
      group: 'nodes',
      data: { id: addedNodes[i] }
    })
  }

  const addedEdgesPtr = cytograph.get_added_edges();
  const addedEdgesCount = cytograph.added_edges_count();
  const addedEdgesRaw = new Uint32Array(memory.buffer, addedEdgesPtr, addedEdgesCount);
  for (var i = 0; i < addedEdgesRaw.length; i+=2) {
    cy.add({
      group: 'edges',
      data: { source: addedEdgesRaw[i], target: addedEdgesRaw[i+1] }
    })
  }
}


function regroupCy(cy) {
  var layout = cy.layout({
    name: 'dagre',
    animationDuration: 300
  });
  layout.run();
}


/**
 *  Initialize the backing wasm graph
 */
function initGraph(cy) {
  const cytograph = CytoGraph.new();
  document.getElementById('addNodeButton').onclick = () => {cytograph.add_node(); populateAdditions(cy, cytograph)}
  document.getElementById('tickTimeButton').onclick = () => cytograph.tick();
  document.getElementById('regroupButton').onclick = () => regroupCy(cy);
}

/**
 *  Initialize Cytoscape graphics
 */
function initCy() {

  var cy = window.cy = cytoscape({
    container: document.getElementById('cy'),

    boxSelectionEnabled: false,
    autounselectify: true,

    layout: {
      name: 'dagre'
    },

    style: [
      {
        selector: 'node',
        style: {
          'background-color': '#11479e'
        }
      },
      {
        selector: 'edge',
        style: {
          'width': 4,
          'target-arrow-shape': 'triangle',
          'line-color': '#9dbaea',
          'target-arrow-color': '#9dbaea',
          'curve-style': 'bezier'
        }
      }, 
      {
        selector: 'node[id]',
        style: {
          label: 'data(id)'
        }
      }
    ],
/*
    elements: {
      nodes: [
        { data: { id: 'n0' } },
        { data: { id: 'n1' } },
        { data: { id: 'n2' } },
        { data: { id: 'n3' } },
        { data: { id: 'n4' } },
       { data: { id: 'n5' } },
        { data: { id: 'n6' } },
        { data: { id: 'n7' } },
        { data: { id: 'n8' } },
        { data: { id: 'n9' } },
        { data: { id: 'n10' } },
        { data: { id: 'n11' } },
        { data: { id: 'n12' } },
        { data: { id: 'n13' } },
        { data: { id: 'n14' } },
        { data: { id: 'n15' } },
        { data: { id: 'n16' } }
      ],
      edges: [
        { data: { source: 'n0', target: 'n1' } },
        { data: { source: 'n1', target: 'n2' } },
        { data: { source: 'n1', target: 'n3' } },
        { data: { source: 'n4', target: 'n5' } },
        { data: { source: 'n4', target: 'n6' } },
        { data: { source: 'n6', target: 'n7' } },
        { data: { source: 'n6', target: 'n8' } },
        { data: { source: 'n8', target: 'n9' } },
        { data: { source: 'n8', target: 'n10' } },
        { data: { source: 'n11', target: 'n12' } },
        { data: { source: 'n12', target: 'n13' } },
        { data: { source: 'n13', target: 'n14' } },
        { data: { source: 'n13', target: 'n15' } },
      ]
    }
*/
  });

  // init everything else
  initGraph(cy);
}

if (document.readyState !== 'loading') {
  initCy();
} else {
  window.addEventListener('DOMContentLoaded', initCy);
}



