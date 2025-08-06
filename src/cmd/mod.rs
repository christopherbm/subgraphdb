pub mod transaction;

use crate::common::{ DirectionType };
use crate::common::kvps::{ KeyValString };
use crate::tokenize::{ SyntaxToken, SyntaxTokenType };
use crate::utils::cons_uuid;

/* @version 0.3.0 */

// ---------------------------------------------------------------------------------------------------------------------
#[derive( Debug )]
pub struct ReadClause 
{ 
  pub order: u16, // must be query order
  pub match_statements: Vec<MatchStatement>,
}
impl ReadClause 
{
  pub fn new ( order: u16 ) -> ReadClause 
  { 
    ReadClause { order: order, match_statements: Vec::new() }
  }

  pub fn add_match_statement ( &mut self, stmt: MatchStatement ) { self.match_statements.push( stmt ); }
}


#[derive( Debug )]
pub struct MatchStatement 
{
  pub order: u16, // must be query order
  pub is_open: bool,
  pub labels_complete: bool, // whether or not labels can still be added
  pub kvps_complete: bool, // whether or not key-value pairs can still be added
  pub transaction_label: Option<String>,
  pub primary_label: Option<String>,
  pub kv_str: Vec<KeyValString>,
}
impl MatchStatement 
{
  pub fn new ( 
    order: u16, is_open: bool, transaction_label: Option<String>, primary_label: Option<String> ) -> MatchStatement 
  {
    MatchStatement { 
      order: order, 
      is_open: is_open, 
      labels_complete: false,
      kvps_complete: false,
      transaction_label: transaction_label, 
      primary_label: primary_label, 
      kv_str: Vec::new()
    }
  }

  pub fn from ( old_stmt: MatchStatement, token: &SyntaxToken ) -> Result<MatchStatement, String> 
  {
    match token.token_type 
    {
      SyntaxTokenType::Label => 
      {
        if old_stmt.transaction_label.is_none() == true && old_stmt.labels_complete == false 
        {
          return Ok( MatchStatement::new(
            old_stmt.order, 
            old_stmt.is_open, 
            Some( token.val.clone() ), 
            old_stmt.primary_label ));
        }
      }

      SyntaxTokenType::PrimaryLabel => 
      {
        if old_stmt.primary_label.is_none() == true && old_stmt.labels_complete == false 
        {
          return Ok( MatchStatement::new(
            old_stmt.order, 
            old_stmt.is_open, 
            old_stmt.transaction_label, 
            Some( token.val.clone() )));
        }
      }

      SyntaxTokenType::OpenNode => 
      {
        let stmt = MatchStatement::new( 
          old_stmt.order, 
          old_stmt.is_open, 
          old_stmt.transaction_label, 
          old_stmt.primary_label );
        return Ok( stmt );
      }

      SyntaxTokenType::CloseNode => 
      {
        let mut stmt = MatchStatement::new( 
          old_stmt.order, 
          old_stmt.is_open, 
          old_stmt.transaction_label, 
          old_stmt.primary_label );
        stmt.is_open = false;
        stmt.kv_str = old_stmt.kv_str;
        stmt.kvps_complete = old_stmt.kvps_complete;
        stmt.labels_complete = old_stmt.labels_complete;
        return Ok( stmt );
      }

      SyntaxTokenType::OpenBrace => 
      {
        let mut stmt = MatchStatement::new( 
          old_stmt.order, 
          old_stmt.is_open, 
          old_stmt.transaction_label, 
          old_stmt.primary_label );
        stmt.labels_complete = true;
        return Ok( stmt );
      }

      SyntaxTokenType::CloseBrace => 
      {
        let mut stmt = MatchStatement::new( 
          old_stmt.order, 
          old_stmt.is_open, 
          old_stmt.transaction_label, 
          old_stmt.primary_label );
        stmt.kvps_complete = true;
        stmt.labels_complete = old_stmt.labels_complete;
        stmt.kv_str = old_stmt.kv_str;
        return Ok( stmt );
      }

      SyntaxTokenType::Key => 
      {
        let mut stmt = MatchStatement::new( 
          old_stmt.order, 
          old_stmt.is_open, 
          old_stmt.transaction_label, 
          old_stmt.primary_label );
        stmt.labels_complete = old_stmt.labels_complete;
        stmt.kv_str = old_stmt.kv_str;
        stmt.kv_str.push( KeyValString::new( token.val.clone(), None ));
        return Ok( stmt );
      }

      SyntaxTokenType::StringValue => 
      {
        if old_stmt.kv_str.len() > 0 
        {
          let mut stmt = MatchStatement::new( 
            old_stmt.order, 
            old_stmt.is_open, 
            old_stmt.transaction_label, 
            old_stmt.primary_label );
          stmt.labels_complete = old_stmt.labels_complete;
          stmt.kv_str = old_stmt.kv_str;
          let kvp_opt = stmt.kv_str.pop();
          stmt.kv_str.push( KeyValString::from( &kvp_opt.unwrap(), token.val.clone() ));
          return Ok( stmt );
        }
      }

      _ => {}
    }
    Err( String::from( "Syntax Error: Match Statement" ))
  }
}

