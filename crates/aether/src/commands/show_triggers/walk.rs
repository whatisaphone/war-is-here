#![allow(clippy::module_name_repetitions)]

use crate::darksiders1::gfc;
use std::convert::TryInto;

pub fn walk_world(visitor: &mut dyn FnMut(&gfc::WorldObject)) {
    let world = match gfc::OblivionGame::get_instance().get_world() {
        Some(world) => world,
        None => return,
    };

    walk_group(&world.root(), visitor);

    for (r, _) in world.region_data().iter().enumerate() {
        let region = match world.get_region(r.try_into().unwrap()) {
            Some(x) => x,
            None => continue,
        };

        for (l, _) in region.layer_data().iter().enumerate() {
            let layer = match region.get_layer(l.try_into().unwrap()) {
                Some(x) => x,
                None => continue,
            };

            walk_group(&layer.root(), visitor);
        }

        for object in region.load_regions().iter() {
            visitor(object);
        }
    }
}

fn walk_group(group: &gfc::WorldGroup, visitor: &mut dyn FnMut(&gfc::WorldObject)) {
    for object in group.objects() {
        visitor(object);

        if let Some(group) = gfc::object_safecast::<gfc::WorldGroup>(object) {
            walk_group(&group, visitor);
        }
    }
}
