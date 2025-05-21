//pub mod enums;
//pub mod in_memory;
//pub mod single_file;
//pub mod multi_file;
//pub mod traits;

use sdb_config::{ SDBConfiguration, FileMode, default_im_config, default_sf_config, default_mf_config };
//use enums::{ SDBType };
//use in_memory::{ InMemoryDB, cons_im_db };
//use single_file::{ SingleFileDB, cons_sf_db };
//use multi_file::{ MultiFileDB, cons_mf_db };

/// Core interface into Database System
#[derive(Debug)]
pub struct SubgraphDB
{ 
  pub config: SDBConfiguration,
}

/// Construct In-Memory Database
//pub fn cons_im_db () -> SubgraphDB 
//{
//  SubgraphDB 
//  {
//    config: default_im_config(),
//    db: SDBType::InMemory( cons_im_db() ),
//  }
//}

/// Initialize Single-File Database
pub fn init_sf_db ( db_path: String, db_name: Option<String> ) -> SubgraphDB 
{
  SubgraphDB 
  {
    config: default_sf_config( db_path, db_name ),
  }
}

/// Construct Multi-File Database
//pub fn cons_mf_db ( db_path: String, db_name: String ) -> SubgraphDB 
//{
//  SubgraphDB 
//  {
//    config: default_mf_config( db_path, db_name ),
//    db: SDBType::MultiFile( cons_mf_db( db_path, db_name )),
//  }
//}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_init_sf_db () 
  {
    let db = init_sf_db( String::from( "path" ), Some( String::from( "name" )));
    println!( "{:?}", db );
    //assert_matches!( sdb.db, SDBType::InMemory(_) );
  }
}