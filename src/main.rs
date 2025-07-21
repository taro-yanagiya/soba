use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use soba::eval_string;
fn main() -> rustyline::Result<()> {
    println!("This is the Soba programming language!");
    
    let mut rl = DefaultEditor::new()?;
    
    // Set maximum history size to 1000 entries
    rl.set_max_history_size(1000)?;
    
    // Load history from file
    let history_file = ".soba_history";
    if rl.load_history(history_file).is_err() {
        // History file doesn't exist, that's fine
    }
    
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                // Add to history
                let _ = rl.add_history_entry(&line);
                
                if line.trim() == "exit" {
                    break;
                }
                
                if line.trim().is_empty() {
                    continue;
                }
                
                match eval_string(&line) {
                    Ok(result) => {
                        println!("{result}");
                    }
                    Err(err) => {
                        println!("{err}");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }
    
    // Save history to file
    let _ = rl.save_history(history_file);
    
    Ok(())
}
