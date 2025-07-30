use std::string::{ FromUtf8Error };
use std::path::PathBuf;
use std::fs::{ exists, File, OpenOptions };
use std::io::Error;
use uuid::Uuid;

/* @version 0.3.0 */

// padded
pub fn cons_uuid () -> String { String::from( Uuid::new_v4().to_string() )}

/// Convert Byte Array into String
pub fn str_from_bytes ( bytes: &[u8] ) -> Result<String, FromUtf8Error> 
{ 
  String::from_utf8( bytes.to_vec() )
}

/// Convert String into Byte Array
pub fn str_to_bytes ( s: String ) -> Vec<u8> 
{ 
  s.into_bytes() 
}

/// Generate padding string of given length
pub fn gen_pad_str ( length: usize ) -> String 
{
  if length == 0 { return String::from( "" )}

  let mut ret = String::from( "\\" );
  while &ret.len() < &length { ret.push_str( ":" ); }
  ret
}

pub fn process_str ( max_len: usize, str_to_proc: String ) -> Result<String, String> 
{
  let start_size: usize = str_to_proc.clone().into_bytes().len();
  if start_size > max_len { return Err( String::from( "String too long" )); }
  if start_size == max_len { return Ok( str_to_proc ); }

  let padding = gen_pad_str( &max_len - &start_size );
  Ok( str_to_proc + &padding )
}

/// Pad String to given length
pub fn pad_str ( max_len: usize, str_to_pad: String ) -> String
{
  let start_size: usize = str_to_pad.clone().into_bytes().len();
  if start_size > max_len { return str_to_pad }
  if start_size == max_len { return str_to_pad }

  let padding = gen_pad_str( &max_len - &start_size );
  str_to_pad + &padding
}

/// Ensure last character is not \ 
pub fn valid_pad_str ( str_to_pad: &str ) -> bool 
{ 
  let back_slash = str_to_pad.ends_with( "\\" );
  let colon = str_to_pad.ends_with( ":" );
  if back_slash == true { return false }
  if colon == true { return false } 
  true
}

/// Test if string has been padded
pub fn is_padded_str ( str_to_check: &str ) -> bool { !valid_pad_str( str_to_check )}

pub fn parse_padded_str ( padded_str: &str ) -> &str
{
  let split: Vec<&str> = padded_str.split( "\\" ).collect();
  split[0]
}

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

/// Create File
pub fn create_file ( path: &PathBuf ) -> Result<File, Error> { File::create( path ) }

/// Open File
pub fn open_file ( path: &PathBuf ) -> Result<File, Error> 
{
  OpenOptions::new().read( true ).write( true ).create( false ).open( path )
}

