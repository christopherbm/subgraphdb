use std::collections::VecDeque;
use cmd::{ 
  BracketStatement, CreateStatement, EdgeStatement, MatchStatement, NodeRefStatement, NodeStatement, ParenStatement, 
  ReadClause };
use cmd::transaction::Transaction;
use common::DirectionType;
use parser::{ parse_syntax };
use tokenize::{ SyntaxToken, SyntaxTokenType };
use datagramv2::grams::{ Label, UUID };
use utils::cons_uuid;

/*
- For every N Transactions, a checkpoint transaction will be called that organizes the data.
- Certain actions will force a checkpoint. These should be isolated, but still part of a transaction.
- reverse add things that can be closed and then reverse these on the way out
*/

// ---------------------------------------------------------------------------------------------------------------------

pub fn process_query ( query: &str, build_id: UUID, nickname: Label ) -> Transaction 
{
  let mut transaction_builder = TransactionBuilder::new(); 
  let tokens = parse_syntax( query );
  for token in tokens.into_iter() { transaction_builder.add_token( token ); }
  
  transaction_builder.close( build_id, nickname )
}


// ---------------------------------------------------------------------------------------------------------------------
#[derive( Debug )]
pub struct TransactionBuilder 
{ 
  pub current_order: u16,
  pub err_state: Option<String>,

  // create
  pub create_statement: Option<CreateStatement>,

  // nodes and edges
  pub paren_statements: VecDeque<ParenStatement>,
  pub bracket_statements: VecDeque<BracketStatement>,

  // read clause
  pub read_clause_order: Option<u16>,
  pub match_statements: VecDeque<MatchStatement>
}

impl TransactionBuilder
{
  pub fn new () -> TransactionBuilder 
  { 
    TransactionBuilder 
    { 
      current_order: 0, 
      err_state: None, 
      
      create_statement: None,

      paren_statements: VecDeque::new(),
      bracket_statements: VecDeque::new(),

      read_clause_order: None, 
      match_statements: VecDeque::new() 
    }
  }

  pub fn close ( &mut self, build_id: UUID, nickname: Label ) -> Transaction 
  {
    let mut transaction = Transaction::new( build_id, nickname, self.current_order );
    
    if self.create_statement.is_some() 
    {
      transaction.create_statement = Some( self.create_statement.as_ref().unwrap().clone() );
    }

    if self.match_statements.len() > 0 && self.read_clause_order.is_some()
    {
      let mut read_clause = ReadClause::new( self.read_clause_order.unwrap() );
      while self.match_statements.len() > 0 
      {
        let stmt_opt = self.match_statements.pop_front();
        if stmt_opt.is_some() { read_clause.add_match_statement( stmt_opt.unwrap() ); }
      }
      transaction.read_clause = Some( read_clause );
    }

    for paren in self.paren_statements.iter() 
    {
      if paren.is_ref() == true 
      {
        transaction.node_ref_statements.push( 
          NodeRefStatement::new( 
            paren.order, 
            None, 
            paren.transaction_label.clone().unwrap() ));   
      }
      else 
      {
        transaction.node_statements.push( 
          NodeStatement::new( 
            cons_uuid(), 
            paren.order, 
            paren.transaction_label.clone(), 
            paren.primary_label.clone().unwrap() ));
      }
    }
    
    for bracket in self.bracket_statements.iter() 
    {
      transaction.edge_statements.push( 
        EdgeStatement::new(
          cons_uuid(), 
          bracket.order, 
          bracket.transaction_label.clone(), 
          bracket.primary_label.clone().unwrap() ));
    }

    if self.err_state.is_some() { transaction.err_state = Some( self.err_state.as_ref().unwrap().clone() ); }
    transaction
  }
}

impl TransactionBuilder 
{
  pub fn close_statement ( &mut self, token: &SyntaxToken ) 
  {
    if self.create_statement.is_some() == true && self.create_statement.as_ref().unwrap().is_open == true
    {
      self.create_statement.as_mut().unwrap().is_open = false;
      return;
    }

    let match_res = self.find_open_match_statement();
    if match_res.is_some() 
    {
      self.match_statements.get_mut( match_res.unwrap() ).unwrap().is_open = false;
      return;
    }

    let paren_res = self.find_open_paren_statement();
    if paren_res.is_some() 
    {
      self.paren_statements.get_mut( paren_res.unwrap() ).unwrap().is_open = false;
      return;
    }

    let bracket_res = self.find_open_bracket_statement();
    if bracket_res.is_some() 
    {
      self.bracket_statements.get_mut( bracket_res.unwrap() ).unwrap().is_open = false;
      return;
    }

    self.err_state = Some( String::from( "Syntax Error: Closing Statement" ));
  }

