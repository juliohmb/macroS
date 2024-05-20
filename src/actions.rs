#[derive(Clone)]
pub struct Action {
    pub event: String,
    pub timestamp: u128,
}

pub enum Event {
    KeyPressed(String),
    MouseMoved((i32, i32)),
    MouseClicked((i32, i32)),
}
