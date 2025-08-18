/*
  Basic Types

  Signed Integer Types

  4. KeyValI16 - For i16 values (16-bit signed)
  5. KeyValI32 - For i32 values (32-bit signed)
  6. KeyValI64 - For i64 values (64-bit signed)
  7. KeyValI128 - For i128 values (128-bit signed)

  Unsigned Integer Types

  9. KeyValU16 - For u16 values (16-bit unsigned)
  10. KeyValU32 - For u32 values (32-bit unsigned)
  11. KeyValU64 - For u64 values (64-bit unsigned)
  12. KeyValU128 - For u128 values (128-bit unsigned)

  Floating-Point Types

  13. KeyValF32 - For f32 values (32-bit float)
  14. KeyValF64 - For f64 values (64-bit float)
*/

use crate::common::{ FALSE_AFFIX, TRUE_AFFIX };
use crate::common::{ validators::validate_label, LABEL_BYTES };
use crate::utils::gen_pad_str;

/// Key-Value Pair
pub trait KVP 
{
  fn unwrap (&self) -> Vec<u8>;
}

/*
  Both the key and value are Label-length strings.
  unwrap -> LabelLabel
*/
#[derive( Debug, Clone )]
pub struct KVPStringGram { pub key: String, pub val: String }
impl KVPStringGram 
{
  pub fn new ( key: String, val: String ) -> Result<KVPStringGram, String> 
  {
    if validate_label( &key ) && validate_label( &val ) 
    {
      return Ok( KVPStringGram { key: key, val: val } );
    }
    Err( String::from( "KVPString invalid" ))
  }
}
impl KVP for KVPStringGram 
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let val_padding = gen_pad_str( LABEL_BYTES - &self.val.bytes().len() );
    ( self.key.clone() + &key_padding + &self.val.clone() + &val_padding ).into_bytes()
  }
}


#[derive( Debug, Clone )]
pub struct KVPBooleanGram { pub key: String, pub val: bool }
impl KVPBooleanGram 
{
  pub fn new ( key: String, val: bool ) -> Result<KVPBooleanGram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPBooleanGram { key: key, val: val } );
    }
    Err( String::from( "KVPBoolean invalid" ))
  }
}
impl KVP for KVPBooleanGram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let bool_str = if self.val { TRUE_AFFIX } else { FALSE_AFFIX };
    ( self.key.clone() + &key_padding + bool_str ).into_bytes()
  }
}


