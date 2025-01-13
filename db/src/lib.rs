#[derive(Debug)]
pub enum ConnType 
{
  FlatClear,
  FlatBinary,
  FlatEncrypted,
  Server
}

#[derive(Debug)]
pub struct DBConnection
{ 
  pub path: Option<String>
}

#[derive(Debug)]
pub struct SubgraphDB
{ 
  pub id: String,
  pub inner_node_order: u64 
}

impl SubgraphDB 
{
  //pub fn edge_count ( &self ) -> usize { self.edges.len() }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() 
  {
    //let result = add(2, 2);
    //assert_eq!(result, 4);
  }
}
