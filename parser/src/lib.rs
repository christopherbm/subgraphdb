use tokenize::{ TokenType, token_type, SyntaxToken, SyntaxTokenType, cons_syntax_token };
/*
  Syntax Examples.
    - SE 1:
        - MATCH ()
        - returns all nodes in default graph

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
  let mut quote_acc = Vec::new();
  let mut props_mode = false;
  let mut quote_mode = false;
  for c in content.chars() 
  { 
    if quote_mode == true && token_type( &c ) != TokenType::Quote 
    {
      quote_acc.push( c.to_string() );
      continue;  
    }

    match token_type( &c ) 
    {
      TokenType::Char => { acc.push( c.to_string() ); },
      
      TokenType::Space | TokenType::NewLine => 
      {
        let token_opt = parse_syntax_token( &acc, props_mode );
        if token_opt.is_some() 
        { 
          tokens.push( token_opt.unwrap() );
          acc = Vec::new();
        }
      },
      
      TokenType::OpenParen => 
      { 
        tokens.push( cons_syntax_token( SyntaxTokenType::OpenNode, c.to_string() ));  
      },
      
      TokenType::CloseParen => 
      { 
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
        props_mode = true;
        tokens.push( cons_syntax_token( SyntaxTokenType::OpenBrace, c.to_string() ));
      },

      TokenType::CloseBrace => 
      {
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
        acc.push( c.to_string() ); // push colon        
        let token_opt = parse_syntax_token( &acc, props_mode );
        if token_opt.is_some() 
        { 
          tokens.push( token_opt.unwrap() );
          acc = Vec::new();
        }
      },
      
      TokenType::Quote => 
      { 
        if quote_mode == false { quote_mode = true; }
        else 
        { 
          quote_mode = false; 
          let token_opt = parse_quote_acc( &quote_acc );
          if token_opt.is_some() 
          { 
            tokens.push( token_opt.unwrap() );
            quote_acc = Vec::new();
          }
        }
      },

      TokenType::Comma => { continue; },
      TokenType::Hyphen => { tokens.push( cons_syntax_token( SyntaxTokenType::EdgeDirection, c.to_string() ));},
      TokenType::LT => { tokens.push( cons_syntax_token( SyntaxTokenType::EdgeDirection, c.to_string() ));},
      TokenType::GT => { tokens.push( cons_syntax_token( SyntaxTokenType::EdgeDirection, c.to_string() ));},
      TokenType::Pipe => { tokens.push( cons_syntax_token( SyntaxTokenType::KeywordOr, c.to_string() ));},
      TokenType::Ampersand => { tokens.push( cons_syntax_token( SyntaxTokenType::KeywordAnd, c.to_string() ));},

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
  if props_mode == true { return parse_props_mode( acc ); }

  let word = acc.join( "" );

  if word == ":" { return None; }

  if word.to_lowercase() == "create" { return Some( cons_syntax_token( SyntaxTokenType::KeywordCreate, word ));}
  if word.to_lowercase() == "graph" { return Some( cons_syntax_token( SyntaxTokenType::KeywordGraph, word ));}
  if word.to_lowercase() == "match" { return Some( cons_syntax_token( SyntaxTokenType::KeywordMatch, word ));}
  if word.to_lowercase() == "return" { return Some( cons_syntax_token( SyntaxTokenType::KeywordReturn, word ));}
  if word.to_lowercase() == "where" { return Some( cons_syntax_token( SyntaxTokenType::KeywordWhere, word ));}
  if word.to_lowercase() == "as" { return Some( cons_syntax_token( SyntaxTokenType::KeywordAs, word ));}
  if word.to_lowercase() == "from" { return Some( cons_syntax_token( SyntaxTokenType::KeywordFrom, word ));}
  if word.to_lowercase() == "insert" { return Some( cons_syntax_token( SyntaxTokenType::KeywordInsert, word ));}
  if word.to_lowercase() == "into" { return Some( cons_syntax_token( SyntaxTokenType::KeywordInto, word ));}
  if word.to_lowercase() == "starts" { return Some( cons_syntax_token( SyntaxTokenType::KeywordStarts, word ));}
  if word.to_lowercase() == "ends" { return Some( cons_syntax_token( SyntaxTokenType::KeywordEnds, word ));}
  if word.to_lowercase() == "with" { return Some( cons_syntax_token( SyntaxTokenType::KeywordWith, word ));}

  if word.len() > 0 && acc[acc.len() - 1] == ":" 
  {
    return Some( cons_syntax_token( SyntaxTokenType::Label, word[0..(word.len() - 1)].to_string() ));
  }
  
  if word.len() > 0 && acc[0] == ":" 
  {
    return Some( cons_syntax_token( SyntaxTokenType::PrimaryLabel, word[1..(word.len())].to_string() ));
  }

  if word.len() > 0 { return Some( cons_syntax_token( SyntaxTokenType::Label, word ));}

  None
}

fn parse_props_mode ( acc: &Vec<String> ) -> Option<SyntaxToken> 
{
  if acc.len() == 0 { return None; }

  let word = acc.join( "" );
  
  if acc[acc.len() - 1] == ":" 
  {
    return Some( cons_syntax_token( SyntaxTokenType::Key, word[0..(word.len() - 1)].to_string() ));
  }
  
  if word.len() > 0 { return Some( cons_syntax_token( SyntaxTokenType::Value, word ));}
  
  None
}

fn parse_quote_acc ( acc: &Vec<String> ) -> Option<SyntaxToken> 
{
  let word = acc.join( "" );
  if word.len() > 0 { return Some( cons_syntax_token( SyntaxTokenType::StringValue, word ));}
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
  }

  #[test]
  fn test_parse_syntax_errors () 
  {
    // create devs_graph
    // quotes not closed
    // test keywords in quotes
  }

  #[test]
  fn test_match_syntax () 
  {  
    // ---
    let tokens1: Vec<SyntaxToken> = parse_syntax( "MATCH ()" );
    assert_eq!( tokens1.len(), 3 );

    
    // ---
    let tokens2: Vec<SyntaxToken> = parse_syntax( "MATCH () FROM devs;" );
    assert_eq!( tokens2.len(), 5 );
    
    
    // ---
    let tokens3: Vec<SyntaxToken> = parse_syntax( "MATCH (n)" );
    assert_eq!( tokens3.len(), 4 );

    
    // ---
    let tokens4: Vec<SyntaxToken> = parse_syntax( "
      MATCH (n)
      FROM devs
      RETURN n.name
    ");
    assert_eq!( tokens4.len(), 8 );

    
    // ---
    let tokens5: Vec<SyntaxToken> = parse_syntax( "MATCH (n:Stop)" );
    assert_eq!( tokens5.len(), 5 );

    
    // -- 
    let tokens6: Vec<SyntaxToken> = parse_syntax( "MATCH (n { mode: 'Rail' })" );
    assert_eq!( tokens6.len(), 8 );


    // -- 
    let tokens7: Vec<SyntaxToken> = parse_syntax( "MATCH (n:(TrainStation & BusStation))" );
    assert_eq!( tokens7.len(), 9 );
    
    
    // -- 
    let tokens8: Vec<SyntaxToken> = parse_syntax( "MATCH (n:(TrainStation | BusStation))" );
    assert_eq!( tokens8.len(), 9 );


    // -- 
    let tokens9: Vec<SyntaxToken> = parse_syntax( "MATCH (n:(TrainStation & BusStation) | StationGroup)" );
    assert_eq!( tokens9.len(), 11 );


    // -- 
    let tokens10: Vec<SyntaxToken> = parse_syntax( "MATCH (n:Station WHERE n.name STARTS WITH 'Preston') RETURN n" );
    assert_eq!( tokens10.len(), 12 );


    // -- 
    let tokens11: Vec<SyntaxToken> = parse_syntax( "MATCH (n:Station WHERE n.name ENDS WITH 'Preston') RETURN n" );
    assert_eq!( tokens11.len(), 12 );
    
    
    //println!( "{:?} LEN:{:?}", tokens10, tokens10.len() );
    
    // 
    // 
    // MATCH (n:Station WHERE n.name ENDS WITH 'Preston') RETURN n
    /*
    let _query_string = "
      MATCH (n:Developer)
      FROM devs
      RETURN n AS Developer
    ";
    */
  }

  #[test]
  fn test_edge_syntax () 
  {
    //println!( "--------------------------------" );
    let tokens = parse_syntax( "(keanu)-[:KNOWS]->(carrie)" );
    //for token in tokens.iter() { println!( "{:?}", token ); }
    //println!( "--------------------------------" );
    assert_eq!( tokens.len(), 12 );

    // -- (should match any relationship)
    // -[r]-
    // -[r]->
    // -[:CALLS_AT]->
    // -[{ distance: 0.24, duration: 'PT4M' }]->
    // -[r WHERE time() + duration(r.duration) < time('22:00') ]->
  }

  // test paths
  // ()
  // (s)--(e)
  // (:Station)--()<--(m WHERE m.departs > time('12:00'))-->()-[:NEXT]->(n)

  #[test]
  fn test_properties_syntax () 
  {
    let query_string = "(florentin:Person { name: 'Florentin', age: 16 })";
    let tokens = parse_syntax( query_string );
    assert_eq!( tokens.len(), 10 );

    // ---
    let tokens1 = parse_syntax( "(veselin:Person { name: 'Veselin', age: 20, ratings: [5.0] })" );
    assert_eq!( tokens1.len(), 14 );
    
    // ---
    let tokens2 = parse_syntax( "(veselin:Person { name: 'Veselin', age: '20', ratings: ['5.0'] })" );
    assert_eq!( tokens2.len(), 14 );

    // ---
    let tokens3 = parse_syntax( "
      (hobbit:Book 
        { 
          name: 'The Hobbit', 
          isbn: 1234, 
          numberOfPages: 310, 
          ratings: [1.0, 2.0, 3.0, 4.5] 
        }
      )," );
    assert_eq!( tokens3.len(), 19 );

    // ---
    let tokens4 = parse_syntax( "(frankenstein:Book { name: 'Frankenstein', isbn: 4242, price: 19.99 })," );
    //for token in tokens4.iter() { println!( "{:?}", token ); }
    assert_eq!( tokens4.len(), 12 );
  }
}
