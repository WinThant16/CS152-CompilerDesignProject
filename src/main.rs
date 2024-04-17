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
  Plus,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  Assign,
  Num(i32),
  Ident(String),
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
      } else {
        return (false, Token::NotToken, "");
      }
    }

    StateMachine::Number => {
      if letter >= '0' && letter <= '9' {
        state = StateMachine::Number;
        success = true;
        index += 1;
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