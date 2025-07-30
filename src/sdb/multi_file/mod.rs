use sdb_config::{ SDBConfiguration, FileMode, ThreadingPattern, OpenMode, default_mf_config };
use crate::traits::{ Database };

/// Multi-File Database
#[derive(Debug)]
pub struct MultiFileDB
{ 
  pub config: Option<SDBConfiguration>,
}

impl Database for MultiFileDB 
{
  fn init( &self ) -> Result<bool, &'static str> { Ok( true ) }
  fn create ( &self ) -> Result<bool, &'static str> { Ok( true ) }
  fn load ( &self ) -> Result<bool, &'static str> { Ok( true ) }
}

/// Construct Multi-File Database
pub fn cons_mf_db ( db_path: String, db_name: Option<String> ) -> MultiFileDB 
{
  MultiFileDB 
  {
    config: Some( default_mf_config( db_path, db_name )),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cons_mf_db () 
  {
    let sdb = cons_mf_db( String::from( "/path" ), Some( String::from( "db_name" )));
    assert_eq!( sdb.config.is_some(), true );

    let config = sdb.config.unwrap();
    assert_eq!( config.db_path, String::from( "/path" ));
    assert_eq!( config.db_name, Some( String::from( "db_name" )));
    assert_eq!( config.writes_allowed, true );
    assert_eq!( config.file_mode, FileMode::Multi );
    assert_eq!( config.threading_pattern, ThreadingPattern::Multi );
    assert_eq!( config.open_mode, OpenMode::OPENCREATE );
  }
}