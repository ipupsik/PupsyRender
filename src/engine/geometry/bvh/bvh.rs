use crate::engine::geometry::bvh::bin::Bin;
use crate::engine::geometry::triangle::Triangle;
use crate::engine::material::diffuse::DiffuseMaterial;
use crate::engine::profile::{Profile, ProfileType};
use crate::engine::{math::ray::*, material::Material};
use crate::engine::geometry::traceable::*;
use glam::{Vec3A};
use std::arch::x86_64::_MM_ROUND_TOWARD_ZERO;
use std::sync::*;
use super::aabb::*;
use super::node::*;
use rand::{Rng};

pub struct BVH {
    pub primitives: Arc<Vec<Arc<dyn Traceable>>>,

    pub primitive_indices: Vec<usize>,
    pub nodes: Vec<Node>,
    pub root_node_index: usize,
    pub nodes_used: usize
}

impl BVH {
    pub fn new(objects: Arc<Vec<Arc<dyn Traceable>>>) -> Self {
        let mut bvh = Self {
            primitives: objects.clone(),
            primitive_indices: vec![0; objects.len()],
            nodes: vec![Node::new(); objects.len() * 2 + 1],
            root_node_index: 0,
            nodes_used: 1
        };

        for (index, primitive) in bvh.primitive_indices.iter_mut().enumerate() {
            *primitive = index;
        }

        if objects.len() > 0 {
            let root = &mut bvh.nodes[bvh.root_node_index];
            root.left_node_or_primitive_index = 0;
            root.primitives_count = objects.len();

            bvh.update_node_bounds(bvh.root_node_index);
            bvh.subdivide(bvh.root_node_index);
        }

        bvh
    }

    fn primitive(&self, index: usize) -> &Arc<dyn Traceable> {
        &self.primitives[self.primitive_indices[index]]
    }

    fn update_node_bounds(&mut self, node_index: usize) {
        let node = &mut self.nodes[node_index];
        node.aabb = AABB::new(Vec3A::MAX, Vec3A::MIN);
        for i in 0..node.primitives_count {
            let primitive_index = node.left_node_or_primitive_index + i;
            let primitive = &self.primitives[self.primitive_indices[primitive_index]];
            let primitive_aabb = primitive.bounding_box();

            node.aabb = node.aabb.extend(primitive_aabb);
        }
    }

    fn subdivide(&mut self, node_index: usize) {
        let (best_axis, best_pos, best_cost) = self.calculate_best_split_plane(&self.nodes[node_index]);

        let parent_cost = Self::calculate_node_cost(&self.nodes[node_index]);
        if parent_cost <= best_cost {
            return;
        }

        let axis = best_axis;
        let split = best_pos;


        // Split primitive array into two parts
        let mut i = self.nodes[node_index].left_node_or_primitive_index;
        let mut j = i + self.nodes[node_index].primitives_count - 1;
        while i <= j {
            if self.primitives[self.primitive_indices[i]].centroid()[axis] < split {
                i += 1;
            }
            else {
                // swap i and j primitives
                if j > 0 {
                    let t = self.primitive_indices[i];
                    self.primitive_indices[i] = self.primitive_indices[j];
                    self.primitive_indices[j] = t;
                    j -= 1;
                }
            }
        }

        let left_primitive_count = i - self.nodes[node_index].left_node_or_primitive_index;
        if left_primitive_count == 0 || left_primitive_count == self.nodes[node_index].primitives_count {
            return;
        }

        let left_child_index = self.nodes_used;
        let right_child_index = left_child_index + 1;
        self.nodes_used += 2;

        self.nodes[left_child_index].left_node_or_primitive_index = self.nodes[node_index].left_node_or_primitive_index;
        self.nodes[left_child_index].primitives_count = left_primitive_count;

        self.nodes[right_child_index].left_node_or_primitive_index = i;
        self.nodes[right_child_index].primitives_count = self.nodes[node_index].primitives_count - left_primitive_count;

        self.nodes[node_index].left_node_or_primitive_index = left_child_index;
        self.nodes[node_index].primitives_count = 0;

        self.update_node_bounds(left_child_index);
        self.update_node_bounds(right_child_index);

        self.subdivide(left_child_index);
        self.subdivide(right_child_index);
    }

