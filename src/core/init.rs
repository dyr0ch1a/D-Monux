use std::fs;
use std::path::Path;

pub fn init_workspace() -> Result<(), String> {
    let monux_dir = Path::new(".monux");


    if monux_dir.exists(){
        return Err("Monux already initialized".into());
    }

    fs::create_dir(monux_dir)
        .map_err(|e| e.to_string())?;
    fs::write(monux_dir.join("config.toml"), b"# Monux configuration\n")
        .map_err(|e| e.to_string())?;

    Ok(())
}
