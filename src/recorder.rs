use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use super::actions::{Action, Event};
use super::keyboard::{get_keyboard_settings, KeyStamp};

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
        // create stopper channel
        let (tx_stopper, rx_stopper) = mpsc::channel();
        self.stopper = Some(tx_stopper);

        // create receiver channel
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);

        // inicio nova thread para captura de eventos
        thread::spawn(move || {
            let device_state = DeviceState::new();
            let start_time = Instant::now();
            let mut actions = Vec::new();

            // variavel declarada aqui para guardar o estado anterior das teclas
            let mut old_keys: Vec<Keycode> = Vec::new();
            let mut key_timestamps = HashMap::new();

            // Obter as configurações do teclado
            let (initial_delay, repeat_rate) = get_keyboard_settings();

            println!("Initial delay: {:?}, Repeat rate: {:?}", initial_delay, repeat_rate);

            while rx_stopper.try_recv().is_err() {
                // Captura de eventos de teclado
                let keys: Vec<Keycode> = device_state.get_keys();
                for key in keys.iter() {
                    if !old_keys.contains(key) {
                        add_event(key, &keys, start_time, &mut actions, &mut key_timestamps, false)
                    }
                    else if let Some(key_stamp) = key_timestamps.get(key) {
                        if key_stamp.hold == true && key_stamp.stamp.elapsed() > repeat_rate {
                            add_event(key, &keys, start_time, &mut actions, &mut key_timestamps, true)
                        }
                        else if key_stamp.stamp.elapsed() > initial_delay{
                            add_event(key, &keys, start_time, &mut actions, &mut key_timestamps, true)
                        }
                    }
                }

                // Atualizar o estado das teclas
                old_keys = keys.clone();

                // Captura de eventos de mouse
                // let mouse: MouseState = device_state.get_mouse();
                // actions.push(Action {
                //     event: Event::MouseMoved(()),
                //     timestamp: start_time.elapsed().as_millis(),
                // });
                // if mouse.button_pressed[1] == true {
                //     actions.push(Action {
                //         event: Event::MouseClicked(mouse.coords),
                //         timestamp: start_time.elapsed().as_millis(),
                //     });
                // }

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
        match &self.rx {
            None => {}
            Some(rx) => {
                self.actions = rx.recv().unwrap();
                for action in self.actions.iter() {
                    print!("{} ", action.as_string());
                }
            }
        }
        
        self.rx = None;
        self.stopper = None;
    }

    pub fn get_actions(&self) -> Vec<Action> {
        self.actions.clone()
    }
}



fn add_event(key: &Keycode, keys: &Vec<Keycode>, start_time: Instant, actions: &mut Vec<Action>, key_timestamps: &mut HashMap<Keycode, KeyStamp>, hold: bool){
    let ctrl = keys.contains(&Keycode::LControl) || keys.contains(&Keycode::RControl);
    let alt = keys.contains(&Keycode::LAlt) || keys.contains(&Keycode::RAlt);
    let shift = keys.contains(&Keycode::LShift) || keys.contains(&Keycode::RShift);
    if key != &Keycode::LControl && key != &Keycode::RControl && key != &Keycode::LAlt && key != &Keycode::RAlt && key != &Keycode::LShift && key != &Keycode::RShift {
        actions.push(Action {
            event: Event::KeyPressed{
                key: key.to_string(),
                ctrl,
                alt,
                shift,
            },
            timestamp: start_time.elapsed().as_millis(),
        });
        key_timestamps.insert(key.clone(), KeyStamp {stamp: Instant::now(), hold});
    }
}