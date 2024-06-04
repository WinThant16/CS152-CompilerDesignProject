// The Rust Programming Language: A Crash Course and Building Our First Lexer
// CS152 Compiler Design using the Rust Programming Language.
// A Handwritten Compiler Using Rust.
// Creating a Lexer By Hand.

// used to get the commandline arguments from the commandline.
use std::env;
// used to interact with the file system
use std::fs;
// for our symbol table
use std::collections::HashMap;

mod interpreter;

struct Expression {
  code: String,
  name: String,
}

static mut VAR_NUM: i64 = 0;
fn create_temp() -> String {
  unsafe {
      VAR_NUM += 1;
      format!("_temp{}", VAR_NUM)
  }
}

static mut whileloopbegin_num: i64 = 0;
fn create_whileloopbegin_label() -> String {
  unsafe {
    whileloopbegin_num += 1;
      format!("loopbegin{}", whileloopbegin_num)
  }
}

static mut whileloopend_num: i64 = 0;
fn create_whileloopend_label() -> String {
  unsafe {
    whileloopend_num += 1;
      format!("endloop_{}", whileloopend_num)
  }
}

static mut iftrue_num: i64 = 0;
fn create_iftrue_label() -> String {
  unsafe {
    iftrue_num += 1;
      format!("iftrue{}", iftrue_num)
  }
}

static mut endif_num: i64 = 0;
fn create_endif_label() -> String {
  unsafe {
    endif_num += 1;
      format!("endif{}", endif_num)
  }
}

static mut else_num: i64 = 0;
fn create_else_label() -> String {
  unsafe {
    else_num += 1;
      format!("else{}", else_num)
  }
}
enum DataType {
  Array,
  Int,
  Function,
}

impl std::fmt::Display for DataType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        DataType::Array => write!(f, "Int[]"),
        DataType::Int => write!(f, "Int"),
        DataType::Function => write!(f, "Function"),
      }
  }
}

