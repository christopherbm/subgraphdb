use std::fmt::{ Display, Formatter, Result };
use common::{ DirectionType };
use tokenize::SyntaxToken;
use utils::{ parse_padded_str };

// ---------------------------------------------------------------------------------------------------------------------
// Instructions (everything padded and validated)

#[derive( Debug )]
pub struct Transaction
{
  pub build_id: String,
  pub db_nickname: String,

  pub graph_name: Option<String>,
  pub create_graph: Option<CreateGraph>,
  
  pub left_node_ref: Option<CreateNodeRef>,
  pub right_node_ref: Option<CreateNodeRef>,
  pub edge_dir: Option<DirectionType>,
  pub edge_primary_label: Option<String>,

  pub create_node: Vec<CreateNode>,
  pub create_edge: Vec<CreateEdge>,

  pub match_clause: Option<MatchClause>,
  pub from_clause: Option<FromClause>,
  pub return_clause: Option<ReturnClause>,

  pub err_state: Option<String>,
}
impl Transaction 
{
  pub fn new ( build_id: String, nickname: String ) -> Transaction 
  {
    Transaction 
    {
      build_id: build_id,
      db_nickname: nickname,

      graph_name: None,
      create_graph: None,

      left_node_ref: None,
      right_node_ref: None,
      edge_dir: None,
      edge_primary_label: None,

      create_node: Vec::new(),
      create_edge: Vec::new(),

      match_clause: None,
      from_clause: None,
      return_clause: None,

      err_state: None,
    }
  }

  pub fn has_writes ( &self ) -> bool
  {
    if self.create_graph.is_some() { return true; }
    if self.create_node.len() > 0 { return true; }
    if self.create_edge.len() > 0 { return true; }
    false
  }

  pub fn find_id_by_transaction_label ( &self, label: &str ) -> Option<String>
  {
    for cn in self.create_node.iter() 
    {
      if cn.transaction_label == label { return Some( cn.id.clone() ); }
    }
    None
  }
}
impl Display for Transaction
{
  fn fmt( &self, f: &mut Formatter ) -> Result 
  {
    let _ = write!( f, "\nBuild Id: {:?} \n", self.build_id );
    let _ = write!( f, "DB Nickname: pad({:?}) \n", parse_padded_str( &self.db_nickname )); 

    let _ = write!( f, "Graph Name: {:?} \n", self.graph_name );
    let _ = write!( f, "Create Graph: {:?} \n", self.create_graph );

    let _ = write!( f, "Create Nodes ({:?}); \n", self.create_node.len() );
    for cn in self.create_node.iter() 
    {
      let _ = write!( f, "  {:?} \n", cn.id );
      let _ = write!( f, "  {:?} : pad({:?}) \n\n", cn.transaction_label, parse_padded_str( &cn.primary_label ));
    }

    let _ = write!( f, "Create Edges ({:?}): \n", self.create_edge.len() );
    for eg in self.create_edge.iter() 
    { 
      let _ = write!( f, "  {:?} \n", eg.left_ref ); 
      let _ = write!( f, "  {:?} \n", eg.right_ref );
      let _ = write!( f, "  {:?} \n", eg.edge_dir );
      let _ = write!( f, "  pad({:?}) \n\n", parse_padded_str( &eg.primary_label ));   
    }
    
    let _ = write!( f, "Left Node Ref: {:?} \n", self.left_node_ref );
    let _ = write!( f, "Right Node Ref: {:?} \n", self.right_node_ref );
    let _ = write!( f, "Edge Direction: {:?} \n", self.edge_dir );
    let _ = write!( f, "Edge Primary: Label {:?} \n", self.edge_primary_label );

    if self.match_clause.is_some() 
    {
      let _ = write!( f, "Match Clause (NodeRef): \n" );
      let _ = write!( 
        f, 
        "  Transaction Label pad({:?}) \n", 
        parse_padded_str( 
          &self.match_clause.as_ref().unwrap().node_ref.transaction_label.as_ref().unwrap_or( 
            &String::from( "::None" )
      ))); 
      
      let _ = write!( 
        f, 
        "  Primary Label pad({:?}) \n", 
        parse_padded_str( 
          &self.match_clause.as_ref().unwrap().node_ref.primary_label.as_ref().unwrap_or(
            &String::from( "::None" )
        ))); 
    }
    else { let _ = write!( f, "Match Clause {:?} \n", self.match_clause ); }

    if self.from_clause.is_some() 
    {
      let _ = write!( f, "From Clause (Graph Name) pad({:?}) \n", 
        parse_padded_str( &self.from_clause.as_ref().unwrap().graph_name ));
    }
    else { let _ = write!( f, "From Clause {:?} \n", self.from_clause ); }

    if self.return_clause.is_some() 
    {
      let _ = write!( f, "Return Clause {:?} AS {:?} \n",
        &self.return_clause.as_ref().unwrap().transaction_label, 
        &self.return_clause.as_ref().unwrap().output_label );
    }
    else { let _ = write!( f, "Return Clause {:?} \n", self.return_clause ); }
    

    let _ = write!( f, "Error State: {:?} \n", self.err_state );    

    write!( f, "\n")
  }
}

