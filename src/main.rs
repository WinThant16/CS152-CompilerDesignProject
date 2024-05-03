// The Rust Programming Language: A Crash Course and Building Our First Lexer
// CS152 Compiler Design using the Rust Programming Language.
// A Handwritten Compiler Using Rust.
// Creating a Lexer By Hand.

// used to get the commandline arguments from the commandline.
use std::env;
// used to interact with the file system
use std::fs;

fn main() {

    // Let us get commandline arguments and store them in a Vec<String>
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file through the commandline arguments for the lexer.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file contents, storing them inside 'code' as a string.
    let filename = &args[1];
    let code = match fs::read_to_string(filename) {
    Err(error) => {
        println!("**Error. File \"{}\": {}", filename, error);
        return;
    }

    Ok(code) => {
        code
    } 

    };

    let tokens = match lex(&code) {
    Err(error_message) => {
        println!("**Error**");
        println!("----------------------");
        println!("{}", error_message);
        println!("----------------------");
        return;
    }

    Ok(data) => data,
    
    };


    // print out the lexer tokens parsed.

    println!("----------------------");
    println!("Finished Lexing the file {}", filename);
    println!("Expression:");
    println!("{code}");
    println!("Here are the Results:");
    println!("----------------------");
    for t in &tokens {
      println!("{:?}", t);
    }

    let mut index: usize = 0;
    println!();

    match parse_program(&tokens, &mut index) {

    Ok(()) => {
        println!("Program Parsed Successfully.");
    }

    Err(message) => {
        println!("**Error**");
        println!("----------------------");
        if tokens.len() == 0 {
            println!("No code has been provided.");
        } else {
            println!("Error: {message}");
            println!("----------------------");
        }
    }

    }


}

// Creating an Enum within Rust.
// Documentation: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// Enums are a way of saying a value is one of a possible set of values.
// Unlike C, Rust enums can have values associated with that particular enum value.
// for example, a Num has a 'i32' value associated with it, 
// but Plus, Subtract, Multiply, etc. have no values associated with it.
#[derive(Debug, Clone)]
enum Token {
  NotToken,
  //math
  Plus,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  Assign,

  Num(i32),
  Ident(String),
  
  //keywords
  If,
  While,
  Read, 
  Func,
  Return,
  Int,
  Print,
  Else,
  Break,
  Continue,

  LeftParen,
  RightParen,
  LeftCurly,
  RightCurly,
  LeftBracket,
  RightBracket,
  Comma,
  Semicolon,
  
  //boolean
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
  Equality,
  NotEqual
}

// In Rust, you can model the function behavior using the type system.
// https://doc.rust-lang.org/std/result/
// Result < Vec<Token>, String>
// means that this function can either return:
// - A list of tokens as a Vec<Token>
// - Or an error message represented as a string
// If there is an error, it will return an error
// If successful, it will return Vec<Token>
// A Result is an enum like this:
// enum Result {
//     Ok(the_result),
//     Err(the_error),
// }


