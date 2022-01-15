use crate::components::{Camera, DirectionalLight, MaterialComponent, VertexList, WorldTransform};
use crate::util::input_events::Event;
use crate::util::vertex_tools::{gen_icosphere, gen_orbit_points_icrf};
use crate::world::world_ui::WorldUi;
use glam::f32::*;
use glam::f64::*;
use glam::EulerRot;
use imgui::*;
use legion::query::{ComponentFilter, EntityFilterTuple};
use legion::*;
use libspace::bodies::Planet;
use libspace::coordinate::{
    CoordinateUnit, IcrfStateVector, PlanetaryReferenceFrame, PlanetaryStateVector,
};
use libspace::timebase::Timebase;

struct OrbitObjectTag {}

pub struct ViewUi {
    visible: bool,
    target_planet: Planet,
    gl_origin: IcrfStateVector,
    world_scale: f64,
    world_scale_unit: CoordinateUnit,
    camera_velocity: Vec3,
    camera_rot: Vec3,
    camera_entity: Entity,
}

impl ViewUi {
    pub fn new(gl: &glow::Context, world: &mut World) -> Result<Self, String> {
        let camera_entity = world.push((
            Camera::new(90.0, 0.01, 10000000000000000.0),
            PlanetaryStateVector {
                planet: Planet::Earth,
                reference_frame: PlanetaryReferenceFrame::Inertial,
                unit: CoordinateUnit::KiloMeter,
                position: DVec3::new(Planet::Earth.body().radius_mean * 5.0, 0.0, 0.0),
                velocity: Default::default(),
            },
            WorldTransform {
                translation: Default::default(),
                scale: Default::default(),
                rotation: Default::default(),
            },
        ));

        let new = Self {
            visible: true,
            target_planet: Planet::Earth,
            gl_origin: IcrfStateVector {
                unit: CoordinateUnit::Meter,
                position: DVec3::new(0.0, 0.0, 0.0),
                velocity: DVec3::new(0.0, 0.0, 0.0),
            },
            world_scale: Planet::Earth.body().radius_mean,
            world_scale_unit: CoordinateUnit::KiloMeter,
            camera_velocity: Vec3::new(0.0, 0.0, 0.0),
            camera_rot: Vec3::new(0.0, 0.0, 0.0),
            camera_entity,
        };

        new.add_planets(gl, world)?;
        // here we would add the sun
        world.push((DirectionalLight {
            direction: Vec3::new(0.0, -1.0, 0.0),
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            ambient: 0.4,
        },));
        Ok(new)
    }

    fn add_planets(&self, gl: &glow::Context, world: &mut World) -> Result<(), String> {
        self.add_planet(gl, world, Planet::Sun)?;
        self.add_planet(gl, world, Planet::Mercury)?;
        //self.add_planet(gl, world, Planet::Venus)?;
        self.add_planet(gl, world, Planet::Earth)?;
        //self.add_planet(gl, world, Planet::Mars)?;
        //self.add_planet(gl, world, Planet::Jupiter)?;
        //self.add_planet(gl, world, Planet::Saturn)?;
        //self.add_planet(gl, world, Planet::Uranus)?;
        //self.add_planet(gl, world, Planet::Neptune)?;
        Ok(())
    }

    fn add_planet(
        &self,
        gl: &glow::Context,
        world: &mut World,
        planet: Planet,
    ) -> Result<(), String> {
        let (vert, index, normal) = gen_icosphere(1.0, 5);
        world.push((
            planet,
            WorldTransform::default(),
            VertexList::create_triangles(gl, &vert, Some(&index), Some(&normal)).unwrap(),
            MaterialComponent("material/earth.toml".to_string()),
        ));

        let (orb_vert, orb_index) = gen_orbit_points_icrf(
            planet.rough_pos_list(&Timebase::new()),
            self.world_scale,
            self.world_scale_unit,
            &self.gl_origin,
        );
        world.push((
            OrbitObjectTag {},
            planet,
            WorldTransform::default(),
            VertexList::create_lines(gl, &orb_vert, Some(&orb_index), None).unwrap(),
            MaterialComponent("material/colored_orbit.toml".to_string()),
        ));
        Ok(())
    }

