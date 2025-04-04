use std::path::PathBuf;
use sdb_config::{ SDBConfiguration, default_sf_config, FileMode, ThreadingPattern, OpenMode };
use crate::traits::{ Database };
use fdm::{ create_file, open_file, write_sdb_config_entry };

static INIT_SF_FAILED: &'static str = "::init_sf_failed";
static LOAD_SF_FAILED: &'static str = "::load_sf_failed";

/// Single-File Database
#[derive(Debug)]
pub struct SingleFileDB
{ 
  pub config: Option<SDBConfiguration>,
}

impl Database for SingleFileDB
{
  fn init( &self ) -> Result<bool, &'static str> { Ok( true ) }
  fn create ( &self ) -> Result<bool, &'static str> { Ok( true ) }
  fn load ( &self ) -> Result<bool, &'static str> { Ok( true ) }
}

/// Construct Single-File Database
pub fn cons_sf_db ( db_path: String, db_name: Option<String> ) -> SingleFileDB 
{
  SingleFileDB 
  {
    config: Some( default_sf_config( db_path, db_name )),
  }
}

/// Initialize Single-File Database
pub fn init_sf_db ( build_id: &'static str, path: PathBuf, nickname: Option<String> ) -> Result<SingleFileDB, &'static str> 
{
  let mut create_res = create_file( &path );
  if create_res.is_ok() 
  {
    let mut file = create_res.as_mut().unwrap();
    write_sdb_config_entry( file, build_id.to_string(), nickname );
  }
  Err( INIT_SF_FAILED )
}

/// Load Single-File Database
pub fn load_sf_db ( path: PathBuf ) -> Result<SingleFileDB, &'static str> 
{
  let open_res = open_file( &path );
  Err( LOAD_SF_FAILED )
}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_cons_sf_db () 
  {
    let sdb = cons_sf_db( String::from( "/path" ), Some( String::from( "db_name" )));
    assert_eq!( sdb.config.is_some(), true );

    let config = sdb.config.unwrap();
    assert_eq!( config.db_path, String::from( "test_path" ));
    assert_eq!( config.db_name, Some( String::from( "test_name" ) ));
    assert_eq!( config.writes_allowed, true );
    assert_eq!( config.file_mode, FileMode::Single );
    assert_eq!( config.threading_pattern, ThreadingPattern::Single );
    assert_eq!( config.open_mode, OpenMode::OPENCREATE );
  }
}