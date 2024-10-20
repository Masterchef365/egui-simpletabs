use egui_simpletabs::TabWidgetExt;

#[derive(Default, Clone, Copy, PartialEq)]
enum Tabs {
    #[default]
    Home,
    TabOne,
    TabTwo,
}

fn main() {
    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;
    let mut tab = Tabs::default();

    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("Tab test", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_tab(&mut tab, Tabs::Home, "Home");
                ui.add_tab(&mut tab, Tabs::TabOne, "TabOne");
                ui.add_tab(&mut tab, Tabs::TabTwo, "TabTwo");
            });

            match tab {
                Tabs::Home => {
                    ui.heading("My egui Application");
                    ui.horizontal(|ui| {
                        let name_label = ui.label("Your name: ");
                        ui.text_edit_singleline(&mut name)
                            .labelled_by(name_label.id);
                    });
                }
                Tabs::TabOne => {
                    ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
                }
                Tabs::TabTwo => {
                    if ui.button("Increment").clicked() {
                        age += 1;
                    }
                    ui.label(format!("Hello '{name}', age {age}"));
                }
            }
        });
    })
    .unwrap();
}