    fn reset_view(&mut self, gl: &glow::Context, world: &mut World) {
        // special case: sun
        if self.target_planet == Planet::Sun {
            if let Ok(mut cam_entry) = world.entry_mut(self.camera_entity) {
                if let Ok(cam_pos) = cam_entry.get_component_mut::<PlanetaryStateVector>() {
                    cam_pos.position = DVec3::new(1.5, 0.5, 0.0);
                    cam_pos.planet = self.target_planet;
                    cam_pos.unit = CoordinateUnit::KiloMeter;
                }
            }
            // world scale
            self.world_scale = 1.0;
            self.world_scale_unit = CoordinateUnit::Au;
        } else {
            // cam
            if let Ok(mut cam_entry) = world.entry_mut(self.camera_entity) {
                if let Ok(cam_pos) = cam_entry.get_component_mut::<PlanetaryStateVector>() {
                    cam_pos.position =
                        DVec3::new(self.target_planet.body().radius_mean * 5.0, 0.0, 0.0);
                    cam_pos.planet = self.target_planet;
                    cam_pos.unit = CoordinateUnit::KiloMeter;
                }
            }
            // world scale
            self.world_scale = self.target_planet.body().radius_mean;
            self.world_scale_unit = CoordinateUnit::KiloMeter;
        }
    }

    fn update_camera(&mut self, gl: &glow::Context, world: &mut World, timebase: &Timebase) {
        let tick_speed: f64 = 1.0 / 60.0;
        let mut cam_query = <(&Camera, &mut PlanetaryStateVector, &mut WorldTransform)>::query();
        for (_cam, cam_pos, cam_transform) in cam_query.iter_mut(world) {
            let icrf = cam_pos.to_icrf(timebase);
            let mut gl_pos =
                icrf.to_gl_coord(self.world_scale, self.world_scale_unit, &self.gl_origin);

            // update rotation
            let old_rot: Quat = cam_transform.rotation.clone();
            let (y, x, z) = old_rot.to_euler(EulerRot::YXZ);
            let new_rot = Quat::from_euler(
                EulerRot::YXZ,
                y + self.camera_rot.y,
                x + self.camera_rot.x,
                z + self.camera_rot.z,
            );
            self.camera_rot = Vec3::default();

            // update translation
            let rotated_speed = new_rot.mul_vec3(self.camera_velocity);
            gl_pos = gl_pos
                + DVec3::new(
                    rotated_speed.x as f64,
                    rotated_speed.y as f64,
                    rotated_speed.z as f64,
                ) * tick_speed;

            // get these updates back into the transform
            let new_icrf = IcrfStateVector::from_gl_coord(
                &gl_pos,
                self.world_scale,
                self.world_scale_unit,
                &self.gl_origin,
            );
            *cam_transform = WorldTransform::from_icrf(
                &new_icrf,
                &self.gl_origin,
                self.world_scale,
                self.world_scale_unit,
                None,
            );
            cam_transform.rotation = new_rot;
            *cam_pos = PlanetaryStateVector::from_icrf(new_icrf, timebase, self.target_planet);
        }
    }
}

impl WorldUi for ViewUi {
    fn main_menu(&mut self, ui: &Ui) {}

