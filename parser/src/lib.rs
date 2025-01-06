use graph::model::node::{ Node };
use graph::model::edge::{ Edge };
use graph::model::graph::{ Graph, cons_graph };

#[derive(PartialEq, Debug)]
enum ParsingStep 
{ 
  InitGraph, 
  BuildingNode, BuildingNodeName, BuildingNodeLabel,
  BuildingNodeWeight, BuildingNodeOrder,
  BuildingEdge, BuildingEdgeName, BuildingEdgeLabel,
  BuildingEdgeWeight, BuildingEdgeOrder,
  BuildingNodeSlash, BuildingEdgeSlash,
  EdgeWaiting, Waiting, Error 
}

#[derive(PartialEq, Debug)]
enum ParseAction 
{ 
  None, CreateNode, CloseNode, CreateEdge, CloseEdge, 
  CloseNodeLabel, CloseEdgeLabel,
  CloseNodeWeight, CloseEdgeWeight,
  CloseNodeOrder, CloseEdgeOrder
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

fn cast_weight ( s: String ) -> Result<i64, <i64 as FromStr>::Err> { s.parse::<i64>() }
fn cast_order ( s: String ) -> Result<u64, <u64 as FromStr>::Err> { s.parse::<u64>() }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works () 
  {
    let state: ParseState = cons_parse_state();
    //assert_eq!(result, 4);
  }
}
