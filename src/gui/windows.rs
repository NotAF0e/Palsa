use eframe::egui;

use super::gui::Gui;

impl Gui {
    pub fn control_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Controls").auto_sized().show(ctx, |ui| {
            let labels = ["Position:", "Scale/zoom:", "Track height:"];
            let speeds = [self.preview_x_scale / 5., 0.01, 0.01];
            let clamp_ranges = [(f32::MIN, 0.0), (0.1, 100.), (0.1, 100.)];

            let mut values = [
                &mut self.preview_x_pos,
                &mut self.preview_x_scale,
                &mut self.preview_y_scale,
            ];

            for i in 0..labels.len() {
                ui.horizontal(|ui| {
                    ui.label(labels[i]);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                        ui.add(
                            egui::DragValue::new(values[i])
                                .clamp_range(clamp_ranges[i].0..=clamp_ranges[i].1)
                                .speed(speeds[i]),
                        );
                    });
                });
                ui.end_row();
            }
        });
    }
}
