use std::string::{ FromUtf8Error };

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
fn gen_pad_str ( length: usize ) -> String 
{
  if length == 0 { return String::from( "" )}

  let mut ret = String::from( "\\" );
  while &ret.len() < &length { ret.push_str( ":" ); }
  ret
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

#[cfg(test)]
mod tests 
{
  use super::*;

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
}
