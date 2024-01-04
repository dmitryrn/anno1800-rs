use std::{fmt::Debug, sync::OnceLock};

use windows::{s, Win32::System::LibraryLoader::GetModuleHandleA};
pub mod array_list;
pub mod class11;
pub mod class20;
pub mod class32;
pub mod class33;
pub mod class34;
pub mod class46;
pub mod production_building;
pub mod production_building_buff;

pub trait AnnoPtr {
    unsafe fn new(address: u64) -> Self;

    fn get_address(&self) -> u64;

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.get_address() + offset) as *const T).read_volatile() }
    }
}

impl<T> AnnoPtr for *const T {
    unsafe fn new(address: u64) -> Self {
        address as _
    }

    fn get_address(&self) -> u64 {
        *self as u64
    }
}

static ANNO1800_BASE: OnceLock<u64> = OnceLock::new();

pub fn get_module_base() -> u64 {
    *ANNO1800_BASE.get_or_init(|| {
        let base = unsafe { GetModuleHandleA(s!("Anno1800.exe")) }.unwrap();
        base.0.try_into().unwrap()
    })
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct BuildingType(pub u32);
pub const BLUEPRINT: BuildingType = BuildingType(0x0000_0000);
pub const POST_OFFICE: BuildingType = BuildingType(0x0000_0217);
pub const BOOTMAKERS: BuildingType = BuildingType(0x0001_befc);
pub const TAILORS_SHOP: BuildingType = BuildingType(0x0001_befe);
pub const LUMBERJACKS_HUT: BuildingType = BuildingType(0x0001_d4c8);
pub const VINEYARD: BuildingType = BuildingType(0x0001_d4ce);
pub const CHAMPAGNE_CELLAR: BuildingType = BuildingType(0x0001_d4d0);
pub const SPECTACLE_FACTORY: BuildingType = BuildingType(0x0001_d4de);
pub const GRAIN_FARM: BuildingType = BuildingType(0x000f_6a10);
pub const CATTLE_FARM: BuildingType = BuildingType(0x000f_6a11);
pub const HOP_FARM: BuildingType = BuildingType(0x000f_6a12);
pub const POTATO_FARM: BuildingType = BuildingType(0x000f_6a13);
pub const SAWMILL: BuildingType = BuildingType(0x000f_6a14);
pub const SHEEP_FARM: BuildingType = BuildingType(0x000f_6a15);
pub const RED_PEPPER_FARM: BuildingType = BuildingType(0x000f_6a16);
pub const PIG_FARM: BuildingType = BuildingType(0x000f_6a17);
pub const FISHERY: BuildingType = BuildingType(0x000f_6a18);
pub const CLAY_PIT: BuildingType = BuildingType(0x000f_6a19);
pub const CONCRETE_FACTORY: BuildingType = BuildingType(0x000f_6a1a);
pub const SOAP_FACTORY: BuildingType = BuildingType(0x000f_6a1b);
pub const BRASS_SMELTERY: BuildingType = BuildingType(0x000f_6a1c);
pub const BRICK_FACTORY: BuildingType = BuildingType(0x000f_6a1d);
pub const SEWING_MACHINE_FACTORY: BuildingType = BuildingType(0x000f_6a1e);
pub const WINDOW_MAKERS: BuildingType = BuildingType(0x000f_6a1f);
pub const LIGHT_BULB_FACTORY: BuildingType = BuildingType(0x000f_6a20);
pub const HUNTING_CABIN: BuildingType = BuildingType(0x000f_6a21);
pub const SAILMAKERS: BuildingType = BuildingType(0x000f_6a22);
pub const BAKERY: BuildingType = BuildingType(0x000f_6a25);
pub const BREWERY: BuildingType = BuildingType(0x000f_6a26);
pub const FUR_DEALER: BuildingType = BuildingType(0x000f_6a47);
pub const ARTISANAL_KITCHEN: BuildingType = BuildingType(0x000f_6a27);
pub const SCHNAPPS_DESTILLERY: BuildingType = BuildingType(0x000f_6a28);
pub const CANNERY: BuildingType = BuildingType(0x000f_6a29);
pub const STEELWORKS: BuildingType = BuildingType(0x000f_6a2a);
pub const FURNACE: BuildingType = BuildingType(0x000f_6a2b);
pub const WEAPON_FACTORY: BuildingType = BuildingType(0x000f_6a2d);
pub const DYNAMITE_FACTORY: BuildingType = BuildingType(0x000f_6a2e);
pub const HEAVY_WEAPONS_FACTORY: BuildingType = BuildingType(0x000f_6a2f);
pub const MOTOR_ASSEMBLY_LINE: BuildingType = BuildingType(0x000f_6a30);
pub const COAL_MINE: BuildingType = BuildingType(0x000f_6a32);
pub const IRON_MINE: BuildingType = BuildingType(0x000f_6a33);
pub const SAND_MINE: BuildingType = BuildingType(0x000f_6a34);
pub const COPPER_MINE: BuildingType = BuildingType(0x000f_6a36);
pub const LIMESTONE_QUARRY: BuildingType = BuildingType(0x000f_6a37);
pub const SALTPETRE_WORKS: BuildingType = BuildingType(0x000f_6a38);
pub const RENDERING_WORKS: BuildingType = BuildingType(0x000f_6a3a);
pub const SLAUGHTERHOUSE: BuildingType = BuildingType(0x000f_6a3e);
pub const FLOUR_MILL: BuildingType = BuildingType(0x000f_6a3b);
pub const MALTHOUSE: BuildingType = BuildingType(0x000f_6a3c);
pub const FRAMEWORK_KNITTERS: BuildingType = BuildingType(0x000f_6a3d);
pub const GLASSMAKERS: BuildingType = BuildingType(0x000f_6a41);
pub const MARQUETRY_WORKSHOP: BuildingType = BuildingType(0x000f_6a42);
pub const FILAMENT_FACTORY: BuildingType = BuildingType(0x000f_6a43);
pub const BICYCLE_FACTORY: BuildingType = BuildingType(0x000f_6a45);
pub const CLOCKMAKERS: BuildingType = BuildingType(0x000f_6a46);
pub const JEWELLERS: BuildingType = BuildingType(0x000f_6a4a);

impl Debug for BuildingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&match *self {
            BLUEPRINT => "Blueprint".to_string(),
            POST_OFFICE => "Post Office".to_string(),
            BOOTMAKERS => "Bootmakers".to_string(),
            TAILORS_SHOP => "Tailor's Shop".to_string(),
            LUMBERJACKS_HUT => "Lumberjack's Hut".to_string(),
            VINEYARD => "Vineyard".to_string(),
            CHAMPAGNE_CELLAR => "Champagne Cellar".to_string(),
            SPECTACLE_FACTORY => "Spectacle Factory".to_string(),
            GRAIN_FARM => "Grain Farm".to_string(),
            CATTLE_FARM => "Cattle Farm".to_string(),
            HOP_FARM => "Hop Farm".to_string(),
            POTATO_FARM => "Potato Farm".to_string(),
            SAWMILL => "Sawmill".to_string(),
            SHEEP_FARM => "Sheep Farm".to_string(),
            RED_PEPPER_FARM => "Red Pepper Farm".to_string(),
            PIG_FARM => "Pig Farm".to_string(),
            FISHERY => "Fishery".to_string(),
            CLAY_PIT => "Clay Pit".to_string(),
            CONCRETE_FACTORY => "Concrete Factory".to_string(),
            SOAP_FACTORY => "Soap Factory".to_string(),
            BRASS_SMELTERY => "Brass Smeltery".to_string(),
            BRICK_FACTORY => "Brick Factory".to_string(),
            SEWING_MACHINE_FACTORY => "Sewing Machine Factory".to_string(),
            WINDOW_MAKERS => "Window Makers".to_string(),
            LIGHT_BULB_FACTORY => "Light Bulb Factory".to_string(),
            HUNTING_CABIN => "Hunting Cabin".to_string(),
            SAILMAKERS => "Sailmakers".to_string(),
            BAKERY => "Bakery".to_string(),
            BREWERY => "Brewery".to_string(),
            FUR_DEALER => "Fur Dealer".to_string(),
            ARTISANAL_KITCHEN => "Artisanal Kitchen".to_string(),
            SCHNAPPS_DESTILLERY => "Schnapps Destillery".to_string(),
            CANNERY => "Cannery".to_string(),
            STEELWORKS => "Steelworks".to_string(),
            FURNACE => "Furnace".to_string(),
            WEAPON_FACTORY => "Weapon Factory".to_string(),
            DYNAMITE_FACTORY => "Dynamite Factory".to_string(),
            HEAVY_WEAPONS_FACTORY => "Heavy Weapons Factory".to_string(),
            MOTOR_ASSEMBLY_LINE => "Motor Assembly Line".to_string(),
            COAL_MINE => "Coal Mine".to_string(),
            IRON_MINE => "Iron Mine".to_string(),
            SAND_MINE => "Sand Mine".to_string(),
            COPPER_MINE => "Copper Mine".to_string(),
            LIMESTONE_QUARRY => "Limestone Quarry".to_string(),
            SALTPETRE_WORKS => "Saltpetre Works".to_string(),
            RENDERING_WORKS => "Rendering Works".to_string(),
            SLAUGHTERHOUSE => "Slaughterhouse".to_string(),
            FLOUR_MILL => "Flour Mill".to_string(),
            MALTHOUSE => "Malthouse".to_string(),
            FRAMEWORK_KNITTERS => "Framework Knitters".to_string(),
            GLASSMAKERS => "Glassmakers".to_string(),
            MARQUETRY_WORKSHOP => "Marquetry Workshop".to_string(),
            FILAMENT_FACTORY => "Filament Factory".to_string(),
            BICYCLE_FACTORY => "Bicycle Factory".to_string(),
            CLOCKMAKERS => "Clockmakers".to_string(),
            JEWELLERS => "Jewellers".to_string(),
            _ => format!("Unknown({:#010x})", self.0),
        })
    }
}

