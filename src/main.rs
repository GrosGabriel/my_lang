use std::io::Write;
use std::io;
//use my_lang::lexer::Lexer;
// use my_lang::parser::Parser;
use my_lang::term::Term;
// use my_lang::token::Token;
use my_lang::typechecker::Type;
// use my_lang::repl::run_with_ctx;
use my_lang::repl::process_line;
use my_lang::repl::import_file;



// fn run(input: &str) {
//     //print!("{} => ", input);
    
//     let tokens = match Lexer::new(input).tokenize() {
//         Ok(t) => t,
//         Err(e) => { println!("Lex error: {}", e); return; }
//     };
//     // println!("\nTokens: {:?}", tokens);  // <-- add this for debugging
    
//     let term = match Parser::new(tokens).parse() {
//         Ok(t) => t,
//         Err(e) => { println!("Parse error: {}", e); return; }
//     };
//     // println!("Term: {:?}", term);  // <-- and this
    
//     let mut ctx = Vec::new();
//     match term.infer_type_ctx(&mut ctx) {
//         Some(ty) => print!("{:?} ", ty),
//         None => { println!("Type error"); return; }
//     };
    
//     let result = term.multistep_cbv();
//     println!("{:?}", result);
// }

// fn main2() {
    // Integers
    // run("1 + 2");
    // run("10 * 5 - 3");
    // run("10 * (5 - 3)");

    // // Booleans
    // run("if true then 42 else 0");
    //run("if 1 == 1 then not (true or true) else not false");

    // // Let
    // run("let x : Int= 5 in x + 1");
    // run("let x : Int= 3 in let y : Int= 4 in x + y");

    // // Fonctions
    // run("(fun x : Int -> x + 1) 41");
    // run("let f : (Int -> Int) = fun x : Int -> x * 2 in f 21");

    // Fix — récursion
    //run("let fact : (Int -> Int) = fix (fun self : (Int -> Int) -> fun n : Int -> if n == 0 then 1 else n * (self (n - 1))) in fact 5");

    // Paires
    // run("fst (1, 2)");
    // run("snd (true, 42)");

    // // Erreurs de type — doivent afficher "Type error"
    // run("1 + true");
    // run("if 42 then 1 else 0");
    // run("(fun x : Int -> x) true");

    // Construction de listes
    // run("nil [Bool]");
    // run("cons true (nil [Bool])");
    // run("cons true (cons false (nil [Bool]))");

    // // CaseList
    // run("caselist (nil [Bool]) false (fun h : Bool -> fun t : [Bool] -> h)");
    // run("caselist (cons true (nil [Bool])) false (fun h : Bool -> fun t : [Bool] -> h)");

    // // RecList - all_true
    // run("reclist (cons true (cons true (nil [Bool]))) true (fun h : Bool -> fun t : [Bool] -> fun ih : Bool -> if h then ih else false)");
    // run("reclist (cons true (cons false (nil [Bool]))) true (fun h : Bool -> fun t : [Bool] -> fun ih : Bool -> if h then ih else false)");

    // // Let avec listes
    // run("let x : [Bool] = nil [Bool] in x");
    // run("let x : [Bool] = cons true (nil [Bool]) in caselist x false (fun h : Bool -> fun t : [Bool] -> h)");

    // // Type errors
    // run("cons true (cons 42 (nil [Bool]))");
    // run("caselist (cons true (nil [Bool])) true (fun h : Bool -> fun t : [Bool] -> nil [Bool])");

    //Tests
    //run("let x : Int = 5 in x");


    // Sums
    // Construction
    // run("inl true (Int)");           // Inl(true) : Bool + Int
    // run("inr 42 (Bool)");            // Inr(42) : Bool + Int

    // // CaseSum - branche inl
    // run("casesum (inl true (Int)) (fun x : Bool -> x) (fun y : Int -> false)");
    // // doit donner true

    // // CaseSum - branche inr
    // run("casesum (inr 42 (Bool)) (fun x : Bool -> x) (fun y : Int -> false)");
    // // doit donner false

    // // Let avec sum
    // run("let x : (Bool + Int) = inl true (Int) in casesum x (fun a : Bool -> a) (fun b : Int -> false)");
    // // doit donner true

    // // Type errors
    // run("casesum (inl true (Int)) (fun x : Bool -> x) (fun y : Int -> 42)");
    // // branches retournent Bool vs Int -> Type error

    // run("casesum (inl true (Int)) (fun x : Int -> x) (fun y : Int -> 42)");
    // // domaine de inl_case est Int mais scrutinee est Bool + Int -> Type error



// }


pub fn main() {
    println!("My Lang Interpreter");
    println!("===================");
    println!();

    let mut ctx: Vec<(String, Type, Term)> = Vec::new();

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        import_file(&args[1], &mut ctx);
    }
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" { break; }
        if input.starts_with("import") {
            let filename = input
                .trim_start_matches("import")
                .trim()
                .trim_matches('"');
            
            import_file(filename, &mut ctx);
        } else {
            process_line(input, &mut ctx);
        }

    }
}