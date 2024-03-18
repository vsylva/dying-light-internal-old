use std::{
    ptr::{addr_of, null},
    sync::Once,
};

use super::c_base_camera::CBaseCamera;
use crate::{
    engine::{CGAME_P, ENGINE_HANDLE},
    GetProcAddress,
    Vec2,
    Vec3,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct CameraFPPDI {
    __: [u8; 0x8],
    pub(crate) base: *mut CBaseCamera,
}
impl CameraFPPDI {
    #[allow(unused)]
    pub(crate) unsafe fn point_to_screen(&mut self, screen_pos: *mut Vec2, obj_pos: *mut Vec3) {
        type PointToScreen = unsafe extern "system" fn(*mut CameraFPPDI, *mut Vec2, *mut Vec3);

        static mut PROC: isize = 0;
        static mut PROC_PTR: *const PointToScreen = null();

        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?PointToScreen@IBaseCamera@@QEAA?BVvec2@@AEBVvec3@@@Z\0".as_ptr(),
            );

            PROC_PTR = addr_of!(PROC) as *const PointToScreen;
        });

        (*PROC_PTR)(self, screen_pos, obj_pos)
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_fov(&mut self) -> f32 {
        type GetFov = unsafe extern "system" fn(*mut CameraFPPDI) -> f32;

        static mut PROC: isize = 0;
        static mut PROC_PTR: *const GetFov = null();

        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?GetFOV@IBaseCamera@@QEAAMXZ\0".as_ptr(),
            );

            PROC_PTR = addr_of!(PROC) as *const GetFov;
        });

        return (*PROC_PTR)(self as *mut CameraFPPDI);
    }

    pub(crate) unsafe fn get_position(&mut self) -> *mut Vec3 {
        type GetPosition = unsafe extern "system" fn(*mut CameraFPPDI, *mut Vec3) -> *mut Vec3;

        static mut PROC: isize = 0;
        static mut PROC_PTR: *const GetPosition = null();

        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?GetPosition@IBaseCamera@@QEBA?BVvec3@@XZ\0".as_ptr(),
            );

            PROC_PTR = addr_of!(PROC) as *const GetPosition;
        });

        let mut pos: Vec3 = Vec3::default();

        return (*PROC_PTR)(self as *mut CameraFPPDI, &mut pos);
    }
}

impl CameraFPPDI {
    pub(crate) unsafe fn world_to_screen(
        &mut self,
        screen_pos: *mut Vec2,
        entyty_pos: *const Vec3,
    ) -> bool {
        static mut SIGHT_X: f32 = 0.0;
        static mut SIGHT_Y: f32 = 0.0;

        SIGHT_X = CGAME_P.read().window_width as f32 / 2.0;
        SIGHT_Y = CGAME_P.read().window_height as f32 / 2.0;

        static mut MATRIX: [[f32; 4]; 4] = [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ];

        MATRIX = self.base.read().matrix;

        static mut VIEW_W: f32 = 0.0;

        static mut ENTYTY_POS: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        ENTYTY_POS = *entyty_pos;

        VIEW_W = MATRIX[3][0] * ENTYTY_POS.x
            + MATRIX[3][1] * ENTYTY_POS.y
            + MATRIX[3][2] * ENTYTY_POS.z
            + MATRIX[3][3];

        if VIEW_W <= 0.01 {
            return false;
        }

        VIEW_W = 1.0 / VIEW_W;

        (*screen_pos).x = SIGHT_X
            + (MATRIX[0][0] * ENTYTY_POS.x
                + MATRIX[0][1] * ENTYTY_POS.y
                + MATRIX[0][2] * ENTYTY_POS.z
                + MATRIX[0][3])
                * VIEW_W
                * SIGHT_X;

        (*screen_pos).y = SIGHT_Y
            - (MATRIX[1][0] * ENTYTY_POS.x
                + MATRIX[1][1] * ENTYTY_POS.y
                + MATRIX[1][2] * ENTYTY_POS.z
                + MATRIX[1][3])
                * VIEW_W
                * SIGHT_Y;

        return true;
    }
}
