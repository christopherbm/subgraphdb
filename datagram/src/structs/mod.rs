use utils::{ pad_str };
use crate::enums::{ DGLabelBytes, DataGramError };
use crate::dg_utils::{ validate_dg_label };

// Graph Reference
#[derive( Clone, Debug, PartialEq )]
pub struct GraphRef
{
  pub nickname: String, // DGLabel
  pub order: u64, // DGU64
}

impl GraphRef
{
  pub fn new ( nickname: String, order: u64 ) -> Result<GraphRef, DataGramError> 
  {
    if nickname.len() == 0 { return Err( DataGramError::InvalidDGLabel )}
    let nickname_actual = pad_str( DGLabelBytes, nickname );
    if !validate_dg_label( &nickname_actual ) { return Err( DataGramError::InvalidDGLabel )}
    return Ok( GraphRef { nickname: nickname_actual, order: order })
  }
}

#[cfg(test)]
mod tests 
{
  use super::*;
  use utils::{ pad_str };

  #[test]
  fn test_graph_ref_new () 
  {
    let graph_res_1 = GraphRef::new( String::from( "" ), 5 );
    assert_eq!( graph_res_1.is_err(), true );

    // ---
    let graph_res_2 = GraphRef::new( 
      pad_str( ByteLengths::CommonString as usize, String::from( "nickname" )), 
      5 );
    assert_eq!( graph_res_2.is_ok(), true );
  }
}