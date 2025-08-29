use std::error::Error;
use std::path::Path;
use std::process::Command;

// We run the tailwindcss executable and compile the main.css. Afterward, we use `include_str!` to
// statically include the css file into our library.

/// Pre-build operation for `wisp_pages`
fn main() -> Result<(), Box<dyn Error>> {
    // Confirm we have `tailwindcss` installed.
    if !Path::new("./node_modules").exists() {
        let output = Command::new("npm.cmd")
            .arg("install")
            .output()?;
        
        if !output.status.success() {
            Err("Failed to install tailwindcss.")?
        }
    }
    
    // We have to use tailwind v3 because for some reason the configuration file isn't a thing in
    // v4? We need the configuration file to tell tailwind to check the rust source files for css
    // classes.
    let output = Command::new("./node_modules/.bin/tailwindcss.cmd")
        .args("--config ./tailwind.config.cjs -i ./tailwind/src/main.css -o ./tailwind/target/main.css".split(' '))
        .output()?;
    
    if !output.status.success() {
        println!("cargo:warning=Failed to run tailwindcss!")
    }
    
    Ok(())
}