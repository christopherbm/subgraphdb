use std::collections::HashMap;
use std::collections::HashSet;

#[derive( Debug, Clone )]
pub struct Node 
{ 
  pub id: String,
  pub delta: u64,
  pub weight: f64,
  pub name: String,
  pub labels: HashSet<String>, 
  pub metadata: HashMap<String, String>
}