// This is a lexer that parses numbers/identifiers and math operations
fn lex(mut code: &str) -> Result<Vec<Token>, String> {
  let mut tokens: Vec<Token> = vec![];
  while code.len() > 0 {
    let (success, token, rest) = lex_number(code);
    if success {
      code = rest; 
      tokens.push(token);
      continue;
    } 
 
    let (success, rest) = lex_space(code);
    if success {
      code = rest;
      continue;
    }

    if code.starts_with("+") {
      code = &code[1..];
      tokens.push(Token::Plus);
      continue;
    }

    if code.starts_with("-") {
      code = &code[1..];
      tokens.push(Token::Subtract);
      continue;
    }

    if code.starts_with("*") {
      code = &code[1..];
      tokens.push(Token::Multiply);
      continue;
    }

    if code.starts_with("/") {
      code = &code[1..];
      tokens.push(Token::Divide);
      continue;
    }

    if code.starts_with("%") {
      code = &code[1..];
      tokens.push(Token::Modulus);
      continue;
    }

    if code.starts_with("==") {
      code = &code[2..];
      tokens.push(Token::Equality);
      continue;
    }

    if code.starts_with("=") {
      code = &code[1..];
      tokens.push(Token::Assign);
      continue;
    }

    if code.starts_with("("){
      code = &code[1..];
      tokens.push(Token::LeftParen);
      continue;
    }
    
    if code.starts_with(")"){
      code = &code[1..];
      tokens.push(Token::RightParen);
      continue;
    }

    // Check for comment
    if code.starts_with('#') {
      code = skip_comment(code);
      continue;
  }

    if code.starts_with("{") {
      code = &code[1..];
      tokens.push(Token::LeftCurly);
      continue;
    }

    if code.starts_with("}") {
      code = &code[1..];
      tokens.push(Token::RightCurly);
      continue;
    }

    if code.starts_with("[") {
      code = &code[1..];
      tokens.push(Token::LeftBracket);
      continue;
    }

    if code.starts_with("]") {
      code = &code[1..];
      tokens.push(Token::RightBracket);
      continue;
    }

    if code.starts_with(",") {
      code = &code[1..];
      tokens.push(Token::Comma);
      continue;
    }

    if code.starts_with(";") {
      code = &code[1..];
      tokens.push(Token::Semicolon);
      continue;
    } 
      
    if code.starts_with("<=") {
      code = &code[2..];
      tokens.push(Token::LessEqual);
      continue;
    }

    if code.starts_with("<") {
      code = &code[1..];
      tokens.push(Token::Less);
      continue;
    }

    if code.starts_with(">=") {
      code = &code[2..];
      tokens.push(Token::GreaterEqual);
      continue;
    }

    if code.starts_with(">") {
      code = &code[1..];
      tokens.push(Token::Greater);
      continue;
    }

    if code.starts_with("!=") {
      code = &code[2..];
      tokens.push(Token::NotEqual);
      continue;
    }

    let (success, token, rest) = lex_identifier(code);
    if success {
      code = rest;
      tokens.push(token);
      continue;
    }

    let symbol = unrecognized_symbol(code);
    return Err(format!("Unidentified symbol {symbol}"));

  }

  return Ok(tokens);
}

fn lex_space(code: &str) -> (bool, &str) {
  for letter in code.chars() {
    if letter.is_whitespace() {
      return (true, &code[1..]);
    } else {
      return (false, code);
    }
  }
  return (false, code);
}

// lex numbers.
fn lex_number(code: &str) -> (bool, Token, &str) {
  enum StateMachine {
    Start,
    Number,
  }

  let mut success = false;
  let mut state = StateMachine::Start;
  let mut index = 0;
  for letter in code.chars() {
    match state {
    StateMachine::Start => {
      if letter >= '0' && letter <= '9' {
        state = StateMachine::Number;
        success = true;
        index += 1;
      }  else {
        return (false, Token::NotToken, "");
      }
    }

    StateMachine::Number => {
      if letter >= '0' && letter <= '9' {
        state = StateMachine::Number;
        success = true;
        index += 1;
      } else if (letter >= 'A' || letter <= 'Z') && (letter >= 'a' && letter <= 'z'){
        print!("Invalid token: {}\n", letter as i32);
        return (false, Token::NotToken, "");
      } else {
        let num = code[..index].parse::<i32>().unwrap();
        return (true, Token::Num(num), &code[index..]);
      }
    }

    }
  }

  if success == true {
    let num: i32 = code.parse::<i32>().unwrap();
    return (true, Token::Num(num), "");
  } else {
    return (false, Token::NotToken, "");
  }
}

// lex identifiers.
fn lex_identifier(code: &str) -> (bool, Token, &str) {
  enum StateMachine {
    Start,
    Ident,
  }

  let mut success = false;
  let mut state = StateMachine::Start;
  let mut index = 0;
  for letter in code.chars() {
    match state {
    StateMachine::Start => {
      if (letter >= 'a' && letter <= 'z') || (letter >= 'A' && letter <= 'Z'){
        state = StateMachine::Ident;
        success = true;
        index += 1;
      } else {
        return (false, Token::NotToken, "");
      }
    }

    StateMachine::Ident => {
      if (letter >= 'A' && letter <= 'Z') || (letter >= 'a' && letter <= 'z') || (letter >= '0' && letter <= '9') || letter == '_' {
        state = StateMachine::Ident;
        success = true;
        index += 1;
      } else {
        let token = &code[..index];
        return (true, create_identifier(token), &code[index..]);
      }
    }

    }
  }

  if success == true {
    return (true, create_identifier(code), "");
  } else {
    return (false, Token::NotToken, "");
  }
}

