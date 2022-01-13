use crate::components::{Camera, DirectionalLight, MaterialComponent, VertexList, WorldTransform};
use crate::util::input_events::Event;
use crate::util::vertex_tools::gen_icosphere;
use crate::world::world_ui::WorldUi;
use glam::f32::*;
use glam::f64::*;
use glow::{Context, TRANSFORM_FEEDBACK};
use imgui::*;
use legion::*;
use libspace::bodies::Planet;
use libspace::coordinate::{CoordinateUnit, IcrfStateVector};
use libspace::timebase::Timebase;

pub struct ViewUi {
    visible: bool,
    target_planet: Planet,
    gl_origin: IcrfStateVector,
    world_scale: f64,
    camera_velocity: Vec3,
    camera_rot: Vec3,
    camera_entity: Entity,
}

impl ViewUi {
    pub fn new(gl: &glow::Context, world: &mut World) -> Result<Self, String> {
        let camera_entity = world.push((
            Camera::new(90.0, 0.01, 1000.0),
            IcrfStateVector {
                unit: CoordinateUnit::Au,
                position: Default::default(),
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
            world_scale: 1.0,
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
        self.add_planet(gl, world, Planet::Mercury)?;
        self.add_planet(gl, world, Planet::Venus)?;
        self.add_planet(gl, world, Planet::Earth)?;
        self.add_planet(gl, world, Planet::Mars)?;
        self.add_planet(gl, world, Planet::Jupiter)?;
        self.add_planet(gl, world, Planet::Saturn)?;
        self.add_planet(gl, world, Planet::Uranus)?;
        self.add_planet(gl, world, Planet::Neptune)?;
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

        Ok(())
    }
}

impl WorldUi for ViewUi {
    fn main_menu(&mut self, ui: &Ui) {}

    fn ui(&mut self, gl: &glow::Context, world: &mut World, ui: &mut Ui) -> Result<(), String> {
        if self.visible {
            ui.window("View Control")
                .opened(&mut self.visible)
                .build(|| {
                    if ui.collapsing_header("Target Planet", TreeNodeFlags::DEFAULT_OPEN) {
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
                    }
                });
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
        Ok(())
    }
}
