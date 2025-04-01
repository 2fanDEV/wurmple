use egui::panel::Side::Right;

use super::UI;

struct Settings {

}


impl UI for Settings {
    fn ui(&self, ctx: &egui::Context) {
        egui::SidePanel::new(Right, "Side Panel")
            .show(&ctx, |ui|
                {    ui.label("Hello World!");
                     let hello_button = ui.button("Hellooo");
                     if hello_button.clicked() {

                     }
                }
            );
    }
}
