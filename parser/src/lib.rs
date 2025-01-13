use std::str::FromStr;
use graph::model::node::{ Node };
use graph::model::edge::{ Edge };
use graph::model::graph::{ Graph, cons_graph };

#[derive(PartialEq, Debug)]
enum ParsingStep 
{ 
  InitGraph, 
  BuildingNode, BuildingNodeName, BuildingNodeLabel, BuildingNodeWeight,
  BuildingEdge, BuildingEdgeName, BuildingEdgeLabel, BuildingEdgeWeight,
  BuildingNodeSlash, BuildingEdgeSlash,
  EdgeWaiting, Waiting, Error 
}

#[derive(PartialEq, Debug)]
enum ParseAction 
{ 
  None, CreateNode, CloseNode, CreateEdge, CloseEdge, 
  CloseNodeLabel, CloseEdgeLabel,
  CloseNodeWeight, CloseEdgeWeight
}

#[derive( Debug )]
struct ParseState
{ 
  pub gph: Graph,
  pub state: ParsingStep,
  pub node_opt: Option<Node>,
  pub node_ref: String,
  pub edge_opt: Option<Edge>,
  pub label_opt: Option<String>,
  pub weight_opt: Option<String>,
  pub order_opt: Option<String>
}

fn cons_parse_state () -> ParseState 
{
  ParseState {
    gph: cons_graph(),
    state: ParsingStep::InitGraph,
    node_opt: None,
    node_ref: ":None".to_string(),
    edge_opt: None,
    label_opt: None,
    weight_opt: None,
    order_opt: None
  }
}

fn parse_open_paren ( _c: &char, state: &ParsingStep ) -> ( ParsingStep, ParseAction ) 
{
  if *state == ParsingStep::InitGraph { return ( ParsingStep::BuildingNode, ParseAction::CreateNode ) }
  if *state == ParsingStep::Waiting { return ( ParsingStep::BuildingNode, ParseAction::CreateNode ) }
  ( ParsingStep::Error, ParseAction::None )
}

fn parse_close_paren ( _c: &char, state: &ParsingStep ) -> ( ParsingStep, ParseAction ) 
{
  if *state == ParsingStep::BuildingNode { return ( ParsingStep::Waiting, ParseAction::CloseNode ) }
  if *state == ParsingStep::BuildingNodeName { return ( ParsingStep::Waiting, ParseAction::CloseNode ) }
  if *state == ParsingStep::BuildingNodeLabel { return ( ParsingStep::Waiting, ParseAction::CloseNode ) }
  if *state == ParsingStep::BuildingNodeWeight { return ( ParsingStep::Waiting, ParseAction::CloseNode ) }
  ( ParsingStep::Error, ParseAction::None )
}

fn parse_hyphen ( _c: &char, state: &ParsingStep ) -> ( ParsingStep, ParseAction ) 
{
  if *state == ParsingStep::Waiting { return ( ParsingStep::BuildingEdge, ParseAction::CreateEdge ) }
  if *state == ParsingStep::EdgeWaiting { return ( ParsingStep::Waiting, ParseAction::None ) }
  if *state == ParsingStep::BuildingEdge { return ( ParsingStep::Waiting, ParseAction::CloseEdge ) }
  if *state == ParsingStep::BuildingEdgeName { return ( ParsingStep::Waiting, ParseAction::CloseEdge ) }
  ( ParsingStep::Error, ParseAction::None )
}

fn parse_space ( _c: &char, state: &ParsingStep ) -> ( ParsingStep, ParseAction ) 
{
  if *state == ParsingStep::InitGraph { return ( ParsingStep::InitGraph, ParseAction::None ) }
  if *state == ParsingStep::Waiting { return ( ParsingStep::Waiting, ParseAction::None ) }
  if *state == ParsingStep::BuildingEdge { return ( ParsingStep::BuildingEdge, ParseAction::None ) }
  if *state == ParsingStep::BuildingNode { return ( ParsingStep::BuildingNode, ParseAction::None ) }
  if *state == ParsingStep::BuildingNodeName { return ( ParsingStep::BuildingNode, ParseAction::None ) }
  if *state == ParsingStep::BuildingEdgeName { return ( ParsingStep::BuildingEdge, ParseAction::None ) }
  if *state == ParsingStep::BuildingNodeLabel { return ( ParsingStep::BuildingNode, ParseAction::CloseNodeLabel ) }
  if *state == ParsingStep::BuildingEdgeLabel { return ( ParsingStep::BuildingEdge, ParseAction::CloseEdgeLabel ) }
  if *state == ParsingStep::BuildingNodeWeight { return ( ParsingStep::BuildingNodeWeight, ParseAction::CloseNodeWeight ) }
  if *state == ParsingStep::BuildingEdgeWeight { return ( ParsingStep::BuildingEdgeWeight, ParseAction::CloseEdgeWeight ) }
  ( ParsingStep::Error, ParseAction::None )
}

