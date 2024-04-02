use std::{
    ffi::{c_void, CStr},
    ptr::null_mut,
};

use self::{
    c_base_camera::CBaseCamera, c_game::CGame, c_level::CLevel, c_model_object::CModelObject,
    camera_fpp_di::CameraFPPDI, camera_manager_di::CameraManagerDI, game_di::GameDI,
    inventory_ammo::InventoryAmmo, inventory_container_di::InventoryContainerDI, level_di::LevelDI,
    local_client_di::LocalClientDI, player_di::PlayerDI,
    session_cooperative_di::SessionCooperativeDI,
};
use crate::{Vec2, Vec3};

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
pub(crate) mod model_object;
pub(crate) mod player_di;
pub(crate) mod session_cooperative_di;

pub(crate) static mut ENGINE_HANDLE: *mut c_void = null_mut();
pub(crate) static mut ENGINE_SIZE: usize = 0;

pub(crate) static mut CGAME_P: *mut CGame = null_mut();

pub(crate) static mut ENGINE: Engine = Engine {
    game_di_p: null_mut(),

    session_cooperative_di_p: null_mut(),

    level_di_p: null_mut(),
    c_level_p: null_mut(),

    local_client_di_p: null_mut(),
    player_di_p: null_mut(),

    camera_manage_p: null_mut(),
    camera_fpp_di_p: null_mut(),
    c_base_camera: null_mut(),

    inventory_container_di_p: null_mut(),
    inventory_ammo_p: null_mut(),

    c_model_obj_p: null_mut(),

    model_type: ModelType::ZombieNormal,
};

impl Engine {
    pub(crate) unsafe fn get_c_model_list_len(&self) -> u32 {
        self.c_level_p.read().get_c_model_list().read().len
    }

    pub(crate) unsafe fn get_bone_joint_pos(&mut self, screen_pos: *mut Vec3, index: u8) {
        self.c_model_obj_p
            .read()
            .model_obj_p
            .read()
            .get_bone_joint_pos(screen_pos, index);
    }

    pub(crate) unsafe fn get_obj_world_position(&mut self, pos: *mut Vec3) -> *mut Vec3 {
        self.c_model_obj_p
            .read()
            .model_obj_p
            .read()
            .get_world_position(pos)
    }

    pub(crate) unsafe fn get_obj_distance_to(&mut self, pos: *const Vec3) -> f32 {
        ENGINE
            .c_model_obj_p
            .read()
            .model_obj_p
            .read()
            .get_distance_to(pos)
    }

    pub(crate) unsafe fn get_player_foot_pos(&self) -> *const Vec3 {
        ENGINE.player_di_p.read().get_player_foot_pos()
    }

    pub(crate) unsafe fn point_to_screen_clamp_to_frustum(
        &mut self,
        screen_pos: *mut Vec2,
        obj_pos: *const Vec3,
    ) -> *mut Vec2 {
        ENGINE
            .camera_fpp_di_p
            .read()
            .point_to_screen_clamp_to_frustum(screen_pos, obj_pos)
    }

    pub(crate) unsafe fn point_to_screen(
        &mut self,
        screen_pos: *mut Vec2,
        obj_pos: *const Vec3,
    ) -> *mut Vec2 {
        self.camera_manage_p
            .read()
            .camera_fpp_di_p
            .read()
            .point_to_screen(screen_pos, obj_pos)
    }

    pub(crate) unsafe fn get_view_w(&self, pos: *const Vec3) -> f32 {
        self.camera_fpp_di_p.read().get_view_w(pos)
    }

    pub(crate) unsafe fn get_player_camera_pos(&self) -> *mut Vec3 {
        self.camera_fpp_di_p.read().get_position()
    }

    pub(crate) unsafe fn raytest_to_target(&self, src: *const Vec3, dest: *const Vec3) -> bool {
        self.c_model_obj_p
            .read()
            .model_obj_p
            .read()
            .raytest_to_target(src, dest)
    }
}

