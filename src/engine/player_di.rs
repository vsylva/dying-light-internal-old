use super::inventory_container_di::InventoryContainerDI;
use crate::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct PlayerDI {
    __: [u8; 1968],
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    ___: [u8; 2400],
    pub(crate) yaw: f32,
    pub(crate) pitch: f32,
    ____: [u8; 224],
    pub(crate) health: f32,
    pub(crate) max_health: f32,
}

impl PlayerDI {
    pub(crate) unsafe fn get_inventory_container_di_p(&mut self) -> *mut *mut InventoryContainerDI {
        (self as *mut PlayerDI)
            .byte_add(0x8E0)
            .cast::<*mut InventoryContainerDI>()
    }

    pub(crate) unsafe fn get_pos(&self) -> *const Vec3 {
        static mut POS: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        POS.x = self.x;
        POS.y = self.y;
        POS.z = self.z;

        std::ptr::addr_of!(POS)
    }
}
