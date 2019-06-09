use std::os::raw::c_char;

extern "C" {
    /**
     * Report the usage of a resource of interest.
     *
     * @param resource one of the values in the tResourceType above (max value 51).
     * @param instanceNumber an index that identifies the resource instance.
     * @param context an optional additional context number for some cases (such as module number).  Set to 0 to omit.
     * @param feature a string to be included describing features in use on a specific resource.  Setting the same resource more than once allows you to change the feature string.
     */
    #[link_name = "FRC_NetworkCommunication_nUsageReporting_report"]
    pub fn report(resource: u8, instanceNumber: u8, context: u8, feature: *const c_char) -> u32;
}
