use std::{fmt::Debug, sync::OnceLock};

use windows::{s, Win32::System::LibraryLoader::GetModuleHandleA};
pub mod class11;
pub mod class20;
pub mod class32;
pub mod class33;
pub mod class34;
pub mod class4;
pub mod class46;

pub trait AnnoPtr {
    unsafe fn new(address: u64) -> Self;

    fn get_address(&self) -> u64;

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.get_address() + offset) as *const T).read_volatile() }
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
pub const LUMBERJACKS_HUT: BuildingType = BuildingType(0x0001_d4c8);
pub const GRAIN_FARM: BuildingType = BuildingType(0x000f_6a10);
pub const HOP_FARM: BuildingType = BuildingType(0x000f_6a12);
pub const POTATO_FARM: BuildingType = BuildingType(0x000f_6a13);
pub const SAWMILL: BuildingType = BuildingType(0x000f_6a14);
pub const SHEEP_FARM: BuildingType = BuildingType(0x000f_6a15);
pub const PIG_FARM: BuildingType = BuildingType(0x000f_6a17);
pub const FISHERY: BuildingType = BuildingType(0x000f_6a18);
pub const CLAY_PIT: BuildingType = BuildingType(0x000f_6a19);
pub const BRICK_FACTORY: BuildingType = BuildingType(0x000f_6a1d);
pub const BAKERY: BuildingType = BuildingType(0x000f_6a25);
pub const BREWERY: BuildingType = BuildingType(0x000f_6a26);
pub const SCHNAPPS_DESTILLERY: BuildingType = BuildingType(0x000f_6a28);
pub const COAL_MINE: BuildingType = BuildingType(0x000f_6a32);
pub const IRON_MINE: BuildingType = BuildingType(0x000f_6a33);
pub const COPPER_MINE: BuildingType = BuildingType(0x000f_6a36);
pub const LIMESTONE_QUARRY: BuildingType = BuildingType(0x000f_6a37);
pub const RENDERING_WORKS: BuildingType = BuildingType(0x000f_6a3a);
pub const SLAUGHTERHOUSE: BuildingType = BuildingType(0x000f_6a3e);
pub const FLOUR_MILL: BuildingType = BuildingType(0x000f_6a3b);
pub const MALTHOUSE: BuildingType = BuildingType(0x000f_6a3c);
pub const FRAMEWORK_KNITTERS: BuildingType = BuildingType(0x000f_6a3d);

impl Debug for BuildingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            BLUEPRINT => f.write_str("Blueprint"),
            POST_OFFICE => f.write_str("Post Office"),
            LUMBERJACKS_HUT => f.write_str("Lumberjack's Hut"),
            GRAIN_FARM => f.write_str("Grain Farm"),
            HOP_FARM => f.write_str("Hop Farm"),
            POTATO_FARM => f.write_str("Potato Farm"),
            SAWMILL => f.write_str("Sawmill"),
            SHEEP_FARM => f.write_str("Sheep Farm"),
            PIG_FARM => f.write_str("Pig Farm"),
            FISHERY => f.write_str("Fishery"),
            CLAY_PIT => f.write_str("Clay Pit"),
            BRICK_FACTORY => f.write_str("Brick Factory"),
            BAKERY => f.write_str("Bakery"),
            BREWERY => f.write_str("Brewery"),
            SCHNAPPS_DESTILLERY => f.write_str("Schnapps Destillery"),
            COAL_MINE => f.write_str("Coal Mine"),
            IRON_MINE => f.write_str("Iron Mine"),
            COPPER_MINE => f.write_str("Copper Mine"),
            LIMESTONE_QUARRY => f.write_str("Limestone Quarry"),
            RENDERING_WORKS => f.write_str("Rendering Works"),
            SLAUGHTERHOUSE => f.write_str("Slaughterhouse"),
            FLOUR_MILL => f.write_str("Flour Mill"),
            MALTHOUSE => f.write_str("Malthouse"),
            FRAMEWORK_KNITTERS => f.write_str("Framework Knitters"),
            _ => f.write_str(&format!("Unknown({:#010x})", self.0)),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct WareType(pub u32);
pub const POTATOS: BuildingType = BuildingType(0x000f_6a13);
pub const WOOD: BuildingType = BuildingType(0x0001_d4c8);

pub struct BoxedArrayListPtr {
    pub address: u64,
}

impl BoxedArrayListPtr {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    pub fn get_first_box(&self) -> u64 {
        self.get(0x00)
    }

    pub fn get_last_box(&self) -> u64 {
        self.get(0x08)
    }

    pub fn get_all<T: AnnoPtr>(&self) -> Vec<T> {
        let mut elements = vec![];
        let mut current = self.get_first_box() as *const u64;
        let last = self.get_last_box() as *const u64;
        while current != last {
            unsafe {
                let ptr = T::new(current.read());
                elements.push(ptr);
                current = current.add(1);
            }
        }

        elements
    }

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.address + offset) as *const T).read_volatile() }
    }
}

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
