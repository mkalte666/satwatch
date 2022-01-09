use crate::components::*;
use crate::rendering::material::Material;
use glow::Context;
use legion::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Renderer {
    material_store: HashMap<String, Material>,
}

impl Renderer {
    pub fn create() -> Self {
        Self {
            material_store: HashMap::new(),
        }
    }

    pub fn load(&mut self, gl: &Context, world: &mut World) -> Result<(), String> {
        let mut mat_query =
            <&MaterialComponent>::query().filter(maybe_changed::<MaterialComponent>());
        for mat_component in mat_query.iter(world) {
            if !self.material_store.contains_key(&mat_component.0) {
                let mat = Material::from_file(gl, &mat_component.0)?;
                self.material_store.insert(mat_component.0.clone(), mat);
            }
        }
        Ok(())
    }

    pub fn draw(&self, gl: &Context, world: &mut World, aspect: f32) {
        let mut cam_query = <(&WorldTransform, &Camera)>::query();
        let mut object_query = <(&WorldTransform, &VertexList, &MaterialComponent)>::query();
        let lights: Vec<DirectionalLight> = Read::<DirectionalLight>::query()
            .iter(world)
            .copied()
            .collect();

        for (cam_transform, camera) in cam_query.iter(world) {
            let vp = camera.get_view_projection(aspect, cam_transform);
            let light = lights.first();
            for (transform, v_list, mat_comp) in object_query.iter(world) {
                if let Some(material) = self.material_store.get(&mat_comp.0) {
                    material.bind(gl);
                    let m = transform.get_model_matrix();
                    material.set_mvp(gl, vp * m);
                    if let Some(l) = light {
                        material.set_directional_light(gl, *l);
                    }
                    v_list.bind_and_draw(gl);
                }
            }
        }
    }
}