fn semantics_check(generated_code: String) -> bool {
  // each symbol name will map to a datatype
  // data types will be array, int, or function

  let mut symbol_table: HashMap<String,DataType> = HashMap::new();
  // we only add from %func, %int, and %int instructions

  //let mut cleaned_code = generated_code.replace(",", "");
  //cleaned_code = cleaned_code.replace("[", "");
  //cleaned_code = cleaned_code.replace("]", "");

  let mut main_function_seen = false;
  let mut scope_name = "";
  let mut line2;

  for gline in generated_code.lines() {
    //println!("gline: {}", gline);
    let line = gline.replace(",", " ");
    //stuff that'll add to symbol table
    if line.starts_with("%func"){
      line2 = line.replace(")", " )");
      line2 = line2.replace("(", " ");
      //println!("line {}",line);
      let func_name = line2.split_whitespace().nth(1).unwrap();
      //println!("func_name: {}", func_name);
      if func_name == "main"{
        main_function_seen = true;
      }
      scope_name = func_name;
      //println!("scope_name: {}", scope_name);
      let key_name = func_name.to_string()+"|main";
      if(symbol_table.contains_key(&key_name)){
        println!("Error: Function {func_name} already defined.");
        return false;
      }
      symbol_table.insert(key_name, DataType::Function);
      // add function parameters to symbol table
      let mut params = line2.split_whitespace().skip(2);
      for param in params{
        if param == ")"{
          break;
        }
        if(param == "%int"){
          continue;
        }
        let key_name = param.to_string()+"|"+scope_name;
        //println!("param {key_name}");
        if(symbol_table.contains_key(&key_name)){
          println!("Error: Duplicate parameter {param} declared in {scope_name}.");
          return false;
        }
        symbol_table.insert(key_name, DataType::Int);
      }
      continue;
    }
    if line.starts_with("%int[]"){
      let var_name = line.split_whitespace().nth(1).unwrap();
      let array_size = line.split_whitespace().nth(2).unwrap();
      //println!("array_name: {}", var_name);
      if(array_size.parse::<i32>().unwrap() <= 0){
        println!("Error: Array size of {var_name} must be greater than 0.");
        return false;
      }
      let key_name = var_name.to_string()+"|"+scope_name;
      //println!("{} is array", key_name);
      if(symbol_table.contains_key(&key_name)){
        println!("Error: Variable {key_name} already declared.");
        return false;
      }
      symbol_table.insert(key_name, DataType::Array);
      continue;
    }
    if line.starts_with("%int"){
      let var_name = line.split_whitespace().nth(1).unwrap();
      //println!("var_name: {}", var_name);
      let key_name = var_name.to_string()+"|"+scope_name;
      if(symbol_table.contains_key(&key_name)){
        println!("Error: Variable {var_name} already declared.");
        return false;
      }
      symbol_table.insert(var_name.to_string()+"|"+scope_name, DataType::Int);
      continue;
    }
    // semantics check the line
    let mut clean_line = line.replace("[", "");
    clean_line = clean_line.replace("]", "");
    clean_line = clean_line.replace("(", "( ");
    clean_line = clean_line.replace(")", "");
    let mut seen_array_type = false;
    let mut seen_tokens: Vec<&str> = vec![];
    for param in clean_line.split_whitespace(){
      if param.starts_with("%") || param.starts_with(":"){
        continue;
      }
        // not a number, not a +, and not in symbol table.. undeclared or undefined
        //println!("param: {}", param);
        let key_name = param.replace("(","")+"|"+scope_name;
        //println!("key_name: {}", key_name);
        if !symbol_table.contains_key(&(key_name)) && !param.parse::<i32>().is_ok() && !param.starts_with("+"){
          if(param.ends_with("(")){
            println!("Error: Undefined function used: {}", param.replace("(", ""));
          }else{
            println!("Error: Undeclared variable used: {}", param);
          }
          return false;
        }
        // param is a +, check if last token is an array
        if param == "+" {
          let last_token = *seen_tokens.last().unwrap();
          let data_type = symbol_table.get(&(last_token.to_string() + "|" + scope_name)).unwrap();
          if(!matches!(data_type, &DataType::Array)){
            println!("Error: Type mismatch. '{last_token}' is not an array.");
            //println!("Error: Attempt to use {data_type} {last_token} like array.");
            return false;
          }else{
            seen_array_type = false;
          }
        }
        // if seenarraytype is true and we passed +, then we are trying to use an array like a number
        if(seen_array_type){
          let last_token = *seen_tokens.last().unwrap();
          println!("Error: Type mismatch. Used '{last_token}' as an integer.");
          //println!("Error: Attempt to use array {last_token} like number.");
          return false;
        }
        // set seenArrayType flag
        if symbol_table.contains_key(&(param.to_string() + "|" + scope_name)){
          let data_type = symbol_table.get(&(param.to_string() + "|" + scope_name)).unwrap();
          if(matches!(data_type, &DataType::Array)){
            seen_array_type = true;
            //println!("set array flag {}", seen_array_type);
          }
        }
        seen_tokens.push(param);
    }
  }
  if(main_function_seen == false){
    println!("Error: Main function not defined.");
    return false;
  }
  return true;
}
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

    Ok(generated_code) => {
        println!("Program Parsed Successfully.");
        println!("{}",&generated_code);
        // let generated_code: String = parse(tokens)?;

        // SEMANTICS CHECK!!!!
        if (semantics_check(generated_code.clone())){
          interpreter::execute_ir(&generated_code);
        }
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
    
    // 

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
fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
  let mut generated_code = String::from("");
  loop {
      match parse_function(tokens, index)? {
      None => {
          break;
      }
      Some(func_code) => {
        generated_code += &func_code;
      }
      }
  }
  return Ok(generated_code);
}

