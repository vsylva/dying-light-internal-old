use std::{
    ptr::{addr_of, null},
    sync::Once,
};

use super::{level_di::LevelDI, session_cooperative_di::SessionCooperativeDI};
use crate::{engine::ENGINE_HANDLE, GetProcAddress};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct GameDI {
    __: [u8; 0x540],
    pub(crate) session_p: *mut SessionCooperativeDI,
}
impl GameDI {
    #[allow(unused)]
    pub(crate) unsafe fn get_screen_width(&mut self) -> i32 {
        type GetScreenWidth = unsafe extern "system" fn(*mut GameDI) -> i32;

        static mut PROC: isize = 0;
        static mut PROC_PTR: *const GetScreenWidth = null();

        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?GetScreenWidth@IGame@@QEAAHXZ\0".as_ptr(),
            );

            PROC_PTR = addr_of!(PROC) as *const GetScreenWidth;
        });

        (*PROC_PTR)(self)
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_screen_height(&mut self) -> i32 {
        type GetScreenHeight = unsafe extern "system" fn(*mut GameDI) -> i32;

        static mut PROC: isize = 0;
        static mut PROC_PTR: *const GetScreenHeight = null();

        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?GetScreenHeight@IGame@@QEAAHXZ\0".as_ptr(),
            );

            PROC_PTR = addr_of!(PROC) as *const GetScreenHeight;
        });

        (*PROC_PTR)(self)
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_active_level_di(&mut self) -> *mut LevelDI {
        type GetActiveLevel = unsafe extern "system" fn(*mut GameDI) -> *mut LevelDI;

        static mut PROC: isize = 0;
        static mut PROC_PTR: *const GetActiveLevel = null();

        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?GetActiveLevel@IGame@@QEAAPEAVILevel@@XZ\0".as_ptr(),
            );

            PROC_PTR = addr_of!(PROC) as *const GetActiveLevel;
        });

        (*PROC_PTR)(self)
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_logical_level(&mut self) -> *mut LevelDI {
        type GetLevelEditor = unsafe extern "system" fn(*mut GameDI) -> *mut LevelDI;

        static mut PROC: isize = 0;
        static mut PROC_PTR: *const GetLevelEditor = null();

        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?GetLevelEditor@IGame@@UEAAPEAVILevel@@XZ\0".as_ptr(),
            );

            PROC_PTR = addr_of!(PROC) as *const GetLevelEditor;
        });

        (*PROC_PTR)(self)
    }
}
