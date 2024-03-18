use super::inventory_ammo::InventoryAmmo;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct InventoryContainerDI {
    __: [u8; 0x48],
    pub(crate) inventory_ammo_p: *mut InventoryAmmo,
}
