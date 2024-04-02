use std::{
    ptr::{addr_of, null},
    sync::Once,
};

use super::{c_base_camera::CBaseCamera, c_level::CLevel};
use crate::engine::ENGINE_HANDLE;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct LevelDI {
    __: [u8; 0x8],
    pub(crate) c_level_p: *mut CLevel,
}

impl LevelDI {
    pub(crate) unsafe fn _get_active_camera(&mut self, n: i32) -> *mut CBaseCamera {
        type GetActiveCamera = unsafe extern "system" fn(*mut LevelDI, i32) -> *mut CBaseCamera;

        pub(crate) static mut PROC: isize = 0;
        pub(crate) static mut PROC_PTR: *const GetActiveCamera = null();

        pub(crate) static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = vcheat::get_proc_address(
                ENGINE_HANDLE as isize,
                "?GetActiveCamera@ILevel@@QEBAPEAVIBaseCamera@@XZ",
            )
            .unwrap();

            PROC_PTR = addr_of!(PROC) as *const GetActiveCamera;
        });

        (*PROC_PTR)(self, n)
    }
}
