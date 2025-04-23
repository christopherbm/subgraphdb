use crate::enums::{ DGU64Bytes, DGLabelBytes };

// len is number of bytes (not characters)
pub fn validate_dg_uuid ( uuid: &str ) -> bool { uuid.len() == DGU64Bytes }

pub fn validate_dg_label ( label: &str ) -> bool { label.len() == DGLabelBytes }

#[cfg(test)]
mod tests 
{
  use super::*;
  use utils::{ pad_str };

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
    assert_eq!( validate_dg_label( &pad_str( DGLabelBytes, String::from( "test" ))), true );
    assert_eq!( validate_dg_label( "test" ), false );
    assert_eq!( validate_dg_label( "" ), false );
    assert_eq!( validate_dg_label( "----------------------------------------------------------------------" ), false );
  }
}