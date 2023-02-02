use egui::widgets;
use eframe::egui;
use egui::*;
use include_bytes_plus::include_bytes;

struct MyEguiApp {
    name: String,
    username: String,
    age: u32,
    is_work: bool,          
    is_alive: bool,
    text : String,
}
impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
     //   setup_custom_fonts(&cc.egui_ctx);
        Self::default()
    }
}
impl Default for MyEguiApp {
    fn default() -> Self {
        Self {      
            name: "Kevin Park".to_owned(),
            username: "".to_owned(),
            age: 30,
            is_work: true,
            is_alive: true,
            text: "Please input".to_owned()
        }
    }
}


impl eframe::App for MyEguiApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Hello World egui Application");
            ui.hyperlink("https://github.com/hyunbin7303");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let username_label = ui.label("User Name : ");
                ui.text_edit_singleline(&mut self.username)
                    .labelled_by(username_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.checkbox(&mut self.is_work, "Is Work");
            ui.checkbox(&mut self.is_alive, "Is Alive");
            ui.label(format!("Hello '{}', age {}, Working: {}, is Alive : {}", self.name, self.age, self.is_work, self.is_alive));

            if ui.button("Clear Input").clicked(){
                self.text.clear();
            }


            ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.label(&self.text);
            });


            

        });


    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    // fonts.font_data.insert(
    //     "my_font".to_owned(),
    //     egui::FontData::from_static(include_bytes_plus::include_bytes!(
    //         "../../../crates/epaint/fonts/Hack-Regular.ttf"
    //     )),
    // );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("My HelloWorld Application", options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}




