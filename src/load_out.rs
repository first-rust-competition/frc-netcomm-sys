pub const MAX_MODULE_NUMBER: u8 = 2;

pub mod target_classes {
    pub type Type = u32;

    pub const UNKNOWN: Type = 0x00;
    pub const FRC1: Type = 0x10;
    pub const FRC2: Type = 0x20;
    pub const FRC3: Type = 0x30;
    pub const ROBORIO: Type = 0x40;

    pub const FAMILY_MASK: Type = 0xF0;
    pub const MODULE_MASK: Type = 0x0F;
}

extern "C" {
    #[link_name = "FRC_NetworkCommunication_nLoadOut_getTargetClass"]
    pub fn get_target_class() -> u32;
}
