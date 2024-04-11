use std::ffi::c_void;

use engine::{c_game::CGame, CGAME_P, ENGINE_HANDLE, ENGINE_SIZE};
use render::RenderLoop;

pub(crate) mod aim;
pub(crate) mod engine;
pub(crate) mod render;

#[no_mangle]
unsafe extern "system" fn DllMain(
    h_module: isize,
    ul_reason_for_call: u32,
    _lp_reserved: *mut core::ffi::c_void,
) -> i32 {
    if ul_reason_for_call == 1 {
        std::thread::spawn(move || unsafe {
            std::thread::sleep(std::time::Duration::from_secs(5));

            let engine_info = vcheat::internal::get_mod_info("engine_x64_rwdi.dll").unwrap();

            ENGINE_HANDLE = engine_info.handle as *mut c_void;
            ENGINE_SIZE = engine_info.size as usize;

            let engine_data = vcheat::read_mem(
                vcheat::internal::get_proc_handle(),
                ENGINE_HANDLE,
                ENGINE_SIZE,
            )
            .unwrap();

            let cgame_p_offset = vcheat::pat_find(
                "48 83 EC 50 48 8B 05 ?? ?? ?? ?? 49 8B F8 48 8B",
                &engine_data,
            )
            .unwrap();

            let cgame_p = ENGINE_HANDLE.byte_add(cgame_p_offset + 4);

            CGAME_P = cgame_p
                .byte_add(cgame_p.byte_add(3).cast::<u32>().read_unaligned() as usize + 7)
                .cast::<*mut CGame>()
                .read();

            if let Err(_) = ::hudhook_mini::Hudhook::builder()
                .with::<hudhook_mini::hooks::dx11::ImguiDx11Hooks>(RenderLoop)
                .with_hmodule(hudhook_mini::windows::Win32::Foundation::HINSTANCE(h_module))
                .build()
                .apply()
            {
                ::hudhook_mini::eject();
            }
        });
    } else if ul_reason_for_call == 0 {
    }

    1
}

type BOOL = i32;

#[link(name = "user32")]
extern "system" {
    fn GetAsyncKeyState(vKey: i32) -> u16;
    fn GetCursorPos(lppoint: *mut POINT) -> BOOL;
    fn ScreenToClient(hwnd: isize, lppoint: *mut POINT) -> BOOL;
    fn FindWindowA(lpClassName: *const u8, lpWindowName: *const u8) -> isize;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
struct POINT {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    unsafe fn is_empty(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    unsafe fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
struct Rotator {
    yaw: f32,
    pitch: f32,
    roll: f32,
}

//  unsafe fn is_dll_loaded(dll_name: &str, interval_sec: u64, end_sec: u64) -> bool {
//     let now = ::std::time::Instant::now();
//     let dur = ::std::time::Duration::from_secs(interval_sec);

//     while now.elapsed().as_secs() < end_sec {
//         if let Ok(ok) = vcheat::internal::get_mod_info(dll_name) {
//             if ok.handle != 0 {
//                 return true;
//             }
//         }

//         ::std::thread::sleep(dur);
//     }

//     false
// }

//  unsafe fn debug_mode() {
//     use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

//     vcheat::alloc_console().unwrap();
//     vcheat::colored_console().unwrap();

//     std::env::set_var("RUST_LOG", "trace");

//     let log_file = hudhook_mini::util::get_dll_path()
//         .map(|mut path| {
//             path.set_extension("txt");
//             path
//         })
//         .and_then(|path| std::fs::File::create(path).ok())
//         .unwrap();

//     tracing_subscriber::registry()
//         .with(
//             fmt::layer().event_format(
//                 fmt::format()
//                     .with_level(true)
//                     .with_file(true)
//                     .with_line_number(true),
//             ),
//         )
//         .with(tracing_subscriber::Layer::boxed(
//             fmt::layer()
//                 .with_file(true)
//                 .with_line_number(true)
//                 .with_writer(std::sync::Mutex::new(log_file))
//                 .with_ansi(false),
//         ))
//         .init();
// }
