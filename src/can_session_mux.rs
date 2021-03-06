//! CANSessionMux.h
//!
//! Defines the API for building a CAN Interface Plugin to support
//! PWM-cable-free CAN motor control on FRC robots.  This allows you
//! to connect any CAN interface to the secure Jaguar CAN driver.

#![allow(non_upper_case_globals)]

pub use self::{
    FRC_NetworkCommunication_CANSessionMux_closeStreamSession as close_stream,
    FRC_NetworkCommunication_CANSessionMux_getCANStatus as get_can_status,
    FRC_NetworkCommunication_CANSessionMux_openStreamSession as open_stream,
    FRC_NetworkCommunication_CANSessionMux_readStreamSession as read_stream,
    FRC_NetworkCommunication_CANSessionMux_receiveMessage as receive_message,
    FRC_NetworkCommunication_CANSessionMux_sendMessage as send_message,
};

#[allow(non_camel_case_types)]
type tCANStreamMessage = CANStreamMessage;

/* automatically generated by rust-bindgen */

pub const CAN_SEND_PERIOD_NO_REPEAT: i32 = 0;
pub const CAN_SEND_PERIOD_STOP_REPEATING: i32 = -1;

/* Flags in the upper bits of the messageID */
pub const CAN_IS_FRAME_REMOTE: u32 = 0x80000000;
pub const CAN_IS_FRAME_11BIT: u32 = 0x40000000;

pub const ERR_CANSessionMux_InvalidBuffer: i32 = -44086;
pub const ERR_CANSessionMux_MessageNotFound: i32 = -44087;
pub const WARN_CANSessionMux_NoToken: i32 = 44087;
pub const ERR_CANSessionMux_NotAllowed: i32 = -44088;
pub const ERR_CANSessionMux_NotInitialized: i32 = -44089;
pub const ERR_CANSessionMux_SessionOverrun: i32 = 44050;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CANStreamMessage {
    pub message_id: u32,
    pub timestamp: u32,
    pub data: [u8; 8usize],
    pub data_size: u8,
}

#[test]
fn bindgen_test_layout_tCANStreamMessage() {
    assert_eq!(
        ::std::mem::size_of::<tCANStreamMessage>(),
        20usize,
        concat!("Size of: ", stringify!(tCANStreamMessage))
    );
    assert_eq!(
        ::std::mem::align_of::<tCANStreamMessage>(),
        4usize,
        concat!("Alignment of ", stringify!(tCANStreamMessage))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tCANStreamMessage>())).message_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tCANStreamMessage),
            "::",
            stringify!(messageID)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tCANStreamMessage>())).timestamp as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tCANStreamMessage),
            "::",
            stringify!(timeStamp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tCANStreamMessage>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tCANStreamMessage),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tCANStreamMessage>())).data_size as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tCANStreamMessage),
            "::",
            stringify!(dataSize)
        )
    );
}
extern "C" {
    pub fn FRC_NetworkCommunication_CANSessionMux_sendMessage(
        messageID: u32,
        data: *const u8,
        dataSize: u8,
        periodMs: i32,
        status: *mut i32,
    );
}
extern "C" {
    pub fn FRC_NetworkCommunication_CANSessionMux_receiveMessage(
        messageID: *mut u32,
        messageIDMask: u32,
        data: *mut u8,
        dataSize: *mut u8,
        timeStamp: *mut u32,
        status: *mut i32,
    );
}
extern "C" {
    pub fn FRC_NetworkCommunication_CANSessionMux_openStreamSession(
        sessionHandle: *mut u32,
        messageID: u32,
        messageIDMask: u32,
        maxMessages: u32,
        status: *mut i32,
    );
}
extern "C" {
    pub fn FRC_NetworkCommunication_CANSessionMux_closeStreamSession(sessionHandle: u32);
}
extern "C" {
    pub fn FRC_NetworkCommunication_CANSessionMux_readStreamSession(
        sessionHandle: u32,
        messages: *mut tCANStreamMessage,
        messagesToRead: u32,
        messagesRead: *mut u32,
        status: *mut i32,
    );
}
extern "C" {
    pub fn FRC_NetworkCommunication_CANSessionMux_getCANStatus(
        percentBusUtilization: *mut f32,
        busOffCount: *mut u32,
        txFullCount: *mut u32,
        receiveErrorCount: *mut u32,
        transmitErrorCount: *mut u32,
        status: *mut i32,
    );
}
