use std::fs::{ File };
use std::io::{ BufWriter, Error, Seek, SeekFrom };
use datagramv2::dg_utils::next_row_affix;
use datagramv2::rows::{ BuildIDRow, DBNicknameRow, GraphRow, PageRow };

use crate::datagramv2;

pub struct CoreExecutor {}

impl CoreExecutor 
{
  /// Read next affix from stream
  pub fn next_affix ( f: &mut File ) -> Option<String> { next_row_affix( f ) }


  /// Return current file stream position
  pub fn file_position ( f: &mut File ) -> Result<u64, Error> { f.stream_position() }


  /// Moves stream back from current position
  pub fn writer_seek_back_to ( position: u64, writer: &mut BufWriter<File> ) 
  {
    let pos_res = writer.stream_position();
    if pos_res.is_ok() 
    {
      let diff = ((pos_res.unwrap() - position) as i64) * -1;
      let _ = writer.seek( SeekFrom::End( diff ));
    }
  }  


  /// Read a GraphRow
  pub fn read_graph_row ( f: &mut File ) -> Result<( String, String ), String>
  {
    let res = GraphRow::read( f );
    if res.is_ok() { return Ok( res.unwrap() ); }
    res
  }


  /// Skip a BuildIdRow
  pub fn skip_build_id_row ( f: &mut File ) { let _ = BuildIDRow::skip( f ); }


  /// Skip a DBNicknameRow
  pub fn skip_db_nickname_row ( f: &mut File ) { let _ = DBNicknameRow::skip( f ); }


  /// Skip a GraphRow
  pub fn skip_graph_row ( f: &mut File ) { let _ = GraphRow::skip( f ); }


  /// Skip all the empty cells in current page.
  pub fn skip_empty_cells ( f: &mut File ) -> Result<(u64, u64), String>
  {
    let res = PageRow::skip_empty_cells( f );
    if res.is_ok() { return Ok( res.unwrap() ); }
    Err( res.unwrap_err().to_string() )
  }
}