// Function to skip comments in the code
// takes a reference to a string (code) as input, returns a slice of the string.
// If the input code contains a newline character ('\n'),  returns a slice starting from the character immediately after the newline.
//If no newline character is found, it returns an empty string.
fn skip_comment(code: &str) -> &str {
  if let Some(pos) = code.find('\n') {
      &code[pos + 1..]
  } else {
      ""
  }
}


fn unrecognized_symbol(code: &str) -> &str {
  enum StateMachine {
    Start,
    Symbol,
  }

  let mut state_machine = StateMachine::Start;
  let mut index = 0;
  for letter in code.chars() {
    match state_machine {
    StateMachine::Start => {
      state_machine = StateMachine::Symbol;
      index += 1;
    } 
    
    StateMachine::Symbol => {
      if letter.is_whitespace() {
        return &code[..index];
      } else {
        index += 1;
      }
    }

    }
  }
  return &code[..index];
} 

fn create_identifier(code: &str) -> Token {
  match code {
  "func" => Token::Func,
  "return" => Token::Return,
  "int" => Token::Int,

  // todo: implement all keywords...
  // ... all keywords...

  "read" => Token::Read,
  "while" => Token::While,
  "if" => Token::If,

  // print, else, break, continue keywords
  "print" => Token::Print,
  "else" => Token::Else,
  "break" => Token::Break,
  "continue" => Token::Continue,
  _ => Token::Ident(String::from(code)),
  }
}



// the <'a> is the "lifetimes" type annotations in Rust.
//
fn peek<'a>(tokens: &'a Vec<Token>, index: usize) -> Option<&'a Token> {
    if index < tokens.len() {
        return Some(&tokens[index])
    } else {
        return None
    }
}

fn peek_result<'a>(tokens: &'a Vec<Token>, index: usize) -> Result<&'a Token, String> {
    if index < tokens.len() {
        return Ok(&tokens[index])
    } else {
        return Err(String::from("expected a token, but got nothing"))
    }
}

fn next<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Option<&'a Token> {
    if *index < tokens.len() {
        let ret = *index;
        *index += 1;
        return Some(&tokens[ret])
    } else {
        return None
    }
}

fn next_result<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<&'a Token, String> {
    if *index < tokens.len() {
        let ret = *index;
        *index += 1;
        return Ok(&tokens[ret])
    } else {
        return Err(String::from("expected a token, but got nothing"))
    }
}

// parse programs with multiple functions
// loop over everything, outputting generated code.
fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  loop {
      let token = peek(tokens, *index); // Peek at the next token in the sequence
      match token {
          None => { // If there are no more tokens, exit the loop
              break;
          }
          Some(token) => {
              if matches!(*token, Token::Func) { // Check if the token is a function declaration
                  parse_function(tokens, index)?; // If it's a function, parse it
              } else {
                  return Err(String::from("Expected function declaration.")); // Otherwise, return an error
              }
              *index += 1; // Move to the next token index
          }
      }
  }
  return Ok(()); // Return Ok if parsing is successful
}

