use std::fs::File;
use std::io::{self, Write};
use super::actions::{Action, Event};

pub fn generate_python_script(actions: Vec<Action>) -> io::Result<()> {
    let mut file = File::create("macro_script.py")?;
    writeln!(file, "import time")?;
    writeln!(file, "from macro_utils import press_key, move_mouse, click_mouse")?;
    writeln!(file, "from macro_utils import main")?;
    writeln!(file, "\n# Script gerado automaticamente\n")?;

    writeln!(file, "actions = [")?;
    for action in actions {
        match action.event {
            Event::KeyPressed { key, ctrl, alt, shift } => {
                writeln!(file, "    {{'event': 'Key pressed: {}', 'timestamp': {}}},", key, action.timestamp)?;
            }
            Event::MouseClicked(coords) => {
                let (x, y) = coords;
                writeln!(file, "    {{'event': 'Mouse clicked: {},{}', 'timestamp': {}}},", x, y, action.timestamp)?;
            }
            _ => {}
        }
    }
    writeln!(file, "]")?;
    writeln!(file, "\nmain(actions)")?;

    Ok(())
}