    fn calculate_best_split_plane(&self, node: &Node) -> (usize, f32, f32) {
        // determine split axis using SAH
        let mut best_axis = 0;
        let mut best_pos = 0.0;
        let mut best_cost = f32::MAX;

        for axis in 0..3 
        {
            let mut min = node.aabb.min[axis];
            let mut max = node.aabb.max[axis];

            for i in 0..node.primitives_count
            {
                let primitive_index = node.left_node_or_primitive_index + i;
                let primitive = &self.primitives[self.primitive_indices[primitive_index]];

                min = min.min(primitive.centroid()[axis]);
                max = max.max(primitive.centroid()[axis]);
            }

            const BINS: usize = 100;
            let mut bins = [Bin::new(); BINS];
            let scale = (max - min) / BINS as f32;

            for i in 0..node.primitives_count
            {
                let primitive_index = node.left_node_or_primitive_index + i;
                let primitive = &self.primitives[self.primitive_indices[primitive_index]];

                let mut bin_index = ((primitive.centroid()[axis] - min) / scale) as usize;
                bin_index = bin_index.min(BINS - 1);

                bins[bin_index].primitive_count += 1;
                bins[bin_index].aabb = bins[bin_index].aabb.extend(primitive.bounding_box());
            }

            let mut left_area = [0.0; BINS - 1];
            let mut right_area = [0.0; BINS - 1];
            let mut left_primitives = [0; BINS - 1];
            let mut right_primitives = [0; BINS - 1];

            // determine triangle counts and bounds for this split candidate
            let mut left_aabb = AABB::new(Vec3A::MAX, Vec3A::MIN);
            let mut right_aabb = AABB::new(Vec3A::MAX, Vec3A::MIN);
            let mut left_count = 0;
            let mut right_count = 0;
            for i in 0..BINS - 1
            {
                left_count += bins[i].primitive_count;
                left_aabb = left_aabb.extend(&bins[i].aabb);
                left_primitives[i] = left_count;
                left_area[i] = left_aabb.area();

                right_count += bins[BINS - 2 - i].primitive_count;
                right_aabb = right_aabb.extend(&bins[BINS - 2 - i].aabb);
                right_primitives[BINS - 2 - i] = right_count;
                right_area[BINS - 2 - i] = right_aabb.area();
            }

            for i in 0..BINS - 1 {
                let candidate_pos = min + scale * i as f32;
                let cost = left_primitives[i] as f32 * left_area[i] +
                    right_primitives[i] as f32 * right_area[i];
                if cost < best_cost {
                    best_pos = candidate_pos;
                    best_axis = axis;
                    best_cost = cost;
                }
            }
        }

        (best_axis, best_pos, best_cost)
    }

    fn calculate_node_cost(node: &Node) -> f32
    {
        return node.primitives_count as f32 * node.aabb.area();
    }

    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, node_index: usize) -> (Option<HitResult>, &dyn Traceable) {
        let mut stack: [*const Node; 64] = [std::ptr::null(); 64];
        let mut node: *const Node = &self.nodes[node_index];
        let mut stack_ptr = 0;

        let mut min_hit_result = HitResult{
            position : Vec3A::ZERO, 
            t : f32::MAX, 
            normal : Vec3A::ZERO, 
            binormal : Vec3A::ZERO, 
            tangent : Vec3A::ZERO, 
            uvs: Vec::new(), 
            front_face: false
        };
        let mut min_index = None;

        unsafe {
            loop {
                if (*node).is_leaf() {
                    for i in 0..(*node).primitives_count {
                        let primitive_index = (*node).left_node_or_primitive_index + i;
                        let (hit_result_option, _) = self.primitive(primitive_index).hit(ray, t_min, t_max);
                        if hit_result_option.is_some() {
                            let hit_result = hit_result_option.unwrap();
                            if hit_result.t < min_hit_result.t {
                                min_hit_result = hit_result;
                                min_index = Some(primitive_index);
                            }
                        }
                    }

                    if stack_ptr == 0 {
                        break;
                    } else {
                        stack_ptr -= 1;
                        node = stack[stack_ptr];
                    }
                } else {
                    let left_node = &self.nodes[(*node).left_node_or_primitive_index];
                    let right_node = &self.nodes[(*node).left_node_or_primitive_index + 1];

                    let left_dist = left_node.aabb.hit(ray, t_min, t_max);
                    let right_dist = right_node.aabb.hit(ray, t_min, t_max);

                    if !left_dist.is_some() && !right_dist.is_some() {
                        if stack_ptr == 0 {
                            break;
                        } else {
                            stack_ptr -= 1;
                            node = stack[stack_ptr];
                        }
                    } else {
                        node = left_node;
                        if right_dist.is_some() {
                            stack[stack_ptr] = right_node;
                            stack_ptr += 1;
                        }
                    }
                }
            }
        }

        
        if min_index.is_some() {
            return (Some(min_hit_result), self.primitive(min_index.unwrap()).as_ref());
        } else {
            return (None, self.primitive(0).as_ref());
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> (Option<HitResult>, &dyn Traceable) {
        return self.intersect(ray, t_min, t_max, self.root_node_index);
    }
}