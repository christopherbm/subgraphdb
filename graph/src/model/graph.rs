use std::collections::{ HashMap, HashSet };
use uuid::Uuid;
use super::node::Node;
use super::edge::{ Edge, EdgeType };
use crate::graph_utils::graph_utils::{ cons_node, cons_edge };

static NODE_MISSING_ERR: &'static str = "::node_missing";
static NODE_EXISTS_ERR: &'static str = "::node_exists";
static EDGE_EXISTS_ERR: &'static str = "::edge_exists";

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

  pub fn add_node ( &mut self, node: Node ) -> Result<bool, &'static str>
  { 
    if self.nodes.contains_key( &node.id ) { return Err( NODE_EXISTS_ERR ) }
    self.nodes.insert( node.id.clone(), node );
    Ok( true )
  }

  pub fn add_edge ( &mut self, edge: Edge ) -> Result<bool, &'static str>
  {
    if self.edges.contains_key( &edge.id ) { return Err( EDGE_EXISTS_ERR ) }
    if !self.nodes.contains_key( &edge.left ) { return Err( NODE_MISSING_ERR ) }
    if !self.nodes.contains_key( &edge.right ) { return Err( NODE_MISSING_ERR ) }
    self.edges.insert( edge.id.clone(), edge );
    Ok( true )
  }

  pub fn get_nodes ( &self ) -> Vec<&Node> 
  {
    let mut ret: Vec<&Node> = self.nodes.values().into_iter().collect::<Vec<_>>();
    ret.sort_by(| a, b | a.inner_order.cmp( &b.inner_order ));
    ret
  }

  pub fn get_edges ( &self ) -> Vec<&Edge> 
  {
    let mut ret: Vec<&Edge> = self.edges.values().into_iter().collect::<Vec<_>>();
    ret.sort_by(| a, b | a.inner_order.cmp( &b.inner_order ));
    ret    
  }

  pub fn find_node_by_id ( &self, id: &str ) -> Option<&Node>
  {
    if self.nodes.contains_key( id ) { return self.nodes.get( id ); }
    None
  }

  pub fn find_edge_by_id ( &self, id: &str ) -> Option<&Edge>
  {
    if self.edges.contains_key( id ) { return self.edges.get( id ); }
    None
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

  #[test]
  fn test_add_node () 
  {
    let mut gph: Graph = cons_graph();
    let node: Node = gph.cons_empty_node();
    let id: String = node.id.clone();
    let add_res = gph.add_node( node );
    assert_eq!( add_res.is_ok(), true );
    assert_eq!( add_res.unwrap(), true );

    let mut node1: Node = gph.cons_empty_node();
    node1.id = id;
    let add_res1 = gph.add_node( node1 );
    assert_eq!( add_res1.is_ok(), false );
  }

  #[test]
  fn test_add_edge () 
  {
    let mut gph: Graph = cons_graph();
    let node: Node = gph.cons_empty_node();
    let nid: String = node.id.clone();
    let node1: Node = gph.cons_empty_node();
    let nid1: String = node1.id.clone();
    let _ = gph.add_node( node );
    let _ = gph.add_node( node1 );

    let mut edge: Edge = gph.cons_empty_edge();
    edge.left = nid.clone();
    edge.right = nid1.clone();
    let id: String = edge.id.clone();
    let add_res = gph.add_edge( edge );
    assert_eq!( add_res.is_ok(), true );
    assert_eq!( add_res.unwrap(), true );

    let mut edge1: Edge = gph.cons_empty_edge();
    edge1.id = id;
    edge1.left = nid.clone();
    edge1.right = nid1.clone();
    let add_res1 = gph.add_edge( edge1 );
    assert_eq!( add_res1.is_ok(), false );
  }

  #[test]
  fn test_get_nodes () 
  {
    let mut gph: Graph = cons_graph();
    let node: Node = gph.cons_empty_node();
    let node1: Node = gph.cons_empty_node();
    let _ = gph.add_node( node );
    let _ = gph.add_node( node1 );

    assert_eq!( gph.get_nodes().len(), 2 );
  }

  #[test]
  fn test_get_edges () 
  {
    let mut gph: Graph = cons_graph();
    let node: Node = gph.cons_empty_node();
    let nid: String = node.id.clone();
    let node1: Node = gph.cons_empty_node();
    let nid1: String = node1.id.clone();
    let _ = gph.add_node( node );
    let _ = gph.add_node( node1 );

    let mut edge: Edge = gph.cons_empty_edge();
    edge.left = nid.clone();
    edge.right = nid1.clone();
    let _ = gph.add_edge( edge );

    assert_eq!( gph.get_edges().len(), 1 );
  }

  #[test]
  fn test_find_node_by_id () 
  {
    let mut gph: Graph = cons_graph();
    let node1: Node = gph.cons_empty_node();
    let node_id: String = node1.id.clone();

    let _ = gph.add_node( node1 );

    let find_res = gph.find_node_by_id( &node_id );
    assert_eq!( find_res.is_some(), true );

    let find_res1 = gph.find_node_by_id( "fail" );
    assert_eq!( find_res1.is_some(), false );
  }

  #[test]
  fn test_find_edge_by_id () 
  {
    let mut gph: Graph = cons_graph();
    let node1: Node = gph.cons_empty_node();
    let node2: Node = gph.cons_empty_node();

    let id1: String = node1.id.clone();
    let id2: String = node2.id.clone();

    let _ = gph.add_node( node1 ); 
    let _ = gph.add_node( node2 );

    let edge1: Edge = gph.cons_edge( String::from( "edge1" ), 0, 0, id1, id2, EdgeType::Undirected );
    let edge_id: String = edge1.id.clone();
    let res = gph.add_edge( edge1 );

    let find_res = gph.find_edge_by_id( &edge_id );
    assert_eq!( find_res.is_some(), true );

    let find_res1 = gph.find_node_by_id( "fail" );
    assert_eq!( find_res1.is_some(), false );
  }
}