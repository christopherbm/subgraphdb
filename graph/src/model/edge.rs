use std::collections::HashMap;
use std::collections::HashSet;

#[derive( Debug, Clone, PartialEq )]
pub enum EdgeType { Undirected, DirectedLeft, DirectedRight, Bidirectional }

#[derive( Debug, Clone )]
pub struct Edge 
{ 
  pub id: String,
  pub order: u64,
  pub inner_order: u64,
  pub delta: u64,
  pub weight: i64,
  pub name: String,
  pub left: String, 
  pub right: String, 
  pub etype: EdgeType,
  pub labels: HashSet<String>, 
  pub metadata: HashMap<String, String>
}