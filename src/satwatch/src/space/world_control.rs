use imgui::*;
use legion::*;

use crate::components::{Camera, MaterialComponent, VertexList, WorldTransform};
use crate::space::dbui::SelectionChanges;
use crate::util::input_events::Event;
use crate::util::vertex_tools::*;
use glam::{Quat, Vec3, Vec4};
use libspace::coordinates::Coordinate;
use libspace::coordinates::*;
use libspace::element_db::ElementDb;
use libspace::element_engine::{ElementEngine, ElementUpdate};
use libspace::timebase::Timebase;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

pub struct WorldControl {
    last_tick: Instant,
    view_coordinate_system: CoordinateSystem,
    new_view_coordinate_system: CoordinateSystem,
    gl_origin: Coordinate,
    world_scale: f64,
    element_db: ElementDb,
    camera_velocity: Vec3,
    camera_rot: Vec3,
    timebase: Timebase,
    selected_sats: HashSet<u64>,
    elements_engine: ElementEngine,
    sat_entities: HashMap<u64, ElementEntity>,
    earth: Option<Entity>,
}

struct ElementEntity {
    entity: Entity,
    orbit: Option<Entity>,
}

impl WorldControl {
    pub fn new() -> Self {
        Self {
            last_tick: Instant::now(),
            view_coordinate_system: CoordinateSystem::Invalid,
            new_view_coordinate_system: CoordinateSystem::EarthCenteredInertial,
            gl_origin: Coordinate::new(CoordinateSystem::EarthCenteredInertial, [0.0, 0.0, 0.0]),
            world_scale: 1.0,
            element_db: ElementDb::new(),
            camera_velocity: Vec3::new(0.0, 0.0, 0.0),
            camera_rot: Vec3::new(0.0, 0.0, 0.0),
            timebase: Timebase::new(),
            selected_sats: HashSet::new(),
            elements_engine: ElementEngine::new(),
            sat_entities: HashMap::new(),
            earth: None,
        }
    }

    pub fn ui(&mut self, gl: &glow::Context, world: &mut World, ui: &mut Ui) -> Result<(), String> {
        ui.window("view").save_settings(false).build(|| {
            ui.text("Select Reference Frame");
            ui.radio_button(
                CoordinateSystem::EarthCenteredInertial.to_string(),
                &mut self.new_view_coordinate_system,
                CoordinateSystem::EarthCenteredInertial,
            );
            ui.radio_button(
                CoordinateSystem::EarthCenteredEarthFixed.to_string(),
                &mut self.new_view_coordinate_system,
                CoordinateSystem::EarthCenteredEarthFixed,
            );
            if ui.button("Reset View") {
                self.new_view_coordinate_system = self.view_coordinate_system;
                self.view_coordinate_system = CoordinateSystem::Invalid;
            }
        });

        let changed =
            crate::space::dbui::draw_db_ui(&mut self.element_db, &mut self.selected_sats, &ui);
        self.handle_element_changes(gl, world, &changed);

        ui.window("Time").save_settings(false).build(|| {
            use chrono::offset::{Local, Utc};
            use chrono::DateTime;
            let now = self.timebase.now();
            let utc: DateTime<Utc> = now.clone().into();
            let local: DateTime<Local> = now.into();
            ui.text(format!("  UTC: {}", utc.format("%+")));
            ui.text(format!("LOCAL: {}", local.format("%+")));

            let mut rt = self.timebase.realtime();
            ui.checkbox("Realtime", &mut rt);
            self.timebase.set_realtime(rt);
            if !rt {
                ui.text(format!(
                    "Time Acceleration: {}",
                    self.timebase.acceleration()
                ));
                let mut accel_i = self.timebase.acceleration() as i32;
                Slider::new("Acceleration", -100000, 100000)
                    .flags(SliderFlags::LOGARITHMIC)
                    .build(&ui, &mut accel_i);
                self.timebase.set_acceleration(accel_i as f64);
            }

            if self.timebase.running() {
                if ui.button("Pause") {
                    self.timebase.set_running(false);
                }
            } else {
                if ui.button("Run") {
                    self.timebase.set_running(true);
                }
            }
        });
        Ok(())
    }

