/// Open Mode
#[derive(Debug, PartialEq)]
pub enum OpenMode 
{ 
  // database is opened in read-only mode; if database does not already exist, an error is returned
  OPENREADONLY,

  // database is opened for reading and writing; if database does not already exist, an error is returned
  OPENREADWRITE, 
  
  // database is opened for reading and writing; if the database does not exist, it is created
  OPENCREATE,
  
  // not loaded yet
  PRELOAD,
}

/// File Mode
#[derive(Debug, PartialEq)]
pub enum FileMode { Single, Multi, InMemory, PreLoad, }

/// Threading Pattern
#[derive(Debug, PartialEq)]
pub enum ThreadingPattern { Single, Multi, PreLoad, }

/// SubgraphDB Configurations
#[derive(Debug)]
pub struct SDBConfiguration
{
  // file path and file name
  pub db_path: String,

  // database file name
  pub db_name: Option<String>,

  // writes allowed
  pub writes_allowed: bool,
  
  pub file_mode: FileMode,
  pub threading_pattern: ThreadingPattern,
  pub open_mode: OpenMode,
}

/// Construct SDBConfiguration 
pub fn cons_sdb_config ( 
  db_path: String, db_name: Option<String>, writes_allowed: bool, file_mode: FileMode, 
  threading_pattern: ThreadingPattern, open_mode: OpenMode ) 
-> SDBConfiguration 
{
  SDBConfiguration 
  { 
    db_path: db_path,
    db_name: db_name,
    writes_allowed: writes_allowed,
    file_mode: file_mode,
    threading_pattern: threading_pattern,
    open_mode: open_mode,
  }
}

/// Construct Default In-Memory SDBConfiguration
pub fn default_im_config () -> SDBConfiguration 
{
  SDBConfiguration 
  { 
    db_path: String::from( "::in-memory::" ),
    db_name: None,
    writes_allowed: true,
    file_mode: FileMode::InMemory,
    threading_pattern: ThreadingPattern::Single,
    open_mode: OpenMode::OPENCREATE,
  }
}

/// Construct Default Single File SDBConfiguration
pub fn default_sf_config ( db_path: String, db_name: Option<String> ) -> SDBConfiguration 
{
  SDBConfiguration 
  { 
    db_path: db_path,
    db_name: db_name,
    writes_allowed: true,
    file_mode: FileMode::Single,
    threading_pattern: ThreadingPattern::Single,
    open_mode: OpenMode::OPENCREATE,
  }
}


/// Construct Default Single File SDBConfiguration
pub fn default_mf_config ( db_path: String, db_name: Option<String> ) -> SDBConfiguration 
{
  SDBConfiguration 
  { 
    db_path: db_path,
    db_name: db_name,
    writes_allowed: true,
    file_mode: FileMode::Multi,
    threading_pattern: ThreadingPattern::Multi,
    open_mode: OpenMode::OPENCREATE,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_im_config () 
  {
    let config = default_im_config();
    assert_eq!( config.db_path, String::from( "::in-memory::" ));
    assert_eq!( config.db_name.is_some(), false );
    assert_eq!( config.writes_allowed, true );
    assert_eq!( config.file_mode, FileMode::InMemory );
    assert_eq!( config.threading_pattern, ThreadingPattern::Single );
    assert_eq!( config.open_mode, OpenMode::OPENCREATE );
  }

  #[test]
  fn test_default_sf_config () 
  {
    let config = default_sf_config( String::from( "test_path" ), Some( String::from( "test_name" )));
    
    assert_eq!( config.db_path, String::from( "test_path" ) );
    assert_eq!( config.db_name.unwrap(), String::from( "test_name" ) );
    assert_eq!( config.writes_allowed, true );
    assert_eq!( config.file_mode, FileMode::Single );
    assert_eq!( config.threading_pattern, ThreadingPattern::Single );
    assert_eq!( config.open_mode, OpenMode::OPENCREATE );
  }

  #[test]
  fn test_default_mf_config () 
  {
    let config = default_mf_config( String::from("test_path"), Some( String::from( "test_name" )));
    
    assert_eq!( config.db_path, String::from("test_path") );
    assert_eq!( config.db_name.unwrap(), String::from("test_name") );
    assert_eq!( config.writes_allowed, true );
    assert_eq!( config.file_mode, FileMode::Multi );
    assert_eq!( config.threading_pattern, ThreadingPattern::Multi );
    assert_eq!( config.open_mode, OpenMode::OPENCREATE );
  }
}