/// Check Path/File exists
pub fn path_exists ( path: PathBuf ) -> Result<bool, Error> { exists( path ) }

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_string_binary () 
  {
    let bytes: Vec<_> = b"test".to_vec();
    assert_eq!( bytes, [116, 101, 115, 116] );

    let string = String::from_utf8( bytes ).expect( "invalid utf8" );
    assert_eq!( string, "test" );

    let bytes: Vec<_> = b"/".to_vec();
    assert_eq!( bytes.len(), 1 );

    let bytes: Vec<_> = b"[".to_vec();
    assert_eq!( bytes.len(), 1 );

    let bytes: Vec<_> = b"]".to_vec();
    assert_eq!( bytes.len(), 1 );
  }

  #[test]
  fn test_vec_slices () 
  {
    let bytes: Vec<_> = str_to_bytes( String::from( "testtest" ) ).to_vec();
    assert_eq!( &bytes[0..4], [116, 101, 115, 116] );
    assert_eq!( &bytes[4..8], [116, 101, 115, 116] );
    
    let res1 = str_from_bytes( &bytes[0..4] );
    assert_eq!( res1.is_ok(), true );
    assert_eq!( res1.unwrap(), "test" );

    let res2 = str_from_bytes( &bytes[4..8] );
    assert_eq!( res2.is_ok(), true );
    assert_eq!( res2.unwrap(), "test" );
  }

  #[test]
  fn test_str_len () 
  {
    let astr = String::from( "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" );
    assert_eq!( astr.into_bytes().len(), 64 );
  
    let esc = String::from( "\\" );
    assert_eq!( esc.into_bytes().len(), 1 );
  }

  #[test]
  fn test_u64_binary () 
  {
    assert_eq!( 0x123u64.to_le_bytes().len(), 8 );
    assert_eq!( 0x1234567890123456u64.to_le_bytes().len(), 8 );
    assert_eq!( 0x0u64.to_le_bytes().len(), 8 );
  }

  #[test]
  fn test_uuid () 
  {
    let double_uuid = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    assert_eq!( double_uuid.into_bytes().len(), 40 );
  }

  #[test]
  fn test_str_from_bytes () 
  {
    let res = str_from_bytes( &[116, 101, 115, 116] );
    assert_eq!( res.is_ok(), true );
    assert_eq!( res.unwrap(), "test" );
  }

  #[test]
  fn test_str_to_bytes () 
  {
    let bytes: Vec<_> = str_to_bytes( String::from( "test" ) ).to_vec();
    assert_eq!( bytes, [116, 101, 115, 116] );
  }

  #[test]
  fn test_gen_pad_str () 
  {
    let pad = gen_pad_str( 10 );
    assert_eq!( pad.into_bytes().len(), 10 );

    let pad_err = gen_pad_str( 0 );
    assert_eq!( pad_err, String::from( "" ));
  }

  #[test]
  fn test_pad_str () 
  {
    let pad1 = pad_str( 10, String::from( "a" ));    

    assert_eq!( pad1, String::from( "a\\::::::::" ));
    assert_eq!( pad1.into_bytes().len(), 10 );

    assert_eq!( 
      pad_str( 64 as usize, String::from( "test" )), 
      String::from( "test\\:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::" ));

    assert_eq!( 
      pad_str( 
        64 as usize, 
        String::from( "1234::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::" )), 
        String::from( "1234::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::" ));

    assert_eq!( 
      pad_str( 
        64 as usize, 
        String::from( "nickname" )), 
        String::from( "nickname\\:::::::::::::::::::::::::::::::::::::::::::::::::::::::" ));
  }

  #[test]
  fn test_valid_pad_str () 
  {
    assert_eq!( valid_pad_str( "test" ), true );
    assert_eq!( valid_pad_str( "test\\" ), false );
    assert_eq!( valid_pad_str( "test:" ), false );
  }

  #[test]
  fn test_is_padded_str () 
  {
    let pad1 = pad_str( 10, String::from( "a" )); 
    assert_eq!( is_padded_str( &pad1 ), true );
    assert_eq!( is_padded_str( "test\\" ), true );
    assert_eq!( is_padded_str( "test:" ), true );
    assert_eq!( is_padded_str( "test" ), false );
  }

  #[test]
  fn test_parse_padded_str () 
  {
    let pad1 = pad_str( 10, String::from( "a" ));

    let pad1_str = parse_padded_str( &pad1 );
    assert_eq!( pad1_str, String::from( "a" ));

    let pad2 = pad_str( 64 as usize, String::from( "nickname" ));
    let pad2_str = parse_padded_str( &pad2 );
    assert_eq!( pad2_str, String::from( "nickname" ));

    let pad3_str = parse_padded_str( "no-padding" );
    assert_eq!( pad3_str, String::from( "no-padding" ));
  }

  #[test]
  fn test_u32_binary () 
  {
    assert_eq!( 0x123u32.to_le_bytes().len(), 4 );
    assert_eq!( u32::MAX.to_le_bytes().len(), 4 ); // 4294967295
    assert_eq!( 0x0u32.to_le_bytes().len(), 4 );
  }

  #[test]
  fn test_page_start () 
  {
    let astr = String::from( "PageStart" );
    assert_eq!( astr.into_bytes().len(), 9 );
  }

  #[test]
  fn test_is_file () 
  {
    assert_eq!( is_file( &PathBuf::from( "../test_data/empty.sdb" )), true );
    assert_eq!( is_file( &PathBuf::from( "../test_data/nope.sdb" )), false );
  }

  #[test]
  fn test_is_dir () 
  {
    assert_eq!( is_dir( &PathBuf::from( "../test_data" )), true );
    assert_eq!( is_dir( &PathBuf::from( "../test_data/" )), true );
    assert_eq!( is_dir( &PathBuf::from( "../nope" )), false );
  }

  #[test]
  fn test_has_file_extension () 
  {
    assert_eq!( has_file_extension( &PathBuf::from( "../test_data/empty.sdb" )), true );
    assert_eq!( has_file_extension( &PathBuf::from( "../test_data" )), false );
  }
}
