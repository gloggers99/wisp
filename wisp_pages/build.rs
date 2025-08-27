use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    // We have to use tailwind v3 because for some reason the configuration file isn't a thing in
    // v4? We need the configuration file to tell tailwind to check the rust source files for css
    // classes.
    let output = Command::new("./node_modules/.bin/tailwindcss.cmd")
        .args("--config ./tailwind.config.cjs -i ./tailwind/src/main.css -o ./tailwind/target/main.css".split(' '))
        .output()?;
    
    if !output.status.success() {
        println!("cargo:warning=Failed to compile tailwind!")
    }
    
    Ok(())
}