#[derive( Debug )]
pub struct CreateGraph { pub id: String, pub name: String, }
impl CreateGraph 
{
  pub fn new ( id: String, name: String ) -> CreateGraph
  {
    CreateGraph { id: id, name: name, }
  }
}

#[derive( Debug )]
pub struct CreateNode { pub id: String, pub transaction_label: String, pub primary_label: String, }
impl CreateNode 
{
  pub fn new ( id: String, transaction_label: String, primary_label: String, ) -> CreateNode
  {
    CreateNode { id: id, transaction_label: transaction_label, primary_label: primary_label }
  }
}

#[derive( Debug, Clone )]
pub struct CreateNodeRef { pub id: String, pub transaction_label: String }
impl CreateNodeRef 
{
  pub fn new ( id: String, transaction_label: String ) -> CreateNodeRef
  {
    CreateNodeRef { id: id, transaction_label: transaction_label }
  }
}

#[derive( Debug )]
pub struct CreateEdge 
{ 
  pub id: String,
  pub left_ref: CreateNodeRef, 
  pub right_ref: CreateNodeRef, 
  pub edge_dir: DirectionType,
  pub primary_label: String,
}
impl CreateEdge 
{
  pub fn new ( 
    id: String,
    left_ref: CreateNodeRef, 
    right_ref: CreateNodeRef, 
    edge_dir: DirectionType, 
    primary_label: String ) -> CreateEdge
  {
    CreateEdge 
    {
      id: id,
      left_ref: left_ref, 
      right_ref: right_ref, 
      edge_dir: edge_dir,
      primary_label: primary_label,
    }
  }
}

#[derive( Debug )]
pub struct MatchClause { pub node_ref: NodeRef, }
impl MatchClause 
{
  pub fn new ( node_ref: NodeRef ) -> MatchClause
  {
    MatchClause { node_ref: node_ref }
  }

  pub fn empty () -> MatchClause
  {
    MatchClause { node_ref: NodeRef::new( None, None )}
  }

  pub fn is_empty ( &self ) -> bool 
  {
    if self.node_ref.transaction_label.is_some() == false 
      && self.node_ref.primary_label.is_some() == false { return true; }
    false
  }
}

#[derive( Debug )]
pub struct NodeRef { pub transaction_label: Option<String>, pub primary_label: Option<String> }
impl NodeRef 
{
  pub fn new ( transaction_label: Option<String>, primary_label: Option<String> ) -> NodeRef
  {
    NodeRef { transaction_label: transaction_label, primary_label: primary_label }
  }
}

#[derive( Debug )]
pub struct FromClause { pub graph_name: String }
impl FromClause
{
  pub fn new ( graph_name: String ) -> FromClause
  {
    FromClause { graph_name: graph_name }
  }
}

#[derive( Debug )]
pub struct ReturnClause { pub transaction_label: String, pub output_label: Option<String> }
impl ReturnClause
{
  pub fn new ( transaction_label: String, output_label: Option<String> ) -> ReturnClause
  {
    ReturnClause { transaction_label: transaction_label, output_label: output_label }
  }
}


pub struct BraceClause 
{
  pub key_val_pairs: Vec<(SyntaxToken, SyntaxToken)>,
}
pub struct ParenClause {}
// ---------------------------------------------------------------------------------------------------------------------

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