use egui::Context;

mod settings;

trait UI {
   fn ui(&self, ctx: &Context); 
}
