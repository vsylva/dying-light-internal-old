use std::ptr::addr_of_mut;

use super::IS_SHOW_UI;

pub(crate) static mut DISTANCE_SWITCH: bool = false;
pub(crate) static mut DISTANCE_EXTRA_SWITCH: bool = false;
pub(crate) static mut BONE_SWITCH: bool = false;
pub(crate) static mut VISIBLE_LINE_SWITCH: bool = false;

pub(crate) static mut AIM_RANGE_SWITCH: bool = false;
pub(crate) static mut AIM_RANGE_CIRCLE_SWITCH: bool = false;

pub(crate) static mut LOCK_AMMO_SWITCH: bool = false;
pub(crate) static mut LOCK_HP_SWITCH: bool = false;

pub(crate) unsafe fn frame(ui: &hudhook::imgui::Ui) {
    ui.window(format!("消逝的光芒内部(DLInternal)"))
        .title_bar(true)
        .size([500.0, 400.0], hudhook::imgui::Condition::FirstUseEver)
        .resizable(true)
        .collapsible(true)
        .movable(true)
        .opened(&mut *addr_of_mut!(IS_SHOW_UI))
        .build(|| main(ui));
}
unsafe fn main(ui: &hudhook::imgui::Ui) {
    ui.text("按 ~ 打开/关闭菜单(Press ~ to open/close Menu)");
    ui.separator();

    if ui.checkbox("距离(DISTANCE)", &mut *addr_of_mut!(DISTANCE_SWITCH)) {
        if !DISTANCE_SWITCH {
            DISTANCE_EXTRA_SWITCH = false;
        }
    }

    if DISTANCE_SWITCH {
        ui.checkbox(
            "距离扩展(EXTRA DISTANCE)",
            &mut *addr_of_mut!(DISTANCE_EXTRA_SWITCH),
        );
    }

    ui.checkbox(
        "可视线(VISIBLE LINE)",
        &mut *addr_of_mut!(VISIBLE_LINE_SWITCH),
    );

    ui.checkbox("骨骼(BONE)", &mut *addr_of_mut!(BONE_SWITCH));

    ui.checkbox("锁定子弹(LOCK AMMO)", &mut *addr_of_mut!(LOCK_AMMO_SWITCH));

    ui.checkbox("锁定生命(LOCK HP)", &mut *addr_of_mut!(LOCK_HP_SWITCH));

    if ui.checkbox("准心自瞄(AIM)", &mut *addr_of_mut!(AIM_RANGE_SWITCH)) {
        if !AIM_RANGE_SWITCH {
            AIM_RANGE_CIRCLE_SWITCH = false;
        }
    }

    if AIM_RANGE_SWITCH {
        ui.checkbox(
            "准心自瞄范围(AIM RANGE)",
            &mut *addr_of_mut!(AIM_RANGE_CIRCLE_SWITCH),
        );
    }
}
