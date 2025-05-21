use tokenize::{ TokenType, token_type, SyntaxToken, SyntaxTokenType, cons_syntax_token };
/*
  CREATE
    (florentin:Person { name: 'Florentin', age: 16 }),
    (adam:Person { name: 'Adam', age: 18 }),
    (veselin:Person { name: 'Veselin', age: 20, ratings: [5.0] }),
    (hobbit:Book { name: 'The Hobbit', isbn: 1234, numberOfPages: 310, ratings: [1.0, 2.0, 3.0, 4.5] }),
    (frankenstein:Book { name: 'Frankenstein', isbn: 4242, price: 19.99 }),

    (florentin)-[:KNOWS { since: 2010 }]->(adam),
    (florentin)-[:KNOWS { since: 2018 }]->(veselin),
    (florentin)-[:READ { numberOfPages: 4 }]->(hobbit),
    (florentin)-[:READ { numberOfPages: 42 }]->(hobbit),
    (adam)-[:READ { numberOfPages: 30 }]->(hobbit),
    (veselin)-[:READ]->(frankenstein)

  CREATE
    (alice:Developer {name:'Alice', age: 38, eyes: 'Brown'}),
    (bob:Administrator {name: 'Bob', age: 25, eyes: 'Blue'}),
    (charlie:Administrator {name: 'Charlie', age: 53, eyes: 'Green'}),
    (daniel:Adminstrator {name: 'Daniel', age: 54, eyes: 'Brown'}),
    (eskil:Designer {name: 'Eskil', age: 41, eyes: 'blue', likedColors: ['Pink', 'Yellow', 'Black']}),
    (alice)-[:KNOWS]->(bob),
    (alice)-[:KNOWS]->(charlie),
    (bob)-[:KNOWS]->(daniel),
    (charlie)-[:KNOWS]->(daniel),
    (bob)-[:MARRIED]->(eskil)

  CREATE
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

  CREATE
    (alice:Developer {name:'Alice', age: 38, eyes: 'Brown'}),
    (bob:Administrator {name: 'Bob', age: 25, eyes: 'Blue'}),
    (charlie:Administrator {name: 'Charlie', age: 53, eyes: 'Green'}),
    (daniel:Adminstrator {name: 'Daniel', age: 54, eyes: 'Brown'}),
    (eskil:Designer {name: 'Eskil', age: 41, eyes: 'blue', likedColors: ['Pink', 'Yellow', 'Black']}),
    (alice)-[:KNOWS]->(bob),
    (alice)-[:KNOWS]->(charlie),
    (bob)-[:KNOWS]->(daniel),
    (charlie)-[:KNOWS]->(daniel),
    (bob)-[:MARRIED]->(eskil)

  CREATE
    (copenhagen:TrainStation {latitude: 55.672874, longitude: 12.564590, city: 'Copenhagen'}),
    (malmo:Office {latitude: 55.611784, longitude: 12.994341, city: 'Malmö'}),
    (copenhagen)-[:TRAVEL_ROUTE]->(malmo)
*/

#[ allow( unused_assignments )]
pub fn parse_syntax ( content: &str ) -> Vec<SyntaxToken>
{
  let mut tokens = Vec::new();
  let mut acc = Vec::new();
  let mut props_mode = false;
  let mut quote_mode = false;
  for c in content.chars() 
  { 
    match token_type( &c ) 
    {
      TokenType::Char => { acc.push( c.to_string() ); },
      
      TokenType::Space | TokenType::NewLine => 
      {
        if quote_mode == true 
        { 
          acc.push( c.to_string() );
          continue;   
        }
        let token_opt = parse_syntax_token( &acc, props_mode );
        if token_opt.is_some() 
        { 
          tokens.push( token_opt.unwrap() );
          acc = Vec::new();
        }
      },
      
      TokenType::OpenParen => 
      { 
        if quote_mode == true { continue; }
        tokens.push( cons_syntax_token( SyntaxTokenType::OpenNode, c.to_string() ));  
      },
      
      TokenType::CloseParen => 
      { 
        if quote_mode == true { continue; }
        let token_opt = parse_syntax_token( &acc, props_mode );
        if token_opt.is_some() 
        { 
          tokens.push( token_opt.unwrap() );
          acc = Vec::new();
        }
        tokens.push(cons_syntax_token( SyntaxTokenType::CloseNode, c.to_string() )); 
      },
      
      TokenType::OpenBrace => 
      {
        if quote_mode == true { continue; }
        props_mode = true;
        tokens.push( cons_syntax_token( SyntaxTokenType::OpenBrace, c.to_string() ));
      },

      TokenType::CloseBrace => 
      {
        if quote_mode == true { continue; }
        props_mode = false;
        tokens.push( cons_syntax_token( SyntaxTokenType::CloseBrace, c.to_string() ));
      },

      TokenType::OpenBracket => { tokens.push( cons_syntax_token( SyntaxTokenType::OpenEdge, c.to_string() )); },
      TokenType::CloseBracket => 
      {
        let token_opt = parse_syntax_token( &acc, props_mode );
        if token_opt.is_some() 
        { 
          tokens.push( token_opt.unwrap() );
          acc = Vec::new();
        } 
        tokens.push( cons_syntax_token( SyntaxTokenType::CloseEdge, c.to_string() )); 
      },

      TokenType::Colon => 
      {
        if quote_mode == true { continue; }
        let token_opt = parse_syntax_token( &acc, props_mode );
        if token_opt.is_some() 
        { 
          tokens.push( token_opt.unwrap() );
          acc = Vec::new();
        }
        acc.push( c.to_string() ); // push colon
      },
      
      TokenType::Quote => 
      { 
        if quote_mode == false { quote_mode = true; }
        else 
        { 
          quote_mode = false; 
          let token_opt = parse_syntax_token( &acc, props_mode );
          if token_opt.is_some() 
          { 
            tokens.push( token_opt.unwrap() );
            acc = Vec::new();
          }
        }
      },

      TokenType::Comma => { continue; },
      TokenType::Hyphen => { tokens.push( cons_syntax_token( SyntaxTokenType::EdgeDirection, c.to_string() ));},
      TokenType::LT => { tokens.push( cons_syntax_token( SyntaxTokenType::EdgeDirection, c.to_string() ));},
      TokenType::GT => { tokens.push( cons_syntax_token( SyntaxTokenType::EdgeDirection, c.to_string() ));},

      _ => {}
    }
  }

  if acc.len() > 0 
  {
    let token_opt = parse_syntax_token( &acc, props_mode );
    if token_opt.is_some() 
    { 
      tokens.push( token_opt.unwrap() );
      acc = Vec::new();
    } 
  }

  tokens
}

