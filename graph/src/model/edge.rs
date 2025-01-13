use std::collections::HashMap;
use std::collections::HashSet;

#[derive( Debug, Clone, PartialEq )]
pub enum EdgeType { Undirected, DirectedLeft, DirectedRight, Bidirectional }

#[derive( Debug, Clone )]
pub struct Edge 
{ 
  pub id: String,
  pub delta: u64,
  pub weight: f64,
  pub name: String,
  pub left: String, 
  pub right: String, 
  pub etype: EdgeType,
  pub labels: HashSet<String>, 
  pub metadata: HashMap<String, String>
}