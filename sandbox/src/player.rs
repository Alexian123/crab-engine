use crate::{Entity, InputManager, KeyCode, LocalTransformComponent, World};

pub struct PlayerController {
    pub walk_speed: f32,
    pub run_speed: f32,
}

impl PlayerController {
    pub fn new(walk_speed: f32, run_speed: f32) -> Self {
        Self {
            walk_speed,
            run_speed,
        }
    }

    pub fn update(&mut self, entity: Entity, dt: f32, input: &InputManager, world: &mut World) {
        if let Some(local_transform) = world.get_component_mut::<LocalTransformComponent>(entity) {
            let speed = if input.is_key_down(KeyCode::ShiftLeft) {
                dt * self.run_speed
            } else {
                dt * self.walk_speed
            };

            let turn_speed = 3.0 * dt;
            if input.is_key_down(KeyCode::KeyQ) {
                local_transform.rotation =
                    glam::Quat::from_rotation_y(turn_speed) * local_transform.rotation;
            }
            if input.is_key_down(KeyCode::KeyE) {
                local_transform.rotation =
                    glam::Quat::from_rotation_y(-turn_speed) * local_transform.rotation;
            }

            let forward = local_transform.rotation * glam::Vec3::Z;
            let right = local_transform.rotation * glam::Vec3::X;

            let forward = glam::Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
            let right = glam::Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

            let mut movement = glam::Vec3::ZERO;
            if input.is_key_down(KeyCode::KeyW) {
                movement += forward;
            }
            if input.is_key_down(KeyCode::KeyS) {
                movement -= forward;
            }
            if input.is_key_down(KeyCode::KeyD) {
                movement -= right;
            }
            if input.is_key_down(KeyCode::KeyA) {
                movement += right;
            }

            local_transform.position += movement.normalize_or_zero() * speed;

            if input.is_key_down(KeyCode::Space) {
                local_transform.position.y += speed;
            }
            if input.is_key_down(KeyCode::ControlLeft) {
                local_transform.position.y -= speed;
            }
        }
    }
}
