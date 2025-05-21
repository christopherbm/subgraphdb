use cmd::{ Transaction, CreateGraph, CreateNode, CreateEdge, CreateNodeRef };
use parser::{ parse_syntax };
use tokenize::{ SyntaxToken, SyntaxTokenType };
use common::{ DirectionType };

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


<Transaction>
  <DB is known and valid for this example. />
  <CreateGraph uuid 0 name="movies" />
  <InsertNode uuid 0 (keanu) Person IndexedProps {...} />
  <InsertNode uuid 1 (carrie) Person IndexedProps {...} />
  <InsertNode uuid 2 (liam) Person IndexedProps {...} />
  <InsertNode uuid 3 (guy) Person IndexedProps {...} />
  <InsertNode uuid 4 (kathryn) Person IndexedProps {...} />
  <InsertNode uuid 5 (jessica) Person IndexedProps {...} />
  <InsertNode uuid 6 (theMatrix) Movie IndexedProps {...} />

  <InsertEdge uuid 0 (keanu) KNOWS (carrie) />
  <InsertEdge uuid 1 (keanu) KNOWS (liam) />
  <InsertEdge uuid 2 (keanu) KNOWS (kathryn) />
  <InsertEdge uuid 3 (kathryn) KNOWS (jessica) />
  <InsertEdge uuid 4 (carrie) KNOWS (guy) />
  <InsertEdge uuid 5 (liam) KNOWS (guy) />
  <InsertEdge uuid 6 (keanu) ACTED-IN (theMatrix) />
  <InsertEdge uuid 7 (carrie) ACTED-IN (theMatrix) />
  
  <InsertNodeUnindexedProps {...} />
</Transaction>



----------------

- For every N Transactions, a checkpoint transaction will be called that organizes the data.
- Certain actions will force a checkpoint. These should be isolated, but still part of a transaction.

MATCH (p:Person) FROM movies RETURN p LIMIT 5
MATCH (p:Person) RETURN p LIMIT 5

*/
//pub fn create_sf_db () {}
//pub fn init_sf_db () {}

pub fn process_query ( query: &str ) -> Transaction
{
  let tokens:Vec<SyntaxToken> = parse_syntax( query );  
  let mut transaction: Transaction = Transaction::new();
  let mut partial: Vec<SyntaxToken> = Vec::new();
  for token in tokens.into_iter() 
  {
    match token.token_type 
    {
      SyntaxTokenType::OpenNode => 
      {
        if partial.len() > 0 
        { 
          processs_partial( &mut transaction, partial );
          partial = Vec::new();
          partial.push( token );
        }
        else { partial.push( token ); }
      },

      SyntaxTokenType::CloseNode => 
      {
        partial.push( token );
        processs_partial( &mut transaction, partial );
        partial = Vec::new();

        process_for_edge( &mut transaction );
      },

      SyntaxTokenType::CloseEdge => 
      {
        partial.push( token );
        processs_partial( &mut transaction, partial );
        partial = Vec::new();
      },

      SyntaxTokenType::EdgeDirection => 
      {
        partial.push( token );
        processs_partial( &mut transaction, partial );
        partial = Vec::new();
      },

      _ => { partial.push( token ); }
    }
  }

  if partial.len() > 0 
  {
    processs_partial( &mut transaction, partial );
    partial = Vec::new();
  }

  transaction
}

fn processs_partial ( transaction: &mut Transaction, partial: Vec<SyntaxToken> )
{
  match partial[0].token_type 
  {
    SyntaxTokenType::KeywordCreate => { process_create_graph_partial( transaction, partial ); },
    
    SyntaxTokenType::OpenNode => { process_open_node_partial( transaction, partial ); },

    SyntaxTokenType::OpenEdge => { process_open_edge_partial( transaction, partial ); },
    
    SyntaxTokenType::EdgeDirection => { process_edge_direction( transaction, partial ); },
    
    _ => 
    { 
      println!( "unmatched syntax token" ); 
    }
  }
}

