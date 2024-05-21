#[derive(Clone)]
pub struct Action {
    pub event: Event,
    pub timestamp: u128,
}

#[derive(Clone)]
pub enum Event {
    KeyPressed(String),
    MouseMoved((i32, i32)),
    MouseClicked((i32, i32)),
}
