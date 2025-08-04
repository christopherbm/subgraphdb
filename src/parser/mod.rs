use crate::tokenize::{ TokenType, token_type, SyntaxToken, SyntaxTokenType };

/* @version 0.3.0 */

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
        tokens.push( SyntaxToken::new( SyntaxTokenType::OpenNode, c.to_string() ));  
      },
      
      TokenType::CloseParen => 
      { 
        let token_opt = parse_syntax_token( &acc, props_mode );
        if token_opt.is_some() 
        { 
          tokens.push( token_opt.unwrap() );
          acc = Vec::new();
        }
        tokens.push(SyntaxToken::new( SyntaxTokenType::CloseNode, c.to_string() )); 
      },
      
      TokenType::OpenBrace => 
      {
        props_mode = true;
        tokens.push( SyntaxToken::new( SyntaxTokenType::OpenBrace, c.to_string() ));
      },

      TokenType::CloseBrace => 
      {
        props_mode = false;
        tokens.push( SyntaxToken::new( SyntaxTokenType::CloseBrace, c.to_string() ));
      },

      TokenType::OpenBracket => { tokens.push( SyntaxToken::new( SyntaxTokenType::OpenEdge, c.to_string() )); },
      TokenType::CloseBracket => 
      {
        let token_opt = parse_syntax_token( &acc, props_mode );
        if token_opt.is_some() 
        { 
          tokens.push( token_opt.unwrap() );
          acc = Vec::new();
        } 
        tokens.push( SyntaxToken::new( SyntaxTokenType::CloseEdge, c.to_string() )); 
      },

      TokenType::Colon => 
      {
        if props_mode  == true 
        {
          acc.push( c.to_string() );
          let token_opt = parse_syntax_token( &acc, props_mode );
          if token_opt.is_some() 
          { 
            tokens.push( token_opt.unwrap() );
            acc = Vec::new();
          }
        }
        else 
        {
          let token_opt = parse_syntax_token( &acc, props_mode );
          if token_opt.is_some() 
          { 
            tokens.push( token_opt.unwrap() );
            acc = Vec::new();
          }
          acc.push( c.to_string() );
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
      TokenType::Hyphen => { tokens.push( SyntaxToken::new( SyntaxTokenType::EdgeDirection, c.to_string() ));},
      TokenType::LT => { tokens.push( SyntaxToken::new( SyntaxTokenType::EdgeDirection, c.to_string() ));},
      TokenType::GT => { tokens.push( SyntaxToken::new( SyntaxTokenType::EdgeDirection, c.to_string() ));},
      TokenType::Pipe => { tokens.push( SyntaxToken::new( SyntaxTokenType::KeywordOr, c.to_string() ));},
      TokenType::Ampersand => { tokens.push( SyntaxToken::new( SyntaxTokenType::KeywordAnd, c.to_string() ));},

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

  if word.to_lowercase() == "create" { return Some( SyntaxToken::new( SyntaxTokenType::KeywordCreate, word ));}
  if word.to_lowercase() == "graph" { return Some( SyntaxToken::new( SyntaxTokenType::KeywordGraph, word ));}
  if word.to_lowercase() == "match" { return Some( SyntaxToken::new( SyntaxTokenType::KeywordMatch, word ));}
  if word.to_lowercase() == "return" { return Some( SyntaxToken::new( SyntaxTokenType::KeywordReturn, word ));}
  if word.to_lowercase() == "where" { return Some( SyntaxToken::new( SyntaxTokenType::KeywordWhere, word ));}
  if word.to_lowercase() == "as" { return Some( SyntaxToken::new( SyntaxTokenType::KeywordAs, word ));}
  if word.to_lowercase() == "from" { return Some( SyntaxToken::new( SyntaxTokenType::KeywordFrom, word ));}
  if word.to_lowercase() == "insert" { return Some( SyntaxToken::new( SyntaxTokenType::KeywordInsert, word ));}
  if word.to_lowercase() == "into" { return Some( SyntaxToken::new( SyntaxTokenType::KeywordInto, word ));}
  if word.to_lowercase() == "starts" { return Some( SyntaxToken::new( SyntaxTokenType::KeywordStarts, word ));}
  if word.to_lowercase() == "ends" { return Some( SyntaxToken::new( SyntaxTokenType::KeywordEnds, word ));}
  if word.to_lowercase() == "with" { return Some( SyntaxToken::new( SyntaxTokenType::KeywordWith, word ));}
  
  if word.len() > 0 && acc[0] == ":" 
  {
    return Some( SyntaxToken::new( SyntaxTokenType::PrimaryLabel, word[1..(word.len())].to_string() ));
  }

  if word.len() > 0 { return Some( SyntaxToken::new( SyntaxTokenType::Label, word )); }

  None
}

fn parse_props_mode ( acc: &Vec<String> ) -> Option<SyntaxToken> 
{
  if acc.len() == 0 { return None; }

  let word = acc.join( "" );

  if acc[acc.len() - 1] == ":" 
  {
    return Some( SyntaxToken::new( SyntaxTokenType::Key, word[0..(word.len() - 1)].to_string() ));
  }
  
  if word.len() > 0 { return Some( SyntaxToken::new( SyntaxTokenType::Value, word ));}
  
  None
}

fn parse_quote_acc ( acc: &Vec<String> ) -> Option<SyntaxToken> 
{
  let word = acc.join( "" );
  if word.len() > 0 
  {
    return Some( SyntaxToken::new( SyntaxTokenType::StringValue, word ));
  }
  None
}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_test () 
  {
    //let tokens: Vec<SyntaxToken> = parse_syntax( "MATCH (n:Stop)" );
    //println!( "{:?}", tokens );

    //let tokens: Vec<SyntaxToken> = parse_syntax( "MATCH (n { mode: 'Rail' })" );
    //println!( "{:?}", tokens );
  }

  #[test]
  fn test_se1 () 
  {
    let tokens: Vec<SyntaxToken> = parse_syntax( "MATCH ()" );
    assert_eq!( tokens.len(), 3 );
  }

    #[test]
  fn test_se2 () 
  {
    let tokens: Vec<SyntaxToken> = parse_syntax( "
    CREATE GRAPH devs
      (alice:Developer)
      (bob:Administrator)
      (chris:Lead)
      (alice)-[:KNOWS]->(bob)
      (alice)-[:KNOWS]->(chris)
      (bob)-[:KNOWS]->(chris)" );
    assert_eq!( tokens.len(), 51 );
  }

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
    
    
    // --
    let tokens12: Vec<SyntaxToken> = parse_syntax( "MATCH (:Movie {title: 'Wall Street'})" );
    assert_eq!( tokens12.len(), 8 );

    println!( "{:?} LEN:{:?}", tokens12, tokens12.len() );
    
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
