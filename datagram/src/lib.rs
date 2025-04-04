#![allow( dead_code )]
use std::string::{ FromUtf8Error };
use std::fs::{ File };

pub static COMMON_NONE: &'static str = "\\:::::::::::::::::::::::::::::::::::::::::::::::::::::::::NONE::";

#[derive( Debug, PartialEq )]
pub enum DataGramError 
{
  InvalidDGUUID,
  InvalidDGCommonString,
  StringLengthExceeded,
  PaddingCannotBeZero,
  InvalidStringTerminus,
  InvalidSDBConfig,
  ErrorWritingSDBConfig,
  ErrorReadingSDBConfig,
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

/// Generate padding string of given length
fn gen_pad_str ( length: usize ) -> String 
{
  if length == 0 { return String::from( "" )}

  let mut ret = String::from( "\\" );
  while &ret.len() < &length { ret.push_str( ":" ); }
  ret
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

/// Convert String into Byte Array
pub fn str_to_bytes ( s: String ) -> Vec<u8> { s.into_bytes() }

/// Convert Byte Array into String
pub fn str_from_bytes ( bytes: &[u8] ) -> Result<String, FromUtf8Error> { String::from_utf8( bytes.to_vec() )}

// ---------------------------------------------------------------------------------------------------------------------
pub enum ByteLengths 
{
  UUID = 36,
  CommonString = 64, // names, labels, etc
  DGU64 = 8,
}

/// Data Gram Types
#[derive( Debug, PartialEq )]
pub enum DataGramType
{
  DGUUID,
  DGCommonString,
  DGLabel,
}

/// Data Gram Types
#[derive( Debug, PartialEq )]
pub struct DGSDBConfig
{
  build_id: String, // DGUUID
  nickname: String, // DGLabel
}
impl DGSDBConfig 
{
  pub fn new ( build_id: String, nickname: Option<String> ) -> Result<DGSDBConfig, DataGramError> 
  {
    if !validate_dg_uuid( &build_id ) { return Err( DataGramError::InvalidDGUUID )}
    if nickname.is_some() 
    {
      let nickname_actual = pad_str( ByteLengths::CommonString as usize, nickname.unwrap() );
      if !validate_dg_label( &nickname_actual ) { return Err( DataGramError::InvalidDGCommonString )}
      return Ok( DGSDBConfig { build_id: build_id, nickname: nickname_actual })
    }
    else 
    {
      return Ok( DGSDBConfig { build_id: build_id, nickname: COMMON_NONE.to_string() })
    }
    Err( DataGramError::InvalidSDBConfig )
  }

  pub fn to_bytes ( &self ) -> Vec<u8> { [ self.build_id.as_bytes(), self.nickname.as_bytes() ].concat() }

  pub fn from_bytes ( bytes: Vec<u8> ) -> Result<DGSDBConfig, DataGramError> 
  {
    if bytes.len() != 100 { return Err( DataGramError::ErrorReadingSDBConfig ) }
    let build_id_res = str_from_bytes( &bytes[0..( ByteLengths::UUID as usize )]);
    let nickname_res = str_from_bytes( &bytes[
      ( ByteLengths::UUID as usize )
      ..
      ( ByteLengths::UUID as usize + ByteLengths::CommonString as usize )]);

    if build_id_res.is_ok() && nickname_res.is_ok() 
    {
      if nickname_res.as_ref().unwrap() == &COMMON_NONE 
      {
        return DGSDBConfig::new( build_id_res.unwrap(), None )
      }
      return DGSDBConfig::new( build_id_res.unwrap(), Some( nickname_res.unwrap() ))
    }
    Err( DataGramError::ErrorReadingSDBConfig )
  }
}

// len is number of bytes
pub fn validate_dg_uuid ( uuid: &str ) -> bool { uuid.len() == ByteLengths::UUID as usize }

pub fn validate_dg_label ( label: &str ) -> bool { label.len() == ByteLengths::CommonString as usize }

// ---------------------------------------------------------------------------------------------------------------------
pub struct DataGramWriter<'a> { file: &'a mut File }
impl DataGramWriter<'_> 
{
  pub fn new ( file: &mut File ) -> DataGramWriter { DataGramWriter { file: file }}
  pub fn write_uuid ( uuid: String ) {}
  pub fn write_label () {}
  pub fn write_sdb_config ( &mut self,  ) // !! return dg error
  {
    
    
    // !! write using as_bytes
    /* !!
    let mut stream = BufWriter::new( file );
    stream.write( &sdb_config_res.unwrap().to_bytes() ).unwrap();
    stream.flush().unwrap();
    */
  }
}

pub struct DataGramReader<'a> { file: &'a mut File }
impl DataGramReader<'_> 
{
  pub fn new ( file: &mut File ) -> DataGramReader { DataGramReader { file: file }}
  pub fn read_uuid () {}
  pub fn read_label () {}
  pub fn read_sdb_config ( &mut self ) {}
}
// ---------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests 
{
  use super::*;
  use std::path::PathBuf;
  use std::fs::{ OpenOptions, File };
  use std::io::{ BufWriter, Error, Read, Write };

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
  fn test_str_to_bytes () 
  {
    let bytes: Vec<_> = str_to_bytes( String::from( "test" ) ).to_vec();
    assert_eq!( bytes, [116, 101, 115, 116] );
  }

  #[test]
  fn test_str_from_bytes () 
  {
    let res = str_from_bytes( &[116, 101, 115, 116] );
    assert_eq!( res.is_ok(), true );
    assert_eq!( res.unwrap(), "test" );
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
      pad_str( ByteLengths::CommonString as usize, String::from( "test" )), 
      String::from( "test\\:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::" ));

    assert_eq!( 
      pad_str( 
        ByteLengths::CommonString as usize, 
        String::from( "1234::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::" )), 
        String::from( "1234::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::" ));

    assert_eq!( 
      pad_str( 
        ByteLengths::CommonString as usize, 
        String::from( "nickname" )), 
        String::from( "nickname\\:::::::::::::::::::::::::::::::::::::::::::::::::::::::" ));
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
    let double_uuid = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" );
    assert_eq!( double_uuid.into_bytes().len(), 36 );
  }

  #[test]
  fn test_str_length () 
  {
    let astr = String::from( "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" );
    assert_eq!( astr.into_bytes().len(), 64 );
  }

  #[test]
  fn test_validate_dg_uuid () 
  {
    assert_eq!( validate_dg_uuid( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), true );
    assert_eq!( validate_dg_uuid( "67e55044-10b1-426f-9247-bb680e5fe0c88" ), false );
    assert_eq!( validate_dg_uuid( "67e55044-10b1-426f-9247-bb680e5fe0c" ), false );
    assert_eq!( validate_dg_uuid( "" ), false );
  }

  #[test]
  fn test_validate_dg_label () 
  {
    assert_eq!( validate_dg_label( &pad_str( ByteLengths::CommonString as usize, String::from( "test" ))), true );
    assert_eq!( validate_dg_label( "test" ), false );
    assert_eq!( validate_dg_label( "" ), false );
    assert_eq!( validate_dg_label( "----------------------------------------------------------------------" ), false );
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
  fn test_common_none () 
  {
    assert_eq!( COMMON_NONE.bytes().len(), ByteLengths::CommonString as usize );
  }

  #[test]
  fn test_dgsdbconfig () 
  {
    let dg_config_1_res = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c" ), 
      Some( String::from( "nickname" )));

    assert_eq!( dg_config_1_res.is_err(), true );
    assert_eq!( dg_config_1_res, Err( DataGramError::InvalidDGUUID ));

    let dg_config_2_res = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      Some( String::from( "----------------------------------------------------------------------" )));

    assert_eq!( dg_config_2_res.is_err(), true );
    assert_eq!( dg_config_2_res, Err( DataGramError::InvalidDGCommonString ));
    
    let dg_config_3_res = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      Some( String::from( "nickname" )));

    assert_eq!( dg_config_3_res.is_ok(), true );

    let dg_config_3 = dg_config_3_res.unwrap();
    assert_eq!( dg_config_3.build_id.len(), ByteLengths::UUID as usize );
    assert_eq!( dg_config_3.nickname.len(), ByteLengths::CommonString as usize );
  }

  #[test]
  fn test_datagram_writer () 
  {
    let path = PathBuf::from( "/platonic3d/subgraphdbv2/test_data/sf/unit_tests/test_datagram_writer.sdb" );
    let file_res: Result<File, Error> = OpenOptions::new()
      .read( true )
      .write( true )
      .truncate( true )
      .create( true )
      .open( path );
    //assert_eq!( COMMON_NONE.bytes().len(), ByteLengths::CommonString as usize );
  }

  #[test]
  fn test_datagram_reader () 
  {
    //assert_eq!( COMMON_NONE.bytes().len(), ByteLengths::CommonString as usize );
  }

  #[test]
  fn test_dgsdbconfig_to_bytes () 
  {
    let dg_config_res = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      Some( String::from( "nickname" )));

    assert_eq!( dg_config_res.is_ok(), true );
    assert_eq!( dg_config_res.unwrap().to_bytes().len(), 100 );
  }

  #[test]
  fn test_dgsdbconfig_from_bytes () 
  {
    // --- some
    let dg_config_res = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      Some( String::from( "nickname" )));
    assert_eq!( dg_config_res.is_ok(), true );

    let bytes = dg_config_res.unwrap().to_bytes();
    assert_eq!( bytes.len(), 100 );
    
    let res = DGSDBConfig::from_bytes( bytes );   
    assert_eq!( res.is_ok(), true );
    
    let config = res.unwrap();
    assert_eq!( config.build_id, String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ));
    assert_eq!( config.nickname, pad_str( ByteLengths::CommonString as usize, String::from( "nickname" )));
    
    // -- none
    let dg_config_res1 = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      None );
    assert_eq!( dg_config_res1.is_ok(), true );

    let bytes1 = dg_config_res1.unwrap().to_bytes();
    assert_eq!( bytes1.len(), 100 );

    let res1 = DGSDBConfig::from_bytes( bytes1 );   
    assert_eq!( res1.is_ok(), true );
    
    let config1 = res1.unwrap();
    assert_eq!( config1.build_id, String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ));
    assert_eq!( config1.nickname, COMMON_NONE );


    // -- error
    let dg_config_res2 = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      None );
    assert_eq!( dg_config_res2.is_ok(), true );

    let bytes2 = dg_config_res2.unwrap().to_bytes();
    assert_eq!( bytes2.len(), 100 );

    let res2 = DGSDBConfig::from_bytes( bytes2[0..99].to_vec() );   
    assert_eq!( res2.is_err(), true );
  }

  #[test]
  fn test_parse_padded_str () 
  {
    let pad1 = pad_str( 10, String::from( "a" ));

    let pad1_str = parse_padded_str( &pad1 );
    assert_eq!( pad1_str, String::from( "a" ));

    let pad2 = pad_str( ByteLengths::CommonString as usize, String::from( "nickname" ));
    let pad2_str = parse_padded_str( &pad2 );
    assert_eq!( pad2_str, String::from( "nickname" ));

    let pad3_str = parse_padded_str( "no-padding" );
    assert_eq!( pad3_str, String::from( "no-padding" ));
  }
}