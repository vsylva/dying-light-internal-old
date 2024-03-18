pub(crate) mod bone;

use std::ptr::{addr_of, addr_of_mut, null};

use hudhook::imgui::ImColor32;

const RADIUS: f32 = 100.0;

use super::{COLOR_BLUE, COLOR_GREEN, COLOR_PURPLE, COLOR_RED};
use crate::{
    aim::{aim, center_distance},
    engine::{EntityType, CGAME_P, ENGINE},
    render::{
        background::bone::{draw_bone, BoneID},
        COLOR_PINK,
        COLOR_WHITE,
    },
    GetAsyncKeyState,
    Vec2,
    Vec3,
};

pub(crate) unsafe fn frame(ui: &hudhook::imgui::Ui) {
    if !ENGINE.get_ptr() {
        return;
    }

    if 0 == ENGINE
        .level_di_p
        .read()
        .level_p
        .read()
        .get_c_model_list()
        .read()
        .len
    {
        return;
    }

    (*ENGINE.player_di_p).health = 200.0;

    if ENGINE.inventory_ammo_p.read().inventory_list.item_p != std::ptr::null_mut() {
        for i in 0..ENGINE.inventory_ammo_p.read().inventory_list.len {
            (*ENGINE.inventory_ammo_p.read().inventory_list.at(i as isize)).count = 60;
        }
    }

    for i in 0..ENGINE
        .level_di_p
        .read()
        .level_p
        .read()
        .get_c_model_list()
        .read()
        .len
    {
        ENGINE.c_model_obj_p = ENGINE
            .level_di_p
            .read()
            .level_p
            .read()
            .get_c_model_list()
            .read()
            .index(i as isize);

        if !ENGINE.get_data() {
            continue;
        }

        static mut MODEL_HEAD_SCREEN_POS: Vec2 = Vec2 {
            x: 0.0,
            y: 0.0,
        };

        static mut MODEL_HEAD_WORLD_POS: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        static mut MODEL_FOOT_SCREEN_POS: Vec2 = Vec2 {
            x: 0.0,
            y: 0.0,
        };

        static mut MODEL_FOOT_WORLD_POS: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        ENGINE
            .c_model_obj_p
            .read()
            .model_obj_p
            .read()
            .get_bone_joint_pos(addr_of_mut!(MODEL_HEAD_WORLD_POS), BoneID::头 as u8);

        if !ENGINE
            .camera_manage_p
            .read()
            .camera_p
            .read()
            .world_to_screen(
                addr_of_mut!(MODEL_HEAD_SCREEN_POS),
                addr_of_mut!(MODEL_HEAD_WORLD_POS),
            )
        {
            continue;
        }

        ENGINE
            .c_model_obj_p
            .read()
            .model_obj_p
            .read()
            .get_world_position(addr_of_mut!(MODEL_FOOT_WORLD_POS));

        ENGINE
            .camera_manage_p
            .read()
            .camera_p
            .read()
            .world_to_screen(
                addr_of_mut!(MODEL_FOOT_SCREEN_POS),
                addr_of_mut!(MODEL_FOOT_WORLD_POS),
            );

        ui.get_background_draw_list().add_text(
            [MODEL_FOOT_SCREEN_POS.x, MODEL_FOOT_SCREEN_POS.y],
            COLOR_WHITE,
            format!(
                "{:.2}m",
                ENGINE
                    .c_model_obj_p
                    .read()
                    .model_obj_p
                    .read()
                    .get_distance_to(ENGINE.local_client_di_p.read().player_di_p.read().get_pos())
            ),
        );

        static mut COLOR: ImColor32 = COLOR_WHITE;

        COLOR = match ENGINE.model_type {
            EntityType::ZombieNormal => COLOR_RED,
            EntityType::ZombieVoleteile | EntityType::SurvivorZombie => COLOR_PURPLE,
            EntityType::SurvivorHuman => COLOR_BLUE,
            EntityType::PlayerHuman => COLOR_GREEN,
            EntityType::PlayerZombie => COLOR_PINK,
        };

        static mut MODEL_VISIBLE: bool = false;

        MODEL_VISIBLE = ENGINE
            .c_model_obj_p
            .read()
            .model_obj_p
            .read()
            .raytest_to_target(
                ENGINE.camera_manage_p.read().camera_p.read().get_position(),
                addr_of_mut!(MODEL_HEAD_WORLD_POS),
            );

        if MODEL_VISIBLE {
            ui.get_background_draw_list()
                .add_line(
                    [CGAME_P.read().window_width as f32 / 2.0, 0.0],
                    [MODEL_HEAD_SCREEN_POS.x, MODEL_HEAD_SCREEN_POS.y],
                    COLOR,
                )
                .build();
        }

        ui.get_background_draw_list()
            .add_circle(
                [
                    CGAME_P.read().window_width as f32 / 2.0,
                    CGAME_P.read().window_height as f32 / 2.0,
                ],
                RADIUS,
                COLOR_WHITE,
            )
            .build();

        static mut LAST_WORLD_POS: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        static mut LAST_SCREEN_POS: Vec2 = Vec2 {
            x: 0.0,
            y: 0.0,
        };

        draw_bone(ui, COLOR, ENGINE.c_model_obj_p);

        static mut LAST_OBJ: *const crate::engine::c_model_object::CModelObject = null();

        static mut WAS_KEY_DOWN: bool = false;

        if GetAsyncKeyState(0x2) != 0 {
            if ENGINE.model_type == EntityType::PlayerZombie
                || ENGINE.model_type == EntityType::ZombieVoleteile
                || ENGINE.model_type == EntityType::ZombieNormal
                || ENGINE.model_type == EntityType::SurvivorZombie
            {
                if !WAS_KEY_DOWN {
                    WAS_KEY_DOWN = true;
                    LAST_WORLD_POS = MODEL_HEAD_WORLD_POS;
                    LAST_OBJ = ENGINE.c_model_obj_p;
                }

                if !LAST_OBJ.is_null() {
                    ENGINE
                        .c_model_obj_p
                        .read()
                        .model_obj_p
                        .read()
                        .get_bone_joint_pos(addr_of_mut!(LAST_WORLD_POS), BoneID::头 as u8);

                    ENGINE
                        .camera_manage_p
                        .read()
                        .camera_p
                        .read()
                        .world_to_screen(
                            addr_of_mut!(LAST_SCREEN_POS),
                            addr_of_mut!(LAST_WORLD_POS),
                        );

                    if center_distance([LAST_SCREEN_POS.x, LAST_SCREEN_POS.y]) < RADIUS {
                        aim(
                            ENGINE.camera_manage_p.read().camera_p.read().get_position(),
                            addr_of!(LAST_WORLD_POS),
                            &mut (*ENGINE.local_client_di_p.read().player_di_p).yaw,
                            &mut (*ENGINE.local_client_di_p.read().player_di_p).pitch,
                        );
                    }
                } else {
                    WAS_KEY_DOWN = true;
                }
            }
        } else {
            WAS_KEY_DOWN = false;
        }
    }
}
