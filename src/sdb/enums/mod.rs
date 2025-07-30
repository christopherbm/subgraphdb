use crate::in_memory::{ InMemoryDB };
use crate::single_file::{ SingleFileDB };
use crate::multi_file::{ MultiFileDB };

/// Database Type
#[ derive( Debug )]
pub enum SDBType
{
  InMemory( InMemoryDB ),
  SingleFile( SingleFileDB ),
  MultiFile( MultiFileDB ),
}