// parse function such as:
// func main(int a, int b) {
//    # ... statements here...
//    # ...
// }
// a loop is done to handle statements.
fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<()>, String> {
  print!("parse_function\n");
  // Check if the next token is 'func'
  match next(tokens, index) {
      None => { // If there are no more tokens, return None
          return Ok(None);
      }
      Some(token) => {
          if !matches!(token, Token::Func) { // If the token is not 'func', return an error
              return Err(String::from("functions must begin with func"));
          }
      }
  }
  // Check if the next token is an identifier
  match next_result(tokens, index)? {
      Token::Ident(_) => {},
      _ => { 
        return Err(String::from("functions must have a function identifier")); 
      }
  };
  // Check if the next token is '('
  if !matches!( next_result(tokens, index)?, Token::LeftParen) {
      return Err(String::from("expected '('"));
  }
  // Loop to parse function parameters
  loop {
      match next_result(tokens, index)? {
          Token::RightParen => { break; }
          Token::Int => {
              match next_result(tokens, index)? {
                  Token::Ident(_) => {
                      match peek_result(tokens, *index)? {
                          Token::Comma => { *index += 1; }
                          Token::RightParen => {}
                          _ => { 
                            return Err(String::from("expected ',' or ')'")); 
                          }
                      }
                  }
                  _ => { 
                    return Err(String::from("expected ident function parameter")); 
                  }
              }
          }
          _ => { 
            return Err(String::from("expected 'int' keyword or ')' token")); 
          }
      }
  }
  // Check if the next token is '{'
  if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
      return Err(String::from("expected '{'"));
  }
  // Loop to parse statements inside the function body
  loop {
      match parse_statement(tokens, index)? {
          None => { break; }
          Some(()) => {}
      }
  }
  // Check if the next token is '}'
  if !matches!(next_result(tokens, index)?, Token::RightCurly) {
      return Err(String::from("expected '}'"));
  }
  return Ok(Some(())); // Return Ok if parsing is successful
}

// parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(a)
// returns epsilon if '}'
fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<()>, String> {
  print!("parse_statement\n");
  match peek(tokens, *index) {
      None => { // If there are no more tokens, return None
          print!("parse statement exit (none)\n");
          return Ok(None);
      }
      Some(token) => {
          match token {
              // If the token is '}', return None
              Token::RightCurly => { 
                return Ok(None); 
              } 
              // If the token is 'int', parse variable declaration
              Token::Int => { 
                *index += 1;
                match next_result(tokens, index)? {
                  Token::Ident(_) => {}

                  _ => {
                      return Err(String::from("expected identifier"));
                  }

                }
                // Check if there's an assignment after variable declaration
                if matches!(peek_result(tokens, *index)?, Token::Assign) {
                    *index += 1;
                    //println!("parse expression after variable declaration\n");
                    parse_expression(tokens, index)?; // Parse the expression after assignment
                } 
              }
              
              // If the token is an identifier
              Token::Ident(_) => {
                *index += 1; // Move to the next token index
                // Check the next token
                match peek(tokens, *index) {
                    Some(Token::Assign) => {
                        // If the next token is '=', parse an assignment
                        *index += 1;
                        //println!("parse expression after identifier");
                        parse_expression(tokens, index)?;
                    }
                    Some(Token::Less) | Some(Token::LessEqual) | Some(Token::Greater) | Some(Token::GreaterEqual) | Some(Token::Equality) | Some(Token::NotEqual) => {
                        // If the next token is a boolean operator, parse a boolean expression
                        //println!("parse boolean expression after variable declaration\n");
                        parse_boolean_expression(tokens, index)?;
                    }
                    _ => {
                        return Err(String::from("unexpected token after identifier"));
                    }
                }
              }
              // If the token is 'return', parse the expression
              Token::Return => { 
                *index += 1; 
                parse_expression(tokens, index)?; 
              }
              // If the token is 'print' 
              Token::Print => { 
                  *index += 1; // Move to the next token index
                  if !matches!(next_result(tokens, index)?, Token::LeftParen) { // If the next token is not '(', return an error
                      return Err(String::from("expect '(' closing statement"));
                  }
                  parse_expression(tokens, index)?; // Parse the expression
                  if !matches!(next_result(tokens, index)?, Token::RightParen) { // If the next token is not ')', return an error
                      return Err(String::from("expect ')' closing statement"));
                  }
              }
              // If the token is 'read'
              Token::Read => { 
                  *index += 1; // Move to the next token index
                  if !matches!(next_result(tokens, index)?, Token::LeftParen) { // If the next token is not '(', return an error
                      return Err(String::from("expect '(' closing statement"));
                  }
                  parse_expression(tokens, index)?; // Parse the expression
                  if !matches!(next_result(tokens, index)?, Token::RightParen) { // If the next token is not ')', return an error
                      return Err(String::from("expect ')' closing statement"));
                  }
              }

              Token::While => { 
                *index += 1; // Move to the next token index
                parse_boolean_expression(tokens, index)?; // Parse boolean expression
                //println!("parsed");
                if !matches!(next_result(tokens, index)?, Token::LeftCurly) { // If the next token is not '{', return an error
                    return Err(String::from("expect '{' for while loop"));
                }
                //println!("after left curly: {:?}", tokens[*index]);
                while !matches!(peek_result(tokens, *index)?, Token::RightCurly){ // while not right bracket
                  //println!("not right bracket: {:?}", tokens[*index]);
                  parse_statement(tokens, index)?;
                }
                *index += 1; // matched a }
                return Ok(Some(())); // skip ; check
              }

              Token::Continue => { 
                *index += 1; // Move to the next token index
              }

              Token::Break => { 
                *index += 1; // Move to the next token index
              }

              // If the token is invalid, return an error
              
              _ => {
                //println!("Token at invalid statement: {:?}", tokens[*index]);
                return Err(String::from("invalid statement.")); } 
          }
          if !matches!(next_result(tokens, index)?, Token::Semicolon) { // If the next token is not ';', return an error
              //println!("not ; : {:?}", tokens[*index]);
              return Err(String::from("expect ';' closing statement after statement"));
          }
          return Ok(Some(())); // Return Ok if parsing is successful
      }
  }
}

