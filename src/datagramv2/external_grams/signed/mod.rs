use crate::common::validators::validate_label;
use crate::common::LABEL_BYTES;
use crate::datagramv2::external_grams::traits::KVP;
use crate::utils::gen_pad_str;

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
  use crate::common::{ I128_BYTES, I16_BYTES, I32_BYTES, I64_BYTES, I8_BYTES };

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