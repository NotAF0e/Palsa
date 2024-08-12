use crate::{gui::tabs::TabType, parse::als::Project};
use eframe::egui::{
    self, widgets::Spinner, Align, IconData, SelectableLabel, TextStyle, TextureHandle, Vec2,
};
use egui_dock::DockState;
use egui_extras::install_image_loaders;
use image;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
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

    pub preview_x_scale: f32,
    pub preview_x_pos: f32,
    pub preview_y_scale: f32,

    pub colors: [String; 70],
    icon_path: String,

    frame_time: Duration,
}

impl Gui {
    pub fn new(receiver: mpsc::Receiver<Result<Vec<Project>, String>>) -> Self {
        Self {
            receiver,
            state: GuiState::Loading,
            error_msg: "THERE WAS AN ERROR BUT HOW???".to_string(),
            projects: None,
            selected_project_als: None,
            dock_state: Gui::default_tab_layout(),

            preview_x_pos: 0.,
            preview_x_scale: 3.,
            preview_y_scale: 13.,

            colors: match load_colors() {
                Ok(colors) => colors,
                Err(_) => init_colors(), // On this arm colors will just be errored and purple
            },
            icon_path: String::from("assets/palsa/icon.png"),

            frame_time: Duration::new(0, 0),
        }
    }
    pub fn run(self) {
        eframe::run_native(
            "Palsa - Preview ableton live sets actually! Made with love by NotAFoe <3",
            eframe::NativeOptions {
                viewport: egui::ViewportBuilder {
                    icon: Some(Gui::load_icon(&self.icon_path).into()),
                    ..Default::default()
                },
                ..Default::default()
            },
            Box::new(|_cc| Box::new(self)),
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

    /// Displays a spinner when loading
    fn handle_loading(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(ui.available_size().y / 3.);
                ui.label(egui::RichText::new("Loading...").size(15.));

                ui.add(
                    egui::Image::from_texture(&self.app_icon(ctx))
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

            self.control_window(ctx);
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

    fn app_icon(&self, ctx: &egui::Context) -> TextureHandle {
        let path = Path::new(&self.icon_path);
        if let Ok(image) = image::open(path) {
            let image = image.to_rgba8();
            let dimensions = image.dimensions();
            let pixels = image.into_raw();

            let texture = ctx.load_texture(
                "icon",
                egui::ColorImage::from_rgba_unmultiplied(
                    [dimensions.0 as _, dimensions.1 as _],
                    &pixels,
                ),
                egui::TextureOptions {
                    ..Default::default()
                },
            );

            texture
        } else {
            panic!("Icon was not found at {:?}", &self.icon_path);
        }
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

fn load_colors() -> Result<[String; 70], io::Error> {
    let path = Path::new("assets/palsa/default-colors.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut colors = init_colors();

    for (i, line) in reader.lines().enumerate() {
        if i >= 70 {
            break; // Stop reading if more than 70 lines are encountered
        }
        colors[i] = line?; // `line?` is the String, use it directly
    }

    // Check if we have exactly 70 colors
    if colors.iter().any(|s| s.is_empty()) {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "File contains fewer than 70 lines",
        ))
    } else {
        Ok(colors)
    }
}

fn init_colors() -> [String; 70] {
    // Initalizes an array of 70 purple hex values (Unloaded texture values)
    // Cant use [String::from("800080"); 70] so im using this bellow to avoid the copy trait :(
    return std::array::from_fn(|_| String::from("800080"));
}
