use glam::f32::*;

use libspace::coordinates::{Coordinate, CoordinateSystem};
pub fn gen_orbit_points(points: Vec<Coordinate>, world_scale: f64) -> (Vec<Vec3>, Vec<u32>) {
    let mut results = Vec::new();
    //01,12,23,...,n0
    let mut indices = Vec::new();

    let elements = points.len() as u32;
    for i in 0..elements {
        let gl_coord: Coordinate = points
            .get(i as usize)
            .unwrap()
            .transform(CoordinateSystem::OpenGl);
        results.push(Vec3::new(
            (gl_coord.position[0] / world_scale) as f32,
            (gl_coord.position[1] / world_scale) as f32,
            (gl_coord.position[2] / world_scale) as f32,
        ));
        if i + 1 < elements {
            indices.push(i);
            indices.push(i + 1);
        }
    }

    (results, indices)
}

pub fn gen_flat_circle_xz(radius: f32, elements: u32) -> (Vec<Vec3>, Vec<u32>) {
    let mut results = Vec::new();

    //01,12,23,...,n0
    let mut indices = Vec::new();

    let step: f32 = 2.0 * std::f32::consts::PI / (elements as f32);
    for i in 0..elements {
        let phase: f32 = (i as f32) * step;
        let x = phase.cos() * radius;
        let z = phase.sin() * radius;
        let y = 0.0 * radius;
        results.push(Vec3::new(x, y, z));
        indices.push(i);
        indices.push((i + 1) % elements);
    }

    (results, indices)
}

// following http://blog.andreaskahler.com/2009/06/creating-icosphere-mesh-in-code.html
pub fn gen_icosahedron(r: f32) -> (Vec<Vec3>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    // vertices
    let t = (1.0 + (5.0_f32).sqrt()) / 2.0;
    vertices.push(Vec3::new(-1.0, t, 0.0));
    vertices.push(Vec3::new(1.0, t, 0.0));
    vertices.push(Vec3::new(-1.0, -t, 0.0));
    vertices.push(Vec3::new(1.0, -t, 0.0));
    vertices.push(Vec3::new(0.0, -1.0, t));
    vertices.push(Vec3::new(0.0, 1.0, t));
    vertices.push(Vec3::new(0.0, -1.0, -t));
    vertices.push(Vec3::new(0.0, 1.0, -t));
    vertices.push(Vec3::new(t, 0.0, -1.0));
    vertices.push(Vec3::new(t, 0.0, 1.0));
    vertices.push(Vec3::new(-t, 0.0, -1.0));
    vertices.push(Vec3::new(-t, 0.0, 1.0));

    for v in &mut vertices {
        *v = v.normalize() * r;
    }

    // indices
    // and you thought the vertices were bad
    indices.extend_from_slice(&[0, 11, 5]);
    indices.extend_from_slice(&[0, 5, 1]);
    indices.extend_from_slice(&[0, 1, 7]);
    indices.extend_from_slice(&[0, 7, 10]);
    indices.extend_from_slice(&[0, 10, 11]);

    indices.extend_from_slice(&[1, 5, 9]);
    indices.extend_from_slice(&[5, 11, 4]);
    indices.extend_from_slice(&[11, 10, 2]);
    indices.extend_from_slice(&[10, 7, 6]);
    indices.extend_from_slice(&[7, 1, 8]);

    indices.extend_from_slice(&[3, 9, 4]);
    indices.extend_from_slice(&[3, 4, 2]);
    indices.extend_from_slice(&[3, 2, 6]);
    indices.extend_from_slice(&[3, 6, 8]);
    indices.extend_from_slice(&[3, 8, 9]);

    indices.extend_from_slice(&[4, 9, 5]);
    indices.extend_from_slice(&[2, 4, 11]);
    indices.extend_from_slice(&[6, 2, 10]);
    indices.extend_from_slice(&[8, 6, 7]);
    indices.extend_from_slice(&[9, 8, 1]);

    (vertices, indices)
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
struct MiddlePointIndex {
    ia: u32,
    ib: u32,
}

fn find_middle_point(
    r: f32,
    a: u32,
    b: u32,
    middle_point_cache: &mut std::collections::HashMap<MiddlePointIndex, u32>,
    vertices: &mut Vec<Vec3>,
) -> u32 {
    let index = if a < b {
        MiddlePointIndex { ia: a, ib: b }
    } else {
        MiddlePointIndex { ia: b, ib: a }
    };
    if let Some(indice) = middle_point_cache.get(&index) {
        return *indice;
    }

    let v1 = vertices.get(a as usize).unwrap();
    let v2 = vertices.get(b as usize).unwrap();
    let mut middle = Vec3::new(
        (v1.x + v2.x) / 2.0,
        (v1.y + v2.y) / 2.0,
        (v1.z + v2.z) / 2.0,
    );
    middle = middle.normalize() * r;

    vertices.push(middle);
    let c = (vertices.len() - 1) as u32;
    middle_point_cache.insert(index, c);

    return c;
}

// following http://blog.andreaskahler.com/2009/06/creating-icosphere-mesh-in-code.html
pub fn gen_icosphere(r: f32, recursion: u32) -> (Vec<Vec3>, Vec<u32>, Vec<Vec3>) {
    let (mut vertices, mut indices) = gen_icosahedron(r);
    let mut normals = Vec::new();

    let mut middle_point_cache = std::collections::HashMap::new();

    for _ in 0..recursion {
        let mut indices2 = Vec::new();
        for index in 0..(indices.len() / 3) {
            let i = index * 3;
            let a = find_middle_point(
                r,
                indices[i],
                indices[i + 1],
                &mut middle_point_cache,
                &mut vertices,
            );
            let b = find_middle_point(
                r,
                indices[i + 1],
                indices[i + 2],
                &mut middle_point_cache,
                &mut vertices,
            );
            let c = find_middle_point(
                r,
                indices[i + 2],
                indices[i],
                &mut middle_point_cache,
                &mut vertices,
            );
            indices2.extend_from_slice(&[indices[i], a, c]);
            indices2.extend_from_slice(&[indices[i + 1], b, a]);
            indices2.extend_from_slice(&[indices[i + 2], c, b]);
            indices2.extend_from_slice(&[a, b, c]);
        }
        indices = indices2;
    }

    for vertex in &vertices {
        normals.push(Vec3::from(vertex.normalize()));
    }

    (vertices, indices, normals)
}
