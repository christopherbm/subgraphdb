use std::string::FromUtf8Error;

pub fn str_to_bytes ( s: String ) -> Vec<u8> { s.into_bytes() }

pub fn str_from_bytes ( bytes: Vec<u8> ) -> Result<String, FromUtf8Error> { String::from_utf8( bytes )}

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
    let res = str_from_bytes( [116, 101, 115, 116].to_vec() );
    assert_eq!( res.is_ok(), true );
    assert_eq!( res.unwrap(), "test" );
  }
}
