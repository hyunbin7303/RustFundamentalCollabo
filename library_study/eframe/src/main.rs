use egui::widgets;
use eframe::egui;

// #[derive(Debug)]
#[derive(Default)]
struct MyEguiApp {
    // name: String,
    // age: u32,
    // is_work: bool,
    // is_alive: bool,
    
}
impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}
// impl Default for MyEguiApp {
//     fn default() -> Self {
//         Self {
//             name: "Kevin Park".to_owned(),
//             age: 30,
//             is_work: true,
//             is_alive: true,
//         }
//     }
// }

impl eframe::App for MyEguiApp {
//     fn new(cc: &eframe::CreationContext<'_>) -> Self {
//         Self::default()
//     }
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }


//    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

//         let mut my_f32 :f32 =  7.0;
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("My egui Application");
//             ui.horizontal(|ui| {
//                 ui.label("Your name: ");
//                 ui.text_edit_singleline(&mut self.name);
//             });
            
//             ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
//             if ui.button("Click each year").clicked() {
//                 if ui.ui_contains_pointer() {
//                     egui::show_tooltip_text(ui.ctx(), egui::Id::new("my_tooltip"), "Please press the button");
//                 }
//                 self.age += 1;
//             }
//             ui.label(format!("Hello '{}', age {}", self.name, self.age));
//             ui.end_row();

//             ui.checkbox(&mut self.is_work, "Checked");
//             ui.add(egui::Checkbox::new(&mut self.is_alive, "Checked"));

//             ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
//             ui.end_row();
//        });
//    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    // eframe::run_native(Box::new(app), native_options);
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}


