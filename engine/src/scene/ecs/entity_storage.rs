use super::Entity;

#[derive(Default)]
pub struct EntityManager {
    generations: Vec<u32>,
    free_entities: Vec<u32>,
}

impl EntityManager {
    pub fn create_entity(&mut self) -> Entity {
        if let Some(free_id) = self.free_entities.pop() {
            Entity {
                index: free_id,
                generation: self.generations[free_id as usize],
            }
        } else {
            self.generations.push(0);
            let index = self.generations.len() as u32 - 1;
            Entity {
                index,
                generation: 0,
            }
        }
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        assert!(self.is_alive(entity));
        self.generations[entity.index as usize] += 1;
        self.free_entities.push(entity.index);
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.generations
            .get(entity.index as usize)
            .is_some_and(|&g| g == entity.generation)
    }
}
