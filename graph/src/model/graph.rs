use std::collections::{ HashMap, HashSet };
use uuid::Uuid;
use super::node::Node;
use super::edge::{ Edge, EdgeType };
use crate::graph_utils::graph_utils::{ cons_node, cons_edge };

static NODE_MISSING_ERR: &'static str = "::node_missing";

#[derive(Debug)]
pub struct Graph 
{ 
  pub id: String,
  pub inner_node_order: u64,
  pub inner_edge_order: u64,
  pub nodes: HashMap<String, Node>, 
  pub edges: HashMap<String, Edge> 
}

impl Graph 
{
  pub fn edge_count ( &self ) -> usize { self.edges.len() }
  pub fn node_count ( &self ) -> usize { self.nodes.len() }
  
  // Graph Order is number of Nodes
  pub fn graph_order ( &self ) -> usize { self.node_count() }

  // Graph Size is number of Edges
  pub fn graph_size ( &self ) -> usize { self.edge_count() }

  pub fn cons_node ( &mut self, name: String, order: u64, weight: i64 ) -> Node 
  {
    cons_node(
      Uuid::new_v4().to_string(),
      name,
      order,
      self.inner_node_order(),
      0,
      weight,
      HashSet::new(),
      HashMap::new())
  }

  pub fn cons_empty_node ( &mut self ) -> Node 
  {
    cons_node(
      Uuid::new_v4().to_string(), 
      String::from( "" ), 
      0, 
      self.inner_node_order(), 
      0, 
      0,
      HashSet::new(), 
      HashMap::new())
  }

  pub fn cons_edge ( &mut self, name: String, order: u64, weight: i64, left: String, right: String, etype: EdgeType ) 
    -> Edge
  {
    cons_edge(
      Uuid::new_v4().to_string(),
      name,
      order,
      self.inner_edge_order(),
      0,
      weight,
      left,
      right,
      etype,
      HashSet::new(),
      HashMap::new())
  }

  pub fn cons_empty_edge ( &mut self ) -> Edge
  {
    cons_edge(
      Uuid::new_v4().to_string(),
      String::from( "" ),
      0,
      self.inner_edge_order(),
      0,
      0,
      String::from( "" ),
      String::from( "" ),
      EdgeType::Undirected,
      HashSet::new(),
      HashMap::new())
  }

  fn inner_node_order ( &mut self ) -> u64 
  {
    let ret = self.inner_node_order.clone();
    self.inner_node_order += 1;
    ret
  }

  fn inner_edge_order ( &mut self ) -> u64 
  {
    let ret = self.inner_edge_order.clone();
    self.inner_edge_order += 1;
    ret
  }
}

pub fn cons_graph () -> Graph
{
  Graph {
    id: Uuid::new_v4().to_string(),
    inner_node_order: 0,
    inner_edge_order: 0,
    nodes: HashMap::new(),
    edges: HashMap::new()
  }
}

#[cfg(test)]
mod tests 
{
  use super::*;
  use crate::model::node::{ Node };
  use crate::model::edge::{ Edge, EdgeType };

  #[test]
  fn test_cons_graph () 
  {
    let gph: Graph = cons_graph();
    assert_eq!( gph.id.len() > 0, true );
    assert_eq!( gph.inner_node_order, 0 );
    assert_eq!( gph.inner_edge_order, 0 );
    assert_eq!( gph.nodes.len(), 0 );
    assert_eq!( gph.edges.len(), 0 );
  }

  #[test]
  fn test_cons_node () 
  {
    let mut gph: Graph = cons_graph();
    let node: Node = gph.cons_node( String::from( "node1" ), 5, 6 );
    assert_eq!( node.id.len() > 0, true );
    assert_eq!( node.name, "node1" );
    assert_eq!( node.order, 5 );
    assert_eq!( node.inner_order, 0 );
    assert_eq!( node.delta, 0 );
    assert_eq!( node.weight, 6 );
    assert_eq!( node.labels.len(), 0 );
    assert_eq!( node.metadata.len(), 0 );
  }

  #[test]
  fn test_cons_empty_node () 
  {
    let mut gph: Graph = cons_graph();
    let node: Node = gph.cons_empty_node();
    assert_eq!( node.id.len() > 0, true );
    assert_eq!( node.name, "" );
    assert_eq!( node.order, 0 );
    assert_eq!( node.inner_order, 0 );
    assert_eq!( node.delta, 0 );
    assert_eq!( node.weight, 0 );
    assert_eq!( node.labels.len(), 0 );
    assert_eq!( node.metadata.len(), 0 );

    let node1: Node = gph.cons_empty_node();
    assert_eq!( node1.inner_order, 1 );
  }

  #[test]
  fn test_cons_edge () 
  {
    let mut gph: Graph = cons_graph();
    let edge: Edge = gph.cons_edge( String::from( "edge" ), 5, 6, String::from( "left" ), String::from( "right" ),
      EdgeType::Undirected );
    assert_eq!( edge.id.len() > 0, true );
    assert_eq!( edge.name, "edge" );
    assert_eq!( edge.order, 5 );
    assert_eq!( edge.inner_order, 0 );
    assert_eq!( edge.delta, 0 );
    assert_eq!( edge.weight, 6 );
    assert_eq!( edge.left, "left" );
    assert_eq!( edge.right, "right" );
    assert_eq!( edge.labels.len(), 0 );
    assert_eq!( edge.metadata.len(), 0 );
  }

  #[test]
  fn test_cons_empty_edge () 
  {
    let mut gph: Graph = cons_graph();
    let edge: Edge = gph.cons_empty_edge();
    assert_eq!( edge.id.len() > 0, true );
    assert_eq!( edge.name, "" );
    assert_eq!( edge.order, 0 );
    assert_eq!( edge.inner_order, 0 );
    assert_eq!( edge.delta, 0 );
    assert_eq!( edge.weight, 0 );
    assert_eq!( edge.left, "" );
    assert_eq!( edge.right, "" );
    assert_eq!( edge.labels.len(), 0 );
    assert_eq!( edge.metadata.len(), 0 );
  }
}