    pub fn handle_input(&mut self, _gl: &glow::Context, _world: &mut World, event: Event) {
        match event {
            Event::None => {}
            Event::HardStop => self.camera_velocity = Vec3::new(0.0, 0.0, 0.0),
            Event::MoveLeft(b) => self.camera_velocity.x = if b { -1.0 } else { 0.0 },
            Event::MoveRight(b) => self.camera_velocity.x = if b { 1.0 } else { 0.0 },
            Event::MoveUp(b) => self.camera_velocity.y = if b { 1.0 } else { 0.0 },
            Event::MoveDown(b) => self.camera_velocity.y = if b { -1.0 } else { 0.0 },
            Event::MoveForwards(b) => self.camera_velocity.z = if b { -1.0 } else { 0.0 },
            Event::MoveBackwards(b) => self.camera_velocity.z = if b { 1.0 } else { 0.0 },
            Event::Rotate(a, b) => {
                self.camera_rot.x += -b;
                self.camera_rot.y += -a;
            }
        }
    }

    pub fn tick(&mut self, gl: &glow::Context, world: &mut World) -> Result<(), String> {
        // tick rate housekeeping
        // with early exit if we dont update
        let tick_duration = Duration::from_secs_f64(1.0 / 60.0);
        let tick_velo_step = 1.0f64 / 60.0f64;

        if Instant::now() - self.last_tick < tick_duration {
            return Ok(());
        }
        self.last_tick = Instant::now();
        self.timebase.tick(tick_duration);
        self.elements_engine.update_timebase(self.timebase.clone());
        while let Some(e) = self.elements_engine.get_more() {
            self.handle_element_update(gl, world, e);
        }

        // if we changed our view system we throw away everything in the world and redo
        if self.new_view_coordinate_system != self.view_coordinate_system {
            world.clear();
            self.sat_entities.clear();
            self.selected_sats.clear();
            self.view_coordinate_system = self.new_view_coordinate_system;
            self.regenerate_world(gl, world)?;
        }

        // camera moves
        // movement is in viewspace, and we dont have a coordinate system for that yet
        // so manual :/
        let mut cam_query = <(&mut Coordinate, &mut WorldTransform, &Camera)>::query();
        for (coord, transform, _cam) in cam_query.iter_mut(world) {
            coord.time = self.timebase.now_j2000_minutes();
            let mut gl_coord: Coordinate = coord.transform(CoordinateSystem::OpenGl);

            let rot_gl =
                Quat::from_rotation_y(self.camera_rot.y + gl_coord.accumulated_rotations[1] as f32)
                    * Quat::from_rotation_x(
                        self.camera_rot.x + gl_coord.accumulated_rotations[0] as f32,
                    )
                    * Quat::from_rotation_z(
                        self.camera_rot.z + gl_coord.accumulated_rotations[2] as f32,
                    );

            let rot_movement = rot_gl.mul_vec3(self.camera_velocity);
            gl_coord.position[0] += rot_movement.x as f64 * self.world_scale * tick_velo_step;
            gl_coord.position[1] += rot_movement.y as f64 * self.world_scale * tick_velo_step;
            gl_coord.position[2] += rot_movement.z as f64 * self.world_scale * tick_velo_step;

            *coord = gl_coord.transform(coord.system);
            *transform = WorldTransform::from_coordinate(coord, &self.gl_origin, self.world_scale);
            transform.rotation = rot_gl;
        }

        if let Some(entity) = self.earth {
            if let Ok(mut entry) = world.entry_mut(entity) {
                if let Ok(coord) = entry.get_component_mut::<Coordinate>() {
                    coord.time = self.timebase.now_j2000_minutes();
                }
            }
        }

        // update all object positions
        let mut position_query =
            <(&mut WorldTransform, &Coordinate)>::query().filter(!component::<Camera>());
        for (transform, coordinate) in position_query.iter_mut(world) {
            *transform =
                WorldTransform::from_coordinate(&coordinate, &self.gl_origin, self.world_scale);
        }
        Ok(())
    }

    fn regenerate_world(&mut self, gl: &glow::Context, world: &mut World) -> Result<(), String> {
        match self.view_coordinate_system {
            CoordinateSystem::Invalid => Ok(()),
            CoordinateSystem::EarthCenteredInertial => self.generate_eci(gl, world),
            CoordinateSystem::EarthCenteredEarthFixed => self.generate_ecef(gl, world),
            _ => Ok(()),
        }
    }

