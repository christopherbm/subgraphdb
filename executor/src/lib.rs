use std::path::PathBuf;
use std::io::{ BufWriter, Write };
use utils::{ create_file };
use datagram::pages::{ SDBConfigPage };
use datagram::enums::DataGramError;
use cmd::{ Transaction, CreateGraph, CreateNode, CreateEdge, CreateNodeRef };

/*
// !! page size needs to be configurable, but row size will be fixed. Page size cannot be changed after db creation.
// !! there can be more than 1 sdb config page depending on number and complexity of graphs
// !! knows that "name" is first of indexed values - this would need to be part of graph config as a contraint
// !! any modification to the graph config or the db config will result in a rewrite of the binary data

- When a read begins, it chooses a particular transaction id to be its "end mark". This way reads can happen along with
  writes but the reads will have "point-in-time" consistency.

// ---------------


*/

// find_page
// find_all_pages
// read_sdb_config_page
// read_graph_page

/*
  - Create Graph
    - how many SDBConfigPages are there?
    - SDBConfigPage(s) has graph?
*/

/// Executes a Transaction
pub fn execute_transaction ( t: &Transaction ) 
{
  println!( "{}", t );
  if t.create_graph.is_some() 
  {
    let cg_res = execute_create_graph( t.create_graph.as_ref().unwrap() );
  }
}

fn execute_create_graph ( cg: &CreateGraph ) -> Result<bool, String> 
{
  println!( "{:?}", cg );
  Ok( true )
}

fn execute_create_node ( nodes: Vec<CreateNode> ) 
{}

fn execute_create_edges ( edges: Vec<CreateEdge> ) 
{}

#[cfg(test)]
mod tests 
{
  use super::*;
  use std::fs::{ remove_file, metadata };
  use planner::{ process_query };

  #[test]
  fn test_execute_transaction () 
  {
    let query_string = "CREATE GRAPH devs";
    let trans:Transaction = process_query( query_string );
    execute_transaction( &trans );
    

    //assert_eq!( true, true );
  }

  #[test]
  fn test_create_sf_db () 
  {
    let page_size: usize = 4096;
    let path = PathBuf::from( "../test_data/tmp_sf_db_1.sdb" );

    let config_res: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ), 
      String::from( "nickname" ),
      Vec::new() );

    let create_file_res = create_file( &path );
    
    let mut stream = BufWriter::new( create_file_res.unwrap() );
    stream.write( &config_res.unwrap().to_rows( &page_size ) ).unwrap();
    stream.flush().unwrap();
    
    assert_eq!( metadata( &path ).unwrap().len(), 4096 );

    let _ = remove_file( &path );
  }

  #[test]
  fn test_create_graph () 
  {
    let page_size: usize = 4096;
    let path = PathBuf::from( "../test_data/tmp_sf_db_2.sdb" );

    let config_res: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ), 
      String::from( "nickname" ),
      Vec::new() );

    let create_file_res = create_file( &path );
    
    let mut stream = BufWriter::new( create_file_res.unwrap() );
    stream.write( &config_res.unwrap().to_rows( &page_size ) ).unwrap();
    stream.flush().unwrap();

    // ---
  }

  #[test]
  fn test_sdb_config () 
  {
    let page_size = 4096;
    let path = PathBuf::from( "../test_data/tmp_sdb_config.sdb" );
    let config: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ), 
      String::from( "nickname" ),
      Vec::new() );

    //println!( "{:?}", config );

    //let mut create_file_res = create_file( &path );
    //write_sdb_config( 
    //  create_file_res.as_mut().unwrap(), 
    //  String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
    //  Some( String::from( "nickname" )));

    //read_sdb_config_entry( &create_file_res.as_mut().unwrap() ); 
    //
    //assert_eq!( has_file_extension( &PathBuf::from( "/test_data" )), false );
    //assert_eq!( has_file_extension( &PathBuf::from( "/test_data/.test1.sdb" )), false );
    
    let _ = remove_file( &path );
  }

  #[test]
  fn test_read_sdb_config_entry () 
  {
    //let path = PathBuf::from( "/platonic3d/subgraphdbv2/test_data/sf/unit_tests/test_read_sdb_config_entry_1.sdb" );
    //let mut open_res = open_file( &path );
    
    //read_sdb_config_entry( &open_res.as_mut().unwrap() ); 

    //assert_eq!( has_file_extension( &PathBuf::from( "/test_data/test1.sdb" )), true );
    //assert_eq!( has_file_extension( &PathBuf::from( "/test_data" )), false );
    //assert_eq!( has_file_extension( &PathBuf::from( "/test_data/.test1.sdb" )), false );
  }
}
