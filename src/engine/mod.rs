use std::{
    ffi::{c_void, CStr},
    ptr::{addr_of_mut, null_mut},
};

use self::{
    c_game::CGame,
    c_model_object::CModelObject,
    camera_manager_di::CameraManagerDI,
    game_di::GameDI,
    inventory_ammo::InventoryAmmo,
    inventory_container_di::InventoryContainerDI,
    level_di::LevelDI,
    local_client_di::LocalClientDI,
    player_di::PlayerDI,
    session_cooperative_di::SessionCooperativeDI,
};

pub(crate) mod c_base_camera;
pub(crate) mod c_game;
pub(crate) mod c_level;
pub(crate) mod c_model_object;
pub(crate) mod camera_fpp_di;
pub(crate) mod camera_manager_di;
pub(crate) mod game_di;
pub(crate) mod inventory;
pub(crate) mod inventory_ammo;
pub(crate) mod inventory_container_di;
pub(crate) mod level_di;
pub(crate) mod local_client_di;
pub mod model_object;
pub(crate) mod player_di;
pub(crate) mod session_cooperative_di;
pub mod vis;

pub(crate) static mut ENGINE_HANDLE: *mut c_void = null_mut();
pub(crate) static mut ENGINE_SIZE: usize = 0;
pub(crate) static mut ENGINE_DATA: Vec<u8> = Vec::new();

pub(crate) static mut CGAME_P: *mut CGame = null_mut();

pub(crate) static mut ENGINE: Engine = Engine {
    game_di_p: null_mut(),
    session_p: null_mut(),
    level_di_p: null_mut(),
    local_client_di_p: null_mut(),
    camera_manage_p: null_mut(),
    player_di_p: null_mut(),
    inventory_container_di_p: null_mut(),
    inventory_ammo_p: null_mut(),

    c_model_obj_p: null_mut(),

    model_type: EntityType::ZombieNormal,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub(crate) struct Array<T> {
    pub(crate) items_p: *mut *mut T,
    pub(crate) len: u32,
    pub(crate) max_len: u32,
}

impl<T> Array<T> {
    pub(crate) unsafe fn index(&self, index: isize) -> *mut T {
        return self.items_p.offset(index).read();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub(crate) struct List<T> {
    pub(crate) item_p: *mut T,
    pub(crate) len: u32,
    pub(crate) max_len: u32,
}

impl<T> List<T> {
    pub(crate) unsafe fn at(&mut self, index: isize) -> *mut T {
        return (&mut (*self.item_p) as *mut T).offset(index);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub(crate) enum EntityType {
    ZombieNormal,
    ZombieVoleteile,
    SurvivorHuman,
    SurvivorZombie,
    PlayerHuman,
    PlayerZombie,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct Engine {
    pub(crate) game_di_p: *mut GameDI,
    pub(crate) session_p: *mut SessionCooperativeDI,
    pub(crate) level_di_p: *mut LevelDI,
    pub(crate) local_client_di_p: *mut LocalClientDI,
    pub(crate) camera_manage_p: *mut CameraManagerDI,
    pub(crate) player_di_p: *mut PlayerDI,
    pub(crate) inventory_container_di_p: *mut InventoryContainerDI,
    pub(crate) inventory_ammo_p: *mut InventoryAmmo,

    pub(crate) c_model_obj_p: *mut CModelObject,

    pub(crate) model_type: EntityType,
}

impl Engine {
    pub(crate) unsafe fn get_ptr(&mut self) -> bool {
        if self.game_di_p.is_null() {
            self.game_di_p = CGAME_P.read().game_di_p;
        }

        if self.game_di_p.is_null() {
            return false;
        }

        self.session_p = self.game_di_p.read().session_p;

        if self.session_p.is_null() {
            return false;
        }

        self.level_di_p = self.game_di_p.read().session_p.read().level_di_p;

        if self.level_di_p.is_null() {
            return false;
        }

        self.local_client_di_p = self.session_p.read().local_client_di_p;
        if self.local_client_di_p.is_null() {
            return false;
        }

        self.player_di_p = self.local_client_di_p.read().player_di_p;
        if self.player_di_p.is_null() {
            return false;
        }

        if self
            .player_di_p
            .read()
            .get_inventory_container_di_p()
            .is_null()
        {
            return false;
        }

        self.inventory_container_di_p = self
            .player_di_p
            .read()
            .get_inventory_container_di_p()
            .read();

        if self.inventory_container_di_p.is_null() {
            return false;
        }

        self.inventory_ammo_p = self.inventory_container_di_p.read().inventory_ammo_p;

        if self.inventory_ammo_p.is_null() {
            return false;
        }

        self.camera_manage_p = self.session_p.read().camera_manager_p;
        if self.camera_manage_p.is_null() {
            return false;
        }

        if self.camera_manage_p.read().camera_p.is_null() {
            return false;
        }

        true
    }

    pub(crate) unsafe fn get_data(&mut self) -> bool {
        if self.c_model_obj_p.is_null() {
            return false;
        }

        if self.c_model_obj_p.read().active == 0 {
            return false;
        }

        if self.c_model_obj_p.read().is_pos_empty() {
            return false;
        }

        if self.c_model_obj_p.read().name_p.is_null() {
            return false;
        }

        static mut MODEL_NAME: String = String::new();

        MODEL_NAME = CStr::from_ptr(self.c_model_obj_p.read().name_p.cast())
            .to_string_lossy()
            .to_string();

        if !MODEL_NAME.contains("zombie")
            && !MODEL_NAME.contains("survivor")
            && !MODEL_NAME.contains("player")
        {
            return false;
        }

        match MODEL_NAME.clone() {
            zombie if zombie.contains("zombie") => {
                if zombie.contains("voleteile") {
                    self.model_type = EntityType::ZombieVoleteile
                } else if zombie.contains("player") {
                    self.model_type = EntityType::PlayerZombie
                } else {
                    self.model_type = EntityType::ZombieNormal
                }
            }

            survivor if survivor.contains("survivor") => {
                if self.c_model_obj_p.read().flag == 0x2000 {
                    self.model_type = EntityType::SurvivorHuman
                } else {
                    self.model_type = EntityType::SurvivorZombie
                }
            }

            player if player.contains("player") && !player.contains("zombie") => {
                self.model_type = EntityType::PlayerHuman;
            }

            _ => self.model_type = EntityType::ZombieNormal,
        }

        true
    }
}

pub(crate) unsafe fn init() {
    let module_engine_info = vcheat::internal::get_mod_info("engine_x64_rwdi.dll").unwrap();

    ENGINE_HANDLE = module_engine_info.handle as *mut c_void;
    ENGINE_SIZE = module_engine_info.size as usize;

    // std::ptr::copy_nonoverlapping(ENGINE_HANDLE.cast(), ENGINE_DATA.as_mut_ptr(), ENGINE_SIZE);

    ENGINE_DATA = vcheat::read_mem(
        vcheat::internal::get_proc_handle(),
        ENGINE_HANDLE,
        ENGINE_SIZE,
    )
    .unwrap();

    let cgame_p_offset = vcheat::pat_find(
        "48 83 EC 50 48 8B 05 ?? ?? ?? ?? 49 8B F8 48 8B ??",
        &mut *addr_of_mut!(ENGINE_DATA),
    )
    .unwrap();

    let mut addr = ENGINE_HANDLE.byte_add(cgame_p_offset).byte_add(4);

    addr = addr.byte_add(addr.byte_add(3).cast::<u32>().read_unaligned() as usize + 7);

    CGAME_P = *addr.cast::<*mut CGame>();
}
