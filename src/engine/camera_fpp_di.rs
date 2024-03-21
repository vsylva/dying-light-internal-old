use std::{
    ptr::{addr_of, null},
    sync::Once,
};

use super::c_base_camera::CBaseCamera;
use crate::{engine::ENGINE_HANDLE, GetProcAddress, Vec2, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct CameraFPPDI {
    __: [u8; 0x8],
    pub(crate) c_base_camera: *mut CBaseCamera,
}
impl CameraFPPDI {
    pub(crate) unsafe fn point_to_screen_clamp_to_frustum(
        &mut self,
        screen_pos: *mut Vec2,
        obj_pos: *const Vec3,
    ) -> *mut Vec2 {
        type PointToScreenClampToFrustum =
            unsafe extern "system" fn(*mut CameraFPPDI, *mut Vec2, *const Vec3) -> *mut Vec2;

        pub(crate) static mut PROC: isize = 0;
        pub(crate) static mut PROC_PTR: *const PointToScreenClampToFrustum = null();

        pub(crate) static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?PointToScreenClampToFrustum@IBaseCamera@@QEAA?BVvec3@@AEBV2@@Z\0".as_ptr(),
            );

            PROC_PTR = addr_of!(PROC) as *const PointToScreenClampToFrustum;
        });

        (*PROC_PTR)(self, screen_pos, obj_pos)
    }

    pub(crate) unsafe fn point_to_screen(
        &mut self,
        screen_pos: *mut Vec2,
        obj_pos: *const Vec3,
    ) -> *mut Vec2 {
        type PointToScreen =
            unsafe extern "system" fn(*mut CameraFPPDI, *mut Vec2, *const Vec3) -> *mut Vec2;

        pub(crate) static mut PROC: isize = 0;
        pub(crate) static mut PROC_PTR: *const PointToScreen = null();

        pub(crate) static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?PointToScreen@IBaseCamera@@QEAA?BVvec2@@AEBVvec3@@@Z\0".as_ptr(),
            );

            PROC_PTR = addr_of!(PROC) as *const PointToScreen;
        });

        (*PROC_PTR)(self, screen_pos, obj_pos)
    }

    pub(crate) unsafe fn _get_fov(&mut self) -> f32 {
        type GetFov = unsafe extern "system" fn(*mut CameraFPPDI) -> f32;

        pub(crate) static mut PROC: isize = 0;
        pub(crate) static mut PROC_PTR: *const GetFov = null();

        pub(crate) static ONCE: Once = Once::new();

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

        pub(crate) static mut PROC: isize = 0;
        pub(crate) static mut PROC_PTR: *const GetPosition = null();

        pub(crate) static ONCE: Once = Once::new();

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
    pub(crate) unsafe fn get_view_w(&self, pos: *const Vec3) -> f32 {
        return self.c_base_camera.read().matrix[3][0] * pos.read().x
            + self.c_base_camera.read().matrix[3][1] * pos.read().y
            + self.c_base_camera.read().matrix[3][2] * pos.read().z
            + self.c_base_camera.read().matrix[3][3];
    }
}
