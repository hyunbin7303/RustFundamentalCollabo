use eframe::{epi, egui};
use egui::widgets;

#[derive(Debug)]
struct MyEguiApp {
    name: String,
    age: u32,
    is_work: bool,
    is_alive: bool,
    
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            name: "Kevin Park".to_owned(),
            age: 30,
            is_work: true,
            is_alive: true,
        }
    }
}

impl epi::App for MyEguiApp {
   fn name(&self) -> &str {
       "Kevin EGUI app testing"
   }

   fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {

        let mut my_f32 :f32 =  7.0;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                if ui.ui_contains_pointer() {
                    egui::show_tooltip_text(ui.ctx(), egui::Id::new("my_tooltip"), "Please press the button");
                }
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            ui.end_row();

            ui.checkbox(&mut self.is_work, "Checked");
            ui.add(egui::Checkbox::new(&mut self.is_alive, "Checked"));


            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            ui.end_row();

            


       });
   }
}

fn Button(ui: &mut egui::Ui,text: &str)->bool{
    
}

fn main() {
    let app = MyEguiApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}


