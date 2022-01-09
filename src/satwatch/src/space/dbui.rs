use imgui::*;
use libspace::element_db::ElementDb;
use log::error;
use std::collections::HashSet;

pub struct SelectionChanges {
    pub added: Vec<u64>,
    pub removed: Vec<u64>,
}

pub fn draw_db_ui(db: &mut ElementDb, selected: &mut HashSet<u64>, ui: &Ui) -> SelectionChanges {
    let mut changes = SelectionChanges {
        added: Vec::new(),
        removed: Vec::new(),
    };

    ui.window("Elements Database")
        .save_settings(false)
        .size([300.0, 300.0], Condition::Appearing)
        .build(|| {
            if ui.button("Full update all") {
                if let Err(e) = db.fetch_full_celestrak() {
                    error!("Something went wrong during update: {}", e);
                } else {
                    db.save();
                }
            }
            if ui.button("YOLO") {
                for (id, _) in db.all() {
                    changes.added.push(*id);
                    selected.insert(*id);
                }
            }
            ui.child_window("Element Table")
                .always_vertical_scrollbar(true)
                .build(|| {
                    ui.columns(4, "Elements Table", true);
                    ui.text("Selected");
                    ui.next_column();
                    ui.text("Name");
                    ui.next_column();
                    ui.text("NORAD Id");
                    ui.next_column();
                    ui.text("?");
                    ui.next_column();
                    for (_, elements) in db.all() {
                        let _id = ui.push_id_usize(elements.norad_id as usize);
                        let mut select_button = selected.contains(&elements.norad_id);
                        if ui.checkbox("##name", &mut select_button) {
                            if select_button {
                                selected.insert(elements.norad_id);
                                changes.added.push(elements.norad_id);
                            } else {
                                selected.remove(&elements.norad_id);
                                changes.removed.push(elements.norad_id);
                            }
                        }

                        ui.next_column();
                        ui.text(format!(
                            "{}",
                            elements
                                .object_name
                                .as_ref()
                                .unwrap_or(&"Unknown".to_string())
                        ));
                        ui.next_column();
                        ui.text(format!("{}", elements.norad_id));
                        ui.next_column();
                        ui.text(format!(
                            "{}",
                            elements
                                .international_designator
                                .as_ref()
                                .unwrap_or(&"Unknown".to_string())
                        ));
                        ui.next_column();
                    }
                });
        });

    changes
}
