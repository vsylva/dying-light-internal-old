pub(crate) mod bone;

use std::ptr::{addr_of, addr_of_mut, null};

use hudhook::imgui::ImColor32;

static mut MODEL_FOOT_SCREEN_POS: Vec2 = Vec2 {
    x: 0.0,
    y: 0.0,
};

static mut MODEL_FOOT_WORLD_POS: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

static mut HEAD_SCREEN_POS: Vec2 = Vec2 {
    x: 0.0,
    y: 0.0,
};
static mut HEAD_WORLD_POS: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

static mut LAST_WORLD_POS: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

static mut LAST_SCREEN_POS: Vec2 = Vec2 {
    x: 0.0,
    y: 0.0,
};

static mut COLOR: ImColor32 = COLOR_WHITE;

static mut IS_HEAD_VISIBLE: bool = false;

const RADIUS: f32 = 100.0;

use super::{COLOR_BLUE, COLOR_GREEN, COLOR_PURPLE, COLOR_RED};
use crate::{
    aim::{aim, center_distance},
    engine::{EntityType, CGAME_P, ENGINE},
    render::{
        background::bone::{draw_bone, BoneID},
        menu,
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

    if menu::LOCK_HP_SWITCH {
        lock_hp();
    }

    if menu::LOCK_AMMO_SWITCH {
        lock_ammo();
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

        if !is_head_in_screen(addr_of_mut!(HEAD_WORLD_POS), addr_of_mut!(HEAD_SCREEN_POS)) {
            continue;
        }

        update_color();

        if menu::DISTANCE_SWITCH {
            distance(ui);
        }

        if menu::VISIBLE_LINE_SWITCH {
            update_is_head_visible();

            if IS_HEAD_VISIBLE {
                visible_line(ui, [HEAD_SCREEN_POS.x, HEAD_SCREEN_POS.y], COLOR);
            }
        }

        if menu::AIM_CIRCLE_SWITCH {
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
        }

        if menu::BONE_SWITCH {
            draw_bone(ui, COLOR, ENGINE.c_model_obj_p);
        }

        if menu::AIM_SWITCH {
            aim_in()
        }
    }
}

pub(crate) unsafe fn lock_ammo() {
    if ENGINE.inventory_ammo_p.read().inventory_list.item_p != std::ptr::null_mut() {
        for i in 0..ENGINE.inventory_ammo_p.read().inventory_list.len {
            (*ENGINE.inventory_ammo_p.read().inventory_list.at(i as isize)).count = 60;
        }
    }
}

pub(crate) unsafe fn lock_hp() {
    if menu::LOCK_HP_SWITCH {
        (*ENGINE.player_di_p).health = 200.0;
    }
}

pub(crate) unsafe fn distance(ui: &hudhook::imgui::Ui) {
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
}

pub(crate) unsafe fn is_head_in_screen(
    head_world_pos: *mut Vec3,
    head_screen_pos: *mut Vec2,
) -> bool {
    ENGINE
        .c_model_obj_p
        .read()
        .model_obj_p
        .read()
        .get_bone_joint_pos(head_world_pos, BoneID::头 as u8);

    ENGINE
        .camera_manage_p
        .read()
        .camera_p
        .read()
        .world_to_screen(head_screen_pos, head_world_pos)
}

pub unsafe fn visible_line(ui: &hudhook::imgui::Ui, screen_pos: [f32; 2], color: ImColor32) {
    ui.get_background_draw_list()
        .add_line(
            [CGAME_P.read().window_width as f32 / 2.0, 0.0],
            screen_pos,
            color,
        )
        .build();
}

pub unsafe fn aim_in() {
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
                LAST_WORLD_POS = HEAD_WORLD_POS;
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
                    .world_to_screen(addr_of_mut!(LAST_SCREEN_POS), addr_of_mut!(LAST_WORLD_POS));

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

pub unsafe fn update_color() {
    COLOR = match ENGINE.model_type {
        EntityType::ZombieNormal => COLOR_RED,
        EntityType::ZombieVoleteile | EntityType::SurvivorZombie => COLOR_PURPLE,
        EntityType::SurvivorHuman => COLOR_BLUE,
        EntityType::PlayerHuman => COLOR_GREEN,
        EntityType::PlayerZombie => COLOR_PINK,
    };
}

pub unsafe fn update_is_head_visible() {
    IS_HEAD_VISIBLE = ENGINE
        .c_model_obj_p
        .read()
        .model_obj_p
        .read()
        .raytest_to_target(
            ENGINE.camera_manage_p.read().camera_p.read().get_position(),
            addr_of_mut!(HEAD_WORLD_POS),
        );
}
