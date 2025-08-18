use std::io::{ Write };
use crate::datagramv2::internal_grams::{ Label, UUID };
use crate::executor::writer::core::CoreWriteExecutor;

pub struct WriteNewDBExecutor {}

impl WriteNewDBExecutor
{
  /// Create a new db file
  pub fn execute_write_new ( 
    build_id: &UUID, db_nickname: &Label, page_size: usize, 
    stream: &mut impl Write ) 
  {
    CoreWriteExecutor::write_new_db( build_id, db_nickname, page_size, stream );
  }
}