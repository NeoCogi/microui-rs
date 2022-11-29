#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
//
// If you need to have the smallest executable, use no_std:
//
#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

// #[no_mangle]
// pub extern "C" fn main() {}

extern crate libc;
mod fixed_collections;
#[path = "./microui.rs"]
pub mod microui;
#[path = "./renderer.rs"]
pub mod renderer;

pub type SDL_SysWMmsg = libc::c_int;

use microui::*;
use renderer::*;
use fixed_collections::*;
use core::fmt::*;
use core::format_args;

//use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
    fn SDL_Init(flags: Uint32) -> libc::c_int;
    fn SDL_PollEvent(event: *mut SDL_Event) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type Uint8 = uint8_t;
pub type Sint16 = int16_t;
pub type Uint16 = uint16_t;
pub type Sint32 = int32_t;
pub type Uint32 = uint32_t;
pub type Sint64 = int64_t;
pub type SDL_Scancode = libc::c_uint;
pub const SDL_NUM_SCANCODES: SDL_Scancode = 512;
pub const SDL_SCANCODE_AUDIOFASTFORWARD: SDL_Scancode = 286;
pub const SDL_SCANCODE_AUDIOREWIND: SDL_Scancode = 285;
pub const SDL_SCANCODE_APP2: SDL_Scancode = 284;
pub const SDL_SCANCODE_APP1: SDL_Scancode = 283;
pub const SDL_SCANCODE_SLEEP: SDL_Scancode = 282;
pub const SDL_SCANCODE_EJECT: SDL_Scancode = 281;
pub const SDL_SCANCODE_KBDILLUMUP: SDL_Scancode = 280;
pub const SDL_SCANCODE_KBDILLUMDOWN: SDL_Scancode = 279;
pub const SDL_SCANCODE_KBDILLUMTOGGLE: SDL_Scancode = 278;
pub const SDL_SCANCODE_DISPLAYSWITCH: SDL_Scancode = 277;
pub const SDL_SCANCODE_BRIGHTNESSUP: SDL_Scancode = 276;
pub const SDL_SCANCODE_BRIGHTNESSDOWN: SDL_Scancode = 275;
pub const SDL_SCANCODE_AC_BOOKMARKS: SDL_Scancode = 274;
pub const SDL_SCANCODE_AC_REFRESH: SDL_Scancode = 273;
pub const SDL_SCANCODE_AC_STOP: SDL_Scancode = 272;
pub const SDL_SCANCODE_AC_FORWARD: SDL_Scancode = 271;
pub const SDL_SCANCODE_AC_BACK: SDL_Scancode = 270;
pub const SDL_SCANCODE_AC_HOME: SDL_Scancode = 269;
pub const SDL_SCANCODE_AC_SEARCH: SDL_Scancode = 268;
pub const SDL_SCANCODE_COMPUTER: SDL_Scancode = 267;
pub const SDL_SCANCODE_CALCULATOR: SDL_Scancode = 266;
pub const SDL_SCANCODE_MAIL: SDL_Scancode = 265;
pub const SDL_SCANCODE_WWW: SDL_Scancode = 264;
pub const SDL_SCANCODE_MEDIASELECT: SDL_Scancode = 263;
pub const SDL_SCANCODE_AUDIOMUTE: SDL_Scancode = 262;
pub const SDL_SCANCODE_AUDIOPLAY: SDL_Scancode = 261;
pub const SDL_SCANCODE_AUDIOSTOP: SDL_Scancode = 260;
pub const SDL_SCANCODE_AUDIOPREV: SDL_Scancode = 259;
pub const SDL_SCANCODE_AUDIONEXT: SDL_Scancode = 258;
pub const SDL_SCANCODE_MODE: SDL_Scancode = 257;
pub const SDL_SCANCODE_RGUI: SDL_Scancode = 231;
pub const SDL_SCANCODE_RALT: SDL_Scancode = 230;
pub const SDL_SCANCODE_RSHIFT: SDL_Scancode = 229;
pub const SDL_SCANCODE_RCTRL: SDL_Scancode = 228;
pub const SDL_SCANCODE_LGUI: SDL_Scancode = 227;
pub const SDL_SCANCODE_LALT: SDL_Scancode = 226;
pub const SDL_SCANCODE_LSHIFT: SDL_Scancode = 225;
pub const SDL_SCANCODE_LCTRL: SDL_Scancode = 224;
pub const SDL_SCANCODE_KP_HEXADECIMAL: SDL_Scancode = 221;
pub const SDL_SCANCODE_KP_DECIMAL: SDL_Scancode = 220;
pub const SDL_SCANCODE_KP_OCTAL: SDL_Scancode = 219;
pub const SDL_SCANCODE_KP_BINARY: SDL_Scancode = 218;
pub const SDL_SCANCODE_KP_CLEARENTRY: SDL_Scancode = 217;
pub const SDL_SCANCODE_KP_CLEAR: SDL_Scancode = 216;
pub const SDL_SCANCODE_KP_PLUSMINUS: SDL_Scancode = 215;
pub const SDL_SCANCODE_KP_MEMDIVIDE: SDL_Scancode = 214;
pub const SDL_SCANCODE_KP_MEMMULTIPLY: SDL_Scancode = 213;
pub const SDL_SCANCODE_KP_MEMSUBTRACT: SDL_Scancode = 212;
pub const SDL_SCANCODE_KP_MEMADD: SDL_Scancode = 211;
pub const SDL_SCANCODE_KP_MEMCLEAR: SDL_Scancode = 210;
pub const SDL_SCANCODE_KP_MEMRECALL: SDL_Scancode = 209;
pub const SDL_SCANCODE_KP_MEMSTORE: SDL_Scancode = 208;
pub const SDL_SCANCODE_KP_EXCLAM: SDL_Scancode = 207;
pub const SDL_SCANCODE_KP_AT: SDL_Scancode = 206;
pub const SDL_SCANCODE_KP_SPACE: SDL_Scancode = 205;
pub const SDL_SCANCODE_KP_HASH: SDL_Scancode = 204;
pub const SDL_SCANCODE_KP_COLON: SDL_Scancode = 203;
pub const SDL_SCANCODE_KP_DBLVERTICALBAR: SDL_Scancode = 202;
pub const SDL_SCANCODE_KP_VERTICALBAR: SDL_Scancode = 201;
pub const SDL_SCANCODE_KP_DBLAMPERSAND: SDL_Scancode = 200;
pub const SDL_SCANCODE_KP_AMPERSAND: SDL_Scancode = 199;
pub const SDL_SCANCODE_KP_GREATER: SDL_Scancode = 198;
pub const SDL_SCANCODE_KP_LESS: SDL_Scancode = 197;
pub const SDL_SCANCODE_KP_PERCENT: SDL_Scancode = 196;
pub const SDL_SCANCODE_KP_POWER: SDL_Scancode = 195;
pub const SDL_SCANCODE_KP_XOR: SDL_Scancode = 194;
pub const SDL_SCANCODE_KP_F: SDL_Scancode = 193;
pub const SDL_SCANCODE_KP_E: SDL_Scancode = 192;
pub const SDL_SCANCODE_KP_D: SDL_Scancode = 191;
pub const SDL_SCANCODE_KP_C: SDL_Scancode = 190;
pub const SDL_SCANCODE_KP_B: SDL_Scancode = 189;
pub const SDL_SCANCODE_KP_A: SDL_Scancode = 188;
pub const SDL_SCANCODE_KP_BACKSPACE: SDL_Scancode = 187;
pub const SDL_SCANCODE_KP_TAB: SDL_Scancode = 186;
pub const SDL_SCANCODE_KP_RIGHTBRACE: SDL_Scancode = 185;
pub const SDL_SCANCODE_KP_LEFTBRACE: SDL_Scancode = 184;
pub const SDL_SCANCODE_KP_RIGHTPAREN: SDL_Scancode = 183;
pub const SDL_SCANCODE_KP_LEFTPAREN: SDL_Scancode = 182;
pub const SDL_SCANCODE_CURRENCYSUBUNIT: SDL_Scancode = 181;
pub const SDL_SCANCODE_CURRENCYUNIT: SDL_Scancode = 180;
pub const SDL_SCANCODE_DECIMALSEPARATOR: SDL_Scancode = 179;
pub const SDL_SCANCODE_THOUSANDSSEPARATOR: SDL_Scancode = 178;
pub const SDL_SCANCODE_KP_000: SDL_Scancode = 177;
pub const SDL_SCANCODE_KP_00: SDL_Scancode = 176;
pub const SDL_SCANCODE_EXSEL: SDL_Scancode = 164;
pub const SDL_SCANCODE_CRSEL: SDL_Scancode = 163;
pub const SDL_SCANCODE_CLEARAGAIN: SDL_Scancode = 162;
pub const SDL_SCANCODE_OPER: SDL_Scancode = 161;
pub const SDL_SCANCODE_OUT: SDL_Scancode = 160;
pub const SDL_SCANCODE_SEPARATOR: SDL_Scancode = 159;
pub const SDL_SCANCODE_RETURN2: SDL_Scancode = 158;
pub const SDL_SCANCODE_PRIOR: SDL_Scancode = 157;
pub const SDL_SCANCODE_CLEAR: SDL_Scancode = 156;
pub const SDL_SCANCODE_CANCEL: SDL_Scancode = 155;
pub const SDL_SCANCODE_SYSREQ: SDL_Scancode = 154;
pub const SDL_SCANCODE_ALTERASE: SDL_Scancode = 153;
pub const SDL_SCANCODE_LANG9: SDL_Scancode = 152;
pub const SDL_SCANCODE_LANG8: SDL_Scancode = 151;
pub const SDL_SCANCODE_LANG7: SDL_Scancode = 150;
pub const SDL_SCANCODE_LANG6: SDL_Scancode = 149;
pub const SDL_SCANCODE_LANG5: SDL_Scancode = 148;
pub const SDL_SCANCODE_LANG4: SDL_Scancode = 147;
pub const SDL_SCANCODE_LANG3: SDL_Scancode = 146;
pub const SDL_SCANCODE_LANG2: SDL_Scancode = 145;
pub const SDL_SCANCODE_LANG1: SDL_Scancode = 144;
pub const SDL_SCANCODE_INTERNATIONAL9: SDL_Scancode = 143;
pub const SDL_SCANCODE_INTERNATIONAL8: SDL_Scancode = 142;
pub const SDL_SCANCODE_INTERNATIONAL7: SDL_Scancode = 141;
pub const SDL_SCANCODE_INTERNATIONAL6: SDL_Scancode = 140;
pub const SDL_SCANCODE_INTERNATIONAL5: SDL_Scancode = 139;
pub const SDL_SCANCODE_INTERNATIONAL4: SDL_Scancode = 138;
pub const SDL_SCANCODE_INTERNATIONAL3: SDL_Scancode = 137;
pub const SDL_SCANCODE_INTERNATIONAL2: SDL_Scancode = 136;
pub const SDL_SCANCODE_INTERNATIONAL1: SDL_Scancode = 135;
pub const SDL_SCANCODE_KP_EQUALSAS400: SDL_Scancode = 134;
pub const SDL_SCANCODE_KP_COMMA: SDL_Scancode = 133;
pub const SDL_SCANCODE_VOLUMEDOWN: SDL_Scancode = 129;
pub const SDL_SCANCODE_VOLUMEUP: SDL_Scancode = 128;
pub const SDL_SCANCODE_MUTE: SDL_Scancode = 127;
pub const SDL_SCANCODE_FIND: SDL_Scancode = 126;
pub const SDL_SCANCODE_PASTE: SDL_Scancode = 125;
pub const SDL_SCANCODE_COPY: SDL_Scancode = 124;
pub const SDL_SCANCODE_CUT: SDL_Scancode = 123;
pub const SDL_SCANCODE_UNDO: SDL_Scancode = 122;
pub const SDL_SCANCODE_AGAIN: SDL_Scancode = 121;
pub const SDL_SCANCODE_STOP: SDL_Scancode = 120;
pub const SDL_SCANCODE_SELECT: SDL_Scancode = 119;
pub const SDL_SCANCODE_MENU: SDL_Scancode = 118;
pub const SDL_SCANCODE_HELP: SDL_Scancode = 117;
pub const SDL_SCANCODE_EXECUTE: SDL_Scancode = 116;
pub const SDL_SCANCODE_F24: SDL_Scancode = 115;
pub const SDL_SCANCODE_F23: SDL_Scancode = 114;
pub const SDL_SCANCODE_F22: SDL_Scancode = 113;
pub const SDL_SCANCODE_F21: SDL_Scancode = 112;
pub const SDL_SCANCODE_F20: SDL_Scancode = 111;
pub const SDL_SCANCODE_F19: SDL_Scancode = 110;
pub const SDL_SCANCODE_F18: SDL_Scancode = 109;
pub const SDL_SCANCODE_F17: SDL_Scancode = 108;
pub const SDL_SCANCODE_F16: SDL_Scancode = 107;
pub const SDL_SCANCODE_F15: SDL_Scancode = 106;
pub const SDL_SCANCODE_F14: SDL_Scancode = 105;
pub const SDL_SCANCODE_F13: SDL_Scancode = 104;
pub const SDL_SCANCODE_KP_EQUALS: SDL_Scancode = 103;
pub const SDL_SCANCODE_POWER: SDL_Scancode = 102;
pub const SDL_SCANCODE_APPLICATION: SDL_Scancode = 101;
pub const SDL_SCANCODE_NONUSBACKSLASH: SDL_Scancode = 100;
pub const SDL_SCANCODE_KP_PERIOD: SDL_Scancode = 99;
pub const SDL_SCANCODE_KP_0: SDL_Scancode = 98;
pub const SDL_SCANCODE_KP_9: SDL_Scancode = 97;
pub const SDL_SCANCODE_KP_8: SDL_Scancode = 96;
pub const SDL_SCANCODE_KP_7: SDL_Scancode = 95;
pub const SDL_SCANCODE_KP_6: SDL_Scancode = 94;
pub const SDL_SCANCODE_KP_5: SDL_Scancode = 93;
pub const SDL_SCANCODE_KP_4: SDL_Scancode = 92;
pub const SDL_SCANCODE_KP_3: SDL_Scancode = 91;
pub const SDL_SCANCODE_KP_2: SDL_Scancode = 90;
pub const SDL_SCANCODE_KP_1: SDL_Scancode = 89;
pub const SDL_SCANCODE_KP_ENTER: SDL_Scancode = 88;
pub const SDL_SCANCODE_KP_PLUS: SDL_Scancode = 87;
pub const SDL_SCANCODE_KP_MINUS: SDL_Scancode = 86;
pub const SDL_SCANCODE_KP_MULTIPLY: SDL_Scancode = 85;
pub const SDL_SCANCODE_KP_DIVIDE: SDL_Scancode = 84;
pub const SDL_SCANCODE_NUMLOCKCLEAR: SDL_Scancode = 83;
pub const SDL_SCANCODE_UP: SDL_Scancode = 82;
pub const SDL_SCANCODE_DOWN: SDL_Scancode = 81;
pub const SDL_SCANCODE_LEFT: SDL_Scancode = 80;
pub const SDL_SCANCODE_RIGHT: SDL_Scancode = 79;
pub const SDL_SCANCODE_PAGEDOWN: SDL_Scancode = 78;
pub const SDL_SCANCODE_END: SDL_Scancode = 77;
pub const SDL_SCANCODE_DELETE: SDL_Scancode = 76;
pub const SDL_SCANCODE_PAGEUP: SDL_Scancode = 75;
pub const SDL_SCANCODE_HOME: SDL_Scancode = 74;
pub const SDL_SCANCODE_INSERT: SDL_Scancode = 73;
pub const SDL_SCANCODE_PAUSE: SDL_Scancode = 72;
pub const SDL_SCANCODE_SCROLLLOCK: SDL_Scancode = 71;
pub const SDL_SCANCODE_PRINTSCREEN: SDL_Scancode = 70;
pub const SDL_SCANCODE_F12: SDL_Scancode = 69;
pub const SDL_SCANCODE_F11: SDL_Scancode = 68;
pub const SDL_SCANCODE_F10: SDL_Scancode = 67;
pub const SDL_SCANCODE_F9: SDL_Scancode = 66;
pub const SDL_SCANCODE_F8: SDL_Scancode = 65;
pub const SDL_SCANCODE_F7: SDL_Scancode = 64;
pub const SDL_SCANCODE_F6: SDL_Scancode = 63;
pub const SDL_SCANCODE_F5: SDL_Scancode = 62;
pub const SDL_SCANCODE_F4: SDL_Scancode = 61;
pub const SDL_SCANCODE_F3: SDL_Scancode = 60;
pub const SDL_SCANCODE_F2: SDL_Scancode = 59;
pub const SDL_SCANCODE_F1: SDL_Scancode = 58;
pub const SDL_SCANCODE_CAPSLOCK: SDL_Scancode = 57;
pub const SDL_SCANCODE_SLASH: SDL_Scancode = 56;
pub const SDL_SCANCODE_PERIOD: SDL_Scancode = 55;
pub const SDL_SCANCODE_COMMA: SDL_Scancode = 54;
pub const SDL_SCANCODE_GRAVE: SDL_Scancode = 53;
pub const SDL_SCANCODE_APOSTROPHE: SDL_Scancode = 52;
pub const SDL_SCANCODE_SEMICOLON: SDL_Scancode = 51;
pub const SDL_SCANCODE_NONUSHASH: SDL_Scancode = 50;
pub const SDL_SCANCODE_BACKSLASH: SDL_Scancode = 49;
pub const SDL_SCANCODE_RIGHTBRACKET: SDL_Scancode = 48;
pub const SDL_SCANCODE_LEFTBRACKET: SDL_Scancode = 47;
pub const SDL_SCANCODE_EQUALS: SDL_Scancode = 46;
pub const SDL_SCANCODE_MINUS: SDL_Scancode = 45;
pub const SDL_SCANCODE_SPACE: SDL_Scancode = 44;
pub const SDL_SCANCODE_TAB: SDL_Scancode = 43;
pub const SDL_SCANCODE_BACKSPACE: SDL_Scancode = 42;
pub const SDL_SCANCODE_ESCAPE: SDL_Scancode = 41;
pub const SDL_SCANCODE_RETURN: SDL_Scancode = 40;
pub const SDL_SCANCODE_0: SDL_Scancode = 39;
pub const SDL_SCANCODE_9: SDL_Scancode = 38;
pub const SDL_SCANCODE_8: SDL_Scancode = 37;
pub const SDL_SCANCODE_7: SDL_Scancode = 36;
pub const SDL_SCANCODE_6: SDL_Scancode = 35;
pub const SDL_SCANCODE_5: SDL_Scancode = 34;
pub const SDL_SCANCODE_4: SDL_Scancode = 33;
pub const SDL_SCANCODE_3: SDL_Scancode = 32;
pub const SDL_SCANCODE_2: SDL_Scancode = 31;
pub const SDL_SCANCODE_1: SDL_Scancode = 30;
pub const SDL_SCANCODE_Z: SDL_Scancode = 29;
pub const SDL_SCANCODE_Y: SDL_Scancode = 28;
pub const SDL_SCANCODE_X: SDL_Scancode = 27;
pub const SDL_SCANCODE_W: SDL_Scancode = 26;
pub const SDL_SCANCODE_V: SDL_Scancode = 25;
pub const SDL_SCANCODE_U: SDL_Scancode = 24;
pub const SDL_SCANCODE_T: SDL_Scancode = 23;
pub const SDL_SCANCODE_S: SDL_Scancode = 22;
pub const SDL_SCANCODE_R: SDL_Scancode = 21;
pub const SDL_SCANCODE_Q: SDL_Scancode = 20;
pub const SDL_SCANCODE_P: SDL_Scancode = 19;
pub const SDL_SCANCODE_O: SDL_Scancode = 18;
pub const SDL_SCANCODE_N: SDL_Scancode = 17;
pub const SDL_SCANCODE_M: SDL_Scancode = 16;
pub const SDL_SCANCODE_L: SDL_Scancode = 15;
pub const SDL_SCANCODE_K: SDL_Scancode = 14;
pub const SDL_SCANCODE_J: SDL_Scancode = 13;
pub const SDL_SCANCODE_I: SDL_Scancode = 12;
pub const SDL_SCANCODE_H: SDL_Scancode = 11;
pub const SDL_SCANCODE_G: SDL_Scancode = 10;
pub const SDL_SCANCODE_F: SDL_Scancode = 9;
pub const SDL_SCANCODE_E: SDL_Scancode = 8;
pub const SDL_SCANCODE_D: SDL_Scancode = 7;
pub const SDL_SCANCODE_C: SDL_Scancode = 6;
pub const SDL_SCANCODE_B: SDL_Scancode = 5;
pub const SDL_SCANCODE_A: SDL_Scancode = 4;
pub const SDL_SCANCODE_UNKNOWN: SDL_Scancode = 0;
pub type SDL_Keycode = Sint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Keysym {
    pub scancode: SDL_Scancode,
    pub sym: SDL_Keycode,
    pub mod_0: Uint16,
    pub unused: Uint32,
}
pub type SDL_JoystickID = Sint32;
pub type SDL_TouchID = Sint64;
pub type SDL_FingerID = Sint64;
pub type SDL_GestureID = Sint64;
pub type C2RustUnnamed = libc::c_uint;
pub const SDL_LASTEVENT: C2RustUnnamed = 65535;
pub const SDL_USEREVENT: C2RustUnnamed = 32768;
pub const SDL_RENDER_DEVICE_RESET: C2RustUnnamed = 8193;
pub const SDL_RENDER_TARGETS_RESET: C2RustUnnamed = 8192;
pub const SDL_SENSORUPDATE: C2RustUnnamed = 4608;
pub const SDL_AUDIODEVICEREMOVED: C2RustUnnamed = 4353;
pub const SDL_AUDIODEVICEADDED: C2RustUnnamed = 4352;
pub const SDL_DROPCOMPLETE: C2RustUnnamed = 4099;
pub const SDL_DROPBEGIN: C2RustUnnamed = 4098;
pub const SDL_DROPTEXT: C2RustUnnamed = 4097;
pub const SDL_DROPFILE: C2RustUnnamed = 4096;
pub const SDL_CLIPBOARDUPDATE: C2RustUnnamed = 2304;
pub const SDL_MULTIGESTURE: C2RustUnnamed = 2050;
pub const SDL_DOLLARRECORD: C2RustUnnamed = 2049;
pub const SDL_DOLLARGESTURE: C2RustUnnamed = 2048;
pub const SDL_FINGERMOTION: C2RustUnnamed = 1794;
pub const SDL_FINGERUP: C2RustUnnamed = 1793;
pub const SDL_FINGERDOWN: C2RustUnnamed = 1792;
pub const SDL_CONTROLLERDEVICEREMAPPED: C2RustUnnamed = 1621;
pub const SDL_CONTROLLERDEVICEREMOVED: C2RustUnnamed = 1620;
pub const SDL_CONTROLLERDEVICEADDED: C2RustUnnamed = 1619;
pub const SDL_CONTROLLERBUTTONUP: C2RustUnnamed = 1618;
pub const SDL_CONTROLLERBUTTONDOWN: C2RustUnnamed = 1617;
pub const SDL_CONTROLLERAXISMOTION: C2RustUnnamed = 1616;
pub const SDL_JOYDEVICEREMOVED: C2RustUnnamed = 1542;
pub const SDL_JOYDEVICEADDED: C2RustUnnamed = 1541;
pub const SDL_JOYBUTTONUP: C2RustUnnamed = 1540;
pub const SDL_JOYBUTTONDOWN: C2RustUnnamed = 1539;
pub const SDL_JOYHATMOTION: C2RustUnnamed = 1538;
pub const SDL_JOYBALLMOTION: C2RustUnnamed = 1537;
pub const SDL_JOYAXISMOTION: C2RustUnnamed = 1536;
pub const SDL_MOUSEWHEEL: C2RustUnnamed = 1027;
pub const SDL_MOUSEBUTTONUP: C2RustUnnamed = 1026;
pub const SDL_MOUSEBUTTONDOWN: C2RustUnnamed = 1025;
pub const SDL_MOUSEMOTION: C2RustUnnamed = 1024;
pub const SDL_KEYMAPCHANGED: C2RustUnnamed = 772;
pub const SDL_TEXTINPUT: C2RustUnnamed = 771;
pub const SDL_TEXTEDITING: C2RustUnnamed = 770;
pub const SDL_KEYUP: C2RustUnnamed = 769;
pub const SDL_KEYDOWN: C2RustUnnamed = 768;
pub const SDL_SYSWMEVENT: C2RustUnnamed = 513;
pub const SDL_WINDOWEVENT: C2RustUnnamed = 512;
pub const SDL_DISPLAYEVENT: C2RustUnnamed = 336;
pub const SDL_APP_DIDENTERFOREGROUND: C2RustUnnamed = 262;
pub const SDL_APP_WILLENTERFOREGROUND: C2RustUnnamed = 261;
pub const SDL_APP_DIDENTERBACKGROUND: C2RustUnnamed = 260;
pub const SDL_APP_WILLENTERBACKGROUND: C2RustUnnamed = 259;
pub const SDL_APP_LOWMEMORY: C2RustUnnamed = 258;
pub const SDL_APP_TERMINATING: C2RustUnnamed = 257;
pub const SDL_QUIT: C2RustUnnamed = 256;
pub const SDL_FIRSTEVENT: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_CommonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DisplayEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub display: Uint32,
    pub event: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub data1: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_WindowEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub event: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub data1: Sint32,
    pub data2: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_KeyboardEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub state: Uint8,
    pub repeat: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub keysym: SDL_Keysym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextEditingEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub text: [libc::c_char; 32],
    pub start: Sint32,
    pub length: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextInputEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub text: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseMotionEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub state: Uint32,
    pub x: Sint32,
    pub y: Sint32,
    pub xrel: Sint32,
    pub yrel: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseButtonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub button: Uint8,
    pub state: Uint8,
    pub clicks: Uint8,
    pub padding1: Uint8,
    pub x: Sint32,
    pub y: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseWheelEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub x: Sint32,
    pub y: Sint32,
    pub direction: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyAxisEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub value: Sint16,
    pub padding4: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyBallEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub ball: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub xrel: Sint16,
    pub yrel: Sint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyHatEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub hat: Uint8,
    pub value: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyButtonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub button: Uint8,
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyDeviceEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerAxisEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub value: Sint16,
    pub padding4: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerButtonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub button: Uint8,
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerDeviceEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_AudioDeviceEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Uint32,
    pub iscapture: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TouchFingerEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub fingerId: SDL_FingerID,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub dx: libc::c_float,
    pub dy: libc::c_float,
    pub pressure: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MultiGestureEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub dTheta: libc::c_float,
    pub dDist: libc::c_float,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub numFingers: Uint16,
    pub padding: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DollarGestureEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub gestureId: SDL_GestureID,
    pub numFingers: Uint32,
    pub error: libc::c_float,
    pub x: libc::c_float,
    pub y: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DropEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub file: *mut libc::c_char,
    pub windowID: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SensorEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
    pub data: [libc::c_float; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_QuitEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_UserEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub code: Sint32,
    pub data1: *mut libc::c_void,
    pub data2: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SysWMEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub msg: *mut SDL_SysWMmsg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SDL_Event {
    pub type_0: Uint32,
    pub common: SDL_CommonEvent,
    pub display: SDL_DisplayEvent,
    pub window: SDL_WindowEvent,
    pub key: SDL_KeyboardEvent,
    pub edit: SDL_TextEditingEvent,
    pub text: SDL_TextInputEvent,
    pub motion: SDL_MouseMotionEvent,
    pub button: SDL_MouseButtonEvent,
    pub wheel: SDL_MouseWheelEvent,
    pub jaxis: SDL_JoyAxisEvent,
    pub jball: SDL_JoyBallEvent,
    pub jhat: SDL_JoyHatEvent,
    pub jbutton: SDL_JoyButtonEvent,
    pub jdevice: SDL_JoyDeviceEvent,
    pub caxis: SDL_ControllerAxisEvent,
    pub cbutton: SDL_ControllerButtonEvent,
    pub cdevice: SDL_ControllerDeviceEvent,
    pub adevice: SDL_AudioDeviceEvent,
    pub sensor: SDL_SensorEvent,
    pub quit: SDL_QuitEvent,
    pub user: SDL_UserEvent,
    pub syswm: SDL_SysWMEvent,
    pub tfinger: SDL_TouchFingerEvent,
    pub mgesture: SDL_MultiGestureEvent,
    pub dgesture: SDL_DollarGestureEvent,
    pub drop: SDL_DropEvent,
    pub padding: [Uint8; 56],
}

pub type Real = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13<'a> {
    pub label: &'a str,
    pub idx: ControlColor,
}
static mut bg: [libc::c_float; 3] = [
    90 as libc::c_int as libc::c_float,
    95 as libc::c_int as libc::c_float,
    100 as libc::c_int as libc::c_float,
];
fn write_log(logbuf: &mut dyn IString, logbuf_updated: &mut i32, text: &str) {
    if logbuf.len() != 0 {
        logbuf.push('\n');
    }
    for c in text.chars() {
        logbuf.push(c);
    }
    *logbuf_updated = 1 as libc::c_int;
}

