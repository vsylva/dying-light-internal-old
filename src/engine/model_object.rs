use std::{
    ptr::{addr_of, null},
    sync::Once,
};

use crate::{engine::ENGINE_HANDLE, GetProcAddress, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub(crate) struct ModelObject {
    __: [u8; 0x338],
    vis: *mut Vis,
    ___: [u8; 0x9A8],
    model_health: *mut ModelHealth,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct Vis {
    __: [u8; 0x194],
    yaw: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct ModelHealth {
    __: [u8; 0x78],
    health: f32,
    ___: [u8; 0x4C],
    max_health: f32,
}

impl ModelObject {
    pub(crate) unsafe fn get_world_position(&mut self, pos: *mut Vec3) -> *mut Vec3 {
        type GetWorldPosition = unsafe extern "system" fn(*mut ModelObject, *mut Vec3) -> *mut Vec3;

        pub(crate) static mut PROC: isize = 0;
        pub(crate) static mut PROC_PTR: *const GetWorldPosition = null();

        pub(crate) static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?GetWorldPosition@IControlObject@@QEBA?AVvec3@@XZ\0".as_ptr(),
            );
            PROC_PTR = addr_of!(PROC) as *const GetWorldPosition;
        });

        (*PROC_PTR)(self, pos)
    }

    pub(crate) unsafe fn get_distance_to(&mut self, pos: *const Vec3) -> f32 {
        type GetDistanceTo = unsafe extern "system" fn(*mut ModelObject, *const Vec3) -> f32;

        pub(crate) static mut PROC: isize = 0;
        pub(crate) static mut PROC_PTR: *const GetDistanceTo = null();

        pub(crate) static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?GetDistanceTo@IControlObject@@QEBAMAEBVvec3@@@Z\0".as_ptr(),
            );
            PROC_PTR = addr_of!(PROC) as *const GetDistanceTo;
        });

        return (*PROC_PTR)(self as *mut ModelObject, pos);
    }

    pub(crate) unsafe fn raytest_to_target(&mut self, src: *const Vec3, dest: *const Vec3) -> bool {
        type RaytestToTarget = unsafe extern "system" fn(
            *const ModelObject,
            *const ModelObject,
            *const Vec3,
            *const Vec3,
            u8,
        ) -> bool;

        pub(crate) static mut PROC: isize = 0;
        pub(crate) static mut PROC_PTR: *const RaytestToTarget = null();

        pub(crate) static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?RaytestToTarget@IControlObject@@QEAA_NPEBV1@AEBVvec3@@1E@Z\0".as_ptr(),
            );
            PROC_PTR = addr_of!(PROC) as *const RaytestToTarget;
        });

        return (*PROC_PTR)(
            self as *const ModelObject,
            self as *const ModelObject,
            src,
            dest,
            4,
        );
    }

    pub(crate) unsafe fn get_bone_joint_pos(&mut self, buf: *mut Vec3, index: u8) {
        type GetBoneJointPos = unsafe extern "system" fn(*mut ModelObject, *mut Vec3, u8);

        pub(crate) static mut PROC: isize = 0;
        pub(crate) static mut PROC_PTR: *const GetBoneJointPos = null();

        pub(crate) static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?GetBoneJointPos@IModelObject@@QEBA?AVvec3@@E@Z\0".as_ptr(),
            );

            PROC_PTR = addr_of!(PROC) as *const GetBoneJointPos;
        });

        (*PROC_PTR)(self, buf, index);
    }
}
