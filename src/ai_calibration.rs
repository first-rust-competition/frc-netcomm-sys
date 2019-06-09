extern "C" {
    #[link_name = "FRC_NetworkCommunication_nAICalibration_getLSBWeight"]
    pub fn get_lsb_weight(aiSystemIndex: u32, channel: u32, status: *mut i32) -> u32;

    #[link_name = "FRC_NetworkCommunication_nAICalibration_getOffset"]
    pub fn get_offset(aiSystemIndex: u32, channel: u32, status: *mut i32) -> i32;
}
