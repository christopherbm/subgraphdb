#[allow(dead_code)]
pub mod graph_utils
{
  use std::collections::{ HashMap, HashSet };
  use crate::model::node::{ Node };
  use crate::model::edge::{ Edge, EdgeType };

  pub fn cons_node ( id: String, name: String, order: u64, inner_order: u64, delta: u64, weight: i64,
    labels: HashSet<String>, metadata: HashMap<String, String> ) -> Node 
  {
    Node {
      id: id,
      name: name,
      order: order,
      inner_order: inner_order,
      delta: delta,
      weight: weight,
      labels: labels,
      metadata: metadata
    }
  }

  pub fn cons_edge ( id: String, name: String, order: u64, inner_order: u64, delta: u64, weight: i64,
    left: String, right: String, etype: EdgeType, labels: HashSet<String>, metadata: HashMap<String, String> ) -> Edge 
  {
    Edge { 
      id: id,
      name: name,
      order: order,
      inner_order: inner_order,
      delta: delta,
      weight: weight,
      left: left, 
      right: right, 
      etype: etype,
      labels: labels, 
      metadata: metadata
    }
  }
}

#[cfg(test)]
mod tests 
{
  use std::collections::{ HashMap, HashSet };
  use crate::model::node::{ Node };
  use crate::model::edge::{ Edge, EdgeType };
  use crate::graph_utils::graph_utils::{ cons_node, cons_edge };

  #[test]
  fn test_cons_node () 
  {
    let node: Node = cons_node(
      String::from( "id" ),
      String::from( "node" ),
      0,
      1,
      2,
      3,
      HashSet::new(),
      HashMap::new()
    );
    
    assert_eq!( node.id, "id" );
    assert_eq!( node.name, "node" );
    assert_eq!( node.order, 0 );
    assert_eq!( node.inner_order, 1 );
    assert_eq!( node.delta, 2 );
    assert_eq!( node.weight, 3 );
    assert_eq!( node.labels.len(), 0 );
    assert_eq!( node.metadata.len(), 0 );
  }

  #[test]
  fn test_cons_edge () 
  {
    let edge: Edge = cons_edge(
      String::from( "id" ),
      String::from( "edge" ),
      0,
      1,
      2,
      3,
      String::from( "left" ),
      String::from( "right" ),
      EdgeType::Undirected,
      HashSet::new(),
      HashMap::new()
    );
    
    assert_eq!( edge.id, "id" );
    assert_eq!( edge.name, "edge" );
    assert_eq!( edge.order, 0 );
    assert_eq!( edge.inner_order, 1 );
    assert_eq!( edge.delta, 2 );
    assert_eq!( edge.weight, 3 );
    assert_eq!( edge.left, "left" );
    assert_eq!( edge.right, "right" );
    assert_eq!( edge.etype, EdgeType::Undirected );
    assert_eq!( edge.labels.len(), 0 );
    assert_eq!( edge.metadata.len(), 0 );
  }
}