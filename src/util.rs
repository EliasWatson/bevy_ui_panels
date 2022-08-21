use bevy::{ecs::query::WorldQuery, prelude::*};

pub fn climb_parents<Q, F>(
    parent_query: &Query<&Parent>,
    target_query: &Query<Q, F>,
    entity: Entity,
) -> Option<Entity>
where
    Q: WorldQuery,
    F: WorldQuery,
{
    let mut current_entity = entity;
    loop {
        current_entity = parent_query.get(current_entity).ok()?.get();

        if target_query.get(current_entity).is_ok() {
            return Some(current_entity);
        }
    }
}