/// !!! //transaction.err_state = Some( String::from( "Syntax Error: Create Graph" ));
fn process_create_graph_partial ( transaction: &mut Transaction, partial: Vec<SyntaxToken> ) 
{
  if partial.len() == 3 
  { 
    transaction.create_graph = Some( CreateGraph::new( partial[2].val.clone() ));
    return;
  }
}

fn process_open_node_partial ( transaction: &mut Transaction, partial: Vec<SyntaxToken> ) 
{
  if partial.len() == 3 
  {
    if transaction.left_node_ref.is_none() 
    {
      transaction.left_node_ref = Some( CreateNodeRef::new( partial[1].val.clone() ));
      return;
    }

    if transaction.right_node_ref.is_none() 
    {
      transaction.right_node_ref = Some( CreateNodeRef::new( partial[1].val.clone() ));
      return;
    }

    transaction.err_state = Some( String::from( "Syntax Error: Node Reference" ));
    return;
  }

  if partial.len() == 4 
  {
    transaction.create_node.push( CreateNode::new( partial[1].val.clone(), partial[2].val.clone() ));
    return;
  }

  transaction.err_state = Some( String::from( "Syntax Error: Node" ));
}

fn process_open_edge_partial ( transaction: &mut Transaction, partial: Vec<SyntaxToken> ) 
{
  if partial.len() == 3 
  {
    transaction.edge_primary_label = Some( partial[1].val.clone() );
    return;
  }
  transaction.err_state = Some( String::from( "Syntax Error: Edge" ));
}

fn process_edge_direction ( transaction: &mut Transaction, partial: Vec<SyntaxToken> ) 
{
  if partial.len() > 1 
  {
    transaction.err_state = Some( String::from( "Syntax Error: edge direction" ));
    println!( "partial" );
    return;
  }
  
  let dir_res = process_direction_type( &partial[0].val );
  if dir_res.is_ok() 
  {
    if transaction.edge_dir.is_none() 
    {
      transaction.edge_dir = Some( dir_res.unwrap() );
      return;
    }
    
    let current_dir = transaction.edge_dir.as_ref().unwrap();
    let new_dir = dir_res.as_ref().unwrap();

    if *current_dir == DirectionType::Undirected && *new_dir == DirectionType::Undirected { return; }
    
    if *current_dir == DirectionType::Undirected && *new_dir == DirectionType::Right 
    {
      transaction.edge_dir = Some( DirectionType::Right );
      return;
    }

    println!( "{:?} {:?}", current_dir, new_dir );
  }
  transaction.err_state = Some( String::from( "Syntax Error: edge direction" ));
}

fn process_direction_type ( dir: &str ) -> Result<DirectionType, String> 
{
  if dir == "-" { return Ok( DirectionType::Undirected ); }
  if dir == "<" { return Ok( DirectionType::Left ); }
  if dir == ">" { return Ok( DirectionType::Right ); }
  Err( String::from( "Syntax Error: Edge Direction Type." ))
}

