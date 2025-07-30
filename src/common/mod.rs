/* @version 0.3.0 */

/*
Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
Floating point: f32, f64
char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
bool either true or false
*/

pub static BOOL_BYTES: usize = 8 as usize;
pub static UUID_BYTES: usize = 40 as usize;
pub static RAW_UUID_BYTES: usize = 36 as usize;
pub static LABEL_BYTES: usize = 64 as usize; // names, labels, etc
pub static U64_BYTES: usize = 8 as usize;
pub static SHORT_STRING_BYTES: usize = 8 as usize;
pub static ROW_AFFIX_BYTES: usize = 8 as usize;

pub static TRUE_AFFIX: &'static str = "[::TRUE]";
pub static FALSE_AFFIX: &'static str = "[:FALSE]";

pub static DIR_UNDIRECTED: &'static str = "[:DIRUD]";
pub static DIR_LEFT: &'static str = "[::DIRL]";
pub static DIR_RIGHT: &'static str = "[::DIRR]";
pub static DIR_BI: &'static str = "[:DIRBI]";

pub static PLACEHOLDER: &'static str = "[::PLCH]";
pub static END_DB: &'static str = "[:::END]";

/// Graph Edge Direction
#[derive( Debug, Clone, PartialEq )]
pub enum DirectionType { Undirected, Left, Right, Bidirectional }

pub fn direction_to_str ( dt: &DirectionType ) -> &str 
{
  match dt 
  {
    DirectionType::Undirected => { return DIR_UNDIRECTED; },
    DirectionType::Left => { return DIR_LEFT; },
    DirectionType::Right => { return DIR_RIGHT; },
    DirectionType::Bidirectional => { return DIR_BI; }
  }
}

pub fn direction_from_str () {}

pub fn bool_to_affix ( b: bool ) -> &'static str 
{
  if b == true { return TRUE_AFFIX; }
  FALSE_AFFIX
}

#[derive( Debug )]
pub struct KeyValString
{
  pub order: u16,
  pub key: String,
  pub val: String,
}


/// Node / Edge Property
#[derive( Debug, PartialEq )]
pub enum NEProperty
{
  // Default Types
  Tag( String ),
  
  //KvpString(( String, Option<String> )),
  //KvpFloat(( String, Option<f64> )),
  //KvpBool(( String, Option<bool> )),

  //ListString(( String, Option<Vec<String>> )),
  //ListFloat(( String, Option<Vec<f64>> )),
  //ListBool(( String, Option<Vec<bool>> )),
  
  // Other Schema Types
  //KvpInteger(( String, Option<u64> )),
}

/// Graph Node
#[derive( Debug )]
pub struct Node
{
  pub id: String,
  pub primary_tag: Option<String>,
  pub properties: Vec<NEProperty>
}
impl Node 
{
  /// Construct Graph Node
  pub fn new ( id: String, primary_tag: Option<String>, properties: Vec<NEProperty> ) -> Node 
  {
    Node { id: id, primary_tag: primary_tag, properties: properties }
  }

  /// Construct empty Graph Node
  pub fn new_empty () -> Node { Node::new( String::from( "" ), None, Vec::new() )}

  /// Add NEProperty to Node
  pub fn io_add_property ( &mut self, prop: NEProperty ) { self.properties.push( prop ); }

  /// Node has any Properties
  pub fn has_props ( &self ) -> bool { self.properties.len() > 0 }
}

/// Graph Edge
#[derive( Debug )]
pub struct Edge
{
  pub id: String,
  pub primary_tag: Option<String>,
  pub left_id: String, 
  pub right_id: String,
  pub direction: DirectionType,
  pub properties: Vec<NEProperty>
}
impl Edge 
{
  /// Construct Graph Edge
  pub fn new (
    id: String, primary_tag: Option<String>, left_id: String, right_id: String, 
    direction: DirectionType, properties: Vec<NEProperty> ) -> Edge 
  {
    Edge
    {
      id: id,
      primary_tag: primary_tag,
      left_id: left_id, 
      right_id: right_id,
      direction: direction,
      properties: properties
    }
  }

