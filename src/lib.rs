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
            engine::init();

            std::thread::sleep(std::time::Duration::from_secs(5));

            render::init(h_module);
        });
    } else if ul_reason_for_call == 0 {
    }

    1
}

pub(crate) type HMODULE = isize;
pub(crate) type PCSTR = *const u8;
// pub(crate) type FARPROC = unsafe extern "system" fn() -> isize;
pub(crate) type FARPROC = isize;
pub(crate) type BOOL = i32;

#[link(name = "user32")]
extern "system" {
    pub(crate) fn GetAsyncKeyState(vKey: i32) -> u16;
    fn GetCursorPos(lppoint: *mut POINT) -> BOOL;

    pub(crate) fn ScreenToClient(hwnd: isize, lppoint: *mut POINT) -> BOOL;
    // pub(crate) fn IsIconic(hWnd: isize) -> i32;
    pub(crate) fn FindWindowA(lpClassName: *const u8, lpWindowName: *const u8) -> isize;
}

#[link(name = "kernel32")]
extern "system" {

    pub(crate) fn GetProcAddress(hmodule: HMODULE, lpprocname: PCSTR) -> FARPROC;

    // pub(crate) fn DisableThreadLibraryCalls(hlibmodule: HMODULE) -> BOOL;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct Vec2 {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct Vec3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl Vec3 {
    pub(crate) unsafe fn is_empty(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub(crate) unsafe fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct Rotator {
    pub(crate) yaw: f32,
    pub(crate) pitch: f32,
    pub(crate) roll: f32,
}

// pub(crate) unsafe fn is_dll_loaded(dll_name: &str, interval_sec: u64, end_sec: u64) -> bool {
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

// pub(crate) unsafe fn debug_mode() {
//     use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

//     vcheat::alloc_console().unwrap();
//     vcheat::colored_console().unwrap();

//     std::env::set_var("RUST_LOG", "trace");

//     let log_file = hudhook::util::get_dll_path()
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
