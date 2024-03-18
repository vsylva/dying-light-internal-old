use super::{inventory::Inventory, List};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct InventoryAmmo {
    __: [u8; 0x40],
    pub(crate) inventory_list: List<Inventory>,
}