impl Engine {
    pub(crate) unsafe fn get_ptr(&mut self) -> bool {
        if self.game_di_p.is_null() {
            self.game_di_p = CGAME_P.read().game_di_p;
        }

        if self.game_di_p.is_null() {
            return false;
        }

        self.session_cooperative_di_p = self.game_di_p.read().session_cooperative_di_p;
        if self.session_cooperative_di_p.is_null() {
            return false;
        }

        self.level_di_p = self.session_cooperative_di_p.read().level_di_p;
        if self.level_di_p.is_null() {
            return false;
        }

        self.c_level_p = self.level_di_p.read().c_level_p;
        if self.c_level_p.is_null() {
            return false;
        }
        if self.c_level_p.read().get_c_model_list().is_null() {
            return false;
        }
        if self.c_level_p.read().get_c_model_list().read().len == 0 {
            return false;
        }

        self.local_client_di_p = self.session_cooperative_di_p.read().local_client_di_p;
        if self.local_client_di_p.is_null() {
            return false;
        }

        self.player_di_p = self.local_client_di_p.read().player_di_p;
        if self.player_di_p.is_null() {
            return false;
        }
        if self.player_di_p.read().is_pos_empty() {
            return false;
        }

        self.camera_manage_p = self.session_cooperative_di_p.read().camera_manager_p;
        if self.camera_manage_p.is_null() {
            return false;
        }

        self.camera_fpp_di_p = self.camera_manage_p.read().camera_fpp_di_p;
        if self.camera_fpp_di_p.is_null() {
            return false;
        }

        self.c_base_camera = self.camera_fpp_di_p.read().c_base_camera;
        if self.c_base_camera.is_null() {
            return false;
        }
        if self.c_base_camera.read().is_pos_empty() {
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
        if self.inventory_ammo_p.read().get_inventory_list().is_null() {
            return false;
        }

        if self.inventory_ammo_p.read().get_inventory_list().read().len == 0 {
            return false;
        }

        true
    }

    pub(crate) unsafe fn get_data(&mut self, index: isize) -> bool {
        self.c_model_obj_p = self.c_level_p.read().get_c_model_list().read().index(index);

        if self.c_model_obj_p.is_null() {
            return false;
        }

        if self.c_model_obj_p.read().is_pos_empty() {
            return false;
        }

        if self.c_model_obj_p.read().active == 0 {
            return false;
        }

        if self.c_model_obj_p.read().name_p.is_null() {
            return false;
        }

        pub(crate) static mut MODEL_NAME: String = String::new();

        MODEL_NAME = CStr::from_ptr(self.c_model_obj_p.read().name_p)
            .to_string_lossy()
            .to_string();

        pub(crate) static mut MODEL_NAME_DOT_OFFSET: usize = 0;

        MODEL_NAME_DOT_OFFSET = if let Some(offset) = MODEL_NAME.find(".") {
            offset
        } else {
            return false;
        };

        pub(crate) static mut MODEL_NAME_ARRAY: Vec<&str> = Vec::new();

        MODEL_NAME_ARRAY = (&MODEL_NAME[..MODEL_NAME_DOT_OFFSET]).split("_").collect();

        if MODEL_NAME_ARRAY.len() < 2 {
            return false;
        }

        match MODEL_NAME_ARRAY[0] {
            "zombie" => match MODEL_NAME_ARRAY[1] {
                "voleteile" => self.model_type = ModelType::ZombieVoleteile,
                "spitter" => self.model_type = ModelType::ZombieSpecial,
                _ => self.model_type = ModelType::ZombieNormal,
            },

            "survivor" => match MODEL_NAME_ARRAY[1] {
                "a" => self.model_type = ModelType::SurvivorHuman,
                _ => self.model_type = ModelType::SurvivorZombie,
            },

            "player" => match MODEL_NAME_ARRAY[1] {
                "zombie" => self.model_type = ModelType::PlayerZombie,
                _ => self.model_type = ModelType::PlayerHuman,
            },
            _ => return false,
        };

        true
    }
}

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
        return ((&mut *self.item_p) as *mut T).offset(index);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub(crate) enum ModelType {
    ZombieNormal,
    ZombieSpecial,
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
    pub(crate) session_cooperative_di_p: *mut SessionCooperativeDI,

    pub(crate) level_di_p: *mut LevelDI,
    pub(crate) c_level_p: *mut CLevel,

    pub(crate) local_client_di_p: *mut LocalClientDI,
    pub(crate) player_di_p: *mut PlayerDI,

    pub(crate) camera_manage_p: *mut CameraManagerDI,
    pub(crate) camera_fpp_di_p: *mut CameraFPPDI,
    pub(crate) c_base_camera: *mut CBaseCamera,

    pub(crate) inventory_container_di_p: *mut InventoryContainerDI,
    pub(crate) inventory_ammo_p: *mut InventoryAmmo,

    pub(crate) c_model_obj_p: *mut CModelObject,

    pub(crate) model_type: ModelType,
}