#[derive(PartialEq, Eq)]
pub struct WareType(pub u32);
pub const POTATOS: BuildingType = BuildingType(0x000f_6a13);
pub const WOOD: BuildingType = BuildingType(0x0001_d4c8);

pub struct HashMapPtr {
    pub address: u64,
}

impl AnnoPtr for HashMapPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}

impl HashMapPtr {
    pub fn get_entry(&self, key: u32) -> u64 {
        let hash = fnv1a_32(key);
        let bucket_address = self.get_data() + 16 * (hash & self.get_bitmask());
        let hashmap_end_marker = self.get_last();
        unsafe {
            let bucket = HashMapBucketPtr::new(bucket_address);
            let bucket_end_marker = bucket.get_last();
            let mut container = HashMapContainerPtr::new(bucket.get_first());
            loop {
                if container.address == hashmap_end_marker {
                    return 0;
                }
                if container.get_key() == key {
                    return container.get_data();
                }

                container = HashMapContainerPtr::new(container.get_next());

                if container.address == bucket_end_marker {
                    return 0;
                }
            }
        }
    }

    fn get_last(&self) -> u64 {
        self.get(0x0000)
    }

    fn get_data(&self) -> u64 {
        self.get(0x0010)
    }

    fn get_bitmask(&self) -> u64 {
        self.get(0x0028)
    }
}

pub struct HashMapBucketPtr {
    pub address: u64,
}

impl HashMapBucketPtr {
    fn get_last(&self) -> u64 {
        self.get(0x0000)
    }

    fn get_first(&self) -> u64 {
        self.get(0x0008)
    }
}

impl AnnoPtr for HashMapBucketPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}

pub struct HashMapContainerPtr {
    pub address: u64,
}

impl HashMapContainerPtr {
    fn get_next(&self) -> u64 {
        self.get(0x0008)
    }

    fn get_key(&self) -> u32 {
        self.get(0x0010)
    }

    fn get_data(&self) -> u64 {
        self.get(0x0018)
    }
}

impl AnnoPtr for HashMapContainerPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}

/// https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
fn fnv1a_32(input: u32) -> u64 {
    let bytes = input.to_le_bytes();
    let mut hash = 0xCBF29CE4_84222325u64;

    for byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }

    hash
}