fn process_for_edge ( transaction: &mut Transaction ) 
{
  if transaction.left_node_ref.is_some() 
    && transaction.right_node_ref.is_some() 
    && transaction.edge_primary_label.is_some()
    && transaction.edge_dir.is_some()
  {
    transaction.create_edge.push( 
      CreateEdge::new( 
        transaction.left_node_ref.clone().unwrap(),
        transaction.right_node_ref.clone().unwrap(),
        transaction.edge_dir.clone().unwrap(),
        transaction.edge_primary_label.clone().unwrap() ));

    transaction.left_node_ref = None; 
    transaction.right_node_ref = None; 
    transaction.edge_primary_label = None;
    transaction.edge_dir = None;
  }
}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_create_graph () 
  {
    let query_string = "CREATE GRAPH devs";
    let trans:Transaction = process_query( query_string );

    assert_eq!( trans.create_graph.is_some(), true );
    assert_eq!( trans.create_graph.unwrap().name, String::from( "devs" ));
    assert_eq!( trans.left_node_ref.is_none(), true );
    assert_eq!( trans.right_node_ref.is_none(), true );
    assert_eq!( trans.edge_dir.is_none(), true );
    assert_eq!( trans.edge_primary_label.is_none(), true );
    assert_eq!( trans.create_node.len(), 0 );
    assert_eq!( trans.create_edge.len(), 0 );
    assert_eq!( trans.err_state.is_some(), false );
  }

  #[test]
  fn test_create_node () 
  {
    let query_string = "
      (alice:Developer),
    ";
    let trans:Transaction = process_query( query_string );

    assert_eq!( trans.create_graph.is_none(), true );
    //assert_eq!( trans.node_ref.is_none(), true );
    //assert_eq!( trans.edge_ref.is_none(), true );
    assert_eq!( trans.create_node.len(), 1 );
    assert_eq!( trans.create_node[0].transaction_label, String::from( "alice" ));
    assert_eq!( trans.create_node[0].primary_label, String::from( "Developer" ));
    assert_eq!( trans.create_edge.len(), 0 );

    // ---
    
    let query_string1 = "
      (alice:Developer),
      (bob:Administrator),
      (charlie:Administrator),
    ";
    let trans1:Transaction = process_query( query_string1 );

    assert_eq!( trans1.create_graph.is_none(), true );
    //assert_eq!( trans1.node_ref.is_none(), true );
    //assert_eq!( trans1.edge_ref.is_none(), true );
    assert_eq!( trans1.create_node.len(), 3 );
    assert_eq!( trans1.create_node[0].transaction_label, String::from( "alice" ));
    assert_eq!( trans1.create_node[0].primary_label, String::from( "Developer" ));
    assert_eq!( trans1.create_node[1].transaction_label, String::from( "bob" ));
    assert_eq!( trans1.create_node[1].primary_label, String::from( "Administrator" ));
    assert_eq!( trans1.create_node[2].transaction_label, String::from( "charlie" ));
    assert_eq!( trans1.create_node[2].primary_label, String::from( "Administrator" ));
    assert_eq!( trans1.create_edge.len(), 0 );
  }

  #[test]
  fn test_create_edge () 
  {
    let query_string = "(alice)-[:KNOWS]->(bob)";
    let trans:Transaction = process_query( query_string );

    assert_eq!( trans.create_graph.is_none(), true );
    assert_eq!( trans.left_node_ref.is_none(), true );
    assert_eq!( trans.right_node_ref.is_none(), true );
    assert_eq!( trans.edge_primary_label.is_none(), true );
    assert_eq!( trans.edge_dir.is_none(), true );
    assert_eq!( trans.create_edge.len(), 1 );
    assert_eq!( trans.err_state.is_none(), true );

    let ce = &trans.create_edge[0];
    assert_eq!( ce.left_ref.transaction_label, "alice" );
    assert_eq!( ce.right_ref.transaction_label, "bob" );
    assert_eq!( ce.edge_dir, DirectionType::Right );
    assert_eq!( ce.primary_label, "KNOWS" );
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
    let qs = "";
  }

  #[test]
  fn test_transaction_1 () 
  {
    let query_string = "
      CREATE GRAPH devs
        (alice:Developer),
        (bob:Administrator),
        (alice)-[:KNOWS]->(bob)
    ";
    
    //println!( "------------------------------------------------------" );
    //let trans: Transaction = process_query( query_string );
    //println!( "{}", trans );
    //println!( "------------------------------------------------------" );

  }

  #[test]
  fn parser_test () 
  {
    let query_string = "
      CREATE GRAPH devs
        (alice:Developer),
        (bob:Administrator),
        (charlie:Administrator),
        (daniel:Adminstrator),
        (eskil:Designer),
        (alice)-[:KNOWS]->(bob),
        (alice)-[:KNOWS]->(charlie),
        (bob)-[:KNOWS]->(daniel),
        (charlie)-[:KNOWS]->(daniel),
        (bob)-[:MARRIED]->(eskil)
    ";
  }

  /*
  #[test]
  fn parser_test_1 () 
  {
    let query_string = "
      CREATE devs
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
    ";

    //let tokens:Vec<SyntaxToken> = parse_syntax( query_string );
    //println!( "{:?}", tokens );

    //for token in tokens.iter() 
    //{
    //  println!( "{:?}", token );
    //}
  }
  */
}
