use std::collections::HashMap;
use std::fs::{ File, exists };
use std::io::{ BufWriter, Error, Read, Write };
use std::path::PathBuf;
use datagram::{ DGSDBConfig };

/// File Data Management

#[derive( Debug, PartialEq )]
pub enum FDMError 
{
  ErrorWritingSDBConfig,
  ErrorReadingSDBConfig,
}

/// Create File
pub fn create_file ( path: &PathBuf ) -> Result<File, Error> { File::create( path ) }

/// Open File
pub fn open_file ( path: &PathBuf ) -> Result<File, Error> { File::open( path ) }

/// Check Path/File exists
pub fn path_exists ( path: PathBuf ) -> Result<bool, Error> { exists( path ) }

/// Path is File
pub fn is_file ( path: &PathBuf ) -> bool { path.is_file() }

/// Path is Directory
pub fn is_dir ( path: &PathBuf ) -> bool { path.is_dir() }

/// Path has File Extension
pub fn has_file_extension ( path: &PathBuf ) -> bool 
{
  match path.extension() 
  {
    None => return false,
    Some(_) => return true
  }
}

// ---------------------------------------------------------------------------------------------------------------------
/// Write DGSDBConfig
pub fn write_sdb_config ( file: &mut File, build_id: String, nickname: Option<String> ) -> Result<File, FDMError> 
{
  let sdb_config_res = DGSDBConfig::new( build_id, nickname );
  if sdb_config_res.is_ok() 
  {
    let mut stream = BufWriter::new( file );
    stream.write( &sdb_config_res.unwrap().to_bytes() ).unwrap();
    stream.flush().unwrap();
  }
  Err( FDMError::ErrorWritingSDBConfig )
}

/// Read DGSDBConfig
pub fn read_sdb_config ( mut file: &File ) -> Result<File, FDMError> 
{
  /*
  let mut buffer = [ 0; ByteLengths::UUID as usize ];
  let _ = file.read_exact( &mut buffer );
  println!( "{:?}", str_from_bytes( buffer.to_vec() ));
  */
  Err( FDMError::ErrorReadingSDBConfig )
}

// ---------------------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests 
{
  use super::*;
  use std::fs::{ remove_file };
  
  // let tmp_dir = String::from( "/platonic3d/subgraphdbv2/test_data/tmp" );

  #[test]
  fn test_is_file () 
  {
    assert_eq!( is_file( &PathBuf::from( "/platonic3d/subgraphdbv2/test_data/sf/test1.sdb" )), true );
    assert_eq!( is_file( &PathBuf::from( "/platonic3d/subgraphdbv2/nope.sdb" )), false );
  }

  #[test]
  fn test_is_dir () 
  {
    assert_eq!( is_dir( &PathBuf::from( "/platonic3d/subgraphdbv2/test_data/test1.sdb" )), false );
    assert_eq!( is_dir( &PathBuf::from( "/platonic3d/subgraphdbv2/test_data" )), true );
    assert_eq!( is_dir( &PathBuf::from( "/platonic3d/subgraphdbv2/test_data/" )), true );
  }

  #[test]
  fn test_has_file_extension () 
  {
    assert_eq!( has_file_extension( &PathBuf::from( "/test_data/test1.sdb" )), true );
    assert_eq!( has_file_extension( &PathBuf::from( "/test_data" )), false );
  }

  #[test]
  fn test_write_sdb_config () 
  {
    let path = PathBuf::from( "/platonic3d/subgraphdbv2/test_data/tmp/test_read_sdb_config_entry.sdb" );
    let mut create_file_res = create_file( &path );
    write_sdb_config( 
      create_file_res.as_mut().unwrap(), 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      Some( String::from( "nickname" )));

    //read_sdb_config_entry( &create_file_res.as_mut().unwrap() ); 
    //assert_eq!( has_file_extension( &PathBuf::from( "/test_data/test1.sdb" )), true );
    //assert_eq!( has_file_extension( &PathBuf::from( "/test_data" )), false );
    //assert_eq!( has_file_extension( &PathBuf::from( "/test_data/.test1.sdb" )), false );
    
    //let _ = remove_file( &path );
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