use std::collections::HashMap;
use std::collections::HashSet;

#[derive( Debug, Clone )]
pub struct Node 
{ 
  pub id: String,
  pub order: u64,
  pub inner_order: u64,
  pub delta: u64,
  pub weight: i64,
  pub name: String,
  pub labels: HashSet<String>, 
  pub metadata: HashMap<String, String>
}