use crate::gui::tabs::TabType;
use crate::AlsData;
use eframe::egui::{
    self, include_image, widgets::Spinner, Align, IconData, SelectableLabel, TextStyle, Vec2,
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
    receiver: mpsc::Receiver<Result<Vec<AlsData>, String>>,
    state: GuiState,
    pub error_msg: String,
    pub all_als: Option<Vec<AlsData>>,
    pub selected_als: Option<usize>,
    pub dock_state: DockState<TabType>,

    frame_time: Duration,
}

impl Gui {
    fn new(receiver: mpsc::Receiver<Result<Vec<AlsData>, String>>) -> Self {
        Self {
            receiver,
            state: GuiState::Loading,
            error_msg: "OH FUCK I ERRORED OUT!!!".to_string(),
            all_als: None,
            selected_als: None,
            dock_state: Gui::default_tab_layout(),

            frame_time: Duration::new(0, 0),
        }
    }
    pub fn run(rx: mpsc::Receiver<Result<Vec<AlsData>, String>>) {
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
        egui::TopBottomPanel::bottom("info bar").show(ctx, |ui| {
            // Left to right side ui elements
            ui.with_layout(egui::Layout::left_to_right(Align::TOP), |ui| {
                if let Some(ref all_als) = self.all_als {
                    ui.label(
                        egui::RichText::new(format!("Loaded {} als files :)", all_als.len()))
                            .size(15.0),
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
    pub fn als_panel(&self, ui: &mut egui::Ui, all_als: &Vec<AlsData>) -> Option<usize> {
        let mut selected_index = self.selected_als;

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for (i, als) in all_als.iter().enumerate() {
                    let is_selected = self.selected_als == Some(i);
                    let response = ui.add(SelectableLabel::new(is_selected, &als.name));

                    if response.clicked() {
                        if is_selected {
                            selected_index = None;
                        } else {
                            selected_index = Some(i);
                        }
                    }
                }
                ui.add_space(50.0);
            });

        selected_index
    }

    /// Displays a spinner when loading
    fn handle_loading(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(ui.available_size().y / 3.0);
                ui.label(egui::RichText::new("Loading...").size(15.0));

                ui.add(
                    egui::Image::new(include_image!("../../assets/logo.png"))
                        .max_size(Vec2 { x: 150.0, y: 150.0 }),
                );

                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    ui.add(Spinner::new().size(20.0));
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
                    ui.label(egui::RichText::new(format!("{}", self.error_msg)).size(50.0));
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
                    self.all_als = Some(als_data);
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
