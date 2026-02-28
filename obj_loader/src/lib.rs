use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

use math::geometry::triangle::{Face3, Triangle3Mesh};
use math::{Normal3, Point2, Point3, Vector3};

use traits::{One, Sqrt, Zero};

use crate::face::ParsedFace3;

pub mod face;
pub mod index;
pub mod normal;
pub mod texture;
pub mod vertex;

pub enum WindingOrder {
    ClockWise,
    CounterClockWise,
}

pub enum NormalType {
    Orthogonal,
    Smoothed,
}

pub enum NormalCreation {
    None,
    Config(WindingOrder, NormalType),
}

fn generate_normals<T>(
    winding: WindingOrder,
    vertices: &Vec<Point3<T>>,
    faces: Vec<ParsedFace3>,
) -> (Vec<Normal3<<T as Div>::Output>>, Vec<ParsedFace3>)
where
    T: Div + Mul + Sub<Output = T> + Copy,
    <T as Div>::Output: Add<Output = <T as Div>::Output>
        + Div<Output = <T as Div>::Output>
        + Mul<Output = <T as Div>::Output>
        + Sqrt<Output = <T as Div>::Output>
        + Copy
        + Zero,
    <T as Mul>::Output:
        Div<Output = <T as Div>::Output> + Sub<Output = <T as Mul>::Output> + One + Copy,
{
    use math::Vector3;

    let mut normals = Vec::new();
    let mut updated_faces = Vec::new();

    for face in faces {
        let idx_a = face.a as usize;
        let idx_b = face.b as usize;
        let idx_c = face.c as usize;

        let v1 = vertices[idx_a];
        let v2 = vertices[idx_b];
        let v3 = vertices[idx_c];

        let va = v2 - v1; // Vector3<T>
        let vb = v3 - v1; // Vector3<T>

        let normal_vec = match winding {
            WindingOrder::CounterClockWise => Vector3::cross(va, vb),
            WindingOrder::ClockWise => Vector3::cross(vb, va),
        } / One::one();

        let normal_vec_norm = normal_vec.normalized(); // Vector3<T>
        let normal = normal_vec_norm.as_normal(); // Normal3<T>

        normals.push(normal);

        let new_face = ParsedFace3 {
            a: face.a,
            b: face.b,
            c: face.c,
            na: Some((normals.len() - 1) as i128),
            nb: Some((normals.len() - 1) as i128),
            nc: Some((normals.len() - 1) as i128),
            uva: face.uva,
            uvb: face.uvb,
            uvc: face.uvc,
        };

        updated_faces.push(new_face);
    }

    (normals, updated_faces)
}

pub fn smooth_normals<T>(
    faces: Vec<ParsedFace3>,
    normals: Vec<Normal3<T>>,
) -> (Vec<ParsedFace3>, Vec<Normal3<T>>)
where
    T: Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Zero
        + One
        + Sqrt<Output = T>
        + Copy
        + Clone
        + PartialEq,
{
    if faces.is_empty() {
        return (faces, normals);
    }

    // Find max vertex index
    let max_vertex = faces
        .iter()
        .map(|f| f.a.max(f.b).max(f.c))
        .max()
        .unwrap_or(0) as usize;

    let mut vertex_normal_sums: Vec<Vector3<<T as Div>::Output>> =
        vec![Vector3::new(Zero::zero(), Zero::zero(), Zero::zero()); max_vertex + 1];
    let mut vertex_counts: Vec<<T as Div>::Output> = vec![Zero::zero(); max_vertex + 1];

    // Accumulate normal vectors for each vertex
    for face in &faces {
        if let (Some(na), Some(nb), Some(nc)) = (face.na, face.nb, face.nc) {
            let na_idx = na as usize;
            let nb_idx = nb as usize;
            let nc_idx = nc as usize;

            if na_idx < normals.len() && nb_idx < normals.len() && nc_idx < normals.len() {
                let normal_a_vec = normals[na_idx].as_vector();
                let normal_b_vec = normals[nb_idx].as_vector();
                let normal_c_vec = normals[nc_idx].as_vector();

                vertex_normal_sums[face.a as usize] =
                    vertex_normal_sums[face.a as usize] + normal_a_vec;
                vertex_normal_sums[face.b as usize] =
                    vertex_normal_sums[face.b as usize] + normal_b_vec;
                vertex_normal_sums[face.c as usize] =
                    vertex_normal_sums[face.c as usize] + normal_c_vec;

                vertex_counts[face.a as usize] = vertex_counts[face.a as usize] + One::one();
                vertex_counts[face.b as usize] = vertex_counts[face.b as usize] + One::one();
                vertex_counts[face.c as usize] = vertex_counts[face.c as usize] + One::one();
            }
        }
    }

    // Compute smoothed normals
    let mut smoothed_normals = Vec::new();
    for (sum_vec, count) in vertex_normal_sums.into_iter().zip(vertex_counts) {
        if count != Zero::zero() {
            let avg_vec = sum_vec / count;
            let normalized = avg_vec.normalized();
            smoothed_normals.push(normalized.as_normal());
        } else {
            // Default normal, e.g., (0,0,1)
            smoothed_normals.push(Normal3::new(Zero::zero(), Zero::zero(), One::one()));
        }
    }

    // Update faces to use vertex indices as normal indices
    let updated_faces = faces
        .into_iter()
        .map(|face| ParsedFace3 {
            a: face.a,
            b: face.b,
            c: face.c,
            na: Some(face.a),
            nb: Some(face.b),
            nc: Some(face.c),
            uva: face.uva,
            uvb: face.uvb,
            uvc: face.uvc,
        })
        .collect();

    (updated_faces, smoothed_normals)
}

