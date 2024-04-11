pub(crate) mod bone;

use std::ptr::{addr_of, addr_of_mut, null};

use hudhook_mini::imgui::ImColor32;

pub(crate) static mut OBJ_HEAD_SCREEN_POS: Vec2 = Vec2 { x: 0.0, y: 0.0 };
pub(crate) static mut OBJ_HEAD_WORLD_POS: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub(crate) static mut OBJ_FOOT_SCREEN_POS: Vec2 = Vec2 { x: 0.0, y: 0.0 };

pub(crate) static mut OBJ_FOOT_WORLD_POS: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub(crate) static mut COLOR: ImColor32 = COLOR_WHITE;

pub(crate) const RADIUS: f32 = 100.0;

pub(crate) static mut IS_IN_SCREEN: bool = false;

use super::{COLOR_BLUE, COLOR_GREEN, COLOR_PURPLE, COLOR_RED};
use crate::{
    aim::get_crosshair_distance_to,
    engine::{ModelType, CGAME_P, ENGINE},
    render::{
        background::bone::{BoneID, BONE_LIST},
        menu, COLOR_PINK, COLOR_WHITE,
    },
    GetAsyncKeyState, Vec2, Vec3,
};

pub(crate) unsafe fn frame(ui: &hudhook_mini::imgui::Ui) {
    if !ENGINE.get_ptr() {
        return;
    }

    if 0 == ENGINE.get_c_model_list_len() {
        return;
    }

    for i in 0..ENGINE.get_c_model_list_len() {
        if !ENGINE.get_data(i as isize) {
            continue;
        }

        main(ui);
    }

    // 锁定血量
    if menu::LOCK_HP_SWITCH {
        if menu::LOCK_HP_SWITCH {
            (*ENGINE.player_di_p).health = 200.0;
        }
    }

    // 锁定子弹
    if menu::LOCK_AMMO_SWITCH {
        if ENGINE
            .inventory_ammo_p
            .read()
            .get_inventory_list()
            .read()
            .item_p
            != std::ptr::null_mut()
        {
            for i in 0..ENGINE
                .inventory_ammo_p
                .read()
                .get_inventory_list()
                .read()
                .len
            {
                (*ENGINE
                    .inventory_ammo_p
                    .read()
                    .get_inventory_list()
                    .read()
                    .at(i as isize))
                .count = 60;
            }
        }
    }
}

unsafe fn main(ui: &hudhook_mini::imgui::Ui) {
    // 获取头部世界坐标
    ENGINE.get_bone_joint_pos(addr_of_mut!(OBJ_HEAD_WORLD_POS), BoneID::头 as u8);

    // 检查物体坐标是否位于屏幕内
    IS_IN_SCREEN = ENGINE.get_view_w(addr_of!(OBJ_HEAD_WORLD_POS)) > 0.01;

    if menu::DISTANCE_SWITCH {
        ENGINE.get_obj_world_position(addr_of_mut!(OBJ_FOOT_WORLD_POS));

        if !menu::DISTANCE_EXTRA_SWITCH {
            if !IS_IN_SCREEN {
                return;
            }

            ENGINE.point_to_screen(
                addr_of_mut!(OBJ_FOOT_SCREEN_POS),
                addr_of_mut!(OBJ_FOOT_WORLD_POS),
            );
        } else {
            ENGINE.point_to_screen_clamp_to_frustum(
                addr_of_mut!(OBJ_FOOT_SCREEN_POS),
                addr_of_mut!(OBJ_FOOT_WORLD_POS),
            );
        }

        ui.get_background_draw_list().add_text(
            [OBJ_FOOT_SCREEN_POS.x, OBJ_FOOT_SCREEN_POS.y],
            COLOR_WHITE,
            format!(
                "{:.2}m",
                ENGINE.get_obj_distance_to(ENGINE.get_player_foot_pos())
            ),
        );
    }

    if !IS_IN_SCREEN {
        return;
    }

    // 调试显示model名
    // ENGINE.point_to_screen(
    //     addr_of_mut!(OBJ_HEAD_SCREEN_POS),
    //     addr_of_mut!(OBJ_HEAD_WORLD_POS),
    // );

    // ui.get_background_draw_list().add_text(
    //     [OBJ_HEAD_SCREEN_POS.x, OBJ_HEAD_SCREEN_POS.y],
    //     COLOR_WHITE,
    //     format!("{}", ENGINE.name),
    // );

    COLOR = match ENGINE.model_type {
        ModelType::ZombieNormal => COLOR_RED,
        ModelType::ZombieVoleteile | ModelType::SurvivorZombie | ModelType::ZombieSpecial => {
            COLOR_PURPLE
        }
        ModelType::SurvivorHuman => COLOR_BLUE,
        ModelType::PlayerHuman => COLOR_GREEN,
        ModelType::PlayerZombie => COLOR_PINK,
    };

    ENGINE.point_to_screen(
        addr_of_mut!(OBJ_HEAD_SCREEN_POS),
        addr_of_mut!(OBJ_HEAD_WORLD_POS),
    );

    if menu::VISIBLE_LINE_SWITCH {
        if ENGINE.raytest_to_target(
            ENGINE.get_player_camera_pos(),
            addr_of_mut!(OBJ_HEAD_WORLD_POS),
        ) {
            ui.get_background_draw_list()
                .add_line(
                    [CGAME_P.read().screen_width as f32 / 2.0, 0.0],
                    [OBJ_HEAD_SCREEN_POS.x, OBJ_HEAD_SCREEN_POS.y],
                    COLOR,
                )
                .build();
        }
    }

    if menu::AIM_RANGE_CIRCLE_SWITCH {
        ui.get_background_draw_list()
            .add_circle(
                [
                    CGAME_P.read().screen_width as f32 / 2.0,
                    CGAME_P.read().screen_height as f32 / 2.0,
                ],
                RADIUS,
                COLOR_WHITE,
            )
            .build();
    }

    if menu::BONE_SWITCH {
        bone(ui, COLOR);
    }

    if menu::AIM_RANGE_SWITCH {
        aim_range();
    }
}

