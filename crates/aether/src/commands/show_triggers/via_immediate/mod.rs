use crate::{
    commands::show_triggers::{
        via_immediate::{
            collect::{categorize_object, prioritize_object, Category, KeepMinCountOrMinPriority},
            draw::draw_object,
        },
        walk::walk_world,
    },
    darksiders1::gfc,
    utils::coordinate_transformer::CoordinateTransformer,
};
use imgui::im_str;

mod collect;
mod draw;

pub fn draw(ui: &imgui::Ui<'_>) {
    let player = match gfc::OblivionGame::get_instance().get_player_actor() {
        Some(player) => player,
        None => return,
    };
    let player_pos = player.get_position();

    let transformer = CoordinateTransformer::create();

    // Sort objects into multiple groups, so we can have categories of objects which
    // are always drawn.
    let mut load_regions = KeepMinCountOrMinPriority::new(1);
    let mut others = KeepMinCountOrMinPriority::new(3);

    walk_world(&mut |object| {
        let object = match gfc::object_safecast::<gfc::DetectorObject>(object) {
            Some(o) => o,
            None => return,
        };

        let category = categorize_object(&object);
        let priority = prioritize_object(&object, &player_pos);
        match category {
            Category::LoadRegion => load_regions.feed(object, priority),
            Category::Other => others.feed(object, priority),
        }
    });

    imgui_token_guard! {
        ui.push_style_var(imgui::StyleVar::WindowBorderSize(0.0));
        ui.push_style_color(imgui::StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.0]);
    }

    imgui::Window::new(im_str!("Triggerrs"))
        .position([0.0, 0.0], imgui::Condition::Always)
        .title_bar(false)
        .resizable(false)
        .build(ui, || {
            let draw_list = ui.get_background_draw_list();
            for object in load_regions.into_iter() {
                draw_object(&draw_list, &transformer, &object, &player_pos);
            }
            for object in others.into_iter() {
                draw_object(&draw_list, &transformer, &object, &player_pos);
            }
        });
}
