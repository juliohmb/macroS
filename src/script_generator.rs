use std::fs::File;
use std::io::{self, Write};
use super::actions::Action;

pub fn generate_python_script(actions: Vec<Action>) -> io::Result<()> {
    let mut file = File::create("macro_script.py")?;
    writeln!(file, "import time")?;
    writeln!(file, "from macro_utils import press_key, move_mouse, click_mouse")?;
    writeln!(file, "from macro_utils import main")?;
    writeln!(file, "\n# Script gerado automaticamente\n")?;

    writeln!(file, "actions = [")?;
    for action in actions {
        match action.event.as_str() {
            e if e.starts_with("Key pressed:") => {
                let key = e.trim_start_matches("Key pressed: ");
                writeln!(file, "    {{'event': 'Key pressed: {}', 'timestamp': {}}},", key, action.timestamp)?;
            }
            e if e.starts_with("Mouse clicked:") => {
                let button = e.trim_start_matches("Mouse clicked: ");
                writeln!(file, "    {{'event': 'Mouse clicked: {}', 'timestamp': {}}},", button, action.timestamp)?;
            }
            _ => {}
        }
    }
    writeln!(file, "]")?;
    writeln!(file, "\nmain(actions)")?;

    Ok(())
}
