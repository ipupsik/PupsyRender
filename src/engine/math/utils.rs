use glam::{Vec2, Vec3A};
use rand::{Rng};

fn random(min : f32, max : f32) -> Vec3A {
    Vec3A::new(rand::thread_rng().gen_range(min..max), 
        rand::thread_rng().gen_range(min..max), 
        rand::thread_rng().gen_range(min..max))
}

pub fn random_in_unit_sphere() -> Vec3A {
    let r1: f32 = rand::thread_rng().gen_range(0.0..1.0);
    let r2: f32 = rand::thread_rng().gen_range(0.0..1.0);

    let phi = 2.0 * std::f32::consts::PI * r1;
    let x = phi.cos() * (r2 * (1.0 - r2)).sqrt();
    let y = phi.sin() * (r2 * (1.0 - r2)).sqrt();
    let z = 1.0 - 2.0 * r2;

    Vec3A::new(x, y, z)
}

pub fn random_cosine_direction() -> Vec3A {
    let r1: f32 = rand::thread_rng().gen_range(0.0..1.0);
    let r2: f32 = rand::thread_rng().gen_range(0.0..1.0);
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * std::f32::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3A::new(x, y, z)
}

pub fn decode_triangle_vec3_indexed(
    buffer : &Vec<u8>, offset: usize, stride: usize, raw_size: usize,
    indices_buffer : &Vec<u8>, indices_offset : usize, indices_stride: usize,  indices_raw_size: usize
) -> [Vec3A; 3] {
    let index1 = decode_int(indices_buffer, indices_offset, indices_raw_size) as usize;
    let index2 = decode_int(indices_buffer, indices_offset + indices_stride, indices_raw_size) as usize;
    let index3 = decode_int(indices_buffer, indices_offset + indices_stride * 2, indices_raw_size) as usize;

    let pos1 = decode_vec3(buffer, offset + index1 * stride, raw_size);
    let pos2 = decode_vec3(buffer, offset + index2 * stride, raw_size);
    let pos3 = decode_vec3(buffer, offset + index3 * stride, raw_size);

    [pos1, pos2, pos3]
}

pub fn decode_triangle_vec3(buffer : &Vec<u8>, offset : usize, stride : usize, 
    raw_size : usize) -> [Vec3A; 3] {
    let pos1 = decode_vec3(buffer, offset, raw_size);
    let pos2 = decode_vec3(buffer, offset + stride, raw_size);
    let pos3 = decode_vec3(buffer, offset + stride * 2, raw_size);

    [pos1, pos2, pos3]
}

pub fn decode_triangle_vec2_indexed(
    buffer : &Vec<u8>, offset: usize, stride: usize, raw_size: usize,
    indices_buffer : &Vec<u8>, indices_offset : usize, indices_stride: usize,  indices_raw_size: usize
) -> [Vec2; 3] {
    let index1 = decode_int(indices_buffer, indices_offset, indices_raw_size) as usize;
    let index2 = decode_int(indices_buffer, indices_offset + indices_stride, indices_raw_size) as usize;
    let index3 = decode_int(indices_buffer, indices_offset + indices_stride * 2, indices_raw_size) as usize;

    let uv1 = decode_vec2(buffer, offset + index1 * stride, raw_size);
    let uv2 = decode_vec2(buffer, offset + index2 * stride, raw_size);
    let uv3 = decode_vec2(buffer, offset + index3 * stride, raw_size);

    [uv1, uv2, uv3]
}

pub fn decode_triangle_vec2(buffer : &Vec<u8>, offset : usize, stride : usize, 
    raw_size : usize) -> [Vec2; 3] {
    let uv1 = decode_vec2(buffer, offset, raw_size);
    let uv2 = decode_vec2(buffer, offset + stride, raw_size);
    let uv3 = decode_vec2(buffer, offset + stride * 2, raw_size);

    [uv1, uv2, uv3]
}

pub fn decode_vec3(buffer : &Vec<u8>, offset : usize, raw_size : usize) -> Vec3A {
    return Vec3A::new(
        f32::from_le_bytes(buffer[offset..offset + raw_size].try_into().expect("Invalid x")),
        f32::from_le_bytes(buffer[offset + raw_size..offset + raw_size * 2].try_into().expect("Invalid y")),
        f32::from_le_bytes(buffer[offset + raw_size * 2..offset + raw_size * 3].try_into().expect("Invalid z"))
    );
}

pub fn decode_vec2(buffer : &Vec<u8>, offset : usize, raw_size : usize) -> Vec2 {
    return Vec2::new(
        f32::from_le_bytes(buffer[offset..offset + raw_size].try_into().expect("Invalid x")),
        f32::from_le_bytes(buffer[offset + raw_size..offset + raw_size * 2].try_into().expect("Invalid y"))
    );
}

pub fn decode_scalar(buffer : &Vec<u8>, offset : usize, raw_size : usize) -> f32 {
    return f32::from_le_bytes(buffer[offset..offset + raw_size].try_into().expect("Invalid scalar"));
}

pub fn decode_int(buffer : &Vec<u8>, offset : usize, raw_size : usize) -> u32 {
    if raw_size == 1 {
        return u8::from_le_bytes(buffer[offset..offset + raw_size].try_into().expect("Invalid index")) as u32;
    } else if raw_size == 2 {
        return u16::from_le_bytes(buffer[offset..offset + raw_size].try_into().expect("Invalid index")) as u32;
    } else if raw_size == 4 {
        return u32::from_le_bytes(buffer[offset..offset + raw_size].try_into().expect("Invalid index"));
    }
    return 0;
}