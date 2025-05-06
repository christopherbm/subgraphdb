use utils::{ pad_str };
use crate::enums::{ DGLABEL_BYTES, DataGramError };
use crate::dg_utils::{ validate_dg_label };

// Graph Reference
#[derive( Clone, Debug, PartialEq )]
pub struct GraphRef
{
  pub nickname: String, // DGLabel
}

impl GraphRef
{
  pub fn new ( nickname: String ) -> Result<GraphRef, DataGramError> 
  {
    if nickname.len() == 0 { return Err( DataGramError::InvalidDGLabel )}
    let nickname_actual = pad_str( DGLABEL_BYTES, nickname );
    if !validate_dg_label( &nickname_actual ) { return Err( DataGramError::InvalidDGLabel )}
    return Ok( GraphRef { nickname: nickname_actual })
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
    let graph_res_1 = GraphRef::new( String::from( "" ));
    assert_eq!( graph_res_1.is_err(), true );

    // ---
    let graph_res_2 = GraphRef::new( pad_str( DGLABEL_BYTES, String::from( "nickname" )));
    assert_eq!( graph_res_2.is_ok(), true );
  }
}