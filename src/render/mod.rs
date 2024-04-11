mod background;
mod menu;

use std::ptr::{addr_of_mut, null};

use hudhook_mini::imgui::ImColor32;

use self::background::bone::init_bone_list;
use crate::{FindWindowA, GetAsyncKeyState, GetCursorPos, ScreenToClient, POINT};

pub(crate) static mut IS_SHOW_UI: bool = true;
pub(crate) static mut WINDOW: isize = 0;

const COLOR_RED: ImColor32 = hudhook_mini::imgui::ImColor32::from_rgb(255, 0, 0);

const COLOR_PURPLE: ImColor32 = hudhook_mini::imgui::ImColor32::from_rgb(102, 0, 153);

const COLOR_BLUE: ImColor32 = hudhook_mini::imgui::ImColor32::from_rgb(0, 0, 255);

const COLOR_GREEN: ImColor32 = hudhook_mini::imgui::ImColor32::from_rgb(0, 255, 0);

const COLOR_PINK: ImColor32 = hudhook_mini::imgui::ImColor32::from_rgb(255, 192, 103);

const COLOR_WHITE: ImColor32 = hudhook_mini::imgui::ImColor32::from_rgb(255, 255, 255);

pub(crate) struct RenderLoop;

impl hudhook_mini::ImguiRenderLoop for RenderLoop {
    fn initialize<'a>(
        &'a mut self,
        _ctx: &mut hudhook_mini::imgui::Context,
        _render_context: &'a mut dyn hudhook_mini::RenderContext,
    ) {
        unsafe {
            set_font(_ctx, 20.0);

            init_bone_list();

            WINDOW = FindWindowA("techland_game_class".as_ptr(), null());
        }
    }

    fn render(&mut self, ui: &mut hudhook_mini::imgui::Ui) {
        unsafe {
            if is_key_down_once(0xC0) {
                IS_SHOW_UI = !IS_SHOW_UI;
            }

            background::frame(ui);

            if IS_SHOW_UI {
                pub(crate) static mut MOUSE_POS: POINT = POINT { x: 0, y: 0 };

                GetCursorPos(addr_of_mut!(MOUSE_POS));
                ScreenToClient(WINDOW, addr_of_mut!(MOUSE_POS));

                (*hudhook_mini::imgui::sys::igGetIO()).MousePos.x = MOUSE_POS.x as f32;
                (*hudhook_mini::imgui::sys::igGetIO()).MousePos.y = MOUSE_POS.y as f32;
                (*hudhook_mini::imgui::sys::igGetIO()).MouseDrawCursor = true;

                if GetAsyncKeyState(0x1) != 0 {
                    (*hudhook_mini::imgui::sys::igGetIO()).MouseDown[0] = true;
                } else {
                    (*hudhook_mini::imgui::sys::igGetIO()).MouseDown[0] = false;
                }
            } else {
                (*hudhook_mini::imgui::sys::igGetIO()).MouseDrawCursor = false;
                return;
            }

            menu::frame(ui);
        }
    }
}

fn set_font(ctx: &mut hudhook_mini::imgui::Context, font_size: f32) {
    let tf_data = hudhook_mini::imgui::FontSource::TtfData {
        data: include_bytes!("../../res/FZHTJW.TTF"),
        size_pixels: font_size,
        config: Some(hudhook_mini::imgui::FontConfig {
            size_pixels: font_size,
            pixel_snap_h: true,
            glyph_ranges: hudhook_mini::imgui::FontGlyphRanges::from_slice(&[
                0x0020, 0x00FF, 0x2000, 0x206F, 0x3000, 0x30FF, 0x31F0, 0x31FF, 0xFF00, 0xFFEF,
                0xFFFD, 0xFFFD, 0x4E00, 0x9FAF, 0,
            ]),
            ..hudhook_mini::imgui::FontConfig::default()
        }),
    };

    ctx.fonts().add_font(&[tf_data]);
}

unsafe fn is_key_down_once(virtual_key_code: i32) -> bool {
    pub(crate) static mut WAS_KEY_DOWN: bool = false;

    if (crate::GetAsyncKeyState(virtual_key_code) & 0x8000) != 0 {
        if !WAS_KEY_DOWN {
            WAS_KEY_DOWN = true;
            return true;
        }
    } else {
        WAS_KEY_DOWN = false;
    }
    false
}