unsafe fn aim_range() {
    pub(crate) static mut CURREN_OBJ: *const crate::engine::c_model_object::CModelObject = null();
    pub(crate) static mut CURRENT_OBJ_WORLD_POS: *const Vec3 = null();
    pub(crate) static mut WAS_KEY_DOWN: bool = false;

    if !(ENGINE.model_type == ModelType::PlayerZombie
        || ENGINE.model_type == ModelType::ZombieVoleteile
        || ENGINE.model_type == ModelType::ZombieNormal
        || ENGINE.model_type == ModelType::SurvivorZombie)
    {
        return;
    }

    if GetAsyncKeyState(0x2) != 0 {
        if get_crosshair_distance_to(
            addr_of!(OBJ_HEAD_SCREEN_POS),
            CGAME_P.read().screen_width as f32,
            CGAME_P.read().screen_height as f32,
        ) > RADIUS
        {
            return;
        }

        if !WAS_KEY_DOWN {
            WAS_KEY_DOWN = true;
            CURRENT_OBJ_WORLD_POS = addr_of!(OBJ_HEAD_WORLD_POS);
            CURREN_OBJ = ENGINE.c_model_obj_p;
        }

        if CURREN_OBJ.is_null() || CURRENT_OBJ_WORLD_POS.is_null() {
            WAS_KEY_DOWN = false;
            return;
        }

        crate::aim::aim(
            ENGINE.get_player_camera_pos(),
            CURRENT_OBJ_WORLD_POS,
            &mut (*ENGINE.local_client_di_p.read().player_di_p).yaw,
            &mut (*ENGINE.local_client_di_p.read().player_di_p).pitch,
        );
    } else {
        WAS_KEY_DOWN = false;
        return;
    }
}

unsafe fn bone(ui: &hudhook_mini::imgui::Ui, color: ImColor32) {
    pub(crate) static mut PREVIOUS_BONE_POS: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub(crate) static mut CURRENT_BONE_POS: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub(crate) static mut PREVIOUS_BONE_SCREEN_POS: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub(crate) static mut CURRENT_BONE_SCREEN_POS: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    for list in &*addr_of!(BONE_LIST) {
        for id in list {
            ENGINE.get_bone_joint_pos(addr_of_mut!(CURRENT_BONE_POS), *id as u8);

            if PREVIOUS_BONE_POS.is_empty() {
                PREVIOUS_BONE_POS = CURRENT_BONE_POS;
                continue;
            }

            if ENGINE.get_view_w(addr_of!(PREVIOUS_BONE_POS)) > 0.01
                && ENGINE.get_view_w(addr_of!(CURRENT_BONE_POS)) > 0.01
            {
                ENGINE.point_to_screen(
                    addr_of_mut!(PREVIOUS_BONE_SCREEN_POS),
                    addr_of!(PREVIOUS_BONE_POS),
                );

                ENGINE.point_to_screen(
                    addr_of_mut!(CURRENT_BONE_SCREEN_POS),
                    addr_of!(CURRENT_BONE_POS),
                );

                ui.get_background_draw_list()
                    .add_line(
                        [PREVIOUS_BONE_SCREEN_POS.x, PREVIOUS_BONE_SCREEN_POS.y],
                        [CURRENT_BONE_SCREEN_POS.x, CURRENT_BONE_SCREEN_POS.y],
                        color,
                    )
                    .thickness(1.5)
                    .build();
            }

            PREVIOUS_BONE_POS = CURRENT_BONE_POS;
            CURRENT_BONE_POS.reset();
        }
        PREVIOUS_BONE_POS.reset();
    }
}