// parsing a simple expression such as:
// "a" (alone)
// "a + b"
// "a * b"
// "a - b"
// NOTE: this cannot parse "complex" expressions such as "a + b * c".
// I leave "a + b * c" as an exercise for the student.
fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  print!("parse_expression\n");
  parse_term(tokens, index)?; // Parse the first term
  loop {
    match peek_result(tokens, *index)? {
        Token::Plus | Token::Subtract | Token::Multiply | Token::Divide | Token::Modulus => {
            *index += 1; // Move to the next token
            parse_term(tokens, index)?; // Parse the next term
        }
        _ => break, // If the next token is not an operator, break the loop
    }
  }
  
  return Ok(()); // Return Ok if parsing is successful
}

fn parse_boolean_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  print!("parse_boolean_expression\n");
  parse_term(tokens, index)?; // Parse the left side of the expression
  match peek_result(tokens, *index)? {
      Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual | Token::Equality | Token::NotEqual => {
          *index += 1; // Move to the next token
          parse_term(tokens, index)?; // Parse the right side of the expression
      }
      _ => return Err(String::from("expected boolean operator")),
  }
  return Ok(());
}


// a term is either a Number or an Identifier.
fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match next_result(tokens, index)? {
    Token::Ident(name) => {
        // Check if the first character of the identifier is a digit
        if name.chars().next().unwrap().is_digit(10) {
            return Err(format!("Variable names cannot start with a digit: {}", name));
        }
        // Successfully parsed an identifier
        return Ok(());
    }
    Token::Num(_) => {
        return Ok(());
    }
    Token::LeftParen => {
      // Parse the expression inside the parentheses
      parse_expression(tokens, index)?;
      // Check if the next token is a right parenthesis
      if let Token::RightParen = next_result(tokens, index)? {
          return Ok(());
      } else {
          return Err(String::from("Expecting ')' after '('"));
      }
    }
    _ => {
        return Err(String::from("invalid expression"));
    }
  }
}

// writing tests!
// testing shows robustness in software, and is good for spotting regressions
// to run a test, type "cargo test" in the terminal.
// Rust will then run all the functions annotated with the "#[test]" keyword.
#[cfg(test)]
mod tests {
    use crate::Token;
    use crate::lex;