    fn ui(&mut self, gl: &glow::Context, world: &mut World, ui: &mut Ui) -> Result<(), String> {
        let mut triggers_reset = false;
        if self.visible {
            ui.window("View Control")
                .opened(&mut self.visible)
                .build(|| {
                    if ui.collapsing_header("Target Planet", TreeNodeFlags::DEFAULT_OPEN) {
                        triggers_reset = triggers_reset || ui.button("Reset View");
                        let old_target = self.target_planet;
                        ui.radio_button(
                            Planet::Sun.to_string(),
                            &mut self.target_planet,
                            Planet::Sun,
                        );
                        ui.radio_button(
                            Planet::Mercury.to_string(),
                            &mut self.target_planet,
                            Planet::Mercury,
                        );
                        ui.radio_button(
                            Planet::Venus.to_string(),
                            &mut self.target_planet,
                            Planet::Venus,
                        );
                        ui.radio_button(
                            Planet::Earth.to_string(),
                            &mut self.target_planet,
                            Planet::Earth,
                        );
                        ui.radio_button(
                            Planet::Mars.to_string(),
                            &mut self.target_planet,
                            Planet::Mars,
                        );
                        ui.radio_button(
                            Planet::Jupiter.to_string(),
                            &mut self.target_planet,
                            Planet::Jupiter,
                        );
                        ui.radio_button(
                            Planet::Saturn.to_string(),
                            &mut self.target_planet,
                            Planet::Saturn,
                        );
                        ui.radio_button(
                            Planet::Uranus.to_string(),
                            &mut self.target_planet,
                            Planet::Uranus,
                        );
                        ui.radio_button(
                            Planet::Neptune.to_string(),
                            &mut self.target_planet,
                            Planet::Neptune,
                        );
                        triggers_reset = triggers_reset || self.target_planet != old_target;
                    }
                });
        }
        if triggers_reset {
            self.reset_view(gl, world);
        }
        Ok(())
    }

    fn handle_input(&mut self, _gl: &glow::Context, _world: &mut World, event: Event) {
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

    fn tick(
        &mut self,
        gl: &glow::Context,
        world: &mut World,
        timebase: &mut Timebase,
    ) -> Result<(), String> {
        // update gl origin to target
        self.gl_origin = self.target_planet.pos_icrf(timebase);

        // update planet positions
        let mut planet_query =
            <(&Planet, &mut WorldTransform)>::query().filter(!component::<OrbitObjectTag>());
        for (planet, transform) in planet_query.iter_mut(world) {
            let pos = planet.pos_icrf(timebase);
            log::trace!("Planet {} position update: {}", planet, pos);
            *transform = WorldTransform::from_icrf(
                &pos,
                &self.gl_origin,
                self.world_scale,
                self.world_scale_unit,
                Some(*transform),
            );
            transform.rotation = planet.gl_rotation_at(timebase);
            // we should also touch scale
            let scale: f64 = (planet.body().radius_mean
                / self
                    .world_scale_unit
                    .to(CoordinateUnit::KiloMeter, &self.world_scale))
            .max(0.01);
            transform.scale = Vec3::new(scale as f32, scale as f32, scale as f32);
        }

        // planet orbits
        let mut orbit_query = <(&OrbitObjectTag, &Planet, &mut VertexList)>::query();
        for (_tag, planet, list) in orbit_query.iter_mut(world) {
            let (orb_vert, orb_index) = gen_orbit_points_icrf(
                planet.rough_pos_list(&Timebase::new()),
                self.world_scale,
                self.world_scale_unit,
                &self.gl_origin,
            );
            *list = VertexList::create_lines(gl, &orb_vert, Some(&orb_index), None)?;
        }

        // update things with planetary positions
        let mut planet_state_query = <(&PlanetaryStateVector, &mut WorldTransform)>::query();
        for (state_vec, transform) in planet_state_query.iter_mut(world) {
            let pos = state_vec.to_icrf(timebase);
            *transform = WorldTransform::from_icrf(
                &pos,
                &self.gl_origin,
                self.world_scale,
                self.world_scale_unit,
                Some(*transform),
            );
        }

        // update things with icrf positions, as the origin can change too
        let mut icrf_state_query = <(&IcrfStateVector, &mut WorldTransform)>::query();
        for (state_vec, transform) in icrf_state_query.iter_mut(world) {
            *transform = WorldTransform::from_icrf(
                &state_vec,
                &self.gl_origin,
                self.world_scale,
                self.world_scale_unit,
                Some(*transform),
            );
        }

        // camera update needs las, as otherwise its transform is updated along the rest
        self.update_camera(gl, world, timebase);
        Ok(())
    }
}
