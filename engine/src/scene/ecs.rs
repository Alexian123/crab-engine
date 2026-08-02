mod component_storage;
mod entity_storage;

use component_storage::ComponentManager;
use entity_storage::EntityManager;
use std::any::Any;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    index: u32,
    generation: u32,
}

pub trait Component: Any {}

#[derive(Default)]
pub struct World {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
}

impl World {
    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity()
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        assert!(self.entity_manager.is_alive(entity));
        self.component_manager.remove_all_components(entity);
        self.entity_manager.remove_entity(entity);
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entity_manager.is_alive(entity)
    }

    pub fn has_component<T: Component>(&self, entity: Entity) -> bool {
        assert!(self.entity_manager.is_alive(entity));
        self.component_manager.has_component::<T>(entity)
    }

    pub fn set_component<T: Component>(&mut self, entity: Entity, component: T) {
        assert!(self.entity_manager.is_alive(entity));
        self.component_manager.set_component::<T>(entity, component);
    }

    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        assert!(self.entity_manager.is_alive(entity));
        self.component_manager.add_component::<T>(entity, component);
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        assert!(self.entity_manager.is_alive(entity));
        self.component_manager.get_component::<T>(entity)
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        assert!(self.entity_manager.is_alive(entity));
        self.component_manager.get_component_mut::<T>(entity)
    }

    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        assert!(self.entity_manager.is_alive(entity));
        self.component_manager.remove_component::<T>(entity);
    }

    pub fn remove_all_components(&mut self, entity: Entity) {
        assert!(self.entity_manager.is_alive(entity));
        self.component_manager.remove_all_components(entity);
    }

    pub fn query<T: Component>(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.component_manager.iter::<T>()
    }

    pub fn query_mut<T: Component>(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.component_manager.iter_mut::<T>()
    }

    pub fn query2<T: Component, U: Component>(&self) -> impl Iterator<Item = (Entity, &T, &U)> {
        self.component_manager.iter2::<T, U>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Transform {
        x: f32,
    }
    impl Component for Transform {}

    #[derive(Debug, PartialEq)]
    struct Velocity {
        x: f32,
    }
    impl Component for Velocity {}

    #[test]
    fn create_entity() {
        let mut world = World::default();

        let e1 = world.create_entity();
        let e2 = world.create_entity();

        assert_ne!(e1, e2);
        assert!(world.is_alive(e1));
        assert!(world.is_alive(e2));
    }

    #[test]
    fn remove_entity() {
        let mut world = World::default();

        let entity = world.create_entity();

        world.remove_entity(entity);

        assert!(!world.is_alive(entity));
    }

    #[test]
    fn recycled_entity_has_new_generation() {
        let mut world = World::default();

        let e1 = world.create_entity();

        world.remove_entity(e1);

        let e2 = world.create_entity();

        assert_eq!(e1.index, e2.index);
        assert_ne!(e1.generation, e2.generation);

        assert!(!world.is_alive(e1));
        assert!(world.is_alive(e2));
    }

    #[test]
    fn add_component() {
        let mut world = World::default();

        let entity = world.create_entity();

        world.add_component(entity, Transform { x: 5.0 });

        assert!(world.has_component::<Transform>(entity));
    }

    #[test]
    fn get_component() {
        let mut world = World::default();

        let entity = world.create_entity();

        world.add_component(entity, Transform { x: 7.0 });

        let transform = world.get_component::<Transform>(entity).unwrap();

        assert_eq!(transform.x, 7.0);
    }

    #[test]
    fn get_component_mut() {
        let mut world = World::default();

        let entity = world.create_entity();

        world.add_component(entity, Transform { x: 1.0 });

        world.get_component_mut::<Transform>(entity).unwrap().x = 42.0;

        assert_eq!(world.get_component::<Transform>(entity).unwrap().x, 42.0);
    }

    #[test]
    fn remove_component() {
        let mut world = World::default();

        let entity = world.create_entity();

        world.add_component(entity, Transform { x: 5.0 });

        world.remove_component::<Transform>(entity);

        assert!(!world.has_component::<Transform>(entity));
    }

    #[test]
    fn remove_entity_removes_components() {
        let mut world = World::default();

        let entity = world.create_entity();

        world.add_component(entity, Transform { x: 1.0 });
        world.add_component(entity, Velocity { x: 2.0 });

        world.remove_entity(entity);

        let entity2 = world.create_entity();

        assert!(!world.has_component::<Transform>(entity2));
        assert!(!world.has_component::<Velocity>(entity2));
    }

    #[test]
    fn query_single_component() {
        let mut world = World::default();

        let e1 = world.create_entity();
        let e2 = world.create_entity();

        world.add_component(e1, Transform { x: 1.0 });
        world.add_component(e2, Transform { x: 2.0 });

        let values: Vec<f32> = world.query::<Transform>().map(|(_, t)| t.x).collect();

        assert_eq!(values.len(), 2);
        assert!(values.contains(&1.0));
        assert!(values.contains(&2.0));
    }

    #[test]
    fn query2_returns_only_matching_entities() {
        let mut world = World::default();

        let e1 = world.create_entity();
        let e2 = world.create_entity();
        let e3 = world.create_entity();

        world.add_component(e1, Transform { x: 1.0 });

        world.add_component(e2, Transform { x: 2.0 });
        world.add_component(e2, Velocity { x: 20.0 });

        world.add_component(e3, Velocity { x: 30.0 });

        let result: Vec<_> = world.query2::<Transform, Velocity>().collect();

        assert_eq!(result.len(), 1);

        let (entity, transform, velocity) = result[0];

        assert_eq!(entity, e2);
        assert_eq!(transform.x, 2.0);
        assert_eq!(velocity.x, 20.0);
    }

    #[test]
    fn query_mut_can_modify_components() {
        let mut world = World::default();

        let entity = world.create_entity();

        world.add_component(entity, Transform { x: 5.0 });

        for (_, transform) in world.query_mut::<Transform>() {
            transform.x += 10.0;
        }

        assert_eq!(world.get_component::<Transform>(entity).unwrap().x, 15.0);
    }

    #[test]
    #[should_panic]
    fn duplicate_component_panics() {
        let mut world = World::default();

        let entity = world.create_entity();

        world.add_component(entity, Transform { x: 1.0 });
        world.add_component(entity, Transform { x: 2.0 });
    }

    #[test]
    #[should_panic]
    fn using_dead_entity_panics() {
        let mut world = World::default();

        let entity = world.create_entity();

        world.remove_entity(entity);

        world.has_component::<Transform>(entity);
    }
}