#[derive( Debug, Clone )]
pub struct KVPi8Gram { pub key: String, pub val: i8 }
impl KVPi8Gram 
{
  pub fn new ( key: String, val: i8 ) -> Result<KVPi8Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPi8Gram { key: key, val: val } );
    }
    Err( String::from( "KVPi8 invalid" ))
  }
}
impl KVP for KVPi8Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[derive( Debug, Clone )]
pub struct KVPu8Gram { pub key: String, pub val: u8 }
impl KVPu8Gram 
{
  pub fn new ( key: String, val: u8 ) -> Result<KVPu8Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPu8Gram { key: key, val: val } );
    }
    Err( String::from( "KVPu8 invalid" ))
  }
}
impl KVP for KVPu8Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[derive( Debug, Clone )]
pub struct KVPu16Gram { pub key: String, pub val: u16 }
impl KVPu16Gram 
{
  pub fn new ( key: String, val: u16 ) -> Result<KVPu16Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPu16Gram { key: key, val: val } );
    }
    Err( String::from( "KVPu16 invalid" ))
  }
}
impl KVP for KVPu16Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[derive( Debug, Clone )]
pub struct KVPi16Gram { pub key: String, pub val: i16 }
impl KVPi16Gram 
{
  pub fn new ( key: String, val: i16 ) -> Result<KVPi16Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPi16Gram { key: key, val: val } );
    }
    Err( String::from( "KVPi16 invalid" ))
  }
}
impl KVP for KVPi16Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[derive( Debug, Clone )]
pub struct KVPu32Gram { pub key: String, pub val: u32 }
impl KVPu32Gram 
{
  pub fn new ( key: String, val: u32 ) -> Result<KVPu32Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPu32Gram { key: key, val: val } );
    }
    Err( String::from( "KVPu32 invalid" ))
  }
}
impl KVP for KVPu32Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[derive( Debug, Clone )]
pub struct KVPi32Gram { pub key: String, pub val: i32 }
impl KVPi32Gram 
{
  pub fn new ( key: String, val: i32 ) -> Result<KVPi32Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPi32Gram { key: key, val: val } );
    }
    Err( String::from( "KVPi32 invalid" ))
  }
}
impl KVP for KVPi32Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[derive( Debug, Clone )]
pub struct KVPu64Gram { pub key: String, pub val: u64 }
impl KVPu64Gram 
{
  pub fn new ( key: String, val: u64 ) -> Result<KVPu64Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPu64Gram { key: key, val: val } );
    }
    Err( String::from( "KVPu64 invalid" ))
  }
}
impl KVP for KVPu64Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[derive( Debug, Clone )]
pub struct KVPi64Gram { pub key: String, pub val: i64 }
impl KVPi64Gram 
{
  pub fn new ( key: String, val: i64 ) -> Result<KVPi64Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPi64Gram { key: key, val: val } );
    }
    Err( String::from( "KVPi64 invalid" ))
  }
}
impl KVP for KVPi64Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[derive( Debug, Clone )]
pub struct KVPu128Gram { pub key: String, pub val: u128 }
impl KVPu128Gram 
{
  pub fn new ( key: String, val: u128 ) -> Result<KVPu128Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPu128Gram { key: key, val: val } );
    }
    Err( String::from( "KVPu128 invalid" ))
  }
}
impl KVP for KVPu128Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[derive( Debug, Clone )]
pub struct KVPi128Gram { pub key: String, pub val: i128 }
impl KVPi128Gram 
{
  pub fn new ( key: String, val: i128 ) -> Result<KVPi128Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPi128Gram { key: key, val: val } );
    }
    Err( String::from( "KVPi128 invalid" ))
  }
}
impl KVP for KVPi128Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[cfg(test)]
mod tests 
{
  use super::*;
  use crate::common::{ 
    I8_BYTES, I16_BYTES, I32_BYTES, I64_BYTES, I128_BYTES, ROW_AFFIX_BYTES, U8_BYTES, U16_BYTES, U32_BYTES, U64_BYTES, 
    U128_BYTES 
  };

