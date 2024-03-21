use super::{inventory::Inventory, List};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct InventoryAmmo {
    __: [u8; 0x40],
    inventory_list: List<Inventory>,
}

impl InventoryAmmo {
    pub(crate) unsafe fn get_inventory_list(&mut self) -> *mut List<Inventory> {
        &mut self.inventory_list
    }
}