    fn generate_eci(&mut self, gl: &glow::Context, world: &mut World) -> Result<(), String> {
        use crate::components::*;
        use crate::util::vertex_tools::*;
        // in this coordinate system we draw world_scale from earth
        let body = libspace::planets::earth::Earth::body();
        self.world_scale = body.radius_mean;
        self.gl_origin = Coordinate::new(CoordinateSystem::EarthCenteredInertial, [0.0, 0.0, 0.0]);
        // camera in front, sun from the right (inaccurate but for now? whatever)
        let camera_pos = Coordinate::new(
            CoordinateSystem::EarthCenteredInertial,
            [body.radius_mean * 3.0, 0.0, 0.0],
        );
        // and planet wherever.
        // we also do not care about the transforms, as they are rebuilt anyway
        world.push((
            WorldTransform::default(),
            camera_pos,
            Camera::new(90.0, 0.001, 100.0),
        ));
        world.push((DirectionalLight {
            direction: Vec3::new(-1.0, 0.0, 0.0),
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            ambient: 0.05,
        },));
        // so planet? some sphere...
        let (planet_vertices, planet_indices, planet_normals) = gen_icosphere(1.0, 4);
        self.earth = Some(world.push((
            WorldTransform::default(),
            Coordinate::new(CoordinateSystem::EarthCenteredEarthFixed, [0.0, 0.0, 0.0]),
            VertexList::create_triangles(
                gl,
                &planet_vertices,
                Some(&planet_indices),
                Some(&planet_normals),
            )?,
            MaterialComponent("material/earth.toml".to_string()),
        )));

        Ok(())
    }

    fn generate_ecef(&mut self, gl: &glow::Context, world: &mut World) -> Result<(), String> {
        let res = self.generate_eci(gl, world);
        if res.is_ok() {
            let mut cam_query = <(&mut Coordinate, &WorldTransform, &Camera)>::query();
            for (coord, _transform, _cam) in cam_query.iter_mut(world) {
                coord.system = CoordinateSystem::EarthCenteredEarthFixed;
            }
        }
        res
    }

    pub fn handle_element_changes(
        &mut self,
        gl: &glow::Context,
        world: &mut World,
        changes: &SelectionChanges,
    ) {
        for add in &changes.added {
            let (sat_vertices, sat_indices, sat_normal) = gen_icosphere(0.01, 4);
            if let Some(element) = self.element_db.get(*add) {
                self.elements_engine.add(element);
                let entity = world.push((
                    WorldTransform::default(),
                    Coordinate::invalid(),
                    VertexList::create_triangles(
                        gl,
                        &sat_vertices,
                        Some(&sat_indices),
                        Some(&sat_normal),
                    )
                    .unwrap(),
                    MaterialComponent("material/sats.toml".to_string()),
                ));
                self.sat_entities.insert(
                    element.norad_id,
                    ElementEntity {
                        entity,
                        orbit: None,
                    },
                );
            }
        }

        for remove in &changes.removed {
            if let Some(e) = self.sat_entities.get(remove) {
                self.elements_engine.remove(*remove);
                world.remove(e.entity);
                if let Some(orb) = e.orbit {
                    world.remove(orb);
                }
                self.sat_entities.remove(remove);
            }
        }
    }

    fn make_orbit_vertex_list(
        world_scale: f64,
        gl: &glow::Context,
        points: Vec<Coordinate>,
    ) -> Result<VertexList, String> {
        let (vert, ind) = gen_orbit_points(points, world_scale);

        VertexList::create_lines(gl, &vert, Some(&ind), None)
    }

    fn handle_element_update(&mut self, gl: &glow::Context, world: &mut World, up: ElementUpdate) {
        if let Some(e) = self.sat_entities.get_mut(&up.id) {
            if let Ok(mut entry) = world.entry_mut(e.entity) {
                if let Ok(pos) = entry.get_component_mut::<Coordinate>() {
                    *pos = up.state.coordinate;
                }
            }

            if let Some(new_points) = up.orbit_points {
                if let Some(entity) = &e.orbit {
                    if let Ok(mut entry) = world.entry_mut(*entity) {
                        if let Ok(vlist) = entry.get_component_mut::<VertexList>() {
                            if let Ok(new_vlist) =
                                Self::make_orbit_vertex_list(self.world_scale, gl, new_points)
                            {
                                *vlist = new_vlist;
                            }
                        }
                    }
                } else {
                    if let Ok(new_vlist) =
                        Self::make_orbit_vertex_list(self.world_scale, gl, new_points)
                    {
                        e.orbit = Some(world.push((
                            WorldTransform::default(),
                            Coordinate::new(
                                CoordinateSystem::EarthCenteredInertial,
                                [0.0, 0.0, 0.0],
                            ),
                            new_vlist,
                            MaterialComponent("material/colored_orbit.toml".to_string()),
                        )));
                    }
                }
            }
        }
    }
}