#[derive( Debug, Clone )]
pub struct CreateStatement 
{
  pub order: u16, // must be query order
  pub is_open: bool,
  pub graph_name: Option<String>,
}
impl CreateStatement 
{
  pub fn new ( order: u16, name: Option<String> ) -> CreateStatement 
  {
    CreateStatement { order: order, is_open: true, graph_name: name }
  }

  pub fn from ( stmt: &CreateStatement, token: SyntaxToken ) -> Result<CreateStatement, String> 
  {
    if token.token_type == SyntaxTokenType::Label 
    {
      return Ok(
          CreateStatement { 
            order: stmt.order, 
            is_open: stmt.is_open, 
            graph_name: Some( token.val )
          }
        )
    }
    Err( String::from( "Syntax Error: Create Statement" ))
  }
}

#[derive( Debug, Clone )]
pub struct ParenStatement 
{
  pub order: u16, // must be query order
  pub is_open: bool,
  pub transaction_label: Option<String>,
  pub primary_label: Option<String>,
}
impl ParenStatement 
{
  pub fn new ( 
    order: u16, is_open: bool, transaction_label: Option<String>, primary_label: Option<String> ) -> ParenStatement 
  {
    ParenStatement 
    { 
      order: order, 
      is_open: is_open, 
      transaction_label: transaction_label, 
      primary_label: primary_label 
    }
  }
  
  pub fn from ( stmt: ParenStatement, token: &SyntaxToken ) -> Result<ParenStatement, String> 
  {
    match token.token_type 
    {
      SyntaxTokenType::Label => 
      {
        return Ok(
          ParenStatement { 
            order: stmt.order, 
            is_open: stmt.is_open, 
            transaction_label: Some( token.val.clone() ), 
            primary_label: stmt.primary_label, 
          }
        )
      },
      SyntaxTokenType::PrimaryLabel => 
      {
        return Ok(
          ParenStatement { 
            order: stmt.order, 
            is_open: stmt.is_open, 
            transaction_label: stmt.transaction_label, 
            primary_label: Some( token.val.clone() ), 
          }
        )
      },
      
      _ => {}
    }
    Err( String::from( "Syntax Error: Paren Statement" ))
  }

  pub fn is_ref ( &self ) -> bool { self.primary_label.is_none() }

  pub fn is_empty ( &self ) -> bool 
  {
    if self.primary_label.is_none() && self.transaction_label.is_none() { return true; }
    false
  }
  
  pub fn to_node_statement ( &self ) -> Result<NodeStatement, String>
  {
    if self.is_ref() == false 
    {
      return Ok( 
        NodeStatement::new( 
          cons_uuid(), 
          self.order, 
          self.transaction_label.clone(), 
          self.primary_label.clone().unwrap() ));
    }
    Err( String::from( "Syntax Error: Paren Statement" ))
  }

  pub fn to_node_ref_statement ( &self, id_ref: Option<String> ) -> Result<NodeRefStatement, String>
  {
    if self.is_ref() 
    {
      return Ok( NodeRefStatement::new( self.order, id_ref, self.transaction_label.clone().unwrap() ));
    }
    Err( String::from( "Syntax Error: Paren Ref Statement" ))
  }
}

#[derive( Debug, Clone )]
pub struct BracketStatement 
{
  pub order: u16, // must be query order
  pub is_open: bool,
  pub transaction_label: Option<String>,
  pub primary_label: Option<String>,
  pub edge_dir: DirectionType,
}
impl BracketStatement 
{
  pub fn new ( 
    order: u16, is_open: bool, transaction_label: Option<String>, primary_label: Option<String>, 
    edge_dir: DirectionType ) -> BracketStatement 
  {
    BracketStatement 
    { 
      order: order, 
      is_open: is_open, 
      transaction_label: transaction_label, 
      primary_label: primary_label,
      edge_dir: edge_dir
    }
  }
  