fn parse_colon ( _c: &char, state: &ParsingStep ) -> ( ParsingStep, ParseAction ) 
{
  if *state == ParsingStep::BuildingNode { return ( ParsingStep::BuildingNodeLabel, ParseAction::None ) }
  if *state == ParsingStep::BuildingEdge { return ( ParsingStep::BuildingEdgeLabel, ParseAction::None ) }
  ( ParsingStep::Error, ParseAction::None )
}

/// Parse [
fn parse_open_bracket ( _c: &char, state: &ParsingStep ) -> ( ParsingStep, ParseAction ) 
{
  if *state == ParsingStep::BuildingEdge { return ( ParsingStep::BuildingEdge, ParseAction::None ) }
  ( ParsingStep::Error, ParseAction::None )
}

fn parse_close_bracket ( _c: &char, state: &ParsingStep ) -> ( ParsingStep, ParseAction ) 
{
  if *state == ParsingStep::BuildingEdgeLabel { return ( ParsingStep::BuildingEdge, ParseAction::CloseEdge ) }
  if *state == ParsingStep::BuildingEdgeName { return ( ParsingStep::BuildingEdge, ParseAction::CloseEdge ) }
  if *state == ParsingStep::BuildingEdge { return ( ParsingStep::BuildingEdge, ParseAction::CloseEdge ) }
  if *state == ParsingStep::BuildingEdgeWeight { return ( ParsingStep::BuildingEdge, ParseAction::CloseEdge ) }
  ( ParsingStep::Error, ParseAction::None )
}

