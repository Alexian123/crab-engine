use super::{
    ChildrenComponent, Entity, LocalTransformComponent, ParentComponent, World,
    WorldTransformComponent,
};
use glam::Mat4;

pub fn set_parent(world: &mut World, child: Entity, new_parent: Entity) {
    if let Some(&ParentComponent(old_parent)) = world.get_component::<ParentComponent>(child) {
        if let Some(children) = world.get_component_mut::<ChildrenComponent>(old_parent) {
            children.0.retain(|&e| e != child);
        }
    }
    world.set_component(child, ParentComponent(new_parent));
    match world.get_component_mut::<ChildrenComponent>(new_parent) {
        Some(children) => children.0.push(child),
        None => world.add_component(new_parent, ChildrenComponent(vec![child])),
    }
}

pub fn clear_parent(world: &mut World, entity: Entity) {
    if let Some(&ParentComponent(old_parent)) = world.get_component::<ParentComponent>(entity) {
        if let Some(children) = world.get_component_mut::<ChildrenComponent>(old_parent) {
            children.0.retain(|&e| e != entity);
        }
    }
    world.remove_component::<ParentComponent>(entity);
}

pub fn update_world_transforms(world: &mut World) {
    let roots: Vec<Entity> = world
        .query::<LocalTransformComponent>()
        .filter(|(entity, _)| !world.has_component::<ParentComponent>(*entity))
        .map(|(entity, _)| entity)
        .collect();

    let mut stack: Vec<(Entity, Mat4)> = roots.into_iter().map(|e| (e, Mat4::IDENTITY)).collect();
    let mut updates: Vec<(Entity, Mat4)> = Vec::new();

    while let Some((entity, parent_world)) = stack.pop() {
        let local_matrix = world
            .get_component::<LocalTransformComponent>(entity)
            .map(LocalTransformComponent::model_matrix)
            .unwrap_or(Mat4::IDENTITY);
        let world_matrix = parent_world * local_matrix;
        updates.push((entity, world_matrix));

        if let Some(children) = world.get_component::<ChildrenComponent>(entity) {
            for &child in &children.0 {
                stack.push((child, world_matrix));
            }
        }
    }

    for (entity, matrix) in updates {
        match world.get_component_mut::<WorldTransformComponent>(entity) {
            Some(wt) => wt.model_matrix = matrix,
            None => world.add_component(
                entity,
                WorldTransformComponent {
                    model_matrix: matrix,
                },
            ),
        }
    }
}