  pub fn from ( stmt: BracketStatement, token: &SyntaxToken ) -> Result<BracketStatement, String> 
  {
    match token.token_type 
    {
      SyntaxTokenType::Label => 
      {
        return Ok(
          BracketStatement { 
            order: stmt.order, 
            is_open: stmt.is_open, 
            transaction_label: Some( token.val.clone() ), 
            primary_label: stmt.primary_label, 
            edge_dir: stmt.edge_dir,
          }
        )
      }

      SyntaxTokenType::PrimaryLabel => 
      {
        return Ok(
          BracketStatement { 
            order: stmt.order, 
            is_open: stmt.is_open, 
            transaction_label: stmt.transaction_label, 
            primary_label: Some( token.val.clone() ),
            edge_dir: stmt.edge_dir,
          }
        )
      }
      
      SyntaxTokenType::EdgeDirection => 
      {
        if token.val == "-" { return Ok( stmt ); }
      }

      _ => {}
    }
    Err( String::from( "Syntax Error: Bracket Statement" ))
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


#[derive( Debug )]
pub struct NodeStatement 
{
  pub id: String,
  pub query_order: u16,
  pub transaction_label: Option<String>,
  pub primary_label: String,
}
impl NodeStatement
{
  pub fn new ( id: String, query_order: u16, transaction_label: Option<String>, primary_label: String ) -> NodeStatement
  {
    NodeStatement { id: id, query_order: query_order, transaction_label: transaction_label, primary_label: primary_label }
  }
}

#[derive( Debug )]
pub struct NodeRefStatement 
{
  pub query_order: u16,
  pub id_ref: Option<String>,
  pub transaction_label: String
}
impl NodeRefStatement
{
  pub fn new ( order: u16, id_ref: Option<String>, transaction_label: String ) -> NodeRefStatement
  {
    NodeRefStatement { query_order: order, id_ref: id_ref, transaction_label: transaction_label }
  }
}

#[derive( Debug )]
pub struct EdgeStatement 
{
  pub id: String,
  pub query_order: u16,
  pub transaction_label: Option<String>,
  pub primary_label: String,
}
impl EdgeStatement
{
  pub fn new ( id: String, order: u16, transaction_label: Option<String>, primary_label: String ) -> EdgeStatement
  {
    EdgeStatement { id: id, query_order: order, transaction_label: transaction_label, primary_label: primary_label }
  }
}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_match_statement () 
  {
    // -- new
    let ms = MatchStatement::new( 
      5, 
      true, 
      Some( String::from( "transaction label" )), 
      Some( String::from( "primary label" )));
    
    assert_eq!( ms.order, 5 );
    assert_eq!( ms.is_open, true );
    assert_eq!( ms.transaction_label, Some( String::from( "transaction label" )));
    assert_eq!( ms.primary_label, Some( String::from( "primary label" )));

    // -- labels
    let ms1 = MatchStatement::new( 5, true, None, None );
    let res1 = MatchStatement::from( ms1, &SyntaxToken::new( SyntaxTokenType::Label, String::from( "transaction label" )));
    assert_eq!( res1.is_ok(), true );
    assert_eq!( res1.as_ref().unwrap().transaction_label.is_some(), true );
    assert_eq!( res1.as_ref().unwrap().transaction_label, Some( String::from( "transaction label" )));

    let res2 = MatchStatement::from( res1.unwrap(), &SyntaxToken::new( SyntaxTokenType::PrimaryLabel, String::from( "primary label" )));
    assert_eq!( res2.is_ok(), true );
    assert_eq!( res2.as_ref().unwrap().primary_label.is_some(), true );
    assert_eq!( res2.as_ref().unwrap().primary_label, Some( String::from( "primary label" )));

    // -- open / close nodes
    let ms2 = MatchStatement::new( 5, true, None, None );
    let res3 = MatchStatement::from( ms2, &SyntaxToken::new( SyntaxTokenType::OpenNode, String::from( "(" )));
    assert_eq!( res3.is_ok(), true );
    assert_eq!( res3.as_ref().unwrap().labels_complete, true );

    let ms4 = MatchStatement::new( 5, true, None, None );
    let res4 = MatchStatement::from( ms4, &SyntaxToken::new( SyntaxTokenType::CloseNode, String::from( ")" )));
    assert_eq!( res4.is_ok(), true );
    assert_eq!( res4.as_ref().unwrap().is_open, false );

    // -- braces
    let ms5 = MatchStatement::new( 5, true, None, None );
    let res5 = MatchStatement::from( ms5, &SyntaxToken::new( SyntaxTokenType::OpenBrace, String::from( "{" )));
    assert_eq!( res5.is_ok(), true );
    assert_eq!( res5.as_ref().unwrap().labels_complete, true );

    let ms6 = MatchStatement::new( 5, true, None, None );
    let res6 = MatchStatement::from( ms6, &SyntaxToken::new( SyntaxTokenType::CloseBrace, String::from( "}" )));
    assert_eq!( res6.is_ok(), true );
    assert_eq!( res6.as_ref().unwrap().kvps_complete, true );

    // -- kv str
    let ms7 = MatchStatement::new( 5, true, None, None );
    let res7 = MatchStatement::from( ms7, &SyntaxToken::new( SyntaxTokenType::Key, String::from( "key" )));
    assert_eq!( res7.is_ok(), true );
    assert_eq!( res7.as_ref().unwrap().kv_str.len(), 1 );

    let res7_1 = MatchStatement::from( res7.unwrap(), &SyntaxToken::new( SyntaxTokenType::StringValue, String::from( "val" )));
    assert_eq!( res7_1.is_ok(), true );
    assert_eq!( res7_1.as_ref().unwrap().kv_str.len(), 1 );
    assert_eq!( res7_1.as_ref().unwrap().kv_str.get( 0 ).unwrap().key, String::from( "key" ));
    assert_eq!( res7_1.as_ref().unwrap().kv_str.get( 0 ).unwrap().val, Some( String::from( "val" )));
  }
}