fn parse_syntax_token ( acc: &Vec<String>, props_mode: bool ) -> Option<SyntaxToken>
{
  if props_mode == true { return parse_props_mode( acc );}

  let word = acc.join( "" );
  if word.to_lowercase() == "create" { return Some( cons_syntax_token( SyntaxTokenType::KeywordCreate, word )); }
  if word.to_lowercase() == "graph" { return Some( cons_syntax_token( SyntaxTokenType::KeywordGraph, word )); }
  if word.to_lowercase() == "match" { return Some( cons_syntax_token( SyntaxTokenType::KeywordMatch, word )); }
  if word.to_lowercase() == "return" { return Some( cons_syntax_token( SyntaxTokenType::KeywordReturn, word )); }
  if word.to_lowercase() == "where" { return Some( cons_syntax_token( SyntaxTokenType::KeywordWhere, word )); }
  if word.to_lowercase() == "as" { return Some( cons_syntax_token( SyntaxTokenType::KeywordAs, word )); }
  if word.to_lowercase() == "from" { return Some( cons_syntax_token( SyntaxTokenType::KeywordFrom, word )); }
  if word.to_lowercase() == "insert" { return Some( cons_syntax_token( SyntaxTokenType::KeywordInsert, word )); }
  if word.to_lowercase() == "into" { return Some( cons_syntax_token( SyntaxTokenType::KeywordInto, word )); }

  if word.len() > 0 && acc[0] == ":" 
  {
    let mut pchars = word.chars();
    pchars.next();
    return Some( cons_syntax_token( SyntaxTokenType::PrimaryLabel, pchars.as_str().to_string() ));
  }

  if word.len() > 0 { return Some( cons_syntax_token( SyntaxTokenType::Label, word ));}

  None
}

fn parse_props_mode ( acc: &Vec<String> ) -> Option<SyntaxToken> 
{
  if acc.len() == 0 { return None; }

  let word = acc.join( "" );
  
  if acc[0] == ":" 
  {
    let mut pchars = word.chars();
    pchars.next();
    return Some( cons_syntax_token( SyntaxTokenType::Value, pchars.as_str().to_string() ));
  }
  
  if word.len() > 0 { return Some( cons_syntax_token( SyntaxTokenType::Key, word ));}
  
  None
}


#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_parse_syntax () 
  {
    let tokens = parse_syntax( "CREATE GRAPH movies\n" );
    assert_eq!( tokens.len(), 3 );

    // ---
    let tokens1 = parse_syntax( "CREATE GRAPH movies " );
    assert_eq!( tokens1.len(), 3 );

    // --- 
    let tokens2 = parse_syntax( "(keanu)" );
    assert_eq!( tokens2.len(), 3 );

    // ---
    let tokens3 = parse_syntax( "(keanu:Person)" );
    assert_eq!( tokens3.len(), 4 );
   
    // ---
    let tokens4 = parse_syntax( "({name:'Keanu Reeves'})," );
    assert_eq!( tokens4.len(), 6 );

    // ---
    let tokens5 = parse_syntax( "(keanu:Person {name:'Keanu Reeves'})," );
    assert_eq!( tokens5.len(), 8 );

    // ---
    let tokens6 = parse_syntax( "CREATE GRAPH movies" );
    assert_eq!( tokens6.len(), 3 );
    

    // ---
    //
    //let tokens6 = parse_syntax( "-[:KNOWS]->" );
    
    //let tokens6 = parse_syntax( "(keanu)-[:KNOWS]->(carrie)" );
    //let tokens4 = parse_syntax( "(keanu:Person {name:'Keanu Reeves', age:58, nationality:'Canadian'})," );
    //println!( "{:?}", tokens6 );
  }

  #[test]
  fn test_parse_syntax_errors () 
  {
    // create devs_graph
    // quotes not closed
    // test keywords in quotes
  }
}
