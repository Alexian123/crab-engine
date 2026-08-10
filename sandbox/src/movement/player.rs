use crate::{Entity, InputManager, KeyCode, LocalTransformComponent, World};
use engine::scene::terrain;

pub struct PlayerController {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub turn_speed: f32,
    pub jump_force: f32,
    pub gravity_factor: f32,
    pub height_offset: f32,
    pub active: bool,
    current_upwards_speed: f32,
    in_air: bool,
}

impl PlayerController {
    pub fn new(
        walk_speed: f32,
        run_speed: f32,
        turn_speed: f32,
        jump_force: f32,
        gravity_factor: f32,
        height_offset: f32,
    ) -> Self {
        Self {
            walk_speed,
            run_speed,
            turn_speed,
            jump_force,
            gravity_factor,
            height_offset,
            active: false,
            current_upwards_speed: 0.0,
            in_air: false,
        }
    }

    pub fn update(&mut self, entity: Entity, dt: f32, input: &InputManager, world: &mut World) {
        if !self.active {
            return;
        }

        if let Some(local_transform) = world.get_component_mut::<LocalTransformComponent>(entity) {
            let speed = if input.is_key_down(KeyCode::ShiftLeft) {
                dt * self.run_speed
            } else {
                dt * self.walk_speed
            };

            let turn_speed = self.turn_speed * dt;
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

            if input.is_key_pressed(KeyCode::Space) {
                self.jump();
            }

            self.current_upwards_speed += self.gravity_factor * dt;
            local_transform.position.y += self.current_upwards_speed * dt;
        }

        let position = world
            .get_component::<LocalTransformComponent>(entity)
            .map(|transform| transform.position);

        if let Some(position) = position {
            let terrain_height =
                terrain::get_height_at_point(position.x, position.z, world) - self.height_offset;

            if let Some(transform) = world.get_component_mut::<LocalTransformComponent>(entity) {
                if transform.position.y < terrain_height {
                    transform.position.y = terrain_height;
                    self.in_air = false;
                    self.current_upwards_speed = 0.0;
                }
            }
        }
    }

    fn jump(&mut self) {
        if !self.in_air {
            self.current_upwards_speed = self.jump_force;
            self.in_air = true;
        }
    }
}
