pub mod transaction;

use common::{ DirectionType, KeyValString };
use tokenize::{ SyntaxToken, SyntaxTokenType };
use utils::cons_uuid;

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
  pub transaction_label: Option<String>,
  pub primary_label: Option<String>,
  pub kv_str: Option<Vec<KeyValString>>,
}
impl MatchStatement 
{
  pub fn new ( 
    order: u16, is_open: bool, transaction_label: Option<String>, primary_label: Option<String>, 
    kv_str: Option<Vec<KeyValString>> ) -> MatchStatement 
  {
    MatchStatement { 
      order: order, 
      is_open: is_open, 
      transaction_label: transaction_label, 
      primary_label: primary_label, 
      kv_str: kv_str 
    }
  }

  pub fn from ( stmt: MatchStatement, token: &SyntaxToken ) -> Result<MatchStatement, String> 
  {
    match token.token_type 
    {
      SyntaxTokenType::Label => 
      {
        return Ok(
          MatchStatement { 
            order: stmt.order, 
            is_open: stmt.is_open, 
            transaction_label: Some( token.val.clone() ), 
            primary_label: stmt.primary_label, 
            kv_str: stmt.kv_str 
          }
        )
      },
      SyntaxTokenType::PrimaryLabel => 
      {
        return Ok(
          MatchStatement { 
            order: stmt.order, 
            is_open: stmt.is_open, 
            transaction_label: stmt.transaction_label, 
            primary_label: Some( token.val.clone() ), 
            kv_str: stmt.kv_str 
          }
        )
      },
      
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
      },
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
      },
      
      SyntaxTokenType::EdgeDirection => 
      {
        if token.val == "-" { return Ok( stmt ); }
      },

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