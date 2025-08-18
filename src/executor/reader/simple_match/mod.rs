use crate::cmd::transaction::Transaction;

/*
MATCH () FROM devs;

MATCH (n)

MATCH (n) FROM devs RETURN n.name

MATCH (n:Stop)

MATCH (n:Developer)
FROM devs
RETURN n AS Developer
*/

pub struct SimpleMatchExecutor<'a> 
{
  pub transaction: &'a Transaction,
  pub path: &'a str,
  pub page_size: usize,
}

impl SimpleMatchExecutor<'_>
{
  pub fn new<'a> ( t: &'a Transaction, path: &'a str, page_size: usize ) -> SimpleMatchExecutor<'a>
  {
    SimpleMatchExecutor 
    {
      transaction: t, 
      path: path, 
      page_size: page_size,
    } 
  }

  pub fn execute ( &mut self ) {}
}

#[cfg(test)]
mod tests 
{ 
  use super::*;
  use std::fs::{ metadata, remove_file };
  use std::path::PathBuf;
  use std::io::BufWriter;
  use crate::datagramv2::internal_grams::{ Label, UUID };
  use crate::planner::process_query;
  use crate::utils::create_file;
  use crate::executor::writer::new_db::WriteNewDBExecutor;

  const PAGE_SIZE: usize = 4096;

  fn build_id () -> UUID { UUID::new( String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" )).unwrap() }
  fn db_nickname () -> Label { Label::new( String::from( "devs" ) ).unwrap() }
  fn write_new_db ( path: &str )
  {
    let open_res = create_file( &PathBuf::from( path ));
    let mut stream = BufWriter::new( open_res.unwrap() );
    let _ = WriteNewDBExecutor::execute_write_new( &build_id(), &db_nickname(), PAGE_SIZE, &mut stream );
  }

  #[test]
  fn test_testing () 
  {
    let path_str = "../test_data/SimpleMatchExecutor_test_testing.sdb";
    write_new_db( path_str );
    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), 4096 as u64 );
    
    // !!! come back with more complex graph representation
    //let query_string = "CREATE GRAPH devs";
    //let t = process_query( &query_string, build_id(), db_nickname() );

    let query_string = "MATCH ()";
    let t = process_query( &query_string, build_id(), db_nickname() );
    println!( "{}", t );

    let mut read_executor = SimpleMatchExecutor::new( &t, path_str, 4096 );
    
    let _ = remove_file( PathBuf::from( path_str ));
    /* 
    let mut write_executor = WriteNewGraphExecutor::new( &t, path_str, 4096 );
    
    write_executor.graph_name = Some( Label::new( String::from( "devs" ), LABEL_BYTES ).unwrap() );
    write_executor.graph_uuid = Some( UUID::new( String::from( "67e55044-10b1-426f-9247-bb680e5fe0cX" )).unwrap() );

    let stmt = NodeStatement::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0cX" ), 
      0, 
      None, 
      String::from( "primary" ));

    let open_res = open_file( &PathBuf::from( &path_str ));
    let mut writer: BufWriter<File> = BufWriter::new( open_res.unwrap() );

    let _ = write_executor.write_node( &stmt, 0, &mut writer );

    assert_eq!( write_executor.err_state, None );
    */
  }
}