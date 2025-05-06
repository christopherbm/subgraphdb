pub mod enums;
pub mod structs;
pub mod dg_utils;
pub mod pages;
pub mod rows;

//use std::string::{ FromUtf8Error };


/* !!
  - all inputs will get translated into a transaction
  - all writes need to end with a write complete bit [WC] to ensure a good write.
  - files also need to end with a TransactionComplete UUID to ensure good transaction write.
  - If either the wc bit or the transactionComplete bit are missing, the db is in a bad state.
*/

#[cfg(test)]
mod tests 
{
  /*
  use std::path::PathBuf;
  use std::io::{ BufWriter, Write };
  use utils::create_file;
  use crate::pages::SDBConfigPage;
  use crate::enums::{ DataGramError };
  use crate::structs::GraphRef;
  */

  #[test]
  fn manage_test_data () 
  {
    /* Create a sf db file with no graphs */
    /*
    let page_size: usize = 4096;
    let path = PathBuf::from( "./test_data/new_sf_db.sdb" );

    let config_res: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ), 
      String::from( "nickname" ),
      Vec::new() );

    let create_file_res = create_file( &path );
    
    let mut stream = BufWriter::new( create_file_res.unwrap() );
    stream.write( &config_res.unwrap().to_rows( &page_size ) ).unwrap();
    stream.flush().unwrap();
    */


    /* Create a sf db file with 1 graph_ref */
    /*
    let page_size: usize = 4096;
    let path = PathBuf::from( "./test_data/sf_db_1_ref.sdb" );

    let gr_res = GraphRef::new( String::from( "graph1" ));
    let mut grs = Vec::new();
    grs.push( gr_res.unwrap() );

    let config_res: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ), 
      String::from( "nickname" ),
      grs );

    let create_file_res = create_file( &path );
    
    let mut stream = BufWriter::new( create_file_res.unwrap() );
    stream.write( &config_res.unwrap().to_rows( &page_size ) ).unwrap();
    stream.flush().unwrap();
    */


    /* Create a sf db file with 10 graph_ref */
    /*
    let page_size: usize = 4096;
    let path = PathBuf::from( "./test_data/sf_db_10_ref.sdb" );

    let mut grs = Vec::new();
    grs.push( GraphRef::new( String::from( "graph1" )).unwrap() );
    grs.push( GraphRef::new( String::from( "graph2" )).unwrap() );
    grs.push( GraphRef::new( String::from( "graph3" )).unwrap() );
    grs.push( GraphRef::new( String::from( "graph4" )).unwrap() );
    grs.push( GraphRef::new( String::from( "graph5" )).unwrap() );
    grs.push( GraphRef::new( String::from( "graph6" )).unwrap() );
    grs.push( GraphRef::new( String::from( "graph7" )).unwrap() );
    grs.push( GraphRef::new( String::from( "graph8" )).unwrap() );
    grs.push( GraphRef::new( String::from( "graph9" )).unwrap() );
    grs.push( GraphRef::new( String::from( "graph10" )).unwrap() );

    let config_res: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ), 
      String::from( "nickname" ),
      grs );

    let create_file_res = create_file( &path );
    
    let mut stream = BufWriter::new( create_file_res.unwrap() );
    stream.write( &config_res.unwrap().to_rows( &page_size ) ).unwrap();
    stream.flush().unwrap();
    */
  }
}