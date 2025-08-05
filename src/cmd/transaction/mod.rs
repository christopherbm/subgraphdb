use std::fmt::{ Display, Formatter, Result };
use crate::datagramv2::grams::{ Label, UUID };
use crate::utils::{ parse_padded_str };
use crate::cmd::{ CreateStatement, EdgeStatement, FromClause, NodeRefStatement, NodeStatement, ReadClause, ReturnClause };

#[derive( Debug )]
pub struct Transaction
{
  pub build_id: UUID,
  pub db_nickname: Label,
  pub query_order: u16,
  pub err_state: Option<String>,

  pub create_statement: Option<CreateStatement>,

  pub node_statements: Vec<NodeStatement>,
  pub node_ref_statements: Vec<NodeRefStatement>,
  pub edge_statements: Vec<EdgeStatement>,

  pub read_clause: Option<ReadClause>,
  pub from_clause: Option<FromClause>,
  pub return_clause: Option<ReturnClause>,
}

impl Transaction 
{
  pub fn new ( build_id: UUID, nickname: Label, query_order: u16 ) -> Transaction 
  {
    Transaction 
    {
      build_id: build_id,
      db_nickname: nickname,
      query_order: query_order,
      err_state: None,

      create_statement: None,

      node_statements: Vec::new(),
      node_ref_statements: Vec::new(),
      edge_statements: Vec::new(),

      read_clause: None,
      from_clause: None,
      return_clause: None,
    }
  }

  
  pub fn has_writes ( &self ) -> bool { self.create_statement.is_some() }

  
  pub fn next_node_statement ( &self, query_order: u16 ) -> Option<&NodeStatement> 
  {
    for stmt in self.node_statements.iter() 
    {
      if stmt.query_order == query_order { return Some( stmt ) }
    }
    None
  }
  
  
  pub fn next_edge_statement ( &self, query_order: u16 ) -> Option<&EdgeStatement> 
  {
    for stmt in self.edge_statements.iter() 
    {
      if stmt.query_order == query_order { return Some( stmt ); }
    }
    None
  }

  
  pub fn next_ref_statement ( &self, query_order: u16 ) -> Option<&NodeRefStatement> 
  {
    for stmt in self.node_ref_statements.iter() 
    {
      if stmt.query_order == query_order { return Some( stmt ) }
    }
    None
  }


  /// Search NodeStatements for given TranactionLabel
  pub fn find_node_by_transaction_label ( &self, transaction_label: &str ) -> Option<&NodeStatement> 
  {
    for stmt in self.node_statements.iter() 
    {
      if stmt.transaction_label.is_some() 
      {
        if stmt.transaction_label.as_ref().unwrap() == transaction_label { return Some( stmt ) }
      }
    }
    None
  } 
}

impl Display for Transaction
{
  fn fmt( &self, f: &mut Formatter ) -> Result 
  {
    let _ = write!( f, "\nBuild Id: {:?} \n", self.build_id.val );
    let _ = write!( f, "DB Nickname: pad({:?}) \n", &self.db_nickname.val ); 
    let _ = write!( f, "Query Order: {:?} \n", &self.query_order ); 

    //let _ = write!( f, "Graph Name: {:?} \n", self.graph_name );
    let _ = write!( f, "Create Statement: {:?} \n", self.create_statement );

    let _ = write!( f, "Nodes ({:?}): \n", self.node_statements.len() );
    for ns in self.node_statements.iter() 
    {
      let _ = write!( f, "  Id {:?} \n", ns.id );
      let _ = write!( f, "  Query Order {:?} \n", ns.query_order );
      let _ = write!( f, "  Transaction Label {:?} \n", ns.transaction_label );
      let _ = write!( f, "  Primary Label {:?} \n\n", ns.primary_label );
    }

    let _ = write!( f, "Node Refs ({:?}): \n", self.node_ref_statements.len() );
    for nr in self.node_ref_statements.iter() 
    {
      let _ = write!( f, "  {:?} \n", nr );
    }

    let _ = write!( f, "Create Edges ({:?}): \n", self.edge_statements.len() );
    for eg in self.edge_statements.iter() 
    { 
      let _ = write!( f, "  {:?} \n", eg ); 
    //  let _ = write!( f, "  {:?} \n", eg.right_ref );
    //  let _ = write!( f, "  {:?} \n", eg.edge_dir );
    //  let _ = write!( f, "  pad({:?}) \n\n", parse_padded_str( &eg.primary_label ));   
    }
    
    //let _ = write!( f, "Left Node Ref: {:?} \n", self.left_node_ref );
    //let _ = write!( f, "Right Node Ref: {:?} \n", self.right_node_ref );
    //let _ = write!( f, "Edge Direction: {:?} \n", self.edge_dir );
    //let _ = write!( f, "Edge Primary: Label {:?} \n", self.edge_primary_label );

    if self.read_clause.is_some() 
    {
      let _ = write!( f, "Read Clause {:?} \n", self.read_clause );
    }
    else { let _ = write!( f, "Read Clause {:?} \n", self.read_clause ); }

    /*
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
    */

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

#[cfg(test)]
mod tests 
{
  //use super::*;
  //use crate::common::LABEL_BYTES;

  #[test]
  fn test_new_transaction () 
  {
    //let build_id_res = UUID::new( String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ));
    //let db_nickname_res = Label::new( String::from( "devs" ), LABEL_BYTES );
    //let transaction: Transaction = Transaction::new( build_id_res.unwrap(), db_nickname_res.unwrap(), 0 );

    //assert_eq!( transaction.build_id.is_ok(), true );
    //assert_eq!( transaction.db_nickname.is_ok(), true );
  }

  #[test]
  fn test_match () 
  {
    //let build_id_res = UUID::new( String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ));
    //let db_nickname_res = Label::new( String::from( "devs" ), LABEL_BYTES );
    //let mut transaction: Transaction = Transaction::new( build_id_res.unwrap(), db_nickname_res.unwrap(), 0 );

    //assert_eq!( transaction.build_id.unwrap().is_ok(), true );
    //assert_eq!( transaction.db_nickname.unwrap().is_ok(), true );

    //let match_token = SyntaxToken::new( SyntaxTokenType::KeywordMatch, String::from( "MATCH" ));

    //transaction.add_token( match_token );

    //println!( "{}", transaction );
  }
}