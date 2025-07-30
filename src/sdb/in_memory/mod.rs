use sdb_config::{ SDBConfiguration, FileMode, ThreadingPattern, OpenMode, default_im_config };
use crate::traits::{ Database };

/// In-Memory Database
#[derive(Debug)]
pub struct InMemoryDB
{ 
  pub config: Option<SDBConfiguration>,
}

impl Database for InMemoryDB 
{
  fn init( &self ) -> Result<bool, &'static str> { Ok( true ) }
  fn create ( &self ) -> Result<bool, &'static str> { Ok( true ) }
  fn load ( &self ) -> Result<bool, &'static str> { Ok( true ) }
}

/// Construct In-Memory Database
pub fn cons_im_db () -> InMemoryDB 
{
  InMemoryDB 
  {
    config: Some( default_im_config() ),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cons_im_db () 
  {
    let sdb = cons_im_db();
    assert_eq!( sdb.config.is_some(), true );

    let config = sdb.config.unwrap();
    assert_eq!( config.db_path, String::from( "::in-memory::" ));
    assert_eq!( config.db_name, None );
    assert_eq!( config.writes_allowed, true );
    assert_eq!( config.file_mode, FileMode::InMemory );
    assert_eq!( config.threading_pattern, ThreadingPattern::Single );
    assert_eq!( config.open_mode, OpenMode::OPENCREATE );
  }
}
