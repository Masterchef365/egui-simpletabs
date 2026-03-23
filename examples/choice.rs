#[derive(Clone, PartialEq, Eq)]
enum Choices {
    A, B, C
}

fn main() {
    let mut value = Choices::A;

    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("Dial test", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui_simpletabs::dial::choice(ui, &mut value, &[
                (Choices::A, "A"),
                (Choices::B, "B"),
                (Choices::C, "C"),
            ]);
        });
    })
    .unwrap();
}

