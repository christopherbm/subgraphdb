use std::fmt::{ Display, Formatter, Result };
use common::{ DirectionType };

// ---------------------------------------------------------------------------------------------------------------------
// Instructions

#[derive( Debug )]
pub struct Transaction
{
  pub create_graph: Option<CreateGraph>,
  
  pub left_node_ref: Option<CreateNodeRef>,
  pub right_node_ref: Option<CreateNodeRef>,
  pub edge_dir: Option<DirectionType>,
  pub edge_primary_label: Option<String>,

  pub create_node: Vec<CreateNode>,
  pub create_edge: Vec<CreateEdge>,

  pub err_state: Option<String>,
}
impl Transaction 
{
  pub fn new () -> Transaction 
  {
    Transaction 
    {
      create_graph: None,

      left_node_ref: None,
      right_node_ref: None,
      edge_dir: None,
      edge_primary_label: None,

      create_node: Vec::new(),
      create_edge: Vec::new(),

      err_state: None,
    }
  }
}
impl Display for Transaction
{
  fn fmt( &self, f: &mut Formatter ) -> Result 
  {
    let _ = write!( f, "\nCreate Graph: {:?} \n", self.create_graph );

    let _ = write!( f, "Create Nodes ({:?}): \n", self.create_node.len() );
    for cn in self.create_node.iter() { let _ = write!( f, "  {:?} \n", cn ); }

    let _ = write!( f, "Create Edges ({:?}): \n", self.create_edge.len() );
    for eg in self.create_edge.iter() 
    { 
      let _ = write!( f, "  {:?} \n", eg.left_ref ); 
      let _ = write!( f, "  {:?} \n", eg.right_ref );
      let _ = write!( f, "  {:?} \n", eg.edge_dir );
      let _ = write!( f, "  {:?} \n", eg.primary_label );   
    }
    
    let _ = write!( f, "Left Node Ref: {:?} \n", self.left_node_ref );
    let _ = write!( f, "Right Node Ref: {:?} \n", self.right_node_ref );
    let _ = write!( f, "Edge Direction: {:?} \n", self.edge_dir );
    let _ = write!( f, "Edge Primary: Label {:?} \n", self.edge_primary_label );
    let _ = write!( f, "Error State: {:?} \n", self.err_state );    

    write!( f, "\n")
  }
}

#[derive( Debug )]
pub struct CreateGraph { pub name: String, }
impl CreateGraph 
{
  pub fn new ( name: String ) -> CreateGraph
  {
    CreateGraph { name: name, }
  }
}

#[derive( Debug )]
pub struct CreateNode { pub transaction_label: String, pub primary_label: String, }
impl CreateNode 
{
  pub fn new ( transaction_label: String, primary_label: String, ) -> CreateNode
  {
    CreateNode { transaction_label: transaction_label, primary_label: primary_label }
  }
}

#[derive( Debug, Clone )]
pub struct CreateNodeRef { pub transaction_label: String }
impl CreateNodeRef 
{
  pub fn new ( transaction_label: String ) -> CreateNodeRef
  {
    CreateNodeRef { transaction_label: transaction_label }
  }
}

#[derive( Debug )]
pub struct CreateEdge 
{ 
  pub left_ref: CreateNodeRef, 
  pub right_ref: CreateNodeRef, 
  pub edge_dir: DirectionType,
  pub primary_label: String,
}
impl CreateEdge 
{
  pub fn new ( 
    left_ref: CreateNodeRef, 
    right_ref: CreateNodeRef, 
    edge_dir: DirectionType, 
    primary_label: String ) -> CreateEdge
  {
    CreateEdge 
    {
      left_ref: left_ref, 
      right_ref: right_ref, 
      edge_dir: edge_dir,
      primary_label: primary_label,
    }
  }
}

#[derive( Debug )]
pub struct CreateSingleFileDB {}

#[derive( Debug )]
pub struct InitSingleFileDB {}

// ---------------------------------------------------------------------------------------------------------------------

#[derive( PartialEq, Debug, Copy, Clone )]
pub enum StatementType 
{ 
  CreateGraph
}

#[derive( Debug, Clone )]
pub struct Statement
{ 
  pub statement_type: StatementType,
  pub name_opt: Option<String>,
}

pub fn cons_statement ( stmt_type: StatementType, name_opt: Option<String> ) -> Statement
{
  Statement { statement_type: stmt_type, name_opt: name_opt }
}

/// Insert Node Command
#[derive( Debug, PartialEq )]
pub struct InsertNode {}

/// Insert Edge Command
#[derive( Debug, PartialEq )]
pub struct InsertEdge {}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn it_works () 
  {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