  /// Construct empty Graph Edge
  pub fn new_empty () -> Edge 
  {
    Edge::new( 
      String::from( "" ), 
      None, 
      String::from( "" ), 
      String::from( "" ),
      DirectionType::Undirected, 
      Vec::new() )
  }

  /// Add NEProperty to Edge
  pub fn io_add_property ( &mut self, prop: NEProperty ) { self.properties.push( prop ); }

  /// Edge has any Properties
  pub fn has_props ( &self ) -> bool { self.properties.len() > 0 }
}

// ---------------------------------------------------------------------------------------------------------------------

/// Construct Tag Property
pub fn cons_tag_property ( tag: String ) -> NEProperty { NEProperty::Tag( tag )}

/// Construct KvpString Property
//pub fn cons_kvp_string_property ( key:String, val: Option<String> ) -> NEProperty { NEProperty::KvpString(( key, val ))}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_cons_node () 
  {
    let node: Node = Node::new( String::from( "id" ), Some( String::from( "node" )), Vec::new() );

    assert_eq!( node.id, String::from( "id" ));
    assert_eq!( node.primary_tag.unwrap(), String::from( "node" ));
    assert_eq!( node.properties.len(), 0 );
  }

  #[test]
  fn test_cons_empty_node () 
  {
    let node: Node = Node::new_empty();

    assert_eq!( node.id, String::from( "" ));
    assert_eq!( node.primary_tag.is_none(), true );
    assert_eq!( node.properties.len(), 0 );
  }

  #[test]
  fn test_cons_edge () 
  {
    let edge: Edge = Edge::new( 
      String::from( "id" ), 
      Some( String::from( "edge" )), 
      String::from( "left" ),
      String::from( "right" ),
      DirectionType::Undirected,
      Vec::new() );

    assert_eq!( edge.id, String::from( "id" ));
    assert_eq!( edge.primary_tag.unwrap(), String::from( "edge" ));
    assert_eq!( edge.left_id, String::from( "left" ));
    assert_eq!( edge.right_id, String::from( "right" ));
    assert_eq!( edge.direction, DirectionType::Undirected );
    assert_eq!( edge.properties.len(), 0 );
  }

  #[test]
  fn test_cons_empty_edge () 
  {
    let edge: Edge = Edge::new_empty();
    
    assert_eq!( edge.id, String::from( "" ));
    assert_eq!( edge.primary_tag.is_some(), false );
    assert_eq!( edge.left_id, String::from( "" ));
    assert_eq!( edge.right_id, String::from( "" ));
    assert_eq!( edge.direction, DirectionType::Undirected );
    assert_eq!( edge.properties.len(), 0 );
  }

  #[test]
  fn test_cons_tag_property () 
  {
    assert_eq!( cons_tag_property( String::from( "tag" )), NEProperty::Tag( String::from( "tag" )));
  }

  #[test]
  fn test_node_io_add_property () 
  {
    let mut node: Node = Node::new_empty();

    assert_eq!( node.properties.len(), 0 );

    node.io_add_property( cons_tag_property( String::from( "tag" )));
    assert_eq!( node.properties.len(), 1 );
  }

  #[test]
  fn test_edge_io_add_property () 
  {
    let mut edge: Edge = Edge::new_empty();

    assert_eq!( edge.properties.len(), 0 );

    edge.io_add_property( cons_tag_property( String::from( "tag" )));
    assert_eq!( edge.properties.len(), 1 );
  }

  #[test]
  fn test_node_has_props () 
  {
    let mut node: Node = Node::new_empty();

    assert_eq!( node.properties.len(), 0 );
    assert_eq!( node.has_props(), false );

    node.io_add_property( cons_tag_property( String::from( "tag" )));
    assert_eq!( node.properties.len(), 1 );
    assert_eq!( node.has_props(), true );
  }

  #[test]
  fn test_edge_has_props () 
  {
    let mut edge: Edge = Edge::new_empty();

    assert_eq!( edge.properties.len(), 0 );
    assert_eq!( edge.has_props(), false );

    edge.io_add_property( cons_tag_property( String::from( "tag" )));
    assert_eq!( edge.properties.len(), 1 );
    assert_eq!( edge.has_props(), true );
  }
}