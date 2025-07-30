use std::io::{ Read };
use std::fs::{ File };
use crate::utils::{ str_from_bytes };
use crate::common::{ LABEL_BYTES, ROW_AFFIX_BYTES, U64_BYTES };

/// Presumes stream position is at the beginning of an Affix
pub fn next_row_affix ( f: &mut File ) -> Option<String>
{
  let mut buffer = [0; ROW_AFFIX_BYTES];
  let _ = f.read_exact( &mut buffer );
  let affix_res = str_from_bytes( &buffer.to_vec() );
  if affix_res.is_ok() { return Some( affix_res.unwrap() );}
  None
}

pub fn next_label ( f: &mut File ) -> Option<String> 
{
  let mut buffer = [0; LABEL_BYTES];
  let _ = f.read_exact( &mut buffer );
  let label_res = str_from_bytes( &buffer.to_vec() );
  if label_res.is_ok() { return Some( label_res.unwrap() );}
  None
}

pub fn next_u64 ( f: &mut File ) -> u64
{
  let mut buffer = [0; U64_BYTES];
  let _ = f.read_exact( &mut buffer );
  u64::from_le_bytes( buffer )
}