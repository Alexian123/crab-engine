use super::Component;
use super::Entity;
use std::any::{Any, TypeId};
use std::collections::HashMap;

trait Storage: Any {
    fn contains(&self, entity: Entity) -> bool;
    fn remove(&mut self, entity: &Entity);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct ComponentStore<T: Component> {
    components: HashMap<Entity, T>,
}

impl<T: Component> Default for ComponentStore<T> {
    fn default() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
}

impl<T: Component> ComponentStore<T> {
    pub fn add(&mut self, entity: Entity, component: T) {
        if self.components.insert(entity, component).is_some() {
            panic!(
                "Entity {:?} already has component {}",
                entity,
                std::any::type_name::<T>(),
            );
        }
    }

    pub fn get(&self, entity: &Entity) -> Option<&T> {
        self.components.get(entity)
    }

    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        self.components.get_mut(entity)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Entity, &T)> {
        self.components.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Entity, &mut T)> {
        self.components.iter_mut()
    }
}

impl<T: Component> Storage for ComponentStore<T> {
    fn contains(&self, entity: Entity) -> bool {
        self.components.contains_key(&entity)
    }

    fn remove(&mut self, entity: &Entity) {
        self.components.remove(entity);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Default)]
pub struct ComponentManager {
    storages: HashMap<TypeId, Box<dyn Storage>>,
}

impl ComponentManager {
    pub fn iter<T: Component>(&self) -> impl Iterator<Item = (&Entity, &T)> {
        self.component_store::<T>()
            .into_iter()
            .flat_map(|store| store.iter())
    }

    pub fn iter_mut<T: Component>(&mut self) -> impl Iterator<Item = (&Entity, &mut T)> {
        self.component_store_mut::<T>()
            .into_iter()
            .flat_map(|store| store.iter_mut())
    }

    pub fn iter2<T: Component, U: Component>(&self) -> impl Iterator<Item = (&Entity, &T, &U)> {
        let t_store = self.component_store::<T>();
        let u_store = self.component_store::<U>();

        t_store.into_iter().flat_map(move |ts| {
            ts.iter().filter_map(move |(entity, t)| {
                u_store
                    .and_then(|us| us.get(entity))
                    .map(|u| (entity, t, u))
            })
        })
    }

    pub fn has_component<T: Component>(&self, entity: Entity) -> bool {
        self.storage::<T>().is_some_and(|s| s.contains(entity))
    }

    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        self.component_store_mut_or_create::<T>()
            .add(entity, component);
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        self.component_store::<T>()?.get(&entity)
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        self.component_store_mut::<T>()?.get_mut(&entity)
    }

    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        if let Some(storage) = self.component_store_mut::<T>() {
            storage.remove(&entity);
        }
    }

    pub fn remove_all_components(&mut self, entity: Entity) {
        for storage in self.storages.values_mut() {
            storage.remove(&entity);
        }
    }

    fn storage<T: Component>(&self) -> Option<&dyn Storage> {
        self.storages.get(&TypeId::of::<T>()).map(Box::as_ref)
    }

    fn storage_mut<T: Component>(&mut self) -> Option<&mut dyn Storage> {
        self.storages.get_mut(&TypeId::of::<T>()).map(Box::as_mut)
    }

    fn storage_mut_or_create<T: Component>(&mut self) -> &mut dyn Storage {
        self.storages
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(ComponentStore::<T>::default()))
            .as_mut()
    }

    fn component_store<T: Component>(&self) -> Option<&ComponentStore<T>> {
        self.storage::<T>()?.as_any().downcast_ref()
    }

    fn component_store_mut<T: Component>(&mut self) -> Option<&mut ComponentStore<T>> {
        self.storage_mut::<T>()?.as_any_mut().downcast_mut()
    }

    fn component_store_mut_or_create<T: Component>(&mut self) -> &mut ComponentStore<T> {
        self.storage_mut_or_create::<T>()
            .as_any_mut()
            .downcast_mut::<ComponentStore<T>>()
            .expect("Storage TypeId does not match ComponentStore<T>")
    }
}
