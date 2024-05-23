use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use std::sync::mpsc;
use std::thread;
use std::time::Instant;
use super::actions::{Action, Event};
use super::keyboard::get_keyboard_settings;

pub struct Recorder {
    actions: Vec<Action>,
    stopper: Option<mpsc::Sender<bool>>,
    rx: Option<mpsc::Receiver<Vec<Action>>>
}

impl Recorder {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            stopper: None,
            rx: None,
        }
    }

    pub fn start(&mut self) {
        let (tx_stopper, rx_stopper) = mpsc::channel();
        self.stopper = Some(tx_stopper);

        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);

        // inicio nova thread para captura de eventos
        thread::spawn(move || {
            let device_state = DeviceState::new();
            let start_time = Instant::now();
            let mut actions = Vec::new();

            // variavel declarada aqui para guardar o estado anterior das teclas
            let mut old_keys: Vec<Keycode> = Vec::new();
            let (initial_delay, repeat_rate) = get_keyboard_settings();

            while rx_stopper.try_recv().is_err() {
                // Captura de eventos de teclado
                let keys: Vec<Keycode> = device_state.get_keys();
                for key in keys.iter() {
                    actions.push(Action {
                        event: Event::KeyPressed(key.to_string()),
                        timestamp: start_time.elapsed().as_millis(),
                    });
                }

                // Captura de eventos de mouse
                let mouse: MouseState = device_state.get_mouse();
                actions.push(Action {
                    event: Event::MouseMoved(mouse.coords),
                    timestamp: start_time.elapsed().as_millis(),
                });
                if mouse.button_pressed[1] == true {
                    actions.push(Action {
                        event: Event::MouseClicked(mouse.coords),
                        timestamp: start_time.elapsed().as_millis(),
                    });
                }
                // println!("Mouse vec: {:?}", mouse.button_pressed);
                // Adicione um delay para evitar sobrecarregar a CPU
                thread::sleep(std::time::Duration::from_millis(10));
            }
            tx.send(actions).unwrap();
        });
    }

    pub fn stop(&mut self) {
        if let Some(tx) = &self.stopper {
            let _ = tx.send(true);
        }
        self.actions = self.rx.take().unwrap().recv().unwrap();
    }

    pub fn get_actions(&self) -> Vec<Action> {
        self.actions.clone()
    }
}