pub fn parse_obj<T>(content: &str, normal_creation: NormalCreation) -> Triangle3Mesh<T>
where
    T: Div + Sub<Output = T> + Mul + FromStr + Copy + Debug,
    <T as Div>::Output: Mul<Output = <T as Div>::Output>
        + Add<Output = <T as Div>::Output>
        + Div<Output = <T as Div>::Output>
        + Zero
        + One
        + Sqrt<Output = <T as Div>::Output>
        + Copy
        + Clone
        + PartialEq
        + FromStr
        + Debug,
    <T as FromStr>::Err: Debug,
    <<T as Div>::Output as FromStr>::Err: Debug,
    <T as Mul>::Output: Add<Output = <T as Mul>::Output>
        + Div<Output = <T as Div>::Output>
        + Sub<Output = <T as Mul>::Output>
        + One
        + Sqrt<Output = T>
        + Zero
        + Copy,
{
    let mut vertices: Vec<Point3<T>> = Vec::new();
    let mut normals: Vec<Normal3<<T as Div>::Output>> = Vec::new();
    let mut uvs: Vec<Point2<<T as Div>::Output>> = Vec::new();
    let mut faces: Vec<ParsedFace3> = Vec::new();

    let lines = content.lines();

    for line in lines {
        if line.starts_with("vn") {
            let normal = normal::parse(line);
            normals.push(normal.unwrap());
        } else if line.starts_with("vt") {
            let tex = texture::parse(line);
            uvs.push(tex.unwrap());
        } else if line.starts_with("v") {
            let vertex = vertex::parse(line);
            vertices.push(vertex.unwrap());
        } else if line.starts_with("f") {
            let face = face::parse(line);
            let face = face.unwrap();

            let vertices_len: i128 = vertices.len().try_into().unwrap();
            let normals_len: i128 = normals.len().try_into().unwrap();
            let uvs_len: i128 = uvs.len().try_into().unwrap();

            let face = ParsedFace3 {
                a: if face.a > 0 {
                    face.a - 1
                } else {
                    vertices_len + face.a
                },
                b: if face.b > 0 {
                    face.b - 1
                } else {
                    vertices_len + face.b
                },
                c: if face.c > 0 {
                    face.c - 1
                } else {
                    vertices_len + face.c
                },
                na: match face.na {
                    Some(n) => Option::Some(if n > 0 { n - 1 } else { normals_len + n }),
                    None => None,
                },
                nb: match face.nb {
                    Some(n) => Option::Some(if n > 0 { n - 1 } else { normals_len + n }),
                    None => None,
                },
                nc: match face.nc {
                    Some(n) => Option::Some(if n > 0 { n - 1 } else { normals_len + n }),
                    None => None,
                },
                uva: match face.uva {
                    Some(uv) => Option::Some(if uv > 0 { uv - 1 } else { uvs_len + uv }),
                    None => None,
                },
                uvb: match face.uvb {
                    Some(uv) => Option::Some(if uv > 0 { uv - 1 } else { uvs_len + uv }),
                    None => None,
                },
                uvc: match face.uvc {
                    Some(uv) => Option::Some(if uv > 0 { uv - 1 } else { uvs_len + uv }),
                    None => None,
                },
            };
            faces.push(face);
        }
    }

    if normals.is_empty() {
        match normal_creation {
            NormalCreation::Config(winding_order, normal_type) => {
                (normals, faces) = generate_normals(winding_order, &vertices, faces);
                match normal_type {
                    NormalType::Smoothed => (faces, normals) = smooth_normals(faces, normals),
                    NormalType::Orthogonal => {}
                }
            }
            NormalCreation::None => {}
        }
    }

    if uvs.is_empty() {
        uvs = vec![Point2::new(Zero::zero(), Zero::zero())];
        faces = faces
            .iter()
            .map(|f| ParsedFace3 {
                a: f.a,
                b: f.b,
                c: f.c,
                na: f.na,
                nb: f.nb,
                nc: f.nc,
                uva: Some(0),
                uvb: Some(0),
                uvc: Some(0),
            })
            .collect();
    }

    // Generate Missing UVs if required

    let mesh = Triangle3Mesh::new(
        vertices,
        normals,
        uvs,
        faces
            .iter()
            .map(|f| {
                Face3::new(
                    f.a.try_into().unwrap(),
                    f.b.try_into().unwrap(),
                    f.c.try_into().unwrap(),
                    f.na.unwrap().try_into().unwrap(),
                    f.nb.unwrap().try_into().unwrap(),
                    f.nc.unwrap().try_into().unwrap(),
                    f.uva.unwrap().try_into().unwrap(),
                    f.uvb.unwrap().try_into().unwrap(),
                    f.uvc.unwrap().try_into().unwrap(),
                )
            })
            .collect(),
    );

    mesh
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_triangle_f32() {
        let model = "v 0.0 0.0 0.0\n\
                           v 0.0 1.0 0.0\n\
                           v 1.0 0.0 0.0\n\
                           vn 0.0 0.0 1.0\n\
                           vt 0.0 0.0\n\
                           vt 0.0 1.0\n\
                           vt 1.0 0.0\n\
                           f 1/1/1 2/2/1 3/3/1";

        let mesh: Triangle3Mesh<f32> = parse_obj(model, NormalCreation::None);

        let vertices = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ];
        let normals = vec![Normal3::new(0.0, 0.0, 1.0)];
        let uvs = vec![
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 1.0),
            Point2::new(1.0, 0.0),
        ];
        let faces = vec![Face3::new(0, 1, 2, 0, 0, 0, 0, 1, 2)];

        assert_eq!(vertices, mesh.vertices);
        assert_eq!(normals, mesh.normals);
        assert_eq!(uvs, mesh.uvs);
        assert_eq!(faces, mesh.faces);
    }

    #[test]
    fn test_parse_single_triangle_negative_index_f32() {
        let model = "v 0.0 0.0 0.0\n\
                           v 0.0 1.0 0.0\n\
                           v 1.0 0.0 0.0\n\
                           vn 0.0 0.0 1.0\n\
                           vt 0.0 0.0\n\
                           vt 0.0 1.0\n\
                           vt 1.0 0.0\n\
                           f -3/-3/-1 -2/-2/-1 -1/-1/-1";

        let mesh: Triangle3Mesh<f32> = parse_obj(model, NormalCreation::None);

        let vertices = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ];
        let normals = vec![Normal3::new(0.0, 0.0, 1.0)];
        let uvs = vec![
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 1.0),
            Point2::new(1.0, 0.0),
        ];
        let faces = vec![Face3::new(0, 1, 2, 0, 0, 0, 0, 1, 2)];

        assert_eq!(vertices, mesh.vertices);
        assert_eq!(normals, mesh.normals);
        assert_eq!(uvs, mesh.uvs);
        assert_eq!(faces, mesh.faces);
    }
}