fn cast_weight ( s: String ) -> Result<f64, <f64 as FromStr>::Err> { s.parse::<f64>() }
fn cast_order ( s: String ) -> Result<f64, <f64 as FromStr>::Err> { s.parse::<f64>() }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works () 
  {
    let state: ParseState = cons_parse_state();
    //assert_eq!(result, 4);
  }

  #[test]
  fn test_parse_open_paren () 
  {
    let res1 = parse_open_paren( &'(' as &char, &ParsingStep::InitGraph );
    assert_eq!( res1.0, ParsingStep::BuildingNode );
    assert_eq!( res1.1, ParseAction::CreateNode );

    let res2 = parse_open_paren( &'(' as &char, &ParsingStep::BuildingNode );
    assert_eq!( res2.0, ParsingStep::Error );
    assert_eq!( res2.1, ParseAction::None );

    let res3 = parse_open_paren( &'(' as &char, &ParsingStep::Waiting );
    assert_eq!( res3.0, ParsingStep::BuildingNode );
    assert_eq!( res3.1, ParseAction::CreateNode );
  }

  #[test]
  fn test_parse_close_paren () 
  {
    let res1 = parse_close_paren( &')' as &char, &ParsingStep::BuildingNode );
    assert_eq!( res1.0, ParsingStep::Waiting );
    assert_eq!( res1.1, ParseAction::CloseNode );

    let res2 = parse_close_paren( &')' as &char, &ParsingStep::Waiting );
    assert_eq!( res2.0, ParsingStep::Error );
    assert_eq!( res2.1, ParseAction::None );

    let res3 = parse_close_paren( &')' as &char, &ParsingStep::BuildingNodeName );
    assert_eq!( res3.0, ParsingStep::Waiting );
    assert_eq!( res3.1, ParseAction::CloseNode );

    let res4 = parse_close_paren( &')' as &char, &ParsingStep::BuildingNodeLabel );
    assert_eq!( res4.0, ParsingStep::Waiting );
    assert_eq!( res4.1, ParseAction::CloseNode );

    let res6 = parse_close_paren( &')' as &char, &ParsingStep::BuildingNodeWeight );
    assert_eq!( res6.0, ParsingStep::Waiting );
    assert_eq!( res6.1, ParseAction::CloseNode );
  }

  #[test]
  fn test_parse_hyphen () 
  {
    let res1 = parse_hyphen( &'-' as &char, &ParsingStep::Waiting );
    assert_eq!( res1.0, ParsingStep::BuildingEdge );
    assert_eq!( res1.1, ParseAction::CreateEdge );

    let res2 = parse_hyphen( &'-' as &char, &ParsingStep::BuildingNode );
    assert_eq!( res2.0, ParsingStep::Error );
    assert_eq!( res2.1, ParseAction::None );

    let res3 = parse_hyphen( &'-' as &char, &ParsingStep::BuildingEdge );
    assert_eq!( res3.0, ParsingStep::Waiting );
    assert_eq!( res3.1, ParseAction::CloseEdge );

    let res4 = parse_hyphen( &'-' as &char, &ParsingStep::EdgeWaiting );
    assert_eq!( res4.0, ParsingStep::Waiting );
    assert_eq!( res4.1, ParseAction::None );

    let res5 = parse_hyphen( &'-' as &char, &ParsingStep::BuildingEdgeName );
    assert_eq!( res5.0, ParsingStep::Waiting );
    assert_eq!( res5.1, ParseAction::CloseEdge );
  }

  #[test]
  fn test_parse_space () 
  {
    let res1 = parse_space( &' ' as &char, &ParsingStep::InitGraph );
    assert_eq!( res1.0, ParsingStep::InitGraph );
    assert_eq!( res1.1, ParseAction::None );

    let res2 = parse_space( &' ' as &char, &ParsingStep::Waiting );
    assert_eq!( res2.0, ParsingStep::Waiting );
    assert_eq!( res2.1, ParseAction::None );

    let res3 = parse_space( &' ' as &char, &ParsingStep::BuildingEdge );
    assert_eq!( res3.0, ParsingStep::BuildingEdge );
    assert_eq!( res3.1, ParseAction::None );

    let res4 = parse_space( &' ' as &char, &ParsingStep::BuildingNode );
    assert_eq!( res4.0, ParsingStep::BuildingNode );
    assert_eq!( res4.1, ParseAction::None );

    let res5 = parse_space( &' ' as &char, &ParsingStep::BuildingNodeName );
    assert_eq!( res5.0, ParsingStep::BuildingNode );
    assert_eq!( res5.1, ParseAction::None );

    let res6 = parse_space( &' ' as &char, &ParsingStep::BuildingEdgeName );
    assert_eq!( res6.0, ParsingStep::BuildingEdge );
    assert_eq!( res6.1, ParseAction::None );

    let res7 = parse_space( &' ' as &char, &ParsingStep::BuildingNodeLabel );
    assert_eq!( res7.0, ParsingStep::BuildingNode );
    assert_eq!( res7.1, ParseAction::CloseNodeLabel );

    let res7 = parse_space( &' ' as &char, &ParsingStep::BuildingNodeLabel );
    assert_eq!( res7.0, ParsingStep::BuildingNode );
    assert_eq!( res7.1, ParseAction::CloseNodeLabel );

    let res8 = parse_space( &' ' as &char, &ParsingStep::BuildingEdgeLabel );
    assert_eq!( res8.0, ParsingStep::BuildingEdge );
    assert_eq!( res8.1, ParseAction::CloseEdgeLabel );

    let res9 = parse_space( &' ' as &char, &ParsingStep::BuildingNodeWeight );
    assert_eq!( res9.0, ParsingStep::BuildingNodeWeight );
    assert_eq!( res9.1, ParseAction::CloseNodeWeight );

    let res10 = parse_space( &' ' as &char, &ParsingStep::BuildingEdgeWeight );
    assert_eq!( res10.0, ParsingStep::BuildingEdgeWeight );
    assert_eq!( res10.1, ParseAction::CloseEdgeWeight );
  }

  #[test]
  fn test_parse_colon () 
  {
    let res1 = parse_colon( &':' as &char, &ParsingStep::Waiting );
    assert_eq!( res1.0, ParsingStep::Error );
    assert_eq!( res1.1, ParseAction::None );

    let res2 = parse_colon( &':' as &char, &ParsingStep::BuildingNode );
    assert_eq!( res2.0, ParsingStep::BuildingNodeLabel );
    assert_eq!( res2.1, ParseAction::None );

    let res3 = parse_colon( &':' as &char, &ParsingStep::BuildingEdge );
    assert_eq!( res3.0, ParsingStep::BuildingEdgeLabel );
    assert_eq!( res3.1, ParseAction::None );
  }

  #[test]
  fn test_parse_open_bracket () 
  {
    let res1 = parse_open_bracket( &'[' as &char, &ParsingStep::Waiting );
    assert_eq!( res1.0, ParsingStep::Error );
    assert_eq!( res1.1, ParseAction::None );

    let res2 = parse_open_bracket( &'[' as &char, &ParsingStep::BuildingEdge );
    assert_eq!( res2.0, ParsingStep::BuildingEdge );
    assert_eq!( res2.1, ParseAction::None );
  }

  #[test]
  fn test_parse_close_bracket () 
  {
    let res1 = parse_close_bracket( &']' as &char, &ParsingStep::Waiting );
    assert_eq!( res1.0, ParsingStep::Error );
    assert_eq!( res1.1, ParseAction::None );

    let res2 = parse_close_bracket( &']' as &char, &ParsingStep::BuildingEdge );
    assert_eq!( res2.0, ParsingStep::BuildingEdge );
    assert_eq!( res2.1, ParseAction::CloseEdge );

    let res3 = parse_close_bracket( &']' as &char, &ParsingStep::BuildingEdgeLabel );
    assert_eq!( res3.0, ParsingStep::BuildingEdge );
    assert_eq!( res3.1, ParseAction::CloseEdge );

    let res4 = parse_close_bracket( &']' as &char, &ParsingStep::BuildingEdgeName );
    assert_eq!( res4.0, ParsingStep::BuildingEdge );
    assert_eq!( res4.1, ParseAction::CloseEdge );

    let res5 = parse_close_bracket( &']' as &char, &ParsingStep::BuildingEdgeWeight );
    assert_eq!( res5.0, ParsingStep::BuildingEdge );
    assert_eq!( res5.1, ParseAction::CloseEdge );
  }
}
