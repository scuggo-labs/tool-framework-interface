pub use stf_proc::main;

pub use eframe;
pub trait STF {
    fn update(&mut self, ui: &mut crate::eframe::egui::Ui);
    fn start(&mut self);
    fn stop(&mut self);
    fn new() -> Self where Self: Sized;
}

