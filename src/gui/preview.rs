use super::gui::Gui;
use crate::parse::{als::AlsData, track::Track};

use eframe::egui;

impl Gui {
    pub fn visual_preview(&mut self, ui: &mut egui::Ui, selected_als_data: AlsData) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            let max_rect = ui.available_rect_before_wrap();
            let painter = ui.painter_at(max_rect);

            for (i, track) in selected_als_data.tracks.iter().enumerate() {
                self.draw_clips(i, track, &painter, max_rect);
            }
        });
    }

    fn draw_clips(
        &mut self,
        i: usize,
        track: &Track,
        painter: &egui::Painter,
        max_rect: egui::Rect,
    ) {
        for clip in &track.clips {
            let clip_rect = egui::Rect::from_x_y_ranges(
                egui::Rangef::new(
                    max_rect.min.x + self.preview_x_pos + clip.start * self.preview_x_scale,
                    max_rect.min.x + self.preview_x_pos + clip.end * self.preview_x_scale,
                ),
                egui::Rangef::new(
                    max_rect.min.y + (i as f32 * self.preview_y_scale),
                    max_rect.min.y
                        + (i as f32 * self.preview_y_scale)
                        + self.preview_y_scale * 0.98,
                ),
            );
            let color = if let Some(track_color) = track.color {
                egui::Color32::from_hex(&self.colors[track_color]).unwrap()
            } else {
                egui::Color32::from_rgb(255, 0, 255)
            };
            painter.rect_filled(clip_rect, 0.1, color);
        }
        painter.text(
            egui::Pos2 {
                x: max_rect.min.x,
                y: max_rect.min.y
                    + (i as f32 * self.preview_y_scale)
                    + self.preview_y_scale * 0.98 / 2.,
            },
            egui::Align2::LEFT_CENTER,
            track.name.clone(),
            egui::FontId::monospace(10.),
            egui::Color32::from_rgb(0, 0, 255),
        );
    }
}
