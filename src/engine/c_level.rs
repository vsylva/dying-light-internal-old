use super::{c_model_object::CModelObject, Array};

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct CLevel {
    __: [u8; 0x928],
    c_model_list: Array<CModelObject>,
}

impl CLevel {
    pub(crate) unsafe fn get_c_model_list(&mut self) -> *mut Array<CModelObject> {
        &mut self.c_model_list
    }
}