  pub fn add_token ( &mut self, token: SyntaxToken ) 
  {
    //println!( "{:?}", token );

    match token.token_type 
    {
      SyntaxTokenType::KeywordMatch => { self.add_match_token( token ); },
      SyntaxTokenType::KeywordCreate => { self.add_create( &token ); },
      SyntaxTokenType::OpenNode => { self.add_open_node( &token ); },
      SyntaxTokenType::CloseNode => { self.add_close_node( &token ); },
      SyntaxTokenType::Label => { self.add_x_label( token ); },
      SyntaxTokenType::PrimaryLabel => { self.add_x_label( token ); },
      SyntaxTokenType::EdgeDirection => 
      {
        if self.find_open_bracket_statement().is_some() { self.try_update_bracket_statements( &token ); }
        else { self.add_bracket( &token ); }
      },
      _ => {}
    }
  }

  pub fn add_match_token ( &mut self, token: SyntaxToken ) 
  {
    if token.token_type != SyntaxTokenType::KeywordMatch 
    {
      self.err_state = Some( String::from( "Syntax Error: Match Clause" ));
      return;
    }
    
    if self.read_clause_order.is_none() 
    { 
      self.read_clause_order = Some( self.current_order );
      self.current_order += 1;
    }
    self.match_statements.push_back( MatchStatement::new( self.current_order, true, None, None, None ));
    self.current_order += 1;
  }

  pub fn add_open_node ( &mut self, token: &SyntaxToken ) 
  { 
    self.close_statement( token );

    if token.token_type != SyntaxTokenType::OpenNode 
    {
      self.err_state = Some( String::from( "Syntax Error: Open Paren" ));
      return;
    }

    self.paren_statements.push_back( ParenStatement::new( self.current_order, true, None, None ));
    self.current_order += 1; 
  }
  
  pub fn add_close_node ( &mut self, token: &SyntaxToken ) { self.close_statement( token ); }
  
  pub fn add_x_label ( &mut self, token: SyntaxToken ) 
  {
    if self.try_update_paren_statements( &token ) == true { return; }
    if self.try_update_bracket_statements( &token ) == true { return; }
    if self.try_update_match_statements( &token ) == true { return; }

    if self.create_statement.is_some() && self.create_statement.as_ref().unwrap().is_open == true 
    {
      if token.token_type == SyntaxTokenType::Label 
      {
        let new_create = CreateStatement::from( &self.create_statement.as_ref().unwrap(), token );
        if new_create.is_ok() 
        {
          self.create_statement = Some( new_create.unwrap() );
          return;
        }
      }
      self.err_state = Some( String::from( "Syntax Error: Create Statement Label" ));
      return;
    }

    self.err_state = Some( String::from( "Syntax Error: Label" ));
  }

  pub fn add_create ( &mut self, _token: &SyntaxToken ) 
  {
    if self.create_statement.is_none() == true 
    {
      self.create_statement = Some( CreateStatement::new( self.current_order, None ));
      self.current_order += 1;
      return;
    }
    self.err_state = Some( String::from( "Syntax Error: Create" ));
  }

  pub fn add_bracket ( &mut self, token: &SyntaxToken ) 
  {
    self.bracket_statements.push_back( 
      BracketStatement::new( self.current_order, true, None, None, TransactionBuilder::token_dir_type( token )));
    self.current_order += 1;
  }

  pub fn try_update_paren_statements ( &mut self, token: &SyntaxToken ) -> bool 
  {
    let paren_res = self.find_open_paren_statement();
    if paren_res.is_some() 
    {
      let remove_res = self.paren_statements.remove( *paren_res.as_ref().unwrap() );
      if remove_res.is_some() 
      {
        let replace_res = ParenStatement::from( remove_res.unwrap(), token );
        if replace_res.is_ok() 
        {
          self.paren_statements.insert( *paren_res.as_ref().unwrap(), replace_res.unwrap() );
          return true;
        }
      }
    }
    false
  }

  pub fn try_update_bracket_statements ( &mut self, token: &SyntaxToken ) -> bool 
  {
    let bracket_res = self.find_open_bracket_statement();
    if bracket_res.is_some() 
    {
      let remove_res = self.bracket_statements.remove( *bracket_res.as_ref().unwrap() );
      if remove_res.is_some() 
      {
        let replace_res = BracketStatement::from( remove_res.unwrap(), token );
        if replace_res.is_ok() 
        {
          self.bracket_statements.insert( *bracket_res.as_ref().unwrap(), replace_res.unwrap() );
          return true;
        }
      }
    }
    false
  }
  
  pub fn try_update_match_statements ( &mut self, token: &SyntaxToken ) -> bool
  {
    let match_res = self.find_open_match_statement();
    if match_res.is_some() 
    {
      let remove_res = self.match_statements.remove( *match_res.as_ref().unwrap() );
      if remove_res.is_some() 
      {
        let replace_res = MatchStatement::from( remove_res.unwrap(), token );
        if replace_res.is_ok() 
        {
          self.match_statements.insert( *match_res.as_ref().unwrap(), replace_res.unwrap() );
          return true;
        }
      }
    }
    false
  }
}

impl TransactionBuilder 
{
  pub fn find_open_match_statement ( &self ) -> Option<usize>
  {
    for ( i, stmt ) in self.match_statements.iter().enumerate()
    {
      if stmt.is_open == true 
      {
        return Some( i ); 
      }
    }
    None
  }

  pub fn find_open_paren_statement ( &self ) -> Option<usize>
  {
    for ( i, stmt ) in self.paren_statements.iter().enumerate()
    {
      if stmt.is_open == true 
      {
        return Some( i ); 
      }
    }
    None
  }

  pub fn find_open_bracket_statement ( &self ) -> Option<usize>
  {
    for ( i, stmt ) in self.bracket_statements.iter().enumerate()
    {
      if stmt.is_open == true { return Some( i ); }
    }
    None
  }

  pub fn token_dir_type ( token: &SyntaxToken ) -> DirectionType 
  {
    if token.token_type == SyntaxTokenType::EdgeDirection 
    {
      if token.val == "-" { return DirectionType::Bidirectional; }
      if token.val == "<" { return DirectionType::Left; }
      if token.val == ">" { return DirectionType::Right; }
    }
    DirectionType::Undirected
  }
}
// ---------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests 
{
  use super::*;
  use common::LABEL_BYTES;

  fn build_id () -> UUID { UUID::new( String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" )).unwrap() }
  fn db_nickname () -> Label { Label::new( String::from( "devs" ), LABEL_BYTES ).unwrap() }

  #[test]
  fn test_se1 () 
  {
    let t = process_query( "MATCH ()", build_id(), db_nickname() );
    assert_eq!( t.read_clause.is_some(), true );
    assert_eq!( t.read_clause.as_ref().unwrap().order, 0 );
    assert_eq!( t.read_clause.as_ref().unwrap().match_statements.len(), 1 );

    let stmt_res = t.read_clause.as_ref().unwrap().match_statements.get( 0 );
    assert_eq!( stmt_res.is_some(), true );
    assert_eq!( stmt_res.as_ref().unwrap().order, 1 );
    assert_eq!( stmt_res.as_ref().unwrap().is_open, false );
  }

  #[test]
  fn test_se2 () 
  {
    let query_string = "
      CREATE GRAPH devs
        (alice:Developer)
        (bob:Administrator)
        (chris:Lead)
        
        (alice)-[:KNOWS]-(bob)
        (alice)-[:KNOWS]-(chris)
        (bob)-[:KNOWS]-(chris)
    ";

    let t = process_query( &query_string, build_id(), db_nickname() );
    //println!( "{}", t );

    assert_eq!( t.create_statement.is_some(), true );
    assert_eq!( t.node_statements.len(), 3 );
    assert_eq!( t.node_ref_statements.len(), 6 );
    assert_eq!( t.edge_statements.len(), 3 );
  }

  #[test]
  fn test_next_statements () 
  {
    let query_string = "
      CREATE GRAPH devs
        (alice:Developer)
        (bob:Administrator)
        (chris:Lead)
        
        (alice)-[:KNOWS]-(bob)
        (alice)-[:KNOWS]-(chris)
        (bob)-[:KNOWS]-(chris)
    ";

    let t = process_query( &query_string, build_id(), db_nickname() );
    println!( "{}", t );

    assert_eq!( t.create_statement.is_some(), true );
    assert_eq!( t.node_statements.len(), 3 );
    assert_eq!( t.node_ref_statements.len(), 6 );
    assert_eq!( t.edge_statements.len(), 3 );

    assert_eq!( t.next_node_statement( 1 ).is_some(), true );
    assert_eq!( t.next_node_statement( 2 ).is_some(), true );
    assert_eq!( t.next_node_statement( 3 ).is_some(), true );

    assert_eq!( t.next_node_statement( 4 ).is_some(), false );
    assert_eq!( t.next_edge_statement( 4 ).is_some(), false );

    assert_eq!( t.next_ref_statement( 4 ).is_some(), true );
    assert_eq!( t.next_edge_statement( 5 ).is_some(), true );
    assert_eq!( t.next_ref_statement( 6 ).is_some(), true );

    assert_eq!( t.next_ref_statement( 7 ).is_some(), true );
    assert_eq!( t.next_edge_statement( 8 ).is_some(), true );
    assert_eq!( t.next_ref_statement( 9 ).is_some(), true );

    assert_eq!( t.next_ref_statement( 10 ).is_some(), true );
    assert_eq!( t.next_edge_statement( 11 ).is_some(), true );
    assert_eq!( t.next_ref_statement( 12 ).is_some(), true );

    assert_eq!( t.query_order, 13 );
  }

  #[test]
  fn test_query_errors () 
  {
    /*
      CREATE db name;
      CREATE;
      CRETE graph;

      ()
      (alice)--[:KNOWS]->(bob)
      (alice)--[:KNOWS]->()
      (alice)-:KNOWS->(bob)
      (alice)<[:KNOWS]>(bob)
    */
    let _qs = "";
  }
}
