use graph::model::node::{ Node };
use graph::model::edge::{ Edge, EdgeType };
use graph::model::graph::{ Graph, cons_graph };

fn test_graph_1 () -> Graph
{
  let mut gph: Graph = cons_graph();
  let nodea: Node = gph.cons_node( String::from( "A" ), 0, 0 );
  let nodeb: Node = gph.cons_node( String::from( "B" ), 0, 0 );
  let nodec: Node = gph.cons_node( String::from( "C" ), 0, 0 );
  let noded: Node = gph.cons_node( String::from( "D" ), 0, 0 );
  let nodee: Node = gph.cons_node( String::from( "E" ), 0, 0 );
  let nodef: Node = gph.cons_node( String::from( "F" ), 0, 0 );
  let nodeg: Node = gph.cons_node( String::from( "G" ), 0, 0 );
  let nodeh: Node = gph.cons_node( String::from( "H" ), 0, 0 );
  let nodei: Node = gph.cons_node( String::from( "I" ), 0, 0 );
  let nodej: Node = gph.cons_node( String::from( "J" ), 0, 0 );
  let nodek: Node = gph.cons_node( String::from( "K" ), 0, 0 );
  let nodel: Node = gph.cons_node( String::from( "L" ), 0, 0 );

  let edge0: Edge = gph.cons_edge( 
    String::from( "AB" ), 
    0, 3, 
    nodea.id.clone(), 
    nodeb.id.clone(),
    EdgeType::Undirected );
  
  let edge1: Edge = gph.cons_edge( 
    String::from( "AC" ), 
    0, 2, 
    nodea.id.clone(), 
    nodec.id.clone(),
    EdgeType::Undirected );
  
  let edge2: Edge = gph.cons_edge( 
    String::from( "AE" ), 
    0, 9, 
    nodea.id.clone(), 
    nodee.id.clone(),
    EdgeType::Undirected );
  
  let edge3: Edge = gph.cons_edge( 
    String::from( "BD" ), 
    0, 2, 
    nodeb.id.clone(), 
    noded.id.clone(),
    EdgeType::Undirected );
  
  let edge4: Edge = gph.cons_edge( 
    String::from( "BE" ), 
    0, 4, 
    nodeb.id.clone(), 
    nodee.id.clone(),
    EdgeType::Undirected );
  
  let edge5: Edge = gph.cons_edge( 
    String::from( "CE" ), 
    0, 6, 
    nodec.id.clone(), 
    nodee.id.clone(),
    EdgeType::Undirected );

  let edge6: Edge = gph.cons_edge( 
    String::from( "CF" ), 
    0, 9, 
    nodec.id.clone(), 
    nodef.id.clone(),
    EdgeType::Undirected );

  let edge7: Edge = gph.cons_edge( 
    String::from( "DG" ), 
    0, 3, 
    noded.id.clone(), 
    nodeg.id.clone(),
    EdgeType::Undirected );

  let edge8: Edge = gph.cons_edge( 
    String::from( "EG" ), 
    0, 1, 
    nodee.id.clone(), 
    nodeg.id.clone(),
    EdgeType::Undirected );
  
  let edge9: Edge = gph.cons_edge( 
    String::from( "EH" ), 
    0, 2, 
    nodee.id.clone(), 
    nodeh.id.clone(),
    EdgeType::Undirected );

  let edge10: Edge = gph.cons_edge( 
    String::from( "FH" ), 
    0, 1, 
    nodef.id.clone(), 
    nodeh.id.clone(),
    EdgeType::Undirected );

  let edge11: Edge = gph.cons_edge( 
    String::from( "FI" ), 
    0, 2, 
    nodef.id.clone(), 
    nodei.id.clone(),
    EdgeType::Undirected );

  let edge12: Edge = gph.cons_edge( 
    String::from( "GJ" ), 
    0, 5, 
    nodeg.id.clone(), 
    nodej.id.clone(),
    EdgeType::Undirected );

  let edge13: Edge = gph.cons_edge( 
    String::from( "HJ" ), 
    0, 5, 
    nodeh.id.clone(), 
    nodej.id.clone(),
    EdgeType::Undirected );
  
  let edge14: Edge = gph.cons_edge( 
    String::from( "HL" ), 
    0, 9, 
    nodeh.id.clone(), 
    nodel.id.clone(),
    EdgeType::Undirected );

  let edge15: Edge = gph.cons_edge( 
    String::from( "HK" ), 
    0, 6, 
    nodeh.id.clone(), 
    nodek.id.clone(),
    EdgeType::Undirected );

  let edge16: Edge = gph.cons_edge( 
    String::from( "IK" ), 
    0, 2, 
    nodei.id.clone(), 
    nodek.id.clone(),
    EdgeType::Undirected );

  let edge17: Edge = gph.cons_edge( 
    String::from( "JL" ), 
    0, 5, 
    nodej.id.clone(), 
    nodel.id.clone(),
    EdgeType::Undirected );

  let edge18: Edge = gph.cons_edge( 
    String::from( "KL" ), 
    0, 3, 
    nodek.id.clone(), 
    nodel.id.clone(),
    EdgeType::Undirected );

  let _ = gph.add_node( nodea );
  let _ = gph.add_node( nodeb );
  let _ = gph.add_node( nodec );
  let _ = gph.add_node( noded );
  let _ = gph.add_node( nodee );
  let _ = gph.add_node( nodef );
  let _ = gph.add_node( nodeg );
  let _ = gph.add_node( nodeh );
  let _ = gph.add_node( nodei );
  let _ = gph.add_node( nodej );
  let _ = gph.add_node( nodek );
  let _ = gph.add_node( nodel );

  let _ = gph.add_edge( edge0 );
  let _ = gph.add_edge( edge1 );
  let _ = gph.add_edge( edge2 );
  let _ = gph.add_edge( edge3 );
  let _ = gph.add_edge( edge4 );
  let _ = gph.add_edge( edge5 );
  let _ = gph.add_edge( edge6 );
  let _ = gph.add_edge( edge7 );
  let _ = gph.add_edge( edge8 );
  let _ = gph.add_edge( edge9 );
  let _ = gph.add_edge( edge10 );
  let _ = gph.add_edge( edge11 );
  let _ = gph.add_edge( edge12 );
  let _ = gph.add_edge( edge13 );
  let _ = gph.add_edge( edge14 );
  let _ = gph.add_edge( edge15 );
  let _ = gph.add_edge( edge16 );
  let _ = gph.add_edge( edge17 );
  let _ = gph.add_edge( edge18 );

  gph
}

#[test]
fn test_test_graph_1 () 
{
  let gph: Graph = test_graph_1();

  assert_eq!( gph.node_count(), 12 );
  assert_eq!( gph.edge_count(), 19 );
  assert_eq!( gph.graph_order(), 12 );
  assert_eq!( gph.graph_size(), 19 );
 
  assert_eq!( gph.get_nodes().len(), 12 );
  assert_eq!( gph.get_edges().len(), 19 );

  assert_eq!( gph.get_nodes()[ 11 ].inner_order, 11 );
  assert_eq!( gph.get_edges()[ 18 ].inner_order, 18 );

  //println!( "{:?}", nodes );
}