use crate::common::{ LABEL_BYTES, RAW_UUID_BYTES };

pub fn validate_label ( val: &str ) -> bool 
{
  let size: usize = val.bytes().len();
  if size <= LABEL_BYTES { return true; }
  false
}

pub fn validate_uuid ( val: &str ) -> bool
{
  if val.bytes().len() == RAW_UUID_BYTES { return true; }
  return false;
}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_validate_label_empty_string() 
  {
    // Empty string should be valid
    assert_eq!( validate_label(""), true );
  }

  #[test]
  fn test_validate_label_single_char() 
  {
    // Single character should be valid
    assert_eq!( validate_label("a"), true );
    assert_eq!( validate_label("Z"), true );
    assert_eq!( validate_label("0"), true );
    assert_eq!( validate_label("_"), true );
  }

  #[test]
  fn test_validate_label_normal_strings() 
  {
    // Normal labels should be valid
    assert_eq!( validate_label("User"), true );
    assert_eq!( validate_label("Person"), true );
    assert_eq!( validate_label("Movie"), true );
    assert_eq!( validate_label("KNOWS"), true );
    assert_eq!( validate_label("has_property"), true );
    assert_eq!( validate_label("node_123"), true );
  }

  #[test]
  fn test_validate_label_exact_limit() 
  {
    // String with exactly LABEL_BYTES bytes should be valid
    let exact_limit = "a".repeat(LABEL_BYTES);
    assert_eq!( exact_limit.len(), LABEL_BYTES );
    assert_eq!( validate_label(&exact_limit), true );
  }

  #[test]
  fn test_validate_label_one_byte_over_limit() 
  {
    // String with LABEL_BYTES + 1 bytes should be invalid
    let over_limit = "a".repeat(LABEL_BYTES + 1);
    assert_eq!( over_limit.len(), LABEL_BYTES + 1 );
    assert_eq!( validate_label(&over_limit), false );
  }

  #[test]
  fn test_validate_label_significantly_over_limit() 
  {
    // String significantly over the limit should be invalid
    let way_over_limit = "a".repeat(LABEL_BYTES * 2);
    assert_eq!( way_over_limit.len(), LABEL_BYTES * 2 );
    assert_eq!( validate_label(&way_over_limit), false );
  }

  #[test]
  fn test_validate_label_unicode() 
  {
    // Test with unicode characters that take multiple bytes
    assert_eq!( validate_label("café"), true ); // 5 bytes (é is 2 bytes)
    assert_eq!( validate_label("こんにちは"), true ); // 15 bytes (each character is 3 bytes)
    assert_eq!( validate_label("🚀"), true ); // 4 bytes (emoji)
    assert_eq!( validate_label("Ñoño"), true ); // 6 bytes
  }

  #[test]
  fn test_validate_label_unicode_edge_cases() 
  {
    // Test unicode strings near the byte limit
    // Each "好" character is 3 bytes in UTF-8
    let chars_needed = LABEL_BYTES / 3;
    let unicode_exact = "好".repeat(chars_needed);
    assert!( unicode_exact.len() <= LABEL_BYTES );
    assert_eq!( validate_label(&unicode_exact), true );

    // Going over the limit with unicode
    let unicode_over = "好".repeat(chars_needed + 1);
    assert!( unicode_over.len() > LABEL_BYTES );
    assert_eq!( validate_label(&unicode_over), false );
  }

  #[test]
  fn test_validate_label_mixed_content() 
  {
    // Test with mixed ASCII and unicode
    assert_eq!( validate_label("User_123_テスト"), true );
    assert_eq!( validate_label("Node#42-αβγ"), true );
    assert_eq!( validate_label("Label with spaces"), true );
  }

  #[test]
  fn test_validate_label_special_characters() 
  {
    // Test with various special characters
    assert_eq!( validate_label("node-123"), true );
    assert_eq!( validate_label("user_profile"), true );
    assert_eq!( validate_label("email@domain"), true );
    assert_eq!( validate_label("price$100"), true );
    assert_eq!( validate_label("50%_discount"), true );
    assert_eq!( validate_label("item#42"), true );
    assert_eq!( validate_label("a&b"), true );
    assert_eq!( validate_label("(test)"), true );
    assert_eq!( validate_label("[array]"), true );
    assert_eq!( validate_label("{object}"), true );
  }

  #[test]
  fn test_validate_label_whitespace() 
  {
    // Test various whitespace scenarios
    assert_eq!( validate_label(" "), true ); // single space
    assert_eq!( validate_label("  "), true ); // multiple spaces
    assert_eq!( validate_label("\t"), true ); // tab
    assert_eq!( validate_label("\n"), true ); // newline
    assert_eq!( validate_label(" leading"), true );
    assert_eq!( validate_label("trailing "), true );
    assert_eq!( validate_label(" both "), true );
    assert_eq!( validate_label("multi  space"), true );
  }

  #[test]
  fn test_validate_label_boundary_values() 
  {
    // Test strings around the boundary
    let near_limit_63 = "a".repeat(LABEL_BYTES - 1);
    let at_limit_64 = "a".repeat(LABEL_BYTES);
    let over_limit_65 = "a".repeat(LABEL_BYTES + 1);

    assert_eq!( validate_label(&near_limit_63), true );
    assert_eq!( validate_label(&at_limit_64), true );
    assert_eq!( validate_label(&over_limit_65), false );
  }

  #[test]
  fn test_validate_label_numeric_strings() 
  {
    // Test with numeric strings
    assert_eq!( validate_label("123"), true );
    assert_eq!( validate_label("456789"), true );
    assert_eq!( validate_label("0"), true );
    assert_eq!( validate_label("00001"), true );
    assert_eq!( validate_label("3.14"), true );
    assert_eq!( validate_label("-42"), true );
  }

  #[test]
  fn test_validate_label_byte_length_not_char_length() 
  {
    // Verify the function checks byte length, not character length
    // "😀" is 1 character but 4 bytes
    let emoji_string = "😀".repeat(16); // 16 emojis = 64 bytes
    assert_eq!( emoji_string.chars().count(), 16 );
    assert_eq!( emoji_string.len(), 64 );
    assert_eq!( validate_label(&emoji_string), true );

    let emoji_string_over = "😀".repeat(17); // 17 emojis = 68 bytes
    assert_eq!( emoji_string_over.chars().count(), 17 );
    assert_eq!( emoji_string_over.len(), 68 );
    assert_eq!( validate_label(&emoji_string_over), false );
  }

  #[test]
  fn test_validate_uuid_valid() 
  {
    // Test standard UUID format (8-4-4-4-12)
    assert_eq!( validate_uuid("550e8400-e29b-41d4-a716-446655440000"), true );
    assert_eq!( validate_uuid("123e4567-e89b-12d3-a456-426614174000"), true );
    assert_eq!( validate_uuid("00000000-0000-0000-0000-000000000000"), true );
    assert_eq!( validate_uuid("ffffffff-ffff-ffff-ffff-ffffffffffff"), true );
  }

  #[test]
  fn test_validate_uuid_invalid_length() 
  {
    // Test UUIDs with incorrect length
    assert_eq!( validate_uuid(""), false ); // Empty string
    assert_eq!( validate_uuid("550e8400"), false ); // Too short
    assert_eq!( validate_uuid("550e8400-e29b-41d4-a716"), false ); // Missing last part
    assert_eq!( validate_uuid("550e8400-e29b-41d4-a716-446655440000-extra"), false ); // Too long
  }
}