use crate::{
    gui::tabs::TabType,
    parse::als::{AlsData, Project},
};
use eframe::egui::{
    self, include_image, widgets::Spinner, Align, Align2, FontId, IconData, SelectableLabel,
    TextStyle, Vec2,
};
use egui_dock::DockState;
use egui_extras::install_image_loaders;
use image;
use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub enum GuiState {
    Loading,
    Loaded,
    Error,
}

pub struct Gui {
    receiver: mpsc::Receiver<Result<Vec<Project>, String>>,
    state: GuiState,
    pub error_msg: String,
    pub projects: Option<Vec<Project>>,
    pub selected_project_als: Option<(usize, usize)>,
    pub dock_state: DockState<TabType>,

    preview_x_scale: f32,
    preview_x_pos: f32,

    frame_time: Duration,
}

impl Gui {
    fn new(receiver: mpsc::Receiver<Result<Vec<Project>, String>>) -> Self {
        Self {
            receiver,
            state: GuiState::Loading,
            error_msg: "THERE WAS AN ERROR BUT HOW???".to_string(),
            projects: None,
            selected_project_als: None,
            dock_state: Gui::default_tab_layout(),

            preview_x_scale: 1.,
            preview_x_pos: 0.,

            frame_time: Duration::new(0, 0),
        }
    }
    pub fn run(rx: mpsc::Receiver<Result<Vec<Project>, String>>) {
        eframe::run_native(
            "Palsa - Preview ableton live sets actually! Made with love by NotAFoe <3",
            eframe::NativeOptions {
                viewport: egui::ViewportBuilder {
                    icon: Some(Gui::load_icon("assets/logo.png").into()),
                    ..Default::default()
                },
                ..Default::default()
            },
            Box::new(|_cc| Box::new(Gui::new(rx))),
        )
        .expect("Failed to run Palsa, perhaps you do not have a graphical user interface?");
    }

    fn load_icon(path: &str) -> IconData {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open(path)
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };

