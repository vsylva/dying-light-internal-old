use std::{
    f32::consts::PI,
    ptr::{addr_of, addr_of_mut, null},
    sync::Once,
};

use super::{c_model_object::Data, vis::Vis};
use crate::{engine::ENGINE_HANDLE, GetProcAddress, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub(crate) struct ModelObject {
    __: [u8; 0x338],
    vis: *mut Vis,
    ___: [u8; 0x9A8],
    data: *mut Data,
}

impl ModelObject {
    pub(crate) unsafe fn get_world_position(&mut self, pos: *mut Vec3) -> *mut Vec3 {
        type GetWorldPosition = unsafe extern "system" fn(*mut ModelObject, *mut Vec3) -> *mut Vec3;

        static mut PROC: isize = 0;
        static mut PROC_PTR: *const GetWorldPosition = null();

        static ONCE: Once = Once::new();

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

        static mut PROC: isize = 0;
        static mut PROC_PTR: *const GetDistanceTo = null();

        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?GetDistanceTo@IControlObject@@QEBAMAEBVvec3@@@Z\0".as_ptr(),
            );
            PROC_PTR = addr_of!(PROC) as *const GetDistanceTo;
        });

        return (*PROC_PTR)(self as *mut ModelObject, pos);
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_forward_vector(&mut self, vector: *mut Vec3) {
        type GetForwardVector = unsafe extern "system" fn(*mut ModelObject, *mut Vec3);

        static mut PROC: isize = 0;
        static mut PROC_PTR: *const GetForwardVector = null();

        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?GetForwardVector@IControlObject@@QEBA?AVvec3@@XZ\0".as_ptr(),
            );
            PROC_PTR = addr_of!(PROC).cast()
        });

        (*PROC_PTR)(self as *mut ModelObject, vector)
    }

    pub(crate) unsafe fn raytest_to_target(&mut self, src: *mut Vec3, dest: *mut Vec3) -> bool {
        type RaytestToTarget = unsafe extern "system" fn(
            *mut ModelObject,
            *mut ModelObject,
            *mut Vec3,
            *mut Vec3,
            u8,
        ) -> bool;

        static mut PROC: isize = 0;
        static mut PROC_PTR: *const RaytestToTarget = null();

        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?RaytestToTarget@IControlObject@@QEAA_NPEBV1@AEBVvec3@@1E@Z\0".as_ptr(),
            );
            PROC_PTR = addr_of!(PROC) as *const RaytestToTarget;
        });

        return (*PROC_PTR)(
            self as *mut ModelObject,
            self as *mut ModelObject,
            src,
            dest,
            4,
        );
    }

    pub(crate) unsafe fn get_bone_joint_pos(&mut self, bone_pos: *mut Vec3, index: u8) {
        type GetBoneJointPos = unsafe extern "system" fn(*mut ModelObject, *mut Vec3, u8);

        static mut PROC: isize = 0;
        static mut PROC_PTR: *const GetBoneJointPos = null();

        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            PROC = GetProcAddress(
                ENGINE_HANDLE as isize,
                "?GetBoneJointPos@IModelObject@@QEBA?AVvec3@@E@Z\0".as_ptr(),
            );

            PROC_PTR = addr_of!(PROC) as *const GetBoneJointPos;
        });

        (*PROC_PTR)(self, bone_pos, index);
    }
}

impl ModelObject {
    #[allow(unused)]
    pub(crate) unsafe fn get_yaw(&mut self) -> f32 {
        static mut FORWARD_VECTOR: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        self.get_forward_vector(addr_of_mut!(FORWARD_VECTOR));

        static mut YAW: f32 = 0.0;

        YAW = 90.0 - FORWARD_VECTOR.x.atan2(FORWARD_VECTOR.z) * 180.0 / PI;

        if YAW > 180.0 {
            YAW -= 360.0
        }

        YAW
    }
}
