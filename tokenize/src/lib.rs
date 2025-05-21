#[derive( Debug, Clone )]
pub struct SyntaxToken
{ 
  pub token_type: SyntaxTokenType,
  pub val: String,
}

pub fn cons_syntax_token ( token_type: SyntaxTokenType, val: String ) -> SyntaxToken 
{
  SyntaxToken { token_type: token_type, val: val }
}

#[derive( Debug, PartialEq, Clone )]
pub enum SyntaxTokenType 
{
  KeywordCreate, KeywordGraph, KeywordReturn, KeywordAs, KeywordWhere,
  KeywordFrom, KeywordInsert, KeywordInto, KeywordMatch,

  Label, PrimaryLabel,
  OpenNode, CloseNode,
  OpenEdge, CloseEdge,
  EdgeDirection, EdgeLeft, EdgeRight,
  OpenBrace, CloseBrace,
  Key, Value,
}

#[derive( Debug, PartialEq )]
pub enum Keyword 
{
  Create, Graph, Return, As, Where, From, Insert, Into, Match,
}

#[derive( Debug, PartialEq )]
pub enum TokenType 
{
  Space, NewLine,
  OpenParen, CloseParen,
  OpenBracket, CloseBracket,
  OpenBrace, CloseBrace,
  Quote,
  FrontSlash,
  Hyphen, Colon, Comma,
  Char,
  LT, GT,
}

pub fn token_type ( c: &char ) -> TokenType 
{
  if is_open_paren( c ) { return TokenType::OpenParen }
  if is_close_paren( c ) { return TokenType::CloseParen }
  if is_hyphen( c ) { return TokenType::Hyphen }
  if is_space( c ) { return TokenType::Space }
  if is_colon( c ) { return TokenType::Colon }
  if is_open_bracket( c ) { return TokenType::OpenBracket }
  if is_close_bracket( c ) { return TokenType::CloseBracket }
  if is_front_slash( c ) { return TokenType::FrontSlash }
  if is_new_line( c ) { return TokenType::NewLine }
  if is_open_brace( c ) { return TokenType::OpenBrace }
  if is_close_brace( c ) { return TokenType::CloseBrace }
  if is_squote( c ) { return TokenType::Quote }
  if is_dquote( c ) { return TokenType::Quote }
  if is_comma( c ) { return TokenType::Comma }
  if is_open_lt( c ) { return TokenType::LT }
  if is_close_gt( c ) { return TokenType::GT }
  TokenType::Char
}

pub fn is_space ( c: &char ) -> bool { *c == ' ' }
pub fn is_new_line ( c: &char ) -> bool { *c == '\n' }
pub fn is_carriage_return ( c: &char ) -> bool { *c == '\r' }
pub fn is_tab ( c: &char ) -> bool { *c == '\t' }

pub fn is_open_brace ( c: &char ) -> bool { *c == '{' }
pub fn is_close_brace ( c: &char ) -> bool { *c == '}' }

pub fn is_open_paren ( c: &char ) -> bool { *c == '(' }
pub fn is_close_paren ( c: &char ) -> bool { *c == ')' }

pub fn is_open_lt ( c: &char ) -> bool { *c == '<' }
pub fn is_close_gt ( c: &char ) -> bool { *c == '>' }

pub fn is_open_bracket ( c: &char ) -> bool { *c == '[' }
pub fn is_close_bracket ( c: &char ) -> bool { *c == ']' }

pub fn is_underscore ( c: &char ) -> bool { *c == '_' }
pub fn is_dollar_sign ( c: &char ) -> bool { *c == '$' }
pub fn is_asterisk ( c: &char ) -> bool { *c == '*' }
pub fn is_front_slash ( c: &char ) -> bool { *c == '/' }
pub fn is_back_slash ( c: &char ) -> bool { *c == '\\' }

pub fn is_squote ( c: &char ) -> bool { *c == '\'' }
pub fn is_dquote ( c: &char ) -> bool { *c == '\"' }
pub fn is_bquote ( c: &char ) -> bool { *c == '`' }

pub fn is_comma ( c: &char ) -> bool { *c == ',' }
pub fn is_semicolon ( c: &char ) -> bool { *c == ';' }
pub fn is_ampersand ( c: &char ) -> bool { *c == '&' }

pub fn is_equals ( c: &char ) -> bool { *c == '=' }
pub fn is_period ( c: &char ) -> bool { *c == '.' }
pub fn is_colon ( c: &char ) -> bool { *c == ':' }
pub fn is_plus ( c: &char ) -> bool { *c == '+' }
pub fn is_minus ( c: &char ) -> bool { *c == '-' }
pub fn is_hyphen ( c: &char ) -> bool { *c == '-' }
pub fn is_hash ( c: &char ) -> bool { *c == '#' }
pub fn is_at ( c: &char ) -> bool { *c == '@' }
pub fn is_pipe ( c: &char ) -> bool { *c == '|' }
pub fn is_bang ( c: &char ) -> bool { *c == '!' }
pub fn is_question_mark ( c: &char ) -> bool { *c == '?' }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_some_char () 
  {
    assert_eq!( is_space( &' ' as &char ), true );
    assert_eq!( is_new_line( &'\n' as &char ), true );
    assert_eq!( is_carriage_return( &'\r' as &char ), true );
    assert_eq!( is_tab( &'\t' as &char ), true );
    
    assert_eq!( is_open_brace( &'{' as &char ), true );
    assert_eq!( is_close_brace( &'}' as &char ), true );

    assert_eq!( is_open_paren( &'(' as &char ), true );
    assert_eq!( is_close_paren( &')' as &char ), true );

    assert_eq!( is_open_lt( &'<' as &char ), true );
    assert_eq!( is_close_gt( &'>' as &char ), true );

    assert_eq!( is_open_bracket( &'[' as &char ), true );
    assert_eq!( is_close_bracket( &']' as &char ), true );

    assert_eq!( is_underscore( &'_' as &char ), true );
    assert_eq!( is_dollar_sign( &'$' as &char ), true );
    assert_eq!( is_asterisk( &'*' as &char ), true );
    assert_eq!( is_front_slash( &'/' as &char ), true );
    assert_eq!( is_back_slash( &'\\' as &char ), true );

    assert_eq!( is_squote( &'\'' as &char ), true );
    assert_eq!( is_dquote( &'"' as &char ), true );
    assert_eq!( is_bquote( &'`' as &char ), true );

    assert_eq!( is_comma( &',' as &char ), true );
    assert_eq!( is_semicolon( &';' as &char ), true );
    assert_eq!( is_ampersand( &'&' as &char ), true );

    assert_eq!( is_equals( &'=' as &char ), true );
    assert_eq!( is_period( &'.' as &char ), true );
    assert_eq!( is_colon( &':' as &char ), true );
    assert_eq!( is_plus( &'+' as &char ), true );
    assert_eq!( is_minus( &'-' as &char ), true );
    assert_eq!( is_hash( &'#' as &char ), true );
    assert_eq!( is_at( &'@' as &char ), true );
    assert_eq!( is_pipe( &'|' as &char ), true );
    assert_eq!( is_bang( &'!' as &char ), true );
    assert_eq!( is_question_mark( &'?' as &char ), true );
  }
}