        IconData {
            rgba: icon_rgba,
            width: icon_width,
            height: icon_height,
        }
    }

    /// An info bar which displays cool info such as the number of files loaded and the frametime
    fn info_bar(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("info_bar").show(ctx, |ui| {
            // Left to right side ui elements
            ui.with_layout(egui::Layout::left_to_right(Align::TOP), |ui| {
                if let Some(ref projects) = self.projects {
                    ui.label(
                        egui::RichText::new(format!(
                            "Loaded {} als files :)",
                            projects
                                .iter()
                                .flat_map(|project| project.als_data.iter())
                                .flatten()
                                .count()
                        ))
                        .size(15.),
                    );
                }

                // Displays frame time of the application
                ui.with_layout(egui::Layout::right_to_left(Align::TOP), |ui| {
                    ui.label(egui::RichText::new(format!("{:?}", self.frame_time)).size(15.0));
                });
            });
        });
    }

    /// Lists the als files and lets the user select one
    pub fn als_panel(
        &mut self,
        ui: &mut egui::Ui,
        projects: &Vec<Project>,
    ) -> Option<(usize, usize)> {
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for (project_index, project) in projects.iter().enumerate() {
                    ui.collapsing(&project.name, |ui| {
                        if let Some(ref als_data) = project.als_data {
                            for (als_index, als) in als_data.iter().enumerate() {
                                let is_selected =
                                    self.selected_project_als == Some((project_index, als_index));
                                let als_response =
                                    ui.add(SelectableLabel::new(is_selected, &als.name));

                                if als_response.clicked() {
                                    if is_selected {
                                        self.selected_project_als = None;
                                    } else {
                                        self.selected_project_als =
                                            Some((project_index, als_index));
                                    }
                                }
                            }
                        }
                    });
                }
                ui.add_space(50.0);
            });

        self.selected_project_als
    }

    pub fn visual_preview(&mut self, ui: &mut egui::Ui, selected_als_data: AlsData) {
        let colors = [
            "#E594A6", "#E6AA45", "#BB9D3E", "#F3F787", "#CEFC44", "#90FF4E", "#94FFAB", "#A3FEE7",
            "#A2BEFB", "#6B75DE", "#9C9EFA", "#C162DF", "#C6539E", "#FFFFFF", "#D94444", "#D6732F",
            "#8D7451", "#F7F656", "#B1FF74", "#76C531", "#6EBDAD", "#8CE5FC", "#669BE9", "#4E75BB",
            "#8460DE", "#A771C1", "#DC2CCF", "#CFCFCF", "#C56B60", "#E6A67A", "#C5AF76", "#F0FFB3",
            "#D4E69D", "#BDD27B", "#A6C58F", "#E0FDE2", "#D8EFF7", "#BBBDE0", "#C8B7E1", "#A991E0",
            "#E2DBE0", "#A8A8A8", "#B6928C", "#A6845C", "#91846C", "#BABC70", "#A9C135", "#8BB257",
            "#9BC0B8", "#A2B0C1", "#90A1BF", "#8A8DC7", "#A092B2", "#B69CBB", "#A97094", "#7A7A7A",
            "#953B3B", "#93563B", "#685145", "#D0C73A", "#879835", "#6EA13F", "#5B9A8C", "#446080",
            "#302190", "#454B9C", "#6142A7", "#9044A8", "#AD346E", "#3F3F3F",
        ]; // Fucking ableton devs hardcoded this shit (I HAD TO DO THIS MANUALLY UGASHDFH)

        ui.with_layout(egui::Layout::left_to_right(Align::TOP), |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add(
                        egui::DragValue::new(&mut self.preview_x_pos)
                            .clamp_range(f32::MIN..=0.0)
                            .speed(self.preview_x_scale / 5.),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.preview_x_scale)
                            .clamp_range(0.1..=100.)
                            .speed(0.1),
                    );

                    let max_rect = ui.available_rect_before_wrap();
                    let painter = ui.painter_at(max_rect);

                    for (i, track) in selected_als_data.tracks.iter().enumerate() {
                        for clip in &track.clips {
                            let clip_rect = egui::Rect::from_x_y_ranges(
                                egui::Rangef::new(
                                    max_rect.min.x
                                        + self.preview_x_pos
                                        + clip.start * self.preview_x_scale,
                                    max_rect.min.x
                                        + self.preview_x_pos
                                        + clip.end * self.preview_x_scale,
                                ),
                                egui::Rangef::new(
                                    max_rect.min.y + (i as f32 * 42.),
                                    max_rect.min.y + (i as f32 * 42.) + 40.,
                                ),
                            );
                            let color = if let Some(track_color) = track.color {
                                egui::Color32::from_hex(colors[track_color]).unwrap()
                            } else {
                                egui::Color32::from_rgb(255, 0, 255)
                            };
                            painter.rect_filled(clip_rect, 0.1, color);
                        }
                        painter.text(
                            egui::Pos2 {
                                x: max_rect.min.x,
                                y: max_rect.min.y + (i as f32 * 42.) + 20.,
                            },
                            Align2::LEFT_CENTER,
                            track.name.clone(),
                            FontId::monospace(15.),
                            egui::Color32::from_rgb(0, 0, 255),
                        );
                    }
                });
        });
    }

    /// Displays a spinner when loading
    fn handle_loading(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(ui.available_size().y / 3.);
                ui.label(egui::RichText::new("Loading...").size(15.));

                ui.add(
                    egui::Image::new(include_image!("../../assets/logo.png"))
                        .max_size(Vec2 { x: 150., y: 150. }),
                );

                ui.add_space(10.);
                ui.vertical_centered(|ui| {
                    ui.add(Spinner::new().size(20.));
                });
            });
        });
    }

    /// Displays the main interface with `tabs`
    fn handle_loaded(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            self.tabs(ctx, frame);
        });
    }

    /// Changes styling to red and displays error in `self.error_msg`
    fn handle_error(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut style: egui::Style = (*ctx.style()).clone();
            style.override_text_style = Some(TextStyle::Heading);
            style.visuals.override_text_color = Some(egui::Color32::RED);
            ctx.set_style(style);

            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    ui.label(egui::RichText::new(format!("{}", self.error_msg)).size(50.));
                },
            );
        });
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        install_image_loaders(&ctx);
        let frame_start = Instant::now();

        if let Ok(received) = self.receiver.try_recv() {
            match received {
                Ok(als_data) => {
                    self.projects = Some(als_data);
                    self.state = GuiState::Loaded;
                }
                Err(error) => {
                    self.error_msg = error;
                    self.state = GuiState::Error;
                }
            }
        }

        match self.state {
            GuiState::Loading => self.handle_loading(ctx),
            GuiState::Loaded => self.handle_loaded(ctx, frame),
            GuiState::Error => self.handle_error(ctx),
        }
        self.info_bar(ctx);

        self.frame_time = frame_start.elapsed();
        ctx.request_repaint();
    }
}
