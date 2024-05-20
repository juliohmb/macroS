use eframe::NativeOptions;
use macro_s::ui::MacroRecorder;


fn main() {
    let native_options = NativeOptions::default();
    let _ = eframe::run_native(
        "Macro Recorder",
        native_options,
        Box::new(|cc| Box::new(MacroRecorder::new(cc))),
    );
}
