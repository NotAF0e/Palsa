use crate::gui::gui::Gui;
use eframe::egui::{self, Label};
use egui_dock::{DockArea, DockState, NodeIndex, Split, Style};

struct TabViewer<'a> {
    gui_handle: &'a mut Gui,
}

#[derive(Clone, PartialEq)]
pub enum TabType {
    AlsFileList,
    AlsViewer,
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = TabType;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            TabType::AlsFileList => "Als File List".to_string().into(),
            TabType::AlsViewer => "Als Viewer".to_string().into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        if let Some(all_als) = &self.gui_handle.all_als {
            match tab {
                TabType::AlsFileList => {
                    self.gui_handle.selected_als = self.gui_handle.als_panel(ui, all_als);
                }
                TabType::AlsViewer => {
                    if let Some(selected_als) = self.gui_handle.selected_als {
                        ui.add(Label::new(format!("{:?}", all_als[selected_als])));
                    }
                }
            }
        }
    }
}

impl Gui {
    pub fn default_tab_layout() -> DockState<TabType> {
        let mut dock_state = DockState::new(vec![TabType::AlsFileList]);

        // Get the index of the root node, which is always 0
        let root_index: NodeIndex = 0.into();

        dock_state.split(
            (egui_dock::SurfaceIndex::main(), root_index),
            Split::Right,
            0.25,
            egui_dock::Node::leaf(TabType::AlsViewer),
        );

        dock_state
    }
}

impl Gui {
    pub fn tabs(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Create a placeholder DockState to temporarily replace self.dock_state
        let placeholder_dock_state = DockState::new(vec![]);

        // Replace self.dock_state with the placeholder and take the original
        let mut dock_state = std::mem::replace(&mut self.dock_state, placeholder_dock_state);

        let mut tab_viewer = TabViewer { gui_handle: self };
        DockArea::new(&mut dock_state)
            .style(Style::from_egui(&ctx.style()))
            .show(ctx, &mut tab_viewer);

        self.dock_state = dock_state;
    }
}
