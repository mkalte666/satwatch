use crate::components::{MaterialComponent, VertexList, WorldTransform};
use crate::util::input_events::Event;
use crate::util::vertex_tools::gen_icosphere;
use crate::world::world_ui::WorldUi;
use imgui::*;
use legion::*;
use libspace::bodies::Planet;
use libspace::coordinate::{CoordinateUnit, PlanetaryReferenceFrame, PlanetaryStateVector};
use libspace::elements::*;
use libspace::timebase::Timebase;
use log::error;
use std::collections::{HashMap, HashSet};

pub struct DbUi {
    visible: bool,
    order: ElementSort,
    db: ElementDb,
    search_term: String,
    search_exact: bool,
    engine: ElementEngine,
    tracked_items: HashMap<u64, Entity>,
}

impl DbUi {
    pub fn new() -> Self {
        Self {
            visible: false,
            order: ElementSort::ByName,
            db: ElementDb::new(),
            search_term: String::new(),
            search_exact: false,
            engine: ElementEngine::new(),
            tracked_items: HashMap::new(),
        }
    }

    fn add_new(&mut self, gl: &glow::Context, world: &mut World, id: u64) -> Result<(), String> {
        if self.tracked_items.contains_key(&id) {
            return Ok(());
        }

        if let Some(elements) = self.db.get(id) {
            let (verts, index, normal) = gen_icosphere(0.005, 2);
            let entity = world.push((
                PlanetaryStateVector {
                    planet: Planet::Earth,
                    reference_frame: PlanetaryReferenceFrame::Inertial,
                    unit: CoordinateUnit::Meter,
                    position: Default::default(),
                    velocity: Default::default(),
                },
                WorldTransform::default(),
                VertexList::create_triangles(gl, &verts, Some(&index), Some(&normal))?,
                MaterialComponent("material/sats.toml".to_string()),
            ));
            self.tracked_items.insert(id, entity);
            self.engine.add(elements);
        }
        Ok(())
    }

    fn remove(&mut self, world: &mut World, id: u64) {
        if let Some(item) = self.tracked_items.get(&id) {
            world.remove(*item);
            self.tracked_items.remove(&id);
            self.engine.remove(id);
        }
    }
}

impl WorldUi for DbUi {
    fn main_menu(&mut self, ui: &Ui) {
        ui.menu("View", || {
            if ui.menu_item("TLE Database") {
                self.visible = true;
            }
        });
    }

    fn ui(&mut self, gl: &glow::Context, world: &mut World, ui: &mut Ui) -> Result<(), String> {
        let serach_results = if self.search_term.is_empty() {
            Vec::new()
        } else {
            self.db
                .index()
                .find_str(&self.search_term, self.search_exact)
        };
        let mut to_add = Vec::new();
        let mut to_remove = Vec::new();

        if self.visible {
            ui.window("TLE Database")
                .opened(&mut self.visible)
                .size([300.0, 300.0], Condition::Appearing)
                .build(|| {
                    if ui.button("Full update all") {
                        if let Err(e) = self.db.fetch_full_celestrak() {
                            error!("Something went wrong during update: {}", e);
                        } else {
                            self.db.save();
                        }
                    }
                    ui.separator();
                    ui.input_text("Search Sats", &mut self.search_term).build();
                    ui.same_line();
                    ui.checkbox("Exact Match", &mut self.search_exact);
                    let items = if self.search_term.is_empty() {
                        self.db.index().get_by(self.order)
                    } else {
                        &serach_results
                    };

                    ui.text(format!("Found {} sats.", items.len()));
                    ui.same_line();
                    if ui.button("Add all results") {
                        to_add.append(&mut items.iter().map(|x| *x).collect::<Vec<u64>>());
                    }
                    ui.same_line();
                    if ui.button("Remove All") {
                        to_remove.extend(self.tracked_items.keys());
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
                            for id in items {
                                let _id_scope = ui.push_id_usize(*id as usize);
                                if let Some(elements) = self.db.get_copy(*id) {
                                    let mut tracked = self.tracked_items.contains_key(id);
                                    if ui.checkbox("Select", &mut tracked) {
                                        if tracked {
                                            to_add.push(*id);
                                        } else {
                                            to_remove.push(*id);
                                        }
                                    }
                                    ui.next_column();
                                    if let Some(e_name) = &elements.object_name {
                                        ui.text(e_name);
                                    }
                                    ui.next_column();
                                    ui.text(format!("{}", elements.norad_id));
                                    ui.next_column();
                                    ui.text("TBA");
                                    ui.next_column();
                                }
                            }
                        });
                });
        }

        for remove in to_remove {
            self.remove(world, remove);
        }

        for add in to_add {
            self.add_new(gl, world, add)
                .unwrap_or_else(|e| log::error!("Adding sat failed: {}", e));
        }

        Ok(())
    }

    fn handle_input(&mut self, _gl: &glow::Context, _world: &mut World, event: Event) {}

    fn tick(
        &mut self,
        gl: &glow::Context,
        world: &mut World,
        timebase: &mut Timebase,
    ) -> Result<(), String> {
        self.engine.update_timebase(timebase.clone());

        while let Some(update) = self.engine.get_more() {
            if let Some(entity) = self.tracked_items.get(&update.id) {
                if let Ok(mut entry) = world.entry_mut(*entity) {
                    if let Ok(state_vec) = entry.get_component_mut::<PlanetaryStateVector>() {
                        *state_vec = update.state;
                    }
                }
            } else {
                self.engine.remove(update.id);
            }
        }
        Ok(())
    }
}

pub struct SelectionChanges {
    pub added: Vec<u64>,
    pub removed: Vec<u64>,
}