  // KVPStringGram Tests
  #[test]
  fn test_kvp_string_gram_new () 
  {
    let kvp = KVPStringGram::new( String::from("name"), String::from("Alice") );
    assert!( kvp.is_ok() );
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "name");
    assert_eq!(kvp_unwrapped.val, "Alice");
  }

  #[test]
  fn test_kvp_string_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPStringGram::new(too_long_key, String::from("value"));
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPString invalid");
  }

  #[test]
  fn test_kvp_string_gram_new_invalid_val() 
  {
    let too_long_val = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPStringGram::new(String::from("key"), too_long_val);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPString invalid");
  }

  #[test]
  fn test_kvp_string_gram_unwrap() 
  {
    let kvp = KVPStringGram::new(String::from("test"), String::from("value")).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES * 2); // Key + Value, both padded to LABEL_BYTES
  }

  #[test]
  fn test_kvp_string_gram_empty_strings() 
  {
    let kvp = KVPStringGram::new(String::from(""), String::from(""));
    assert!(kvp.is_ok());
    let unwrapped = kvp.unwrap().unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES * 2);
  }

  #[test]
  fn test_kvp_string_gram_unicode() 
  {
    let kvp = KVPStringGram::new(String::from("café"), String::from("niño"));
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "café");
    assert_eq!(kvp_unwrapped.val, "niño");
    let unwrapped = kvp_unwrapped.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES * 2);
  }

  // KVPBooleanGram Tests
  #[test]
  fn test_kvp_boolean_gram_new_valid() 
  {
    let kvp_true = KVPBooleanGram::new(String::from("active"), true);
    assert!(kvp_true.is_ok());
    let kvp_true_unwrapped = kvp_true.unwrap();
    assert_eq!(kvp_true_unwrapped.key, "active");
    assert_eq!(kvp_true_unwrapped.val, true);

    let kvp_false = KVPBooleanGram::new(String::from("enabled"), false);
    assert!(kvp_false.is_ok());
    let kvp_false_unwrapped = kvp_false.unwrap();
    assert_eq!(kvp_false_unwrapped.key, "enabled");
    assert_eq!(kvp_false_unwrapped.val, false);
  }

  #[test]
  fn test_kvp_boolean_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPBooleanGram::new(too_long_key, true);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPBoolean invalid");
  }

  #[test]
  fn test_kvp_boolean_gram_unwrap_true() 
  {
    let kvp = KVPBooleanGram::new(String::from("isActive"), true).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!( unwrapped.len(), LABEL_BYTES + ROW_AFFIX_BYTES );
  }

  #[test]
  fn test_kvp_boolean_gram_unwrap_false() 
  {
    let kvp = KVPBooleanGram::new(String::from("isEnabled"), false).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!( unwrapped.len(), LABEL_BYTES + ROW_AFFIX_BYTES );
  }

  // KVPi8Gram Tests
  #[test]
  fn test_kvp_i8_gram_new_valid() 
  {
    let kvp = KVPi8Gram::new(String::from("temperature"), -10);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "temperature");
    assert_eq!(kvp_unwrapped.val, -10);
  }

  #[test]
  fn test_kvp_i8_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPi8Gram::new(too_long_key, 42);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPi8 invalid");
  }

  #[test]
  fn test_kvp_i8_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPi8Gram::new(String::from("min"), i8::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, -128);

    // Test maximum value
    let kvp_max = KVPi8Gram::new(String::from("max"), i8::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 127);

    // Test zero
    let kvp_zero = KVPi8Gram::new(String::from("zero"), 0);
    assert!(kvp_zero.is_ok());
    assert_eq!(kvp_zero.as_ref().unwrap().val, 0);
  }

  #[test]
  fn test_kvp_i8_gram_unwrap() 
  {
    let kvp = KVPi8Gram::new(String::from("level"), 42).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + I8_BYTES);
  }

  #[test]
  fn test_kvp_i8_gram_unwrap_negative() 
  {
    let kvp = KVPi8Gram::new(String::from("temp"), -50).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + I8_BYTES);
  }

  #[test]
  fn test_kvp_i8_gram_unicode_key() 
  {
    let kvp = KVPi8Gram::new(String::from("café_temp"), -5);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "café_temp");
    assert_eq!(kvp_unwrapped.val, -5);
  }

  // KVPu8Gram Tests
  #[test]
  fn test_kvp_u8_gram_new_valid() 
  {
    let kvp = KVPu8Gram::new(String::from("age"), 25);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "age");
    assert_eq!(kvp_unwrapped.val, 25);
  }

  #[test]
  fn test_kvp_u8_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPu8Gram::new(too_long_key, 100);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPu8 invalid");
  }

  #[test]
  fn test_kvp_u8_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPu8Gram::new(String::from("min"), u8::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, 0);

    // Test maximum value
    let kvp_max = KVPu8Gram::new(String::from("max"), u8::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 255);
  }

  #[test]
  fn test_kvp_u8_gram_unwrap() 
  {
    let kvp = KVPu8Gram::new(String::from("score"), 200).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U8_BYTES);
  }

  #[test]
  fn test_kvp_u8_gram_max_length_key() 
  {
    let max_key = "a".repeat(LABEL_BYTES);
    let kvp = KVPu8Gram::new(max_key.clone(), 128);
    assert!(kvp.is_ok());
    let unwrapped = kvp.unwrap().unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U8_BYTES);
  }

  #[test]
  fn test_kvp_u8_gram_unicode_key() 
  {
    let kvp = KVPu8Gram::new(String::from("niño_age"), 8);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "niño_age");
    assert_eq!(kvp_unwrapped.val, 8);
  }

  // KVPu16Gram Tests
  #[test]
  fn test_kvp_u16_gram_new_valid() 
  {
    let kvp = KVPu16Gram::new(String::from("port"), 8080);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "port");
    assert_eq!(kvp_unwrapped.val, 8080);
  }

  #[test]
  fn test_kvp_u16_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPu16Gram::new(too_long_key, 65535);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPu16 invalid");
  }

  #[test]
  fn test_kvp_u16_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPu16Gram::new(String::from("min"), u16::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, 0);

    // Test maximum value
    let kvp_max = KVPu16Gram::new(String::from("max"), u16::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 65535);

    // Test common values
    let kvp_mid = KVPu16Gram::new(String::from("mid"), 32768);
    assert!(kvp_mid.is_ok());
    assert_eq!(kvp_mid.as_ref().unwrap().val, 32768);
  }

  #[test]
  fn test_kvp_u16_gram_unwrap() 
  {
    let kvp = KVPu16Gram::new(String::from("year"), 2024).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U16_BYTES);
  }

  #[test]
  fn test_kvp_u16_gram_unicode_key() 
  {
    let kvp = KVPu16Gram::new(String::from("año_count"), 2025);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "año_count");
    assert_eq!(kvp_unwrapped.val, 2025);
  }

  // KVPi16Gram Tests
  #[test]
  fn test_kvp_i16_gram_new_valid() 
  {
    let kvp = KVPi16Gram::new(String::from("temperature"), -100);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "temperature");
    assert_eq!(kvp_unwrapped.val, -100);
  }

  #[test]
  fn test_kvp_i16_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPi16Gram::new(too_long_key, -32768);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPi16 invalid");
  }

  #[test]
  fn test_kvp_i16_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPi16Gram::new(String::from("min"), i16::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, -32768);

    // Test maximum value
    let kvp_max = KVPi16Gram::new(String::from("max"), i16::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 32767);

    // Test zero
    let kvp_zero = KVPi16Gram::new(String::from("zero"), 0);
    assert!(kvp_zero.is_ok());
    assert_eq!(kvp_zero.as_ref().unwrap().val, 0);

    // Test negative one
    let kvp_neg_one = KVPi16Gram::new(String::from("neg_one"), -1);
    assert!(kvp_neg_one.is_ok());
    assert_eq!(kvp_neg_one.as_ref().unwrap().val, -1);
  }

  #[test]
  fn test_kvp_i16_gram_unwrap() 
  {
    let kvp = KVPi16Gram::new(String::from("altitude"), 1500).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + I16_BYTES);
  }

  #[test]
  fn test_kvp_i16_gram_unicode_key() 
  {
    let kvp = KVPi16Gram::new(String::from("深度_meters"), -200);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "深度_meters");
    assert_eq!(kvp_unwrapped.val, -200);
  }

  // KVPu32Gram Tests
  #[test]
  fn test_kvp_u32_gram_new_valid() 
  {
    let kvp = KVPu32Gram::new(String::from("population"), 1000000);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "population");
    assert_eq!(kvp_unwrapped.val, 1000000);
  }

  #[test]
  fn test_kvp_u32_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPu32Gram::new(too_long_key, 4294967295);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPu32 invalid");
  }

  #[test]
  fn test_kvp_u32_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPu32Gram::new(String::from("min"), u32::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, 0);

    // Test maximum value
    let kvp_max = KVPu32Gram::new(String::from("max"), u32::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 4294967295);

    // Test common boundary values
    let kvp_mid = KVPu32Gram::new(String::from("mid"), 2147483648); // 2^31
    assert!(kvp_mid.is_ok());
    assert_eq!(kvp_mid.as_ref().unwrap().val, 2147483648);
  }

  #[test]
  fn test_kvp_u32_gram_unwrap() 
  {
    let kvp = KVPu32Gram::new(String::from("bytes_count"), 1073741824).unwrap(); // 1GB in bytes
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U32_BYTES);
  }


  #[test]
  fn test_kvp_u32_gram_unicode_key() 
  {
    let kvp = KVPu32Gram::new(String::from("размер_bytes"), 2048000);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "размер_bytes");
    assert_eq!(kvp_unwrapped.val, 2048000);
  }

  // KVPi32Gram Tests
  #[test]
  fn test_kvp_i32_gram_new_valid() 
  {
    let kvp = KVPi32Gram::new(String::from("temperature_c"), -25);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "temperature_c");
    assert_eq!(kvp_unwrapped.val, -25);
  }

  #[test]
  fn test_kvp_i32_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPi32Gram::new(too_long_key, -2147483648);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPi32 invalid");
  }

  #[test]
  fn test_kvp_i32_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPi32Gram::new(String::from("min"), i32::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, -2147483648);

    // Test maximum value
    let kvp_max = KVPi32Gram::new(String::from("max"), i32::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 2147483647);

    // Test zero
    let kvp_zero = KVPi32Gram::new(String::from("zero"), 0);
    assert!(kvp_zero.is_ok());
    assert_eq!(kvp_zero.as_ref().unwrap().val, 0);

    // Test negative one
    let kvp_neg_one = KVPi32Gram::new(String::from("neg_one"), -1);
    assert!(kvp_neg_one.is_ok());
    assert_eq!(kvp_neg_one.as_ref().unwrap().val, -1);
  }

  #[test]
  fn test_kvp_i32_gram_unwrap() 
  {
    let kvp = KVPi32Gram::new(String::from("profit_loss"), -150000).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + I32_BYTES);
  }

  // KVPu64Gram Tests
  #[test]
  fn test_kvp_u64_gram_new_valid() 
  {
    let kvp = KVPu64Gram::new(String::from("file_size"), 1099511627776); // 1TB in bytes
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "file_size");
    assert_eq!(kvp_unwrapped.val, 1099511627776);
  }

  #[test]
  fn test_kvp_u64_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPu64Gram::new(too_long_key, 18446744073709551615);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPu64 invalid");
  }

  #[test]
  fn test_kvp_u64_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPu64Gram::new(String::from("min"), u64::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, 0);

    // Test maximum value
    let kvp_max = KVPu64Gram::new(String::from("max"), u64::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 18446744073709551615);

    // Test common boundary values
    let kvp_mid = KVPu64Gram::new(String::from("mid"), 9223372036854775808); // 2^63
    assert!(kvp_mid.is_ok());
    assert_eq!(kvp_mid.as_ref().unwrap().val, 9223372036854775808);

    // Test large practical values
    let kvp_petabyte = KVPu64Gram::new(String::from("petabyte"), 1125899906842624); // 1PB in bytes
    assert!(kvp_petabyte.is_ok());
    assert_eq!(kvp_petabyte.as_ref().unwrap().val, 1125899906842624);
  }

  #[test]
  fn test_kvp_u64_gram_unwrap() 
  {
    let kvp = KVPu64Gram::new(String::from("timestamp"), 1735689600000).unwrap(); // Timestamp in milliseconds
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U64_BYTES);
  }

  #[test]
  fn test_kvp_u64_gram_max_length_key() 
  {
    let max_key = "a".repeat(LABEL_BYTES);
    let kvp = KVPu64Gram::new(max_key.clone(), 9999999999999999999);
    assert!(kvp.is_ok());
    let unwrapped = kvp.unwrap().unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U64_BYTES);
  }

  #[test]
  fn test_kvp_u64_gram_unicode_key() 
  {
    let kvp = KVPu64Gram::new(String::from("容量_bytes"), 5368709120); // 5GB in bytes
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "容量_bytes");
    assert_eq!(kvp_unwrapped.val, 5368709120);
  }

  // KVPi64Gram Tests
  #[test]
  fn test_kvp_i64_gram_new_valid() 
  {
    let kvp = KVPi64Gram::new(String::from("balance"), -9223372036854775808); // i64::MIN
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "balance");
    assert_eq!(kvp_unwrapped.val, -9223372036854775808);
  }

  #[test]
  fn test_kvp_i64_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPi64Gram::new(too_long_key, -1000000000000000000);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPi64 invalid");
  }

  #[test]
  fn test_kvp_i64_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPi64Gram::new(String::from("min"), i64::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, -9223372036854775808);

    // Test maximum value
    let kvp_max = KVPi64Gram::new(String::from("max"), i64::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 9223372036854775807);

    // Test zero
    let kvp_zero = KVPi64Gram::new(String::from("zero"), 0);
    assert!(kvp_zero.is_ok());
    assert_eq!(kvp_zero.as_ref().unwrap().val, 0);

    // Test negative one
    let kvp_neg_one = KVPi64Gram::new(String::from("neg_one"), -1);
    assert!(kvp_neg_one.is_ok());
    assert_eq!(kvp_neg_one.as_ref().unwrap().val, -1);

    // Test positive one
    let kvp_pos_one = KVPi64Gram::new(String::from("pos_one"), 1);
    assert!(kvp_pos_one.is_ok());
    assert_eq!(kvp_pos_one.as_ref().unwrap().val, 1);
  }

  #[test]
  fn test_kvp_i64_gram_unwrap() 
  {
    let kvp = KVPi64Gram::new(String::from("timestamp"), 1735689600000).unwrap(); // Unix timestamp in milliseconds
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + I64_BYTES);
  }

  #[test]
  fn test_kvp_i64_gram_unwrap_negative() 
  {
    let kvp = KVPi64Gram::new(String::from("deficit"), -500000000000).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + I64_BYTES);
  }

  // KVPu128Gram Tests
  #[test]
  fn test_kvp_u128_gram_new_valid() 
  {
    let kvp = KVPu128Gram::new(String::from("large_number"), 340282366920938463463374607431768211455); // u128::MAX
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "large_number");
    assert_eq!(kvp_unwrapped.val, 340282366920938463463374607431768211455);
  }

  #[test]
  fn test_kvp_u128_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPu128Gram::new(String::from("min"), u128::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, 0);

    // Test maximum value
    let kvp_max = KVPu128Gram::new(String::from("max"), u128::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 340282366920938463463374607431768211455);

    // Test powers of 2 boundaries
    let kvp_2_64 = KVPu128Gram::new(String::from("pow_64"), 1u128 << 64);
    assert!(kvp_2_64.is_ok());
    assert_eq!(kvp_2_64.as_ref().unwrap().val, 18446744073709551616);

    let kvp_2_127 = KVPu128Gram::new(String::from("pow_127"), 1u128 << 127);
    assert!(kvp_2_127.is_ok());
    assert_eq!(kvp_2_127.as_ref().unwrap().val, 170141183460469231731687303715884105728);
  }

  #[test]
  fn test_kvp_u128_gram_unwrap() 
  {
    let kvp = KVPu128Gram::new(String::from("uuid_num"), 12345678901234567890123456789012345678).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U128_BYTES);
  }

  #[test]
  fn test_kvp_u128_gram_max_length_key() 
  {
    let max_key = "a".repeat(LABEL_BYTES);
    let kvp = KVPu128Gram::new(max_key.clone(), u128::MAX);
    assert!(kvp.is_ok());
    let unwrapped = kvp.unwrap().unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U128_BYTES);
  }

  #[test]
  fn test_kvp_u128_gram_unicode_key() 
  {
    let kvp = KVPu128Gram::new(String::from("巨大数字_value"), 999999999999999999999999999999999999); 
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "巨大数字_value");
    assert_eq!(kvp_unwrapped.val, 999999999999999999999999999999999999);
  }

  // KVPi128Gram Tests
  #[test]
  fn test_kvp_i128_gram_new_valid() 
  {
    let kvp = KVPi128Gram::new(String::from("large_signed"), -170141183460469231731687303715884105728); // i128::MIN
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "large_signed");
    assert_eq!(kvp_unwrapped.val, -170141183460469231731687303715884105728);
  }

  #[test]
  fn test_kvp_i128_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPi128Gram::new(too_long_key, -123456789012345678901234567890);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPi128 invalid");
  }

  #[test]
  fn test_kvp_i128_gram_edge_values() 
  {
    // Test minimum value (most negative)
    let kvp_min = KVPi128Gram::new(String::from("min"), i128::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, -170141183460469231731687303715884105728);

    // Test maximum value (most positive)
    let kvp_max = KVPi128Gram::new(String::from("max"), i128::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 170141183460469231731687303715884105727);

    // Test zero
    let kvp_zero = KVPi128Gram::new(String::from("zero"), 0);
    assert!(kvp_zero.is_ok());
    assert_eq!(kvp_zero.as_ref().unwrap().val, 0);

    // Test negative one
    let kvp_neg_one = KVPi128Gram::new(String::from("neg_one"), -1);
    assert!(kvp_neg_one.is_ok());
    assert_eq!(kvp_neg_one.as_ref().unwrap().val, -1);

    // Test positive one
    let kvp_pos_one = KVPi128Gram::new(String::from("pos_one"), 1);
    assert!(kvp_pos_one.is_ok());
    assert_eq!(kvp_pos_one.as_ref().unwrap().val, 1);
  }

  #[test]
  fn test_kvp_i128_gram_unwrap() 
  {
    let kvp = KVPi128Gram::new(String::from("value"), 12345678901234567890123456789012345678).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + I128_BYTES);
  }

  #[test]
  fn test_kvp_i128_gram_unwrap_negative() 
  {
    let kvp = KVPi128Gram::new(String::from("deficit"), -98765432109876543210987654321098765432).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + I128_BYTES);
  }

  #[test]
  fn test_kvp_i128_gram_max_length_key() 
  {
    let max_key = "a".repeat(LABEL_BYTES);
    let kvp = KVPi128Gram::new(max_key.clone(), i128::MIN);
    assert!(kvp.is_ok());
    let unwrapped = kvp.unwrap().unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + I128_BYTES);
  }
}