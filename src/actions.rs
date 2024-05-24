#[derive(Clone, Debug)]
pub struct Action {
    pub event: Event,
    pub timestamp: u128,
}

#[derive(Clone, Debug)]
pub enum Event {
    KeyPressed{
        key: String,
        ctrl: bool,
        alt: bool,
        shift: bool,
    },
    MouseMoved(()),
    MouseClicked((i32, i32)),
}


impl Action {
    pub fn as_string(&self) -> String {
        match &self.event {
            Event::KeyPressed { key, ctrl, alt, shift } => {
                let mut result = String::new();
                if *ctrl { result.push_str("CTRL+"); }
                if *alt { result.push_str("ALT+"); }
                if *shift { result.push_str("SHIFT+"); }
                result.push_str(key);
                result
            }
            Event::MouseMoved(_) => String::from("Mouse moved"),
            Event::MouseClicked(coords) => {
                let (x, y) = coords;
                format!("Mouse clicked: {},{}", x, y)
            }
        }
    }
}