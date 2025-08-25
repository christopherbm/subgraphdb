use crate::common::validators::validate_label;
use crate::common::LABEL_BYTES;
use crate::datagramv2::external_grams::traits::KVP;
use crate::utils::gen_pad_str;


#[derive( Debug, Clone )]
pub struct KVPf32Gram { pub key: String, pub val: f32 }
impl KVPf32Gram 
{
  pub fn new ( key: String, val: f32 ) -> Result<KVPf32Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPf32Gram { key: key, val: val } );
    }
    Err( String::from( "KVPf32 invalid" ))
  }
}
impl KVP for KVPf32Gram
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
  use crate::common::F32_BYTES;

  // KVPf32Gram Tests
  #[test]
  fn test_kvp_f32_gram_new_valid() 
  {
    let kvp = KVPf32Gram::new(String::from("temperature"), 23.5);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "temperature");
    assert_eq!(kvp_unwrapped.val, 23.5);
  }

  #[test]
  fn test_kvp_f32_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPf32Gram::new(too_long_key, 3.14159);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPf32 invalid");
  }

  #[test]
  fn test_kvp_f32_gram_edge_values() 
  {
    // Test minimum positive value
    let kvp_min_pos = KVPf32Gram::new(String::from("min_positive"), f32::MIN_POSITIVE);
    assert!(kvp_min_pos.is_ok());
    assert_eq!(kvp_min_pos.as_ref().unwrap().val, f32::MIN_POSITIVE);

    // Test maximum value
    let kvp_max = KVPf32Gram::new(String::from("max"), f32::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, f32::MAX);

    // Test minimum value (most negative)
    let kvp_min = KVPf32Gram::new(String::from("min"), f32::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, f32::MIN);

    // Test zero
    let kvp_zero = KVPf32Gram::new(String::from("zero"), 0.0);
    assert!(kvp_zero.is_ok());
    assert_eq!(kvp_zero.as_ref().unwrap().val, 0.0);

    // Test negative zero
    let kvp_neg_zero = KVPf32Gram::new(String::from("neg_zero"), -0.0);
    assert!(kvp_neg_zero.is_ok());
    assert_eq!(kvp_neg_zero.as_ref().unwrap().val, -0.0);
  }

  #[test]
  fn test_kvp_f32_gram_special_values() 
  {
    // Test infinity
    let kvp_inf = KVPf32Gram::new(String::from("infinity"), f32::INFINITY);
    assert!(kvp_inf.is_ok());
    assert!(kvp_inf.as_ref().unwrap().val.is_infinite());
    assert!(kvp_inf.as_ref().unwrap().val.is_sign_positive());

    // Test negative infinity
    let kvp_neg_inf = KVPf32Gram::new(String::from("neg_infinity"), f32::NEG_INFINITY);
    assert!(kvp_neg_inf.is_ok());
    assert!(kvp_neg_inf.as_ref().unwrap().val.is_infinite());
    assert!(kvp_neg_inf.as_ref().unwrap().val.is_sign_negative());

    // Test NaN
    let kvp_nan = KVPf32Gram::new(String::from("not_a_number"), f32::NAN);
    assert!(kvp_nan.is_ok());
    assert!(kvp_nan.as_ref().unwrap().val.is_nan());
  }

  #[test]
  fn test_kvp_f32_gram_unwrap() 
  {
    let kvp = KVPf32Gram::new(String::from("pi"), 3.14159265).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + F32_BYTES);
  }

  #[test]
  fn test_kvp_f32_gram_unwrap_negative() 
  {
    let kvp = KVPf32Gram::new(String::from("negative"), -123.456).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + F32_BYTES);
  }

  #[test]
  fn test_kvp_f32_gram_unwrap_structure() 
  {
    let kvp = KVPf32Gram::new(String::from("test"), 42.5).unwrap();
    let unwrapped = kvp.unwrap();
    
    // Verify the structure matches expected format
    assert_eq!(unwrapped.len(), LABEL_BYTES + F32_BYTES);
    
    // Extract and verify the key part
    let key_bytes = &unwrapped[0..LABEL_BYTES];
    let key_str = String::from_utf8_lossy(key_bytes);
    assert!(key_str.starts_with("test"));
    
    // Extract and verify the value part
    let val_bytes = &unwrapped[LABEL_BYTES..LABEL_BYTES + F32_BYTES];
    let val_array: [u8; 4] = val_bytes.try_into().unwrap();
    let val = f32::from_le_bytes(val_array);
    assert_eq!(val, 42.5);
  }

  #[test]
  fn test_kvp_f32_gram_max_length_key() 
  {
    let max_key = "a".repeat(LABEL_BYTES);
    let kvp = KVPf32Gram::new(max_key.clone(), 1.0);
    assert!(kvp.is_ok());
    let unwrapped = kvp.unwrap().unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + F32_BYTES);
  }

  #[test]
  fn test_kvp_f32_gram_unicode_key() 
  {
    let kvp = KVPf32Gram::new(String::from("温度_celsius"), 36.6);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "温度_celsius");
    assert_eq!(kvp_unwrapped.val, 36.6);
  }

  #[test]
  fn test_kvp_f32_gram_special_chars_key() 
  {
    let kvp = KVPf32Gram::new(String::from("float-value_32"), 999.999);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "float-value_32");
    assert_eq!(kvp_unwrapped.val, 999.999);
  }

  #[test]
  fn test_kvp_f32_gram_precision() 
  {
    // Test f32 precision limits
    let test_cases = vec![
      ("small", 0.0000001),
      ("medium", 1234.5678),
      ("large", 1234567.8),
      ("very_large", 12345678.0), // Note: precision loss expected
    ];

    for (key, val) in test_cases {
      let kvp = KVPf32Gram::new(String::from(key), val);
      assert!(kvp.is_ok());
      let kvp_unwrapped = kvp.unwrap();
      assert_eq!(kvp_unwrapped.key, key);
      // For f32, we may have precision issues, so we check if it's close
      assert!((kvp_unwrapped.val - val).abs() < 0.001 || kvp_unwrapped.val == val);
    }
  }

  #[test]
  fn test_kvp_f32_gram_scientific_notation() 
  {
    // Test values typically expressed in scientific notation
    let test_cases = vec![
      ("avogadro_simplified", 6.022e23),
      ("planck_constant", 6.626e-34),
      ("speed_of_light", 2.998e8),
      ("electron_mass", 9.109e-31),
      ("earth_mass", 5.972e24),
    ];

    for (key, val) in test_cases {
      let kvp = KVPf32Gram::new(String::from(key), val);
      assert!(kvp.is_ok());
      let kvp_unwrapped = kvp.unwrap();
      assert_eq!(kvp_unwrapped.key, key);
      assert_eq!(kvp_unwrapped.val, val);
    }
  }
}