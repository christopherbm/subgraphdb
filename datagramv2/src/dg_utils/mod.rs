use std::io::{ Read };
use std::fs::{ File };
use utils::{ str_from_bytes };
use common::{ LABEL_BYTES, ROW_AFFIX_BYTES };

/// Presumes stream position is at the beginning of an Affix
pub fn next_row_affix ( f: &mut File ) -> Option<String>
{
  let mut buffer = [0; ROW_AFFIX_BYTES];
  let _ = f.read_exact( &mut buffer );
  let mut cont = false;
  for byte in buffer.iter() 
  { 
    if *byte != 0 
    {
      cont = true;
      break;
    }
  }
  if cont == true 
  {
    let affix_res = str_from_bytes( &buffer.to_vec() );
    if affix_res.is_ok() { return Some( affix_res.unwrap() );}
  }
  None
}

pub fn next_label ( f: &mut File ) -> Option<String> 
{
  let mut buffer = [0; LABEL_BYTES];
  let _ = f.read_exact( &mut buffer );
  let mut cont = false;
  for byte in buffer.iter() 
  { 
    if *byte != 0 
    {
      cont = true;
      break;
    }
  }
  if cont == true 
  {
    let label_res = str_from_bytes( &buffer.to_vec() );
    if label_res.is_ok() { return Some( label_res.unwrap() );}
  }
  None
}