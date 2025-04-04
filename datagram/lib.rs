static STR_LEN_EXCEEDED: &'static str = "::str_len_exceeded";
static PAD_CANNOT_BE_ZERO: &'static str = "::pad_cannot_be_zero";
static PADDING_ERROR: &'static str = "::padding_error";

/// Pad String to given length
pub fn pad_str ( max_len: usize, str_to_pad: String ) -> Result<String, &'static str> 
{
  let start_size: usize = str_to_pad.clone().into_bytes().len();
  if start_size > max_len { return Err( STR_LEN_EXCEEDED )}
  if start_size == max_len { return Ok( str_to_pad )}

  let padding = gen_pad_str( &max_len - &start_size );
  if padding.is_err() { return Err( PADDING_ERROR ) }
  Ok( str_to_pad + &padding.unwrap() )
}

// Generate padding string of given length
fn gen_pad_str ( length: usize ) -> Result<String, &'static str>  
{
  if length == 0 { return Err( PAD_CANNOT_BE_ZERO ) }

  let mut ret = String::from( "\\" );
  while &ret.len() < &length { ret.push_str( ":" ); }
  Ok( ret )
}

// ---------------------------------------------------------------------------------------------------------------------
pub enum ByteLengths 
{
  BuildVersion = 36,
  CommonString = 64, // names, labels, etc
  DGU64 = 8,
}

/// Data Gram Types
#[derive(Debug, PartialEq)]
pub enum DataGramType
{
  DGString,
  DGPaddedString,
  DGInteger,
  DGPaddedInteger,
  DGU64,
  DGUUID,
}

#[derive(Debug)]
pub struct DataGram {}

#[derive(Debug)]
pub struct DataSheet
{ 
}

#[derive(Debug)]
pub struct DataBook
{ 
}
// ---------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests 
{
  use super::*;

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
    assert_eq!( pad.unwrap().into_bytes().len(), 10 );

    let pad_err = gen_pad_str( 0 );
    assert_eq!( pad_err.is_err(), true );
    assert_eq!( pad_err, Err( PAD_CANNOT_BE_ZERO ) );
  }

  #[test]
  fn test_pad_str () 
  {
    let pad1 = pad_str( 10, String::from( "a" ) );    
    assert_eq!( pad1.clone().unwrap(), String::from( "a\\::::::::" ) );
    assert_eq!( pad1.clone().unwrap().into_bytes().len(), 10 );
  }
}