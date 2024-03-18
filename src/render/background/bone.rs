use std::{
    collections::LinkedList,
    ptr::{addr_of, addr_of_mut},
};

use hudhook::imgui::ImColor32;

use crate::{
    engine::{c_model_object::CModelObject, ENGINE},
    Vec2,
    Vec3,
};

pub(crate) unsafe fn draw_bone(
    ui: &hudhook::imgui::Ui,
    color: ImColor32,
    c_model_obj_p: *mut CModelObject,
) {
    static mut PREVIOUS_BONE_POS: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    static mut CURRENT_BONE_POS: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    static mut PREVIOUS_BONE_SCREEN_POS: Vec2 = Vec2 {
        x: 0.0,
        y: 0.0,
    };

    static mut CURRENT_BONE_SCREEN_POS: Vec2 = Vec2 {
        x: 0.0,
        y: 0.0,
    };

    for list in &*addr_of!(BONE_LIST) {
        for id in list {
            c_model_obj_p
                .read()
                .model_obj_p
                .read()
                .get_bone_joint_pos(addr_of_mut!(CURRENT_BONE_POS), *id as u8);

            if PREVIOUS_BONE_POS.is_empty() {
                PREVIOUS_BONE_POS = CURRENT_BONE_POS;
                continue;
            }

            if ENGINE
                .camera_manage_p
                .read()
                .camera_p
                .read()
                .world_to_screen(
                    addr_of_mut!(PREVIOUS_BONE_SCREEN_POS),
                    addr_of!(PREVIOUS_BONE_POS),
                )
                && ENGINE
                    .camera_manage_p
                    .read()
                    .camera_p
                    .read()
                    .world_to_screen(
                        addr_of_mut!(CURRENT_BONE_SCREEN_POS),
                        addr_of!(CURRENT_BONE_POS),
                    )
            {
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

pub(crate) static mut 头部: std::collections::LinkedList<BoneID> = LinkedList::new();

pub(crate) static mut 左肩: std::collections::LinkedList<BoneID> = LinkedList::new();

pub(crate) static mut 右肩: std::collections::LinkedList<BoneID> = LinkedList::new();

pub(crate) static mut 躯干: std::collections::LinkedList<BoneID> = LinkedList::new();

pub(crate) static mut 左臂: std::collections::LinkedList<BoneID> = LinkedList::new();

pub(crate) static mut 右臂: std::collections::LinkedList<BoneID> = LinkedList::new();

pub(crate) static mut 左腿: std::collections::LinkedList<BoneID> = LinkedList::new();

pub(crate) static mut 右腿: std::collections::LinkedList<BoneID> = LinkedList::new();

pub(crate) static mut BONE_LIST: LinkedList<LinkedList<BoneID>> = LinkedList::new();

pub(crate) unsafe fn init_bone_list() {
    头部 = LinkedList::from([BoneID::头, BoneID::下脖子, BoneID::上脖子]);

    左肩 = LinkedList::from([BoneID::顶脊柱, BoneID::左锁骨]);

    右肩 = LinkedList::from([BoneID::顶脊柱, BoneID::右锁骨]);

    左臂 = LinkedList::from([BoneID::左锁骨, BoneID::左上臂, BoneID::左前臂, BoneID::左手]);

    右臂 = LinkedList::from([BoneID::右锁骨, BoneID::右上臂, BoneID::右前臂, BoneID::右手]);

    躯干 = LinkedList::from([
        BoneID::上脖子,
        BoneID::顶脊柱,
        BoneID::上脊柱,
        BoneID::下脊柱,
        BoneID::底脊柱,
        BoneID::骨盆,
    ]);

    左腿 = LinkedList::from([
        BoneID::骨盆,
        BoneID::左大腿,
        BoneID::左小腿,
        BoneID::左脚,
        BoneID::左脚掌,
    ]);

    右腿 = LinkedList::from([
        BoneID::骨盆,
        BoneID::右大腿,
        BoneID::右小腿,
        BoneID::右脚,
        BoneID::右脚掌,
    ]);

    BONE_LIST.push_back(头部.clone());
    BONE_LIST.push_back(左肩.clone());
    BONE_LIST.push_back(右肩.clone());
    BONE_LIST.push_back(躯干.clone());
    BONE_LIST.push_back(左臂.clone());
    BONE_LIST.push_back(右臂.clone());
    BONE_LIST.push_back(左腿.clone());
    BONE_LIST.push_back(右腿.clone());
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u32)]
pub(crate) enum BoneID {
    骨盆 = 0,
    底脊柱,
    下脊柱,
    上脊柱,
    顶脊柱,
    下脖子,
    上脖子,
    _虚空点,
    头,
    _眼部相机,
    左锁骨,
    左上臂,
    左前臂,
    左手,
    右锁骨,
    右上臂,
    右前臂,
    右手,
    左大腿,
    右大腿,
    左小腿,
    右小腿,
    左脚,
    右脚,
    左脚掌,
    右脚掌,
}
