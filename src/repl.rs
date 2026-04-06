use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::term::Term;
use crate::token::Token;
use crate::typechecker::Type;


pub fn run_with_ctx(input: &str, ctx: &Vec<(String, Type, Term)>) {
    let tokens = match Lexer::new(input).tokenize() {
        Ok(t) => t,
        Err(e) => { println!("Lex error: {}", e); return; }
    };
    
    let term = match Parser::new(tokens).parse() {
        Ok(t) => t,
        Err(e) => { println!("Parse error: {}", e); return; }
    };

    // Substitute all declarations from the context into the term
    let term = ctx.iter().fold(term, |t, (name, _, val)| {
        t.subst(name, val)
    });

    // typecheck and evaluation
    let mut type_ctx = Vec::new();
    match term.infer_type_ctx(&mut type_ctx) {
        Some(ty) => print!("{:?} ", ty),
        None => { println!("Type error"); return; }
    };

    let result = term.multistep_cbv();
    println!("{:?}", result);
}



pub fn process_line(input: &str, ctx: &mut Vec<(String, Type, Term)>) {
    let tokens = match Lexer::new(input).tokenize() {
        Ok(t) => t,
        Err(e) => { println!("Lex error: {}", e); return; }
    };

    if tokens.is_empty() {
        return;
    }
  
    let starts_with_let = matches!(tokens.first(), Some(Token::Let));

    let mut depth = 0;
    let mut has_top_level_in = false;
    for tok in &tokens {
        match tok {
            Token::Let => depth += 1,
            Token::In => {
                depth -= 1;
                if depth == 0 {
                    has_top_level_in = true;
                    break;
                }
            }
            _ => {}
        }
    }

    if starts_with_let && !has_top_level_in { // This is thus a top-level declaration
        let mut parser = Parser::new(tokens);
        match parser.parse_decl() {
            Ok((name, ty, val)) => {
                // Substitute the context into the value before typechecking
                let val = ctx.iter().fold(val, |t, (n, _, v)| t.subst(n, v));
                // typecheck
                match val.infer_type() {
                    Some(ty_inferred) if ty_inferred == ty => {
                        // Reduce the value to normal form before storing in context
                        let val = val.multistep_cbv();
                        println!("Declared {:?} {} = {:?}", ty, name, val);
                        ctx.push((name, ty, val));
                    }
                    _ => println!("Type error in declaration of {}", name),
                }
            }
            Err(e) => println!("Parse error: {}", e),
        }
    } else {
        // Substitute the context into the input and process
        run_with_ctx(input, &ctx);
    }
}


pub fn import_file(filename: &str, ctx: &mut Vec<(String, Type, Term)>) {
    match std::fs::read_to_string(filename) {
        Ok(content) => {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with("//") { continue; }
                
                if line.starts_with("import") {
                    let nested = line
                        .trim_start_matches("import")
                        .trim()
                        .trim_matches('"');
                    import_file(nested, ctx);  // recursive
                } else {
                    process_line(line, ctx);
                }
            }
            println!("Imported: {}", filename);
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}