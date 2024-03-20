use std::ptr::addr_of_mut;

pub(crate) static mut DISTANCE_SWITCH: bool = false;
pub(crate) static mut BONE_SWITCH: bool = false;
pub(crate) static mut VISIBLE_LINE_SWITCH: bool = false;
pub(crate) static mut AIM_CIRCLE_SWITCH: bool = false;
pub(crate) static mut AIM_SWITCH: bool = false;
pub(crate) static mut LOCK_AMMO_SWITCH: bool = false;
pub(crate) static mut LOCK_HP_SWITCH: bool = false;

pub(crate) unsafe fn frame(ui: &hudhook::imgui::Ui) {
    ui.window(format!("消逝的光芒内部(DLInternal)"))
        .title_bar(true)
        .size([500.0, 400.0], hudhook::imgui::Condition::FirstUseEver)
        .resizable(true)
        .collapsible(true)
        .movable(true)
        .build(|| main(ui));
}
pub(crate) unsafe fn main(ui: &hudhook::imgui::Ui) {
    ui.text("按 ~ 打开/关闭菜单");
    ui.text("Press ~ to open/close Menu");

    ui.text("按住 Caps 以显示光标");
    ui.text("Hold Caps to display the cursor");

    ui.checkbox("距离(DISTANCE)", &mut *addr_of_mut!(DISTANCE_SWITCH));

    ui.checkbox(
        "可视线(VISIBLE LINE)",
        &mut *addr_of_mut!(VISIBLE_LINE_SWITCH),
    );

    ui.checkbox("骨骼(BONE)", &mut *addr_of_mut!(BONE_SWITCH));

    ui.checkbox("锁定子弹(LOCK AMMO)", &mut *addr_of_mut!(LOCK_AMMO_SWITCH));

    ui.checkbox("锁定生命(LOCK HP)", &mut *addr_of_mut!(LOCK_HP_SWITCH));

    ui.checkbox("自瞄圆(AIM CIRCLE)", &mut *addr_of_mut!(AIM_CIRCLE_SWITCH));

    ui.checkbox("自瞄(AIM)", &mut *addr_of_mut!(AIM_SWITCH));
}
