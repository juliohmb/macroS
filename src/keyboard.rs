use winreg::enums::*;
use winreg::RegKey;
use std::time::Duration;

pub fn get_keyboard_settings() -> (Duration, Duration) {
    // Abrir a chave do registro para consultar as configurações do teclado
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let keyboard = hkcu.open_subkey("Control Panel\\Keyboard").expect("Cannot open registry key");

    // Ler os valores de KeyboardDelay e KeyboardSpeed
    let keyboard_speed: String = keyboard.get_value("KeyboardSpeed").expect("Cannot read KeyboardSpeed");
    let keyboard_speed: u32 = keyboard_speed.parse().expect("Cannot parse KeyboardSpeed");
    let keyboard_delay: u32 = keyboard.get_value("KeyboardDelay").expect("Cannot read KeyboardDelay");

    print!("KeyboardSpeed: {}, KeyboardDelay: {}\n", keyboard_speed, keyboard_delay);
    // Converter os valores para milissegundos
    // KeyboardDelay: 0 (250 ms), 1 (500 ms), 2 (750 ms), 3 (1000 ms)
    let initial_delay_ms = match keyboard_delay {
        0 => 250,
        1 => 500,
        2 => 750,
        3 => 1000,
        _ => 500, // Valor padrão
    };

    // KeyboardSpeed: 0 (aprox. 31 ms), 31 (aprox. 2 ms)
    // Convertendo para intervalo entre repetições em milissegundos
    let repeat_rate_ms = 1000 / (keyboard_speed + 1);

    (
        Duration::from_millis(initial_delay_ms),
        Duration::from_millis(repeat_rate_ms.into())
    )
}