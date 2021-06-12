use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut input = String::new();
    let mut env = is_basic::Env::default();
    loop {

        write!(stdout,"is_basic> ");
        stdout.flush()?;


        stdin.read_line(&mut input)?;
        
        match run(input.trim(), &mut env) {
            Ok(Some(val)) => writeln!(stdout, "{}", val)?,
            Ok(None) => {}
            Err(msg) => writeln!(stderr, "{}", msg)?,
        }
        
        input.clear();
    }
}

fn run(input: &str, env: &mut is_basic::Env) -> Result<Option<is_basic::Val>, String> {
    let parse = is_basic::parse(input).map_err(|msg| format!("Parsing error: {}", msg))?;

    let evaled = parse
        .eval(env)
        .map_err(|msg| format!("Evaluation error: {}", msg))?;
    

    if evaled == is_basic::Val::Unit {
        Ok(None)
    } else {
        Ok(Some(evaled))
    }
    
}