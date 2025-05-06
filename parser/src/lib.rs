use tokenize::{ TokenType, token_type };

/*
CREATE movies
    (keanu:Person {name:'Keanu Reeves', age:58, nationality:'Canadian'}),
    (carrie:Person {name:'Carrie Anne Moss', age:55, nationality:'American'}),
    (liam:Person {name:'Liam Neeson', age:70, nationality:'Northern Irish'}),
    (guy:Person {name:'Guy Pearce', age:55, nationality:'Australian'}),
    (kathryn:Person {name:'Kathryn Bigelow', age:71, nationality:'American'}),
    (jessica:Person {name:'Jessica Chastain', age:45, address:''}),
    (theMatrix:Movie {title:'The Matrix'}),
    (keanu)-[:KNOWS]->(carrie),
    (keanu)-[:KNOWS]->(liam),
    (keanu)-[:KNOWS]->(kathryn),
    (kathryn)-[:KNOWS]->(jessica),
    (carrie)-[:KNOWS]->(guy),
    (liam)-[:KNOWS]->(guy),
    (keanu)-[:ACTED_IN]->(theMatrix),
    (carrie)-[:ACTED_IN]->(theMatrix)
*/

#[derive( PartialEq, Debug )]
enum ParsingStep 
{ 
  Init,
  BuildingWord,
  BuildingStatement,
}

#[derive( Debug )]
struct ParseState
{ 
  pub step: ParsingStep,
  pub word_opt: Option<String>,
  pub statement_opt: Option<Statement>,
  pub seq: Vec<Statement>,
}

fn cons_parse_state () -> ParseState 
{
  ParseState {
    step: ParsingStep::Init,
    word_opt: None,
    statement_opt: None,
    seq: Vec::new(),
  }
}

#[derive( PartialEq, Debug )]
enum StatementType 
{ 
  CreateGraph
}

#[derive( Debug )]
struct Statement
{ 
  pub statement_type: StatementType,
}

pub fn parse ( content: &str ) {}

pub fn parse_actual ( content: &str ) -> ParseState
{
  let mut parse_state = cons_parse_state();
  for c in content.chars() 
  {
    match token_type( &c ) 
    {
      TokenType::NewLine => { /* ignore */ },

      TokenType::Char => { parse_char( &c, &mut parse_state )},

      TokenType::Space => { parse_space( &c, &mut parse_state )},

      TokenType::OpenParen => { parse_open_paren( &c, &mut parse_state )},

      _ => {}
    }
  }
  parse_state
}

pub fn parse_char ( c: &char, state: &mut ParseState )
{
  match state.step 
  {
    ParsingStep::Init => 
    {
      state.step = ParsingStep::BuildingWord;
      state.word_opt = Some( c.to_string() );
    },
    
    ParsingStep::BuildingWord => 
    {
      state.word_opt = Some( state.word_opt.as_mut().unwrap().to_owned() + &c.to_string() );
    },

    _ => {}
  }
}

pub fn parse_space ( c: &char, state: &mut ParseState ) 
{
  match state.step 
  {    
    ParsingStep::BuildingWord => { parse_statement( c, state ); },

    _ => {}
  }
}

pub fn parse_open_paren ( _c: &char, state: &mut ParseState )
{

}

pub fn parse_statement ( c: &char, state: &mut ParseState ) 
{
  match state.step 
  {    
    ParsingStep::BuildingWord => 
    {
      let is_stmt = is_statement_keyword( state.word_opt.as_ref().unwrap() );
      if is_stmt.is_some()
      {
        println!( "{:?}", is_stmt );
        state.step = ParsingStep::BuildingStatement;
      }
    },

    _ => {}
  }
}

fn is_statement_keyword ( word: &str ) -> Option<StatementType>
{
  if word.to_lowercase() == "create" { return Some( StatementType::CreateGraph );}
  None
}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_parse () 
  {
    //assert_eq!(result, 4);
  }

  #[test]
  fn test_parse_actual () 
  {
    let state = parse_actual( "CREATE" );
    assert_eq!( state.word_opt.is_some(), true );
    assert_eq!( state.word_opt.clone().unwrap(), String::from( "CREATE" ));
    assert_eq!( state.word_opt.clone().unwrap().to_lowercase(), String::from( "create" ));

    // ---
    let state1 = parse_actual( "CREATE " );
    //println!( "{:?}", state );
  }
}
