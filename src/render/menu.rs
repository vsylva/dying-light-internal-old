pub(crate) unsafe fn frame(ui: &hudhook::imgui::Ui) {
    ui.window(format!("消逝的光芒hack"))
        .title_bar(true)
        .size([500.0, 400.0], hudhook::imgui::Condition::FirstUseEver)
        .resizable(true)
        .collapsible(true)
        .movable(true)
        .build(|| {});
}
