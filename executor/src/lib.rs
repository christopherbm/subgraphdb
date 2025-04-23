use std::path::PathBuf;
use datagram::pages::{ SDBConfigPage };
use datagram::enums::DataGramError;

/*
00 -> [PageStart name:"db config"][wc]
01 -> [BuildUUID][db_nickname][wc]
.. -> [Graph 0 nickname:"movies" start_index:0B][wc]
0A -> [PageEnd "dbconfig"][wc]
0B -> [PageStart name:"graph 0 page" end_index:0C next_page_index:00][wc]
.. -> [node_order:0][UUID][Person]["Keanu Reaves"][node_conns:1 2 4][edge_cons:0 1 2][WC]
.. -> [node_order:1][UUID][Person]["Carrie Anne Moss"][node_conns:0 3][edge_cons:0 4][WC]
0C -> [PageEnd name:"graph 0 page"][wc]

// ---------------

// !! would u32 be faster for order values since its smaller (bytes) than u64?
// !! page size needs to be configurable, but row size will be fixed. Page size cannot be changed after db creation.
// !! there can be more than 1 sdb config page depending on number and complexity of graphs
// !! knows that "name" is first of indexed values - this would need to be part of graph config as a contraint
// !! any modification to the graph config or the db config will result in a rewrite of the binary data

- When a read begins, it chooses a particular transaction id to be its "end mark". This way reads can happen along with
  writes but the reads will have "point-in-time" consistency.

// ---------------
MATCH (p:Person) FROM movies RETURN p LIMIT 5

*/

// find_page
// find_all_pages
// read_sdb_config_page
// read_graph_page

// !! write using as_bytes
/* !!
let mut stream = BufWriter::new( file );
stream.write( &sdb_config_res.unwrap().to_bytes() ).unwrap();
stream.flush().unwrap();
*/

/*
  let mut buffer = [ 0; ByteLengths::UUID as usize ];
  let _ = file.read_exact( &mut buffer );
  println!( "{:?}", str_from_bytes( buffer.to_vec() ));
*/

/// Executes a Transaction
pub fn execute_transaction () {}

pub fn write_sdb_config () {}

pub fn read_sdb_config () {}

#[cfg(test)]
mod tests 
{
  use super::*;
  use std::fs::{ remove_file };

  #[test]
  fn test_execute_transaction () 
  {
    assert_eq!( true, true );
  }

  #[test]
  fn test_sdb_config () 
  {
    let page_size = 4096;
    let path = PathBuf::from( "../test_data/tmp_sdb_config.sdb" );
    let config: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      String::from( "nickname" ),
      Vec::new() );

    println!( "{:?}", config );

    //let mut create_file_res = create_file( &path );
    //write_sdb_config( 
    //  create_file_res.as_mut().unwrap(), 
    //  String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
    //  Some( String::from( "nickname" )));

    //read_sdb_config_entry( &create_file_res.as_mut().unwrap() ); 
    //assert_eq!( has_file_extension( &PathBuf::from( "/test_data/test1.sdb" )), true );
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