    #[test]
    fn lexer_test() {

        let toks = lex("1 <2").unwrap();
        assert!(toks.len() == 3);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::Less));
        assert!(matches!(toks[2], Token::Num(2)));

        let toks = lex("1 > 2").unwrap();
        assert!(toks.len() == 3);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::Greater));
        assert!(matches!(toks[2], Token::Num(2)));

        let toks = lex("1 <= 2").unwrap();
        assert!(toks.len() == 3);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::LessEqual));
        assert!(matches!(toks[2], Token::Num(2)));

        let toks = lex("1>=2").unwrap();
        assert!(toks.len() == 3);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::GreaterEqual));
        assert!(matches!(toks[2], Token::Num(2)));

        let toks = lex("1==2").unwrap();
        assert!(toks.len() == 3);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::Equality));
        assert!(matches!(toks[2], Token::Num(2)));

        let toks = lex("1!=2").unwrap();
        assert!(toks.len() == 3);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::NotEqual));
        assert!(matches!(toks[2], Token::Num(2)));

        // test that lexer works on correct cases
        let toks = lex("1 + 2 + 3").unwrap();
        assert!(toks.len() == 5);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(2)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::Num(3)));


        let toks = lex("3 + 215 +-").unwrap();
        assert!(toks.len() == 5);
        assert!(matches!(toks[0], Token::Num(3)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(215)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::Subtract));

        // test that the lexer catches invalid tokens
        assert!(matches!(lex("^^^"), Err(_)));

        //test that lexer identifies left paren and right paren
        let toks = lex("( ( ( ( ) ) ) )").unwrap();
        assert!(toks.len() == 8);
        assert!(matches!(toks[0], Token::LeftParen));
        assert!(matches!(toks[1], Token::LeftParen));
        assert!(matches!(toks[2], Token::LeftParen));
        assert!(matches!(toks[3], Token::LeftParen));
        assert!(matches!(toks[4], Token::RightParen));
        assert!(matches!(toks[5], Token::RightParen));
        assert!(matches!(toks[6], Token::RightParen));
        assert!(matches!(toks[7], Token::RightParen));


        // test for print
        let toks = lex("print ").unwrap();
        assert!(toks.len() == 1);
        assert!(matches!(toks[0], Token::Print));

         // test for else
         let toks = lex("else ").unwrap();
         assert!(toks.len() == 1);
         assert!(matches!(toks[0], Token::Else));

          // test for break
        let toks = lex("break ").unwrap();
        assert!(toks.len() == 1);
        assert!(matches!(toks[0], Token::Break));

         // test for continue
        let toks = lex("continue ").unwrap();
        assert!(toks.len() == 1);
        assert!(matches!(toks[0], Token::Continue));

         //test for comments
        let toks = lex("#Hello \n 1").unwrap();
        assert!(toks.len() == 1);
        assert!(matches!(toks[0], Token::Num(1)));
    }

}

#[cfg(test)]
mod parser_tests {
    use crate::{lex, parse_statement, Token};

    #[test]
    fn test_assignment() {
        // Test valid assignments
        let tokens = lex("a = 1 + 2;").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();

        // Test assignment with a boolean expression
        let tokens = lex("b = a > 5;").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();

        // Test assignment with a parenthesized expression
        let tokens = lex("c = (a * 3);").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();

        // Test assignment with multiple operators
        let tokens = lex("d = (a + b) / (c - 1);").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();
    }

    #[test]
    fn test_error_handling() {
        // Test missing semicolon
        let tokens = lex("e = a + b").unwrap();
        assert!(matches!(parse_statement(&tokens, &mut 0), Err(_)));

        // Test invalid expression
        let tokens = lex("f = a * ;").unwrap();
        assert!(matches!(parse_statement(&tokens, &mut 0), Err(_)));

        // Test assignment with an invalid identifier
        let tokens = lex("3 = a + b;").unwrap();
        assert!(matches!(parse_statement(&tokens, &mut 0), Err(_)));
    }

    #[test]
    fn test_control_flow() {
        // Test if statement
        let tokens = lex("if a > 5 { b = 10; } else { b = 5; }").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();

        // Test while loop
        let tokens = lex("while a < 10 { a = a + 1; }").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();

        // Test for loop
        let tokens = lex("for i = 0; i < 10; i = i + 1 { println(i); }").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();
    }

    #[test]
    fn test_function_definition() {
        // Test function definition
        let tokens = lex("fn add(x, y) { return x + y; }").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();

        // Test function call
        let tokens = lex("result = add(3, 5);").unwrap();
        parse_statement(&tokens, &mut 0).unwrap();
    }
}
