use std::fmt::Debug;
pub mod class32;
pub mod class33;
pub mod class34;
pub mod class4;

#[derive(PartialEq, Eq)]
pub struct BuildingType(pub u32);
pub const LUMBERJACKS_HUT: BuildingType = BuildingType(0x0001_d4c8);
pub const POTATO_FARM: BuildingType = BuildingType(0x000f_6a13);
pub const SAWMILL: BuildingType = BuildingType(0x000f_6a14);
pub const SHEEP_FARM: BuildingType = BuildingType(0x000f_6a15);
pub const FISHERY: BuildingType = BuildingType(0x000f_6a18);
pub const SCHNAPPS_DESTILLERY: BuildingType = BuildingType(0x000f_6a28);
pub const FRAMEWORK_KNITTERS: BuildingType = BuildingType(0x000f_6a3d);

pub struct WareType(pub u32);
pub const POTATOS: BuildingType = BuildingType(0x000f_6a13);
pub const WOOD: BuildingType = BuildingType(0x0001_d4c8);

impl Debug for BuildingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LUMBERJACKS_HUT => f.write_str("Lumberjack's Hut"),
            POTATO_FARM => f.write_str("Potato Farm"),
            SAWMILL => f.write_str("Sawmill"),
            SHEEP_FARM => f.write_str("Sheep Farm"),
            FISHERY => f.write_str("Fishery"),
            SCHNAPPS_DESTILLERY => f.write_str("Schnapps Destillery"),
            FRAMEWORK_KNITTERS => f.write_str("Framework Knitters"),
            _ => f.write_str(&format!("Unknown({:010x}", self.0)),
        }
    }
}
