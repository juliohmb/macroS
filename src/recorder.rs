use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use super::actions::Action;

pub struct Recorder {
    actions: Arc<Mutex<Vec<Action>>>,
    recording: Arc<Mutex<bool>>,
}

impl Recorder {
    pub fn new() -> Self {
        Self {
            actions: Arc::new(Mutex::new(Vec::new())),
            recording: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&mut self) {
        let actions = Arc::clone(&self.actions);
        let recording = Arc::clone(&self.recording);

        *recording.lock().unwrap() = true;

        thread::spawn(move || {
            let device_state = DeviceState::new();
            let start_time = Instant::now();

            while *recording.lock().unwrap() {
                // Captura de eventos de teclado
                let keys: Vec<Keycode> = device_state.get_keys();
                for key in keys.iter() {
                    actions.lock().unwrap().push(Action {
                        event: format!("Key pressed: {:?}", key),
                        timestamp: start_time.elapsed().as_millis(),
                    });
                }

                // Captura de eventos de mouse
                let mouse: MouseState = device_state.get_mouse();
                actions.lock().unwrap().push(Action {
                    event: format!("Mouse moved to: {:?}", mouse.coords),
                    timestamp: start_time.elapsed().as_millis(),
                });

                // Adicione um delay para evitar sobrecarregar a CPU
                thread::sleep(std::time::Duration::from_millis(10));
            }
        });
    }

    pub fn stop(&mut self) {
        let recording = Arc::clone(&self.recording);
        *recording.lock().unwrap() = false;
    }

    pub fn get_actions(&self) -> Vec<Action> {
        self.actions.lock().unwrap().clone()
    }
}