fn test_window(logbuf: &mut dyn IString, logbuf_updated: &mut i32, ctx: &mut Context) {
    if !ctx
        .mu_begin_window_ex(
            "Demo Window",
            rect(40 as libc::c_int, 40 as libc::c_int, 300 as libc::c_int, 450 as libc::c_int),
            WidgetOption::None,
        )
        .is_none()
    {
        let mut win = ctx.mu_get_current_container_rect();
        win.w = if win.w > 240 as libc::c_int { win.w } else { 240 as libc::c_int };
        win.h = if win.h > 300 as libc::c_int { win.h } else { 300 as libc::c_int };

        ctx.set_current_container_rect(&win);

        let mut buff = FixedString::<128>::new();

        if !ctx.mu_header_ex("Window Info", WidgetOption::None).is_none() {
            let win_0 = ctx.mu_get_current_container_rect();
            ctx.mu_layout_row(2 as libc::c_int, [54 as libc::c_int, -(1 as libc::c_int)].as_mut_ptr(), 0 as libc::c_int);
            ctx.mu_label("Position:");
            buff.write_fmt(format_args!("{}, {}", win_0.x, win_0.y)).unwrap();
            ctx.mu_label(buff.as_str());
            buff.clear();
            ctx.mu_label("Size:");
            buff.write_fmt(format_args!("{}, {}", win_0.w, win_0.h)).unwrap();
            ctx.mu_label(buff.as_str());
        }
        if !ctx.mu_header_ex("Test Buttons", WidgetOption::Expanded).is_none() {
            ctx.mu_layout_row(
                3 as libc::c_int,
                [86 as libc::c_int, -(110 as libc::c_int), -(1 as libc::c_int)].as_mut_ptr(),
                0 as libc::c_int,
            );
            ctx.mu_label("Test buttons 1:");
            if !ctx.mu_button_ex("Button 1", Icon::None, WidgetOption::AlignCenter).is_none() {
                write_log(logbuf, logbuf_updated, "Pressed button 1");
            }
            if !ctx.mu_button_ex("Button 2", Icon::None, WidgetOption::AlignCenter).is_none() {
                write_log(logbuf, logbuf_updated, "Pressed button 2");
            }
            ctx.mu_label("Test buttons 2:");
            if !ctx.mu_button_ex("Button 3", Icon::None, WidgetOption::AlignCenter).is_none() {
                write_log(logbuf, logbuf_updated, "Pressed button 3");
            }
            if !ctx.mu_button_ex("Popup", Icon::None, WidgetOption::AlignCenter).is_none() {
                ctx.mu_open_popup("Test Popup");
            }
            if !ctx.mu_begin_popup("Test Popup").is_none() {
                if !ctx.mu_button_ex("Hello", Icon::None, WidgetOption::AlignCenter).is_none() {
                    write_log(logbuf, logbuf_updated, "Hello")
                }
                if !ctx.mu_button_ex("World", Icon::None, WidgetOption::AlignCenter).is_none() {
                    write_log(logbuf, logbuf_updated, "World")
                }
                ctx.mu_end_popup();
            }
        }
        if !ctx.mu_header_ex("Tree and Text", WidgetOption::Expanded).is_none() {
            ctx.mu_layout_row(2 as libc::c_int, [140 as libc::c_int, -(1 as libc::c_int)].as_mut_ptr(), 0 as libc::c_int);
            ctx.mu_layout_begin_column();
            if !ctx.mu_begin_treenode_ex("Test 1", WidgetOption::None).is_none() {
                if !ctx.mu_begin_treenode_ex("Test 1a", WidgetOption::None).is_none() {
                    ctx.mu_label("Hello");
                    ctx.mu_label("world");
                    ctx.mu_end_treenode();
                }
                if !ctx.mu_begin_treenode_ex("Test 1b", WidgetOption::None).is_none() {
                    if !ctx.mu_button_ex("Button 1", Icon::None, WidgetOption::AlignCenter).is_none() {
                        write_log(logbuf, logbuf_updated, "Pressed button 1");
                    }
                    if !ctx.mu_button_ex("Button 2", Icon::None, WidgetOption::AlignCenter).is_none() {
                        write_log(logbuf, logbuf_updated, "Pressed button 2");
                    }
                    ctx.mu_end_treenode();
                }
                ctx.mu_end_treenode();
            }
            if !ctx.mu_begin_treenode_ex("Test 2", WidgetOption::None).is_none() {
                ctx.mu_layout_row(2 as libc::c_int, [54 as libc::c_int, 54 as libc::c_int].as_mut_ptr(), 0 as libc::c_int);
                if !ctx.mu_button_ex("Button 3", Icon::None, WidgetOption::AlignCenter).is_none() {
                    write_log(logbuf, logbuf_updated, "Pressed button 3");
                }
                if !ctx.mu_button_ex("Button 4", Icon::None, WidgetOption::AlignCenter).is_none() {
                    write_log(logbuf, logbuf_updated, "Pressed button 4");
                }
                if !ctx.mu_button_ex("Button 5", Icon::None, WidgetOption::AlignCenter).is_none() {
                    write_log(logbuf, logbuf_updated, "Pressed button 5");
                }
                if !ctx.mu_button_ex("Button 6", Icon::None, WidgetOption::AlignCenter).is_none() {
                    write_log(logbuf, logbuf_updated, "Pressed button 6");
                }
                ctx.mu_end_treenode();
            }
            if !ctx.mu_begin_treenode_ex("Test 3", WidgetOption::None).is_none() {
                unsafe {
                    static mut checks: [bool; 3] = [true, false, true];
                    ctx.mu_checkbox("Checkbox 1", &mut checks[0]);
                    ctx.mu_checkbox("Checkbox 2", &mut checks[1]);
                    ctx.mu_checkbox("Checkbox 3", &mut checks[2]);
                }
                ctx.mu_end_treenode();
            }
            ctx.mu_layout_end_column();
            ctx.mu_layout_begin_column();
            ctx.mu_layout_row(1 as libc::c_int, [-(1 as libc::c_int)].as_mut_ptr(), 0 as libc::c_int);
            ctx.mu_text(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas lacinia, sem eu lacinia molestie, mi risus faucibus ipsum, eu varius magna felis a nulla."
                ,
            );
            ctx.mu_layout_end_column();
        }
        if !ctx.mu_header_ex("Background Color", WidgetOption::Expanded).is_none() {
            unsafe {
                ctx.mu_layout_row(2 as libc::c_int, [-(78 as libc::c_int), -(1 as libc::c_int)].as_mut_ptr(), 74 as libc::c_int);
                ctx.mu_layout_begin_column();
                ctx.mu_layout_row(2 as libc::c_int, [46 as libc::c_int, -(1 as libc::c_int)].as_mut_ptr(), 0 as libc::c_int);
                ctx.mu_label("Red:");
                ctx.mu_slider_ex(
                    &mut bg[0],
                    0 as libc::c_int as Real,
                    255 as libc::c_int as Real,
                    0 as libc::c_int as Real,
                    "%.2",
                    WidgetOption::AlignCenter,
                );
                ctx.mu_label("Green:");
                ctx.mu_slider_ex(
                    &mut bg[1],
                    0 as libc::c_int as Real,
                    255 as libc::c_int as Real,
                    0 as libc::c_int as Real,
                    "%.2",
                    WidgetOption::AlignCenter,
                );
                ctx.mu_label("Blue:");
                ctx.mu_slider_ex(
                    &mut bg[2],
                    0 as libc::c_int as Real,
                    255 as libc::c_int as Real,
                    0 as libc::c_int as Real,
                    "%.2",
                    WidgetOption::AlignCenter,
                );
                ctx.mu_layout_end_column();
                let r: Rect = ctx.mu_layout_next();
                ctx.mu_draw_rect(
                    r,
                    color(
                        bg[0 as libc::c_int as usize] as u8,
                        bg[1 as libc::c_int as usize] as u8,
                        bg[2 as libc::c_int as usize] as u8,
                        255,
                    ),
                );
                let mut buff = FixedString::<128>::new();
                buff.write_fmt(format_args!(
                    "#{:02X}{:02X}{:02X}",
                    bg[0 as libc::c_int as usize] as libc::c_int, bg[1 as libc::c_int as usize] as libc::c_int, bg[2 as libc::c_int as usize] as libc::c_int,
                ))
                .unwrap();
                ctx.mu_draw_control_text(buff.as_str(), r, ControlColor::Text, WidgetOption::AlignCenter);
            }
        }
        ctx.mu_end_window();
    }
}
unsafe extern "C" fn log_window(logbuf: &mut dyn IString, logbuf_updated: &mut i32, submit_buf: &mut dyn IString, ctx: &mut Context) {
    if !ctx
        .mu_begin_window_ex(
            "Log Window",
            rect(350 as libc::c_int, 40 as libc::c_int, 300 as libc::c_int, 200 as libc::c_int),
            WidgetOption::None,
        )
        .is_none()
    {
        ctx.mu_layout_row(1 as libc::c_int, [-(1 as libc::c_int)].as_mut_ptr(), -(25 as libc::c_int));
        ctx.mu_begin_panel_ex("Log Output", WidgetOption::None);
        let mut scroll = ctx.mu_get_current_container_scroll();
        let content_size = ctx.mu_get_current_container_content_size();
        ctx.mu_layout_row(1 as libc::c_int, [-(1 as libc::c_int)].as_mut_ptr(), -(1 as libc::c_int));
        ctx.mu_text(logbuf.as_str());
        ctx.mu_end_panel();
        if *logbuf_updated != 0 {
            scroll.y = content_size.y;
            ctx.set_current_container_scroll(&scroll);
            *logbuf_updated = 0 as libc::c_int;
        }
        let mut submitted: libc::c_int = 0 as libc::c_int;
        ctx.mu_layout_row(2 as libc::c_int, [-(70 as libc::c_int), -(1 as libc::c_int)].as_mut_ptr(), 0 as libc::c_int);
        if ctx.mu_textbox_ex(submit_buf, WidgetOption::None).is_submitted() {
            ctx.mu_set_focus(ctx.last_id);
            submitted = 1 as libc::c_int;
        }
        if !ctx.mu_button_ex("Submit", Icon::None, WidgetOption::AlignCenter).is_none() {
            submitted = 1 as libc::c_int;
        }
        if submitted != 0 {
            write_log(logbuf, logbuf_updated, submit_buf.as_str());
            submit_buf.clear();
        }
        ctx.mu_end_window();
    }
}
unsafe extern "C" fn uint8_slider(ctx: &mut Context, value: &mut u8, low: libc::c_int, high: libc::c_int) -> ResourceState {
    static mut tmp: libc::c_float = 0.;
    tmp = *value as libc::c_float;
    ctx.mu_push_id_from_ptr(value);
    let res = ctx.mu_slider_ex(&mut tmp, low as Real, high as Real, 0 as libc::c_int as Real, ":.2", WidgetOption::AlignCenter);
    *value = tmp as libc::c_uchar;
    ctx.mu_pop_id();
    return res;
}
unsafe extern "C" fn style_window(ctx: &mut Context) {
    static mut colors: [C2RustUnnamed_13; 15] = [
        {
            let init = C2RustUnnamed_13 { label: "text", idx: ControlColor::Text };
            init
        },
        {
            let init = C2RustUnnamed_13 {
                label: "border:",
                idx: ControlColor::Border,
            };
            init
        },
        {
            let init = C2RustUnnamed_13 {
                label: "windowbg:",
                idx: ControlColor::WindowBG,
            };
            init
        },
        {
            let init = C2RustUnnamed_13 {
                label: "titlebg:",
                idx: ControlColor::TitleBG,
            };
            init
        },
        {
            let init = C2RustUnnamed_13 {
                label: "titletext:",
                idx: ControlColor::TitleText,
            };
            init
        },
        {
            let init = C2RustUnnamed_13 {
                label: "panelbg:",
                idx: ControlColor::PanelBG,
            };
            init
        },
        {
            let init = C2RustUnnamed_13 {
                label: "button:",
                idx: ControlColor::Button,
            };
            init
        },
        {
            let init = C2RustUnnamed_13 {
                label: "buttonhover:",
                idx: ControlColor::ButtonHover,
            };
            init
        },
        {
            let init = C2RustUnnamed_13 {
                label: "buttonfocus:",
                idx: ControlColor::ButtonFocus,
            };
            init
        },
        {
            let init = C2RustUnnamed_13 { label: "base:", idx: ControlColor::Base };
            init
        },
        {
            let init = C2RustUnnamed_13 {
                label: "basehover:",
                idx: ControlColor::BaseHover,
            };
            init
        },
        {
            let init = C2RustUnnamed_13 {
                label: "basefocus:",
                idx: ControlColor::BaseFocus,
            };
            init
        },
        {
            let init = C2RustUnnamed_13 {
                label: "scrollbase:",
                idx: ControlColor::ScrollBase,
            };
            init
        },
        {
            let init = C2RustUnnamed_13 {
                label: "scrollthumb:",
                idx: ControlColor::ScrollThumb,
            };
            init
        },
        {
            let init = C2RustUnnamed_13 { label: "", idx: ControlColor::Text };
            init
        },
    ];
    if !ctx
        .mu_begin_window_ex(
            "Style Editor",
            rect(350 as libc::c_int, 250 as libc::c_int, 300 as libc::c_int, 240 as libc::c_int),
            WidgetOption::None,
        )
        .is_none()
    {
        let sw: libc::c_int = (ctx.mu_get_current_container_body().w as libc::c_double * 0.14f64) as libc::c_int;
        ctx.mu_layout_row(
            6 as libc::c_int,
            [80 as libc::c_int, sw, sw, sw, sw, -(1 as libc::c_int)].as_mut_ptr(),
            0 as libc::c_int,
        );
        let mut i = 0;
        while colors[i].label.len() > 0 {
            ctx.mu_label(colors[i].label);
            let color = (*ctx).style.colors.as_mut_ptr().offset(i as isize);
            uint8_slider(ctx, &mut (*color).r, 0 as libc::c_int, 255 as libc::c_int);
            uint8_slider(ctx, &mut (*color).g, 0 as libc::c_int, 255 as libc::c_int);
            uint8_slider(ctx, &mut (*color).b, 0 as libc::c_int, 255 as libc::c_int);
            uint8_slider(ctx, &mut (*color).a, 0 as libc::c_int, 255 as libc::c_int);
            let next_layout = ctx.mu_layout_next();
            ctx.mu_draw_rect(next_layout, (*ctx).style.colors[i]);
            i += 1;
        }
        ctx.mu_end_window();
    }
}