// parse function such as:
// func main(int a, int b) {
//    # ... statements here...
//    # ...
// }
// a loop is done to handle statements.
fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<String>, String> {
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
  let func_ident = match next_result(tokens, index)? {
      Token::Ident(func_ident) => func_ident,
      _ => { 
        return Err(String::from("functions must have a function identifier")); 
      }
  };
  // Check if the next token is '('
  if !matches!( next_result(tokens, index)?, Token::LeftParen) {
      return Err(String::from("expected '('"));
  }
  
  let mut code = format!("%func {}", func_ident);
  let mut params: Vec<String> = vec![];

  // Loop to parse function parameters
  loop {
      match next_result(tokens, index)? {
          Token::RightParen => { break; }
          Token::Int => {
              match next_result(tokens, index)? {
                  Token::Ident(param) => {
                    params.push(param.clone());  
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
  // if there are params, add them to the function signature
  print!("params: {:?}\n", params);
  if(params.len() > 0){
    code += &format!("(");
    for param in params.iter() {
      code += &format!("%int {}", param);
      code += &format!(", ");
    }
    code += &format!(")");
    code = code.replace(", )", ")");
  }
  code += &format!("\n");

  // Check if the next token is '{'
  if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
      return Err(String::from("expected '{'"));
  }
  // Loop to parse statements inside the function body
  loop {
      match parse_statement(tokens, index)? {
          None => { break; }
          Some(statement) => {
            code += &statement;
          }
      }
  }
  code += "%endfunc\n\n";
  // Check if the next token is '}'
  if !matches!(next_result(tokens, index)?, Token::RightCurly) {
      return Err(String::from("expected '}'"));
  }
  return Ok(Some(code)); // Return Ok if parsing is successful
}

// parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(a)
// returns epsilon if '}'
fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<String>, String> {
  print!("parse_statement {:?}\n", tokens[*index]);
  match peek(tokens, *index) {
      None => { // If there are no more tokens, return None
          print!("parse statement exit (none)\n");
          return Ok(None);
      }
      Some(token) => {
        let codenode: Option<String>;
          match token {
              // If the token is '}', return None
              Token::RightCurly => {
                codenode = None;
                return Ok(codenode); 
              } 
              // If the token is 'int', parse variable declaration
              Token::Int => { 
                *index += 1;
                match next_result(tokens, index)? {
                  
                  // need to return intermediate code for arrays here
                  Token::LeftBracket => {
                    let array_param = parse_array_form(tokens, index)?;
                    match peek(tokens, *index) {
                      Some(Token::Ident(ident)) => {
                        *index += 1;
                        // let m_expr = parse_term(tokens, index)?; // Parse the next term
                        // let t = create_temp();
                        // let instr = format!("%int {}\n{opcode} {}, {}, {}\n", t, t, expr.name, m_expr.name);
                        // expr.code += &m_expr.code;
                        // expr.code += &instr;
                        // expr.name = t;

                        let statement = format!("%int[] {}, {}\n", ident, array_param.name);
                        codenode = Some(statement);
                      }
                      _ => {
                        return Err(String::from("expected identifier"));
                      }
                    }
                  }

                  Token::Ident(ident) => {
                    let statement = format!("%int {}\n", ident);
                    codenode = Some(statement);
                    //println!("bsdgdfsdf");
                  }

                  _ => {
                      return Err(String::from("expected identifier"));
                  }

                }
                // <NOT REQUIRED/PART OF PROMPT> Check if there's an assignment after variable declaration
                //if matches!(peek_result(tokens, *index)?, Token::Assign) {
                 //   *index += 1;
                    //println!("parse expression after variable declaration\n");
                //} 
                
              }
              
              // If the token is an identifier
              Token::Ident(ident) => {
                *index += 1; // Move to the next token index
                // Check the next token
                match peek(tokens, *index) {
                    Some(Token::Assign) => {
                        // If the next token is '=', parse an assignment
                        *index += 1;
                        println!("parse expression after identifier");
                        let expr = parse_expression(tokens, index)?;
                        let code = format!("{}%mov {}, {}\n", expr.code, ident, expr.name);
                        codenode = Some(code);
                        // codenode = None;
                        
                    }
                    Some(Token::Less) | Some(Token::LessEqual) | Some(Token::Greater) | Some(Token::GreaterEqual) | Some(Token::Equality) | Some(Token::NotEqual) => {
                        // If the next token is a boolean operator, parse a boolean expression
                        //println!("parse boolean expression after variable declaration\n");
                        // parse_boolean_expression(tokens, index)?;
                        let bool_expr = parse_boolean_expression(tokens, index)?;
                        let code = format!("{}%mov {}\n", bool_expr.code, bool_expr.name);
                        codenode = Some(code);
                    }
                    Some(Token::LeftBracket) =>{
                        let array_param =parse_array_form(tokens, index)?;
                        //println!("current tok4: {:?}", tokens[*index]);
                        match peek(tokens, *index){
                          Some(Token::Assign) => {
                            // If the next token is '=', parse an assignment
                            *index += 1;
                            //println!("parse expression after identifier");
                            // parse_expression(tokens, index)?;
                            let expr = parse_expression(tokens, index)?;
                            let code = format!("{}%mov [{} + {}], {}\n", expr.code, ident, array_param.name, expr.name);
                            codenode = Some(code);
                          }
                          Some(Token::Less) | Some(Token::LessEqual) | Some(Token::Greater) | Some(Token::GreaterEqual) | Some(Token::Equality) | Some(Token::NotEqual) => {
                            // If the next token is a boolean operator, parse a boolean expression
                            //println!("parse boolean expression after variable declaration\n");
                            // parse_boolean_expression(tokens, index)?;
                            let bool_expr = parse_boolean_expression(tokens, index)?;
                            let code = format!("{}%mov {}\n", bool_expr.code, bool_expr.name);
                            codenode = Some(code);
                          }
                          _ => {
                            codenode = None;
                            return Err(String::from("unexpected token after identifier []"));
                        }
                        }

                    }
                    _ => {
                        codenode = None;
                        return Err(String::from("unexpected token after identifier"));
                    }
                }
              }
              // If the token is 'return', parse the expression
              Token::Return => { 
                *index += 1; 
                let expr = parse_expression(tokens, index)?;
                let code = format!("{}%ret {}\n", expr.code, expr.name);
                codenode = Some(code);
              }
              // If the token is 'print' 
              Token::Print => { 
                  *index += 1; // Move to the next token index
                  if !matches!(next_result(tokens, index)?, Token::LeftParen) { // If the next token is not '(', return an error
                      return Err(String::from("expect '(' closing statement"));
                  }
                  let mut expr = parse_term(tokens, index)?;
                  print!("parse print expression: {}\n", expr.name);
                  let array_num;
                  if(matches!(peek_result(tokens, *index)?, Token::LeftBracket)){
                    array_num = parse_array_form(tokens, index)?;
                    let t = create_temp();
                    expr.code = format!("{}%int {}\n%mov {}, [{} + {}]\n", expr.code,t,t, expr.name, array_num.name);
                    expr.name = t;
                  }
                  let code = format!("{}%out {}\n", expr.code, expr.name);
                  if !matches!(next_result(tokens, index)?, Token::RightParen) { // If the next token is not ')', return an error
                      return Err(String::from("expect ')' closing statement"));
                  }
                  codenode = Some(code)
              }
              // If the token is 'read'
              Token::Read => { 
                  *index += 1; // Move to the next token index
                  if !matches!(next_result(tokens, index)?, Token::LeftParen) { // If the next token is not '(', return an error
                      return Err(String::from("expect '(' closing statement"));
                  }
                  let expr = parse_expression(tokens, index)?;
                  let code = format!("{}%input {}\n", expr.code, expr.name);
                  if !matches!(next_result(tokens, index)?, Token::RightParen) { // If the next token is not ')', return an error
                      return Err(String::from("expect ')' closing statement"));
                  }
                  codenode = Some(code);
              }

              Token::While => { 
                //println!("While");
                *index += 1; // Move to the next token index
                let while_label = create_whileloopbegin_label();
                let endwhile_label = create_whileloopend_label();
                let mut statementCode: String = String::from("");
                let condition = parse_boolean_expression(tokens, index)?; // Parse boolean expression
                let mut code = format!(":{}\n", while_label);
                code += &format!("{}%branch_ifn {}, :{}\n",condition.code, condition.name, endwhile_label);
                //println!("parsed");
                if !matches!(next_result(tokens, index)?, Token::LeftCurly) { // If the next token is not '{', return an error
                    return Err(String::from("expect '{' for while loop"));
                }
                //println!("after left curly: {:?}", tokens[*index]);
                while !matches!(peek_result(tokens, *index)?, Token::RightCurly){ // while not right bracket
                  //println!("not right bracket: {:?}", tokens[*index]);
                  // parse_statement(tokens, index)?;
                  match parse_statement(tokens, index)? {
                    None => { /* do nothing lol */ }
                    Some(statement) => {
                      statementCode += &statement;
                    }
                  }
                }
                *index += 1; // matched a }
                code += &statementCode;
                code += &format!("%jmp :{}\n",while_label);
                code += &format!(":{}\n", endwhile_label);
                
                // return Ok(Some(String::from("TODO:While"))); // skip ; check
                return Ok(Some(code)); // skip ; check
              }
            
              Token::If => {
                //println!("if");
                *index += 1; // Move to the next token index
                // create labels
                let iftrue_label = create_iftrue_label();
                let endif_label = create_endif_label();
                //let else_label = create_else_label();
                println!("before if parse boolean expression: {:?}", tokens[*index]);
                let conditional = parse_boolean_expression(tokens, index)?; // Parse boolean expression
                println!("after if parse boolean expression: {:?}", tokens[*index]);
                let mut code = format!("{}%branch_if {}, :{}\n",conditional.code, conditional.name, iftrue_label);
                let mut statementCode: String = String::from("");
                if !matches!(next_result(tokens, index)?, Token::LeftCurly) { // If the next token is not '{', return an error
                  return Err(String::from("expect '{' closing statement"));
                }
                while !matches!(peek_result(tokens, *index)?, Token::RightCurly){ // while not right bracket
                  match parse_statement(tokens, index)? {
                    None => { /* do nothing lol */ }
                    Some(statement) => {
                      statementCode += &statement;
                    }
                  }
                }
                *index += 1;
                // handle else
                if matches!(peek_result(tokens, *index)?, Token::Else) {
                  *index += 1; // Move to the next token index
                  if !matches!(next_result(tokens, index)?, Token::LeftCurly) { // If the next token is not '{', return an error
                    return Err(String::from("expect '{' opening statement"));
                  }
                  while !matches!(peek_result(tokens, *index)?, Token::RightCurly){ // while not right bracket
                    match parse_statement(tokens, index)? {
                      None => { /* do nothing lol */ }
                      Some(statement) => {
                        code += &statement;
                      }
                    }
                  }

                  *index += 1;
                }
                code += &format!("%jmp :{}\n", endif_label);
                code += &format!(":{}\n", iftrue_label);
                code += &statementCode;
                code += &format!(":{}\n", endif_label);
                return Ok(Some(code)); // skip ; check
              }
/*
              Token::Continue => { 
                *index += 1; // Move to the next token index
              }

              Token::Break => { 
                *index += 1; // Move to the next token index
              }
*/
              // If the token is invalid, return an error
              
              _ => {
                codenode = None;
                println!("Token at invalid statement: {:?}", tokens[*index]);
                return Err(String::from("invalid statement.")); } 
          }
          //println!("before ; : {:?}", tokens[*index]);
          if !matches!(next_result(tokens, index)?, Token::Semicolon) { // If the next token is not ';', return an error
              println!("not ; : {:?}", tokens[*index]);
              return Err(String::from("expect ';' closing statement after statement"));
          }
          return Ok(codenode); // Return Ok if parsing is successful
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
fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
  let mut expr = parse_multiply_expression(tokens, index)?; // this gets the identifier or num
  print!("parse_expression {}\n", expr.name);
  //parse_term(tokens, index)?; // Parse the first term
  if(matches!(peek_result(tokens, *index)?, Token::LeftBracket)){
    let arraynum = parse_array_form(tokens, index)?;
    let t = create_temp();
    expr.code = format!("{}%int {}\n%mov {}, [{} + {}]\n", expr.code,t,t, expr.name, arraynum.name);
    expr.name = format!("{}", t);
  }
  loop {
    let opcode = match peek_result(tokens, *index)?{
      Token::Plus => "%add",
      Token::Subtract => "%sub",
      // Token::Multiply => "%mult",
      // Token::Divide => "%div",
      // Token::Modulus => "%mod",
      _ => break,
    };
    *index += 1; //go to next token
    let mut right_expr = parse_multiply_expression(tokens, index)?; // Parse the next higher precedence expression
    let temp_var = create_temp(); // Create a temporary variable for the result
    let code = format!("%int {}\n{} {}, {}, {}\n", temp_var, opcode, temp_var, expr.name, right_expr.name);
    expr.code += &right_expr.code; // Combine the code from both expressions
    expr.code += &code;
    expr.name = temp_var; // Update the name to the temporary variable
  }
  Ok(expr) // Return the final expression
}

// Function to parse and evaluate expressions involving multiplication, division, and modulus
fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
  // Start by parsing a term (either a number or an expression inside parentheses)
  let mut expr = parse_term(tokens, index)?;
  // Loop to handle multiplication, division, and modulus operations
  loop {
      let opcode = match peek_result(tokens, *index)? {
          Token::Multiply => "%mult",
          Token::Divide => "%div",
          Token::Modulus => "%mod",
          _ => break, // If it's not a multiplication, division, or modulus token, break the loop
      };
      *index += 1; // Move to the next token
      let mut right_expr = parse_term(tokens, index)?; // Parse the next term
      let temp_var = create_temp(); // Create a temporary variable for the result
      let code = format!("%int {}\n{} {}, {}, {}\n", temp_var, opcode, temp_var, expr.name, right_expr.name);
      expr.code += &right_expr.code; // Combine the code from both expressions
      expr.code += &code;
      expr.name = temp_var; // Update the name to the temporary variable
  }
  Ok(expr) // Return the final expression
}


fn parse_array_form(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
  //println!("current tok: {:?}", tokens[*index]);
  //println!("run parse expression after [");
  if(matches!(peek_result(tokens, *index)?,Token::LeftBracket)){
    *index += 1;
  }
  let number = parse_term(tokens, index)?; 
  //println!("current tok2: {:?}", tokens[*index]);
  if !matches!(next_result(tokens, index)?, Token::RightBracket) {
    return Err(String::from("expected ']'"));
  }
  //println!("after ]: {:?}", tokens[*index]);
  //*index += 1;

  return Ok(number)
}

fn parse_boolean_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
  print!("parse_boolean_expression\n");
  println!("term: {:?}", tokens[*index]);
  let mut expr = parse_term(tokens, index)?; // Parse the left side of the expression
  println!("after term: {:?}", tokens[*index]);
  let mut instr = "";
  //println!("parsed : {:?}", tokens[*index]);

  if(matches!(peek_result(tokens, *index)?,Token::LeftBracket)){
    parse_array_form(tokens,index)?;
  }
  let opcode = match peek_result(tokens, *index)?{
    Token::Less => "%lt",
    Token::LessEqual => "%le",
    Token::Greater => "%gt",
    Token::GreaterEqual => "%ge",
    Token::Equality => "%eq",
    Token::NotEqual => "%neq",
    _ => "%mov"
  };

  match peek_result(tokens, *index)? {
      Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual | Token::Equality | Token::NotEqual => {
          *index += 1; // Move to the next token
          println!("boolean operator : {:?}", tokens[*index]);
          // parse_term(tokens, index)?; // Parse the right side of the expression
          println!("term: {:?}", tokens[*index]);
          let m_expr = parse_term(tokens, index)?; // Parse the next term
          println!("after term: {:?}", tokens[*index]);
          let t = create_temp();
          let code = format!("%int {t}\n{opcode} {t}, {}, {}\n", expr.name, m_expr.name);
          expr.code += &m_expr.code;
          expr.code += &code;
          expr.name = t;
      }
      _ => {
        println!("not boolean operator : {:?}", tokens[*index]);
        return Err(String::from("expected boolean operator"))
      },
  }
  return Ok(expr);
}


// a term is either a Number or an Identifier.
// fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
//   match next_result(tokens, index)? {
//     Token::Ident(name) => {
//         // Check if the first character of the identifier is a digit
//         if name.chars().next().unwrap().is_digit(10) {
//             return Err(format!("Variable names cannot start with a digit: {}", name));
//         }
//         // Successfully parsed an identifier
//         let expr = Expression {
//           code : String::from(""),
//           name : name.clone(),
//         };
//         return Ok(expr);
//     }
//     Token::Num(num) => {
//         let expr = Expression {
//           code : String::from(""),
//           name : format!("{}", num),
//         };
//         return Ok(expr);
//     }
//     Token::LeftParen => {
//       // Parse the expression inside the parentheses
//       parse_expression(tokens, index)?;
//       // Check if the next token is a right parenthesis
//       if let Token::RightParen = next_result(tokens, index)? {
//         let expr = Expression {
//           code : String::from(""),
//           name : format!("{}", "("),
//         };
//           return Ok(expr);
//       } else {
//           return Err(String::from("Expecting ')' after '('"));
//       }
//     }
//     _ => {
//         return Err(String::from("invalid expression"));
//     }
//   }
// }

// fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
//   match next_result(tokens, index)? {
//       Token::Ident(name) => {
//           let mut expr = Expression {
//               code: String::new(),
//               name: name.clone(),
//           };
//           if(matches!(peek_result(tokens, *index)?, Token::LeftParen)){
//             println!("function call {}", name);
//             *index += 1;
//             let mut params: Vec<String> = vec![];
//             loop{
//               match peek_result(tokens, *index)? {
//                 Token::RightParen => {
//                   *index += 1;
//                   break;
//                 }
//                 _ => {
//                   let expr_param = parse_expression(tokens, index)?;
//                   params.push(expr_param.name);
//                   expr.code += &expr_param.code;
//                   if(matches!(peek_result(tokens, *index)?, Token::Comma)){
//                     *index += 1;
//                   } else {
//                      // pass the right paren
//                     *index += 1;
//                     break;
//                   }
//                 }
//               }
//             }
//             // done with params
//             // what we will write the return value to
//             let t = create_temp();
//             expr.code += &format!("%int {}\n", t);
//             expr.code += &format!("%call {}, {}({})\n", t, name, params.join(","));
//             expr.name = t;
          
//           }
//           Ok(expr)
//       },
//       Token::Num(num) => {
//           let expr = Expression {
//               code: String::new(),
//               name: format!("{}", num),
//           };
//           Ok(expr)
//       },
//       Token::LeftParen => {
//           let expr = parse_expression(tokens, index)?;
//           if !matches!(next_result(tokens, index)?, Token::RightParen) {
//               Err(String::from("Expecting ')' after '('"))
//           } else {
//               Ok(expr)
//           }
//       },
//       _ => Err(String::from("invalid expression")),
//   }
// }

fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
  match next_result(tokens, index)? {
      Token::Num(num) => {
          let expr = Expression {
              code: String::from(""),
              name: format!("{}", num),
          };
          Ok(expr)
      }
      Token::Ident(name) => {
          let mut expr = Expression {
              code: String::new(),
              name: name.clone(),
          };
          // Check for function call or array indexing
          match peek_result(tokens, *index)? {
              Token::LeftParen => {
                  *index += 1;
                  let mut params: Vec<String> = vec![];
                  loop {
                      match peek_result(tokens, *index)? {
                          Token::RightParen => {
                              *index += 1;
                              break;
                          }
                          _ => {
                              let expr_param = parse_expression(tokens, index)?;
                              params.push(expr_param.name);
                              expr.code += &expr_param.code;
                              if matches!(peek_result(tokens, *index)?, Token::Comma) {
                                  *index += 1;
                              } else {
                                  *index += 1;
                                  break;
                              }
                          }
                      }
                  }
                  let temp_var = create_temp();
                  expr.code += &format!("%int {}\n", temp_var);
                  expr.code += &format!("%call {}, {}({})\n", temp_var, name, params.join(","));
                  expr.name = temp_var;
              }
              Token::LeftBracket => {
                  *index += 1;
                  let index_expr = parse_expression(tokens, index)?;
                  if !matches!(next_result(tokens, index)?, Token::RightBracket) {
                      return Err(String::from("Expecting ']' after '['"));
                  }
                  let temp_var = create_temp();
                  expr.code += &format!("%int {}\n", temp_var);
                  expr.code += &format!("%mov {}, [{} + {}]\n", temp_var, name, index_expr.name);
                  expr.name = temp_var;
              }
              _ => {}
          }
          Ok(expr)
      }
      Token::LeftParen => {
          let expr = parse_expression(tokens, index)?;
          if !matches!(next_result(tokens, index)?, Token::RightParen) {
              Err(String::from("Expecting ')' after '('"))
          } else {
              Ok(expr)
          }
      }
      _ => Err(String::from("invalid expression")),
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