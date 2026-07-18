use crate::config::MAX_LEVEL;

pub struct UiState {
    pub show_credits: bool,
    pub show_go_to_level: bool,
    pub level_input_buffer: String,
}

impl UiState {
    pub fn new(current_level: usize) -> Self {
        Self {
            show_credits: false,
            show_go_to_level: false,
            level_input_buffer: current_level.to_string(),
        }
    }
}

pub fn update_ui(ui_state: &mut UiState, current_level: usize) -> Option<usize> {
    let mut level_to_load = None;

    egui_macroquad::ui(|egui_ctx| {
        egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Game", |ui| {
                    if ui.button("Go To Level").clicked() {
                        ui_state.show_go_to_level = true;
                        ui_state.level_input_buffer = current_level.to_string();
                        ui.close_menu();
                    }
                    if ui.button("Credits").clicked() {
                        ui_state.show_credits = true;
                        ui.close_menu();
                    }
                    if ui.button("Source Code").clicked() {
                        let _ = open::that("https://github.com/Pugsby/opensokoban");
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });

        if ui_state.show_credits {
            egui::Window::new("Credits")
                .resizable(false)
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    ui.label("Creator: Pugsby");
                    ui.label("Assets: Vellidragon");
                    ui.vertical_centered(|ui| {
                        if ui.button("Close").clicked() {
                            ui_state.show_credits = false;
                        }
                    });
                });
        }

        if ui_state.show_go_to_level {
            egui::Window::new("Go To Level")
                .resizable(false)
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Enter Level (1-60):");
                        let response = ui.text_edit_singleline(&mut ui_state.level_input_buffer);
                        response.request_focus();
                    });

                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        if ui.button("Load").clicked() || (ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                            if let Ok(number) = ui_state.level_input_buffer.parse::<usize>() {
                                if number >= 1 && number <= MAX_LEVEL {
                                    level_to_load = Some(number);
                                    ui_state.show_go_to_level = false;
                                }
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            ui_state.show_go_to_level = false;
                        }
                    });
                });
        }
    });

    level_to_load
}