unsafe extern "C" fn process_frame(logbuf: &mut dyn IString, logbuf_updated: &mut i32, submit_buf: &mut dyn IString, ctx: &mut Context) {
    ctx.begin();
    style_window(ctx);
    log_window(logbuf, logbuf_updated, submit_buf, ctx);
    test_window(logbuf, logbuf_updated, ctx);
    ctx.end();
}

static mut key_map: [libc::c_char; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    MU_KEY_BACKSPACE as libc::c_int as libc::c_char,
    0,
    0,
    0,
    0,
    MU_KEY_RETURN as libc::c_int as libc::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    MU_KEY_CTRL as libc::c_int as libc::c_char,
    MU_KEY_SHIFT as libc::c_int as libc::c_char,
    MU_KEY_ALT as libc::c_int as libc::c_char,
    0,
    MU_KEY_CTRL as libc::c_int as libc::c_char,
    MU_KEY_SHIFT as libc::c_int as libc::c_char,
    MU_KEY_ALT as libc::c_int as libc::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];

#[no_mangle]
unsafe extern "C" fn main() {
    let mut logbuf = FixedString::<65536>::new();
    let mut logbuf_updated: libc::c_int = 0 as libc::c_int;
    let mut submit_buf = FixedString::<128>::new();
    unsafe {
        SDL_Init(
            0x1 as libc::c_uint
                | 0x10 as libc::c_uint
                | 0x20 as libc::c_uint
                | 0x4000 as libc::c_uint
                | 0x200 as libc::c_uint
                | 0x1000 as libc::c_uint
                | 0x2000 as libc::c_uint
                | 0x8000 as libc::c_uint,
        );
        r_init();
        let mut ctx = Context::new();
        ctx.char_width = Some(r_get_char_width);
        ctx.char_height = Some(r_get_char_height);
        loop {
            let mut e: SDL_Event = SDL_Event { type_0: 0 };
            while SDL_PollEvent(&mut e) != 0 {
                match e.type_0 {
                    256 => {
                        exit(0 as libc::c_int);
                    }
                    1024 => {
                        ctx.mu_input_mousemove(e.motion.x, e.motion.y);
                    }
                    1027 => {
                        ctx.mu_input_scroll(0 as libc::c_int, e.wheel.y * -(30 as libc::c_int));
                    }
                    771 => {
                        let mut text = FixedString::<32>::new();
                        let u8_txt = e.text.text.as_slice();
                        for c in u8_txt {
                            if *c != 0 {
                                text.push(*c as u8 as char);
                            } else {
                                break;
                            }
                        }
                        ctx.mu_input_text(text.as_str());
                    }
                    1025 | 1026 => {
                        let b = match e.button.button {
                            1 => MouseButton::Left,
                            2 => MouseButton::Middle,
                            3 => MouseButton::Right,
                            _ => MouseButton::None,
                        };

                        if !b.is_none() && e.type_0 == SDL_MOUSEBUTTONDOWN as libc::c_int as libc::c_uint {
                            ctx.mu_input_mousedown(e.button.x, e.button.y, b);
                        }
                        if !b.is_none() && e.type_0 == SDL_MOUSEBUTTONUP as libc::c_int as libc::c_uint {
                            ctx.mu_input_mouseup(e.button.x, e.button.y, b);
                        }
                    }
                    768 | 769 => {
                        let c: libc::c_int = key_map[(e.key.keysym.sym & 0xff as libc::c_int) as usize] as libc::c_int;
                        if c != 0 && e.type_0 == SDL_KEYDOWN as libc::c_int as libc::c_uint {
                            ctx.mu_input_keydown(c);
                        }
                        if c != 0 && e.type_0 == SDL_KEYUP as libc::c_int as libc::c_uint {
                            ctx.mu_input_keyup(c);
                        }
                    }
                    _ => {}
                }
            }
            process_frame(&mut logbuf, &mut logbuf_updated, &mut submit_buf, &mut ctx);
            r_clear(color(
                bg[0 as libc::c_int as usize] as u8,
                bg[1 as libc::c_int as usize] as u8,
                bg[2 as libc::c_int as usize] as u8,
                255,
            ));
            let mut cmd_id = 0;
            loop {
                match ctx.mu_next_command(cmd_id) {
                    Some((command, id)) => {
                        match command {
                            Command::Text { str_start, str_len, pos, color, .. } => {
                                let str = &ctx.text_stack[str_start..str_start + str_len];
                                r_draw_text(str, pos, color);
                            }
                            Command::Rect { rect, color } => {
                                r_draw_rect(rect, color);
                            }
                            Command::Icon { id, rect, color } => {
                                r_draw_icon(id, rect, color);
                            }
                            Command::Clip { rect } => {
                                r_set_clip_rect(rect);
                            }
                            _ => {}
                        }
                        cmd_id = id;
                    }
                    None => break,
                }
            }
            r_present();
        }
    }
}
