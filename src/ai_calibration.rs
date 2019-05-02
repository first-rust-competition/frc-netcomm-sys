pub use self::{
    FRC_NetworkCommunication_nAICalibration_getLSBWeight as get_lsb_weight,
    FRC_NetworkCommunication_nAICalibration_getOffset as get_offset,
};

/* automatically generated by rust-bindgen */

extern "C" {
    pub fn FRC_NetworkCommunication_nAICalibration_getLSBWeight(
        aiSystemIndex: u32,
        channel: u32,
        status: *mut i32,
    ) -> u32;
    pub fn FRC_NetworkCommunication_nAICalibration_getOffset(
        aiSystemIndex: u32,
        channel: u32,
        status: *mut i32,
    ) -> i32;
}
