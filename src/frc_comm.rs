//! **Note**: It is recommended to use the WPILib HAL instead of using these functions directly.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw;

/* automatically generated by rust-bindgen (mostly) */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
}

pub const kAllianceStationID_red1: AllianceStationID_t = 0;
pub const kAllianceStationID_red2: AllianceStationID_t = 1;
pub const kAllianceStationID_red3: AllianceStationID_t = 2;
pub const kAllianceStationID_blue1: AllianceStationID_t = 3;
pub const kAllianceStationID_blue2: AllianceStationID_t = 4;
pub const kAllianceStationID_blue3: AllianceStationID_t = 5;
pub type AllianceStationID_t = u32;

pub const kMatchType_none: MatchType_t = 0;
pub const kMatchType_practice: MatchType_t = 1;
pub const kMatchType_qualification: MatchType_t = 2;
pub const kMatchType_elimination: MatchType_t = 3;
pub type MatchType_t = u32;

#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct ControlWord_t {
    _bitfield_1: __BindgenBitfieldUnit<[u8; 4], u32>,
}
#[test]
fn bindgen_test_layout_ControlWord_t() {
    assert_eq!(
        ::std::mem::size_of::<ControlWord_t>(),
        4usize,
        concat!("Size of: ", stringify!(ControlWord_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ControlWord_t>(),
        4usize,
        concat!("Alignment of ", stringify!(ControlWord_t))
    );
}
impl ControlWord_t {
    #[inline]
    pub fn enabled(self) -> bool {
        self._bitfield_1.get_bit(0)
    }
    #[inline]
    pub fn set_enabled(&mut self, val: bool) {
        self._bitfield_1.set_bit(0, val)
    }
    #[inline]
    pub fn autonomous(self) -> bool {
        self._bitfield_1.get_bit(1)
    }
    #[inline]
    pub fn set_autonomous(&mut self, val: bool) {
        self._bitfield_1.set_bit(1, val)
    }
    #[inline]
    pub fn test(self) -> bool {
        self._bitfield_1.get_bit(2)
    }
    #[inline]
    pub fn set_test(&mut self, val: bool) {
        self._bitfield_1.set_bit(2, val)
    }
    #[inline]
    pub fn estop(self) -> bool {
        self._bitfield_1.get_bit(3)
    }
    #[inline]
    pub fn set_estop(&mut self, val: bool) {
        self._bitfield_1.set_bit(3, val)
    }
    #[inline]
    pub fn fms_attached(self) -> bool {
        self._bitfield_1.get_bit(4)
    }
    #[inline]
    pub fn set_fms_attached(&mut self, val: bool) {
        self._bitfield_1.set_bit(4, val)
    }
    #[inline]
    pub fn ds_attached(self) -> bool {
        self._bitfield_1.get_bit(5)
    }
    #[inline]
    pub fn set_ds_attached(&mut self, val: bool) {
        self._bitfield_1.set_bit(5, val)
    }
    #[inline]
    pub fn new(
        enabled: bool,
        autonomous: bool,
        test: bool,
        estop: bool,
        fms_attached: bool,
        ds_attached: bool,
    ) -> Self {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> =
            Default::default();
        __bindgen_bitfield_unit.set_bit(0, enabled);
        __bindgen_bitfield_unit.set_bit(1, autonomous);
        __bindgen_bitfield_unit.set_bit(2, test);
        __bindgen_bitfield_unit.set_bit(3, estop);
        __bindgen_bitfield_unit.set_bit(4, fms_attached);
        __bindgen_bitfield_unit.set_bit(5, ds_attached);
        Self {
            _bitfield_1: __bindgen_bitfield_unit,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JoystickAxes_t {
    pub count: u16,
    pub axes: [i16; 1usize],
}
#[test]
fn bindgen_test_layout_JoystickAxes_t() {
    assert_eq!(
        ::std::mem::size_of::<JoystickAxes_t>(),
        4usize,
        concat!("Size of: ", stringify!(JoystickAxes_t))
    );
    assert_eq!(
        ::std::mem::align_of::<JoystickAxes_t>(),
        2usize,
        concat!("Alignment of ", stringify!(JoystickAxes_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<JoystickAxes_t>())).count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JoystickAxes_t),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<JoystickAxes_t>())).axes as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(JoystickAxes_t),
            "::",
            stringify!(axes)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JoystickPOV_t {
    pub count: u16,
    pub povs: [i16; 1usize],
}
#[test]
fn bindgen_test_layout_JoystickPOV_t() {
    assert_eq!(
        ::std::mem::size_of::<JoystickPOV_t>(),
        4usize,
        concat!("Size of: ", stringify!(JoystickPOV_t))
    );
    assert_eq!(
        ::std::mem::align_of::<JoystickPOV_t>(),
        2usize,
        concat!("Alignment of ", stringify!(JoystickPOV_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<JoystickPOV_t>())).count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JoystickPOV_t),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<JoystickPOV_t>())).povs as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(JoystickPOV_t),
            "::",
            stringify!(povs)
        )
    );
}
extern "C" {
    pub fn FRC_NetworkCommunication_Reserve(instance: *mut raw::c_void) -> raw::c_int;
}
extern "C" {
    #[doc = " Send a console output line to the Driver Station"]
    #[doc = " @param line a null-terminated string"]
    #[doc = " @return 0 on success, other on failure"]
    pub fn FRC_NetworkCommunication_sendConsoleLine(line: *const raw::c_char) -> raw::c_int;
}
extern "C" {
    #[doc = " Send an error to the Driver Station"]
    #[doc = " @param isError true if error, false if warning"]
    #[doc = " @param errorCode value of error condition"]
    #[doc = " @param isLVCode true if error code is defined in errors.txt, false if not (i.e. made up for C++)"]
    #[doc = " @param details error description that contains details such as which resource number caused the failure"]
    #[doc = " @param location Source file, function, and line number that the error was generated - optional"]
    #[doc = " @param callStack The details about what functions were called through before the error was reported - optional"]
    #[doc = " @return 0 on success, other on failure"]
    pub fn FRC_NetworkCommunication_sendError(
        isError: raw::c_int,
        errorCode: i32,
        isLVCode: raw::c_int,
        details: *const raw::c_char,
        location: *const raw::c_char,
        callStack: *const raw::c_char,
    ) -> raw::c_int;
}
extern "C" {
    pub fn FRC_NetworkCommunication_getControlWord(controlWord: *mut ControlWord_t) -> raw::c_int;
}
extern "C" {
    pub fn FRC_NetworkCommunication_getWatchdogActive() -> raw::c_int;
}
extern "C" {
    pub fn FRC_NetworkCommunication_getAllianceStation(
        allianceStation: *mut AllianceStationID_t,
    ) -> raw::c_int;
}
extern "C" {
    pub fn FRC_NetworkCommunication_getMatchInfo(
        eventName: *mut raw::c_char,
        matchType: *mut MatchType_t,
        matchNumber: *mut u16,
        replayNumber: *mut u8,
        gameSpecificMessage: *mut u8,
        gameSpecificMessageSize: *mut u16,
    ) -> raw::c_int;
}
extern "C" {
    pub fn FRC_NetworkCommunication_getMatchTime(matchTime: *mut f32) -> raw::c_int;
}
extern "C" {
    pub fn FRC_NetworkCommunication_getJoystickAxes(
        joystickNum: u8,
        axes: *mut JoystickAxes_t,
        maxAxes: u8,
    ) -> raw::c_int;
}
extern "C" {
    pub fn FRC_NetworkCommunication_getJoystickButtons(
        joystickNum: u8,
        buttons: *mut u32,
        count: *mut u8,
    ) -> raw::c_int;
}
extern "C" {
    pub fn FRC_NetworkCommunication_getJoystickPOVs(
        joystickNum: u8,
        povs: *mut JoystickPOV_t,
        maxPOVs: u8,
    ) -> raw::c_int;
}
extern "C" {
    pub fn FRC_NetworkCommunication_setJoystickOutputs(
        joystickNum: u8,
        hidOutputs: u32,
        leftRumble: u16,
        rightRumble: u16,
    ) -> raw::c_int;
}
extern "C" {
    pub fn FRC_NetworkCommunication_getJoystickDesc(
        joystickNum: u8,
        isXBox: *mut u8,
        type_: *mut u8,
        name: *mut raw::c_char,
        axisCount: *mut u8,
        axisTypes: *mut u8,
        buttonCount: *mut u8,
        povCount: *mut u8,
    ) -> raw::c_int;
}
extern "C" {
    pub fn FRC_NetworkCommunication_getVersionString(version: *mut raw::c_char);
}
extern "C" {
    pub fn FRC_NetworkCommunication_observeUserProgramStarting() -> raw::c_int;
}
extern "C" {
    pub fn FRC_NetworkCommunication_observeUserProgramDisabled();
}
extern "C" {
    pub fn FRC_NetworkCommunication_observeUserProgramAutonomous();
}
extern "C" {
    pub fn FRC_NetworkCommunication_observeUserProgramTeleop();
}
extern "C" {
    pub fn FRC_NetworkCommunication_observeUserProgramTest();
}
