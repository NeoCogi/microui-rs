//
// Copyright 2022-Present (c) Raja Lehtihet & Wael El Oraiby
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors
// may be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//
// -----------------------------------------------------------------------------
// Ported to rust from https://github.com/rxi/microui/ and the original license
//
// Copyright (c) 2020 rxi
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//
use microui::*;
use ::libc;

pub type SDL_Window = libc::c_int;

extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn SDL_CreateWindow(title: *const libc::c_char, x: libc::c_int, y: libc::c_int, w: libc::c_int, h: libc::c_int, flags: Uint32) -> *mut SDL_Window;
    fn SDL_GL_CreateContext(window_0: *mut SDL_Window) -> SDL_GLContext;
    fn SDL_GL_SwapWindow(window_0: *mut SDL_Window);
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    fn glDrawElements(mode: GLenum, count: GLsizei, type_0: GLenum, indices: *const libc::c_void);
    fn glTexCoordPointer(size: GLint, type_0: GLenum, stride: GLsizei, ptr: *const libc::c_void);
    fn glColorPointer(size: GLint, type_0: GLenum, stride: GLsizei, ptr: *const libc::c_void);
    fn glVertexPointer(size: GLint, type_0: GLenum, stride: GLsizei, ptr: *const libc::c_void);
    fn glLoadIdentity();
    fn glPopMatrix();
    fn glPushMatrix();
    fn glViewport(x: GLint, y: GLint, width_0: GLsizei, height_0: GLsizei);
    fn glOrtho(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, near_val: GLdouble, far_val: GLdouble);
    fn glMatrixMode(mode: GLenum);
    fn glGetError() -> GLenum;
    fn glEnableClientState(cap: GLenum);
    fn glDisable(cap: GLenum);
    fn glEnable(cap: GLenum);
    fn glScissor(x: GLint, y: GLint, width_0: GLsizei, height_0: GLsizei);
    fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    fn glClear(mask: GLbitfield);
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalFormat: GLint,
        width_0: GLsizei,
        height_0: GLsizei,
        border: GLint,
        format: GLenum,
        type_0: GLenum,
        pixels: *const libc::c_void,
    );
    fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    fn glBindTexture(target: GLenum, texture: GLuint);
    fn __assert_fail(__assertion: *const libc::c_char, __file: *const libc::c_char, __line: libc::c_uint, __function: *const libc::c_char) -> !;
}

pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type Uint32 = uint32_t;
pub type C2RustUnnamed = libc::c_uint;

pub const SDL_WINDOW_VULKAN: C2RustUnnamed = 268435456;
pub const SDL_WINDOW_POPUP_MENU: C2RustUnnamed = 524288;
pub const SDL_WINDOW_TOOLTIP: C2RustUnnamed = 262144;
pub const SDL_WINDOW_UTILITY: C2RustUnnamed = 131072;
pub const SDL_WINDOW_SKIP_TASKBAR: C2RustUnnamed = 65536;
pub const SDL_WINDOW_ALWAYS_ON_TOP: C2RustUnnamed = 32768;
pub const SDL_WINDOW_MOUSE_CAPTURE: C2RustUnnamed = 16384;
pub const SDL_WINDOW_ALLOW_HIGHDPI: C2RustUnnamed = 8192;
pub const SDL_WINDOW_FOREIGN: C2RustUnnamed = 2048;
pub const SDL_WINDOW_FULLSCREEN_DESKTOP: C2RustUnnamed = 4097;
pub const SDL_WINDOW_MOUSE_FOCUS: C2RustUnnamed = 1024;
pub const SDL_WINDOW_INPUT_FOCUS: C2RustUnnamed = 512;
pub const SDL_WINDOW_INPUT_GRABBED: C2RustUnnamed = 256;
pub const SDL_WINDOW_MAXIMIZED: C2RustUnnamed = 128;
pub const SDL_WINDOW_MINIMIZED: C2RustUnnamed = 64;
pub const SDL_WINDOW_RESIZABLE: C2RustUnnamed = 32;
pub const SDL_WINDOW_BORDERLESS: C2RustUnnamed = 16;
pub const SDL_WINDOW_HIDDEN: C2RustUnnamed = 8;
pub const SDL_WINDOW_SHOWN: C2RustUnnamed = 4;
pub const SDL_WINDOW_OPENGL: C2RustUnnamed = 2;
pub const SDL_WINDOW_FULLSCREEN: C2RustUnnamed = 1;

pub type SDL_GLContext = *mut libc::c_void;
pub type GLenum = libc::c_uint;
pub type GLbitfield = libc::c_uint;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLubyte = libc::c_uchar;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
pub type GLdouble = libc::c_double;

pub const ATLAS_HEIGHT: C2RustUnnamed_1 = 128;
pub const ATLAS_WIDTH: C2RustUnnamed_1 = 128;
pub const ATLAS_WHITE: C2RustUnnamed_0 = 5;
pub const ATLAS_FONT: C2RustUnnamed_0 = 6;

pub type C2RustUnnamed_0 = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;

static mut atlas_texture: [libc::c_uchar; 16384] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0x20, 0xc0, 0x20, 0, 0, 0, 0, 0, 0, 0x9, 0x32, 0xb, 0, 0, 0, 0, 0x31, 0x35, 0x1, 0, 0, 0, 0x15, 0x35, 0x1d, 0x30, 0x19, 0, 0, 0xf, 0x35,
    0x6, 0, 0, 0x1f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xb, 0x2d, 0, 0, 0, 0x14, 0x26, 0, 0, 0x31, 0x2a, 0, 0, 0, 0x10, 0x31, 0, 0, 0, 0, 0x6, 0x3e, 0x8, 0, 0,
    0, 0, 0, 0, 0x6, 0x3e, 0x8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xc, 0x40, 0xd, 0, 0, 0, 0, 0x31, 0x35, 0x2f, 0x2, 0, 0, 0, 0, 0, 0, 0x18, 0x41,
    0x37, 0x8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x21, 0xe0, 0xea, 0x2c, 0, 0, 0, 0, 0x36, 0xc4, 0xdb, 0xb2, 0xd9, 0xc1, 0x1a, 0, 0, 0xea, 0xff, 0x39,
    0, 0, 0, 0x9e, 0xff, 0x88, 0xbe, 0x9c, 0, 0, 0x72, 0xff, 0x48, 0, 0, 0xbb, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1c, 0xe4, 0xce, 0x8d, 0, 0, 0xb5, 0x60, 0, 0,
    0xea, 0xfa, 0x2c, 0, 0, 0x4e, 0xeb, 0, 0, 0x1c, 0x8f, 0xea, 0xea, 0xee, 0x92, 0x1f, 0, 0, 0x1c, 0x8f, 0xea, 0xea, 0xee, 0x92, 0x1f, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0x42, 0xf3, 0xcd, 0xf5, 0x3a, 0, 0, 0, 0xea, 0xf2, 0xef, 0xe5, 0x8f, 0x2f, 0, 0, 0xf, 0xa0, 0xfe, 0xf2, 0xf1, 0xfa, 0x33, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0x21, 0xe0, 0xea, 0x2d, 0, 0, 0, 0, 0x35, 0xdc, 0x33, 0, 0x6, 0, 0x5a, 0xd7, 0x13, 0, 0xea, 0xd8, 0x92, 0, 0, 0x9, 0xf0, 0xd9, 0x88, 0x7b,
    0xda, 0, 0, 0xb9, 0xe9, 0x91, 0, 0x5, 0xf4, 0x3, 0x35, 0x2, 0x32, 0x1f, 0, 0x4, 0x37, 0x24, 0, 0x5b, 0xa9, 0x1b, 0xe7, 0x1, 0x44, 0xd0, 0x2, 0, 0, 0xea,
    0xe4, 0xc3, 0x1, 0, 0x4e, 0xeb, 0, 0, 0x8f, 0xd8, 0x42, 0x1, 0x3a, 0xd0, 0x9b, 0, 0, 0x8f, 0xd8, 0x42, 0x1, 0x3a, 0xd0, 0x9b, 0, 0x2e, 0x1a, 0, 0x3, 0x36,
    0x19, 0, 0x4, 0x36, 0, 0xa3, 0xa0, 0, 0xb5, 0x8d, 0, 0, 0, 0xea, 0x6d, 0x1, 0x3d, 0xac, 0xe5, 0x3, 0, 0xa1, 0xeb, 0x63, 0xc, 0x3, 0x2e, 0x1, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0x21, 0xe1, 0xeb, 0x2d, 0, 0, 0, 0, 0, 0xd6, 0x38, 0x37, 0xb4, 0xd6, 0xe9, 0x35, 0x9e, 0x5c, 0, 0xea, 0x87, 0xe8, 0x3, 0, 0x56, 0xcc,
    0xba, 0x88, 0x38, 0xff, 0x1a, 0x9, 0xf7, 0x84, 0xd9, 0, 0x39, 0xfe, 0xe, 0xff, 0xb7, 0xe1, 0xf3, 0x94, 0xbc, 0xde, 0xfb, 0x97, 0x73, 0x9b, 0xb, 0xfb, 0xb,
    0xcf, 0x44, 0x1, 0, 0, 0xea, 0x65, 0xf7, 0x63, 0, 0x4e, 0xeb, 0, 0x1, 0xe0, 0x79, 0, 0, 0, 0x6b, 0xea, 0x3, 0x1, 0xe0, 0x79, 0, 0, 0, 0x6b, 0xea, 0x3,
    0xb0, 0xa4, 0, 0x39, 0xfe, 0xa0, 0, 0x3d, 0xfd, 0, 0x6e, 0xe0, 0x5b, 0xef, 0x41, 0, 0, 0, 0xea, 0x6d, 0, 0, 0x17, 0xfd, 0x47, 0x18, 0xff, 0x8f, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0xe, 0, 0, 0, 0, 0, 0x21, 0xe1, 0xeb, 0x2d, 0, 0, 0, 0, 0, 0x16, 0xe3, 0, 0xc5, 0x51, 0, 0xd4, 0x37, 0x61, 0x99, 0, 0xea, 0x4d, 0xe9, 0x45,
    0, 0xb2, 0x70, 0xbf, 0x88, 0x4, 0xf0, 0x59, 0x48, 0xe9, 0x18, 0xfd, 0x23, 0x77, 0xd4, 0xe, 0xff, 0x9f, 0x2, 0x54, 0xff, 0x86, 0, 0x69, 0xf0, 0x39, 0xd1,
    0x61, 0xca, 0x60, 0xb6, 0x6f, 0xe3, 0x98, 0, 0xea, 0x4a, 0x7f, 0xec, 0x15, 0x4e, 0xeb, 0, 0x32, 0xff, 0x36, 0, 0, 0, 0x29, 0xff, 0x3e, 0x32, 0xff, 0x36, 0,
    0, 0, 0x29, 0xff, 0x3f, 0x68, 0xe6, 0, 0x83, 0xaf, 0xe7, 0x1, 0x80, 0xc9, 0, 0x18, 0xf4, 0xff, 0x53, 0, 0x15, 0x3b, 0, 0xea, 0x6d, 0, 0, 0, 0xc5, 0xa2,
    0x41, 0xff, 0x39, 0, 0x40, 0x73, 0x73, 0x35, 0, 0, 0, 0x3d, 0xed, 0x45, 0, 0, 0, 0x22, 0xe1, 0xeb, 0x2d, 0, 0, 0, 0, 0, 0, 0x4f, 0xb3, 0x12, 0xf9, 0x4,
    0x3, 0xef, 0x2c, 0x6a, 0x94, 0, 0xea, 0x4d, 0x95, 0x9e, 0x14, 0xf6, 0x18, 0xc1, 0x88, 0, 0xb2, 0x96, 0x8e, 0xa4, 0, 0xcb, 0x6a, 0xb5, 0x92, 0xe, 0xff,
    0x59, 0, 0x27, 0xff, 0x3d, 0, 0x3a, 0xff, 0x4, 0x71, 0xae, 0x40, 0xe0, 0x2e, 0xee, 0x1e, 0xd5, 0, 0xea, 0x4d, 0x8, 0xd9, 0xa0, 0x4b, 0xeb, 0, 0x20, 0xfe,
    0x45, 0, 0, 0, 0x37, 0xff, 0x2c, 0x20, 0xfe, 0x45, 0, 0, 0, 0x37, 0xff, 0x3b, 0x21, 0xff, 0x29, 0xcc, 0x4a, 0xe9, 0x30, 0xc3, 0x81, 0x14, 0xdf, 0xab, 0xbd,
    0xcd, 0x14, 0x9c, 0xb5, 0, 0xea, 0x6d, 0, 0, 0, 0xdc, 0xa6, 0x20, 0xfe, 0x42, 0, 0x63, 0xb2, 0xf4, 0x76, 0, 0, 0, 0x13, 0xd0, 0xf6, 0x45, 0, 0x22, 0xe1,
    0xeb, 0x2e, 0, 0, 0, 0, 0, 0, 0, 0x3f, 0xe7, 0x3, 0xd7, 0x5e, 0x75, 0xf7, 0x8d, 0xc7, 0x4a, 0, 0xea, 0x4d, 0x3c, 0xf0, 0x71, 0xb7, 0, 0xc1, 0x88, 0, 0x70,
    0xc9, 0xcb, 0x5d, 0, 0x83, 0xa2, 0xe9, 0x4f, 0xe, 0xff, 0x43, 0, 0x26, 0xff, 0x2d, 0, 0x39, 0xff, 0, 0, 0, 0x7c, 0x9a, 0x1f, 0xf1, 0, 0xaf, 0, 0xea, 0x4d,
    0, 0x44, 0xfe, 0x83, 0xeb, 0, 0, 0xcd, 0x89, 0, 0, 0, 0x7b, 0xd9, 0, 0, 0xcd, 0x89, 0, 0, 0, 0x7b, 0xf8, 0x8, 0, 0xd8, 0x75, 0xf8, 0xd, 0xa7, 0x79, 0xf7,
    0x39, 0x5b, 0xfc, 0xa, 0x9, 0xba, 0xd4, 0xf6, 0x39, 0, 0xea, 0x6d, 0, 0, 0x31, 0xff, 0x75, 0, 0xcc, 0x8f, 0, 0, 0, 0xdb, 0x76, 0, 0, 0, 0, 0x13, 0xd0,
    0xf6, 0x63, 0xe1, 0xeb, 0x2e, 0, 0, 0, 0, 0, 0, 0, 0, 0x11, 0xf3, 0x44, 0x2c, 0x96, 0x87, 0x29, 0xa3, 0x64, 0, 0, 0xea, 0x4d, 0x1, 0xe1, 0xf3, 0x5b, 0,
    0xc1, 0x88, 0, 0x2e, 0xf7, 0xf5, 0x17, 0, 0x3a, 0xec, 0xfc, 0x10, 0xe, 0xff, 0x43, 0, 0x26, 0xff, 0x2d, 0, 0x39, 0xff, 0, 0, 0x17, 0xe6, 0x19, 0x2, 0xee,
    0x13, 0xd0, 0, 0xea, 0x4d, 0, 0, 0xa5, 0xf6, 0xeb, 0, 0, 0x7d, 0xec, 0x7a, 0x24, 0x73, 0xe7, 0x87, 0, 0, 0x7d, 0xec, 0x7a, 0x24, 0x73, 0xe7, 0x9c, 0, 0,
    0x91, 0xda, 0xbd, 0, 0x61, 0xd9, 0xed, 0x3, 0x2b, 0xfe, 0x67, 0x1d, 0x70, 0xfc, 0xf0, 0x1a, 0, 0xea, 0x7f, 0x31, 0x81, 0xdc, 0xdc, 0xa, 0, 0x79, 0xef,
    0x83, 0x23, 0x1c, 0xe1, 0x76, 0, 0, 0, 0, 0, 0x13, 0xd0, 0xff, 0xec, 0x2e, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x6b, 0xdc, 0x78, 0x20, 0xd, 0x3c, 0x3b, 0, 0, 0,
    0xea, 0x4d, 0, 0x8a, 0xf3, 0xb, 0, 0xc1, 0x88, 0, 0x1, 0xea, 0xce, 0, 0, 0x3, 0xed, 0xc9, 0, 0xe, 0xff, 0x43, 0, 0x26, 0xff, 0x2d, 0, 0x39, 0xff, 0, 0,
    0x98, 0x7e, 0, 0, 0x78, 0xe7, 0xb0, 0, 0xea, 0x4d, 0, 0, 0x18, 0xef, 0xeb, 0, 0, 0x6, 0x54, 0xb6, 0xfb, 0xbb, 0x57, 0x7, 0, 0, 0x6, 0x54, 0xb6, 0xfc, 0xff,
    0x8d, 0x1, 0, 0, 0x49, 0xff, 0x74, 0, 0x1b, 0xfe, 0xa8, 0, 0, 0x7f, 0xde, 0xff, 0xe7, 0x72, 0xb1, 0xdb, 0, 0xea, 0xff, 0xf8, 0xd6, 0x92, 0x13, 0, 0, 0x6,
    0x52, 0xb4, 0xfb, 0xff, 0xe5, 0x55, 0, 0, 0, 0, 0, 0, 0x13, 0xbc, 0x2e, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x44, 0xb7, 0xde, 0xdb, 0xad, 0x50, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0x8, 0, 0, 0, 0, 0, 0, 0, 0, 0x1b, 0xda, 0xbb, 0x7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xf, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xf, 0xc,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x26, 0xe6, 0xa0, 0x2, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x31, 0x17, 0, 0, 0, 0x2a, 0x1e, 0, 0x35,
    0x13, 0, 0, 0, 0x2c, 0x1c, 0, 0, 0xc, 0x27, 0, 0x29, 0x9, 0, 0, 0, 0x3c, 0x24, 0, 0, 0, 0x31, 0x35, 0x30, 0x8, 0, 0, 0, 0, 0, 0x11, 0x44, 0x1f, 0, 0, 0x31,
    0x17, 0, 0, 0x13, 0x35, 0, 0x31, 0x35, 0x2b, 0x2, 0, 0, 0, 0x31, 0x35, 0x2a, 0x2, 0, 0, 0x33, 0x16, 0, 0, 0, 0xf, 0x35, 0x27, 0x28, 0, 0, 0, 0x1f, 0x2e,
    0xb, 0xbc, 0x31, 0, 0, 0, 0, 0, 0, 0, 0, 0x13, 0xbc, 0x2a, 0xb, 0xbc, 0x31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0xea, 0x6d, 0, 0, 0, 0xc9, 0x8e, 0, 0xfc, 0x5b, 0, 0, 0, 0xd2, 0x84, 0, 0, 0x59, 0xa9, 0, 0xea, 0xf, 0, 0, 0x34, 0xfe, 0xce, 0, 0, 0,
    0xea, 0xf2, 0xed, 0xfa, 0xce, 0x1a, 0, 0x9, 0xb5, 0xfd, 0xe7, 0xf5, 0xaf, 0, 0xea, 0x6d, 0, 0xc, 0xcf, 0xa7, 0, 0xea, 0xf2, 0xf3, 0xea, 0x94, 0, 0, 0xea,
    0xf4, 0xf6, 0xec, 0x9d, 0, 0xc0, 0x9e, 0, 0, 0, 0x7a, 0xe5, 0x58, 0xf8, 0x26, 0, 0xd, 0xe4, 0x7a, 0xe, 0xff, 0x43, 0, 0, 0, 0, 0, 0, 0, 0, 0x1a, 0xff,
    0x39, 0xe, 0xff, 0x43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xea, 0x6d, 0, 0, 0, 0xc9, 0x8e,
    0, 0xfc, 0x5b, 0, 0, 0, 0xd2, 0x84, 0, 0, 0x89, 0x79, 0x1c, 0xdd, 0, 0, 0, 0x93, 0xad, 0xf9, 0x2e, 0, 0, 0xea, 0x6d, 0, 0x19, 0xe3, 0x8c, 0, 0xab, 0xe1,
    0x4f, 0x2, 0x7, 0x1f, 0, 0xea, 0x6d, 0x3, 0xb2, 0xc6, 0x8, 0, 0xea, 0x6d, 0x3, 0x4d, 0xff, 0x33, 0, 0xea, 0x6d, 0x3, 0x49, 0xff, 0x3a, 0x67, 0xee, 0x6, 0,
    0, 0xd0, 0x8e, 0, 0xb3, 0xbb, 0, 0x8e, 0xcf, 0x5, 0xe, 0xff, 0x41, 0x12, 0x25, 0, 0, 0, 0, 0x18, 0x20, 0x18, 0xff, 0x39, 0xe, 0xff, 0x41, 0x2e, 0x33, 0x1,
    0, 0x3, 0x35, 0x1, 0x2e, 0x34, 0x1, 0, 0, 0, 0, 0, 0, 0x4, 0x39, 0x5, 0, 0, 0x3, 0x36, 0x2, 0x11, 0x25, 0, 0, 0, 0, 0, 0, 0, 0xea, 0x6d, 0, 0, 0, 0xc9,
    0x8e, 0, 0xfc, 0x5b, 0, 0, 0, 0xd2, 0x84, 0x41, 0xd3, 0xf0, 0xe3, 0xdd, 0xf4, 0xd3, 0, 0x6, 0xec, 0x52, 0xb4, 0x8d, 0, 0, 0xea, 0x6d, 0, 0x6, 0xdd, 0x86,
    0xb, 0xfc, 0x81, 0, 0, 0, 0, 0, 0xea, 0x6d, 0x8d, 0xdd, 0x15, 0, 0, 0xea, 0x6d, 0, 0x7, 0xf3, 0x64, 0, 0xea, 0x6d, 0, 0xa, 0xf4, 0x69, 0x13, 0xfa, 0x4a, 0,
    0x27, 0xff, 0x34, 0, 0x1c, 0xf1, 0x86, 0xf7, 0x33, 0, 0xe, 0xff, 0xaa, 0xd3, 0xf8, 0xab, 0x1, 0, 0x87, 0xf6, 0xda, 0xa5, 0xff, 0x39, 0xe, 0xff, 0xc0, 0xe5,
    0xf2, 0xd5, 0x3, 0xe, 0xff, 0xb4, 0xe5, 0xf2, 0xd6, 0x4, 0, 0, 0, 0, 0x6d, 0xe7, 0xdd, 0xe7, 0x6d, 0, 0xe, 0xff, 0x98, 0xd2, 0xf8, 0xab, 0x1, 0, 0, 0, 0,
    0, 0xea, 0xe9, 0xd9, 0xd9, 0xd9, 0xf7, 0x8e, 0, 0xfc, 0x5b, 0, 0, 0, 0xd2, 0x84, 0x9, 0x1d, 0xec, 0x2e, 0x93, 0x87, 0x1d, 0, 0x52, 0xf1, 0x9, 0x5d, 0xe8,
    0x4, 0, 0xea, 0xe4, 0xd3, 0xf6, 0xb1, 0xd, 0x3c, 0xff, 0x37, 0, 0, 0, 0, 0, 0xea, 0xc9, 0xff, 0x60, 0, 0, 0, 0xea, 0x8a, 0x49, 0x9e, 0xf1, 0xd, 0, 0xea,
    0xa6, 0x71, 0xbb, 0xe1, 0x1a, 0, 0xb4, 0xa0, 0, 0x7d, 0xda, 0, 0, 0, 0x74, 0xff, 0x93, 0, 0, 0xe, 0xff, 0x9c, 0x2, 0x2a, 0xff, 0x32, 0xe, 0xfa, 0x5a, 0,
    0x6f, 0xff, 0x39, 0xe, 0xff, 0xa6, 0x4, 0x1e, 0xff, 0x42, 0xe, 0xff, 0xa5, 0x4, 0x1e, 0xff, 0x42, 0, 0, 0, 0x7, 0xf1, 0x59, 0, 0x55, 0xf2, 0x9, 0xe, 0xff,
    0x99, 0x1, 0x2d, 0xff, 0x32, 0, 0, 0, 0, 0, 0xea, 0x98, 0x4c, 0x4c, 0x4c, 0xd9, 0x8e, 0, 0xfc, 0x5b, 0, 0, 0, 0xd2, 0x84, 0x37, 0x58, 0xef, 0x4a, 0xc6,
    0x83, 0x38, 0, 0xb1, 0xd5, 0x6f, 0x7d, 0xff, 0x4c, 0, 0xea, 0x94, 0x47, 0x70, 0xe7, 0x83, 0x22, 0xff, 0x42, 0, 0, 0, 0, 0, 0xea, 0xd2, 0xaf, 0xe2, 0x10, 0,
    0, 0xea, 0xf5, 0xda, 0x94, 0x3e, 0, 0, 0xea, 0xd6, 0xc4, 0xfd, 0x27, 0, 0, 0x5b, 0xef, 0x6, 0xd3, 0x81, 0, 0, 0x1, 0xbc, 0xeb, 0xd2, 0x6, 0, 0xe, 0xff,
    0x56, 0, 0, 0xdd, 0x77, 0x50, 0xfb, 0xc, 0, 0x23, 0xff, 0x39, 0xe, 0xff, 0x5c, 0, 0, 0xeb, 0x68, 0xe, 0xff, 0x5c, 0, 0, 0xeb, 0x68, 0, 0, 0, 0x4c, 0xfb,
    0xb, 0, 0x8, 0xf8, 0x51, 0xe, 0xff, 0x52, 0, 0, 0xde, 0x77, 0, 0, 0, 0, 0, 0xea, 0x6d, 0, 0, 0, 0xc9, 0x8e, 0, 0xd3, 0x85, 0, 0, 0x9, 0xf4, 0x6b, 0x7b,
    0xc7, 0xdf, 0xa7, 0xfd, 0xa9, 0x7f, 0x16, 0xfa, 0xc4, 0xb8, 0xb8, 0xe2, 0xab, 0, 0xea, 0x6d, 0, 0, 0x8e, 0xdf, 0, 0xd2, 0x89, 0, 0, 0, 0, 0, 0xea, 0x6d,
    0xf, 0xe0, 0xa1, 0, 0, 0xea, 0x6d, 0, 0, 0, 0, 0, 0xea, 0x6d, 0x1, 0xc6, 0xa5, 0, 0, 0xc, 0xf5, 0x69, 0xff, 0x28, 0, 0, 0x60, 0xec, 0x21, 0xe4, 0x80, 0,
    0xe, 0xff, 0x65, 0, 0x1, 0xea, 0x67, 0x43, 0xfe, 0x12, 0, 0x25, 0xff, 0x39, 0xe, 0xff, 0x43, 0, 0, 0xea, 0x68, 0xe, 0xff, 0x43, 0, 0, 0xea, 0x68, 0, 0, 0,
    0x4c, 0xfe, 0x16, 0, 0x11, 0xfd, 0x40, 0xe, 0xff, 0x65, 0, 0x1, 0xeb, 0x67, 0, 0, 0, 0, 0, 0xea, 0x6d, 0, 0, 0, 0xc9, 0x8e, 0, 0x72, 0xdf, 0x4b, 0x23,
    0x8c, 0xfa, 0x28, 0, 0x7c, 0x81, 0x1a, 0xe5, 0, 0, 0x70, 0xeb, 0x5, 0, 0, 0x5c, 0xf8, 0, 0xea, 0x7f, 0x22, 0x4a, 0xdd, 0x95, 0, 0x84, 0xed, 0x7c, 0x1d,
    0x24, 0x32, 0, 0xea, 0x6d, 0, 0x40, 0xfd, 0x52, 0, 0xea, 0x6d, 0, 0, 0, 0, 0, 0xea, 0x6d, 0, 0x31, 0xfc, 0x49, 0, 0, 0xa8, 0xe3, 0xcd, 0, 0, 0x17, 0xed,
    0x62, 0, 0x54, 0xf9, 0x2d, 0xe, 0xff, 0xbc, 0x1d, 0x5f, 0xff, 0x21, 0x7, 0xf5, 0x7e, 0x11, 0x8d, 0xff, 0x39, 0xe, 0xff, 0x43, 0, 0, 0xea, 0x68, 0xe, 0xff,
    0x43, 0, 0, 0xea, 0x68, 0, 0, 0, 0x14, 0xf2, 0x88, 0x15, 0x82, 0xe8, 0x3, 0xe, 0xff, 0xbc, 0x1e, 0x61, 0xff, 0x20, 0, 0, 0, 0, 0, 0xea, 0x6d, 0, 0, 0,
    0xc9, 0x8e, 0, 0x10, 0x85, 0xdc, 0xfe, 0xd9, 0x4c, 0, 0, 0xaa, 0x50, 0x4b, 0xb4, 0, 0, 0xcf, 0x97, 0, 0, 0, 0xc, 0xf4, 0, 0xea, 0xff, 0xff, 0xdd, 0x93,
    0x1c, 0, 0x8, 0x5c, 0xc0, 0xfe, 0xed, 0x7b, 0, 0xea, 0x6d, 0, 0, 0x8d, 0xea, 0, 0xea, 0x6d, 0, 0, 0, 0, 0, 0xea, 0x6d, 0, 0, 0x95, 0xe0, 0, 0, 0x4f, 0xff,
    0x74, 0, 0, 0xa8, 0xc3, 0x1, 0, 0, 0xb7, 0xca, 0xe, 0xfd, 0x6f, 0xd4, 0xe5, 0x72, 0, 0, 0x5c, 0xd9, 0xdb, 0x63, 0xef, 0x39, 0xe, 0xff, 0x43, 0, 0, 0xea,
    0x68, 0xe, 0xff, 0x43, 0, 0, 0xea, 0x68, 0, 0, 0, 0, 0x43, 0xdb, 0xfc, 0xb7, 0x46, 0, 0xe, 0xff, 0x8c, 0xd4, 0xe4, 0x70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0xc, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xe, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1, 0x6, 0, 0, 0, 0, 0x2, 0x4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0xa, 0, 0, 0, 0xe, 0xff, 0x43, 0x1, 0x6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xe, 0xff, 0x43, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xb, 0xc2, 0x33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0x11, 0x79, 0, 0, 0, 0, 0x58, 0x8a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xa, 0x2f, 0, 0, 0, 0, 0, 0x2e, 0x14, 0, 0, 0x4, 0x32, 0x3b, 0x3, 0, 0,
    0, 0x21, 0x38, 0x4, 0, 0, 0, 0, 0x7, 0x3b, 0xf, 0, 0x2e, 0x35, 0x35, 0x35, 0x16, 0, 0, 0x3, 0x26, 0x49, 0x1f, 0x1e, 0x33, 0x33, 0x33, 0x33, 0x33, 0, 0,
    0x18, 0x3a, 0x5, 0, 0, 0, 0x14, 0x37, 0x3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x2f, 0, 0x31, 0x35, 0x35, 0x35, 0x27, 0,
    0x31, 0x35, 0x35, 0x35, 0x26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x3c, 0xa6, 0xea, 0x9e, 0x53, 0xf, 0x3, 0x5f, 0xa3, 0, 0x13, 0, 0, 0, 0,
    0, 0, 0, 0x6d, 0xe9, 0xea, 0xb4, 0x23, 0, 0x7, 0x98, 0xff, 0x61, 0, 0x27, 0xd4, 0xee, 0xe8, 0xe8, 0x2f, 0x32, 0xcf, 0xdd, 0xe5, 0xef, 0x33, 0, 0, 0, 0x89,
    0xff, 0x41, 0, 0xea, 0xf9, 0xf6, 0xf6, 0x64, 0, 0x25, 0xcf, 0xfe, 0xd1, 0x6c, 0x90, 0xf6, 0xf6, 0xf6, 0xf9, 0xff, 0, 0xa0, 0xea, 0xca, 0xee, 0x3e, 0, 0x9e,
    0xf3, 0xe2, 0xe3, 0x1c, 0, 0, 0, 0, 0, 0x1, 0, 0, 0, 0, 0, 0, 0x1, 0, 0, 0, 0, 0, 0, 0, 0x1f, 0xfc, 0, 0xea, 0xf4, 0xec, 0xec, 0xac, 0, 0xea, 0xf4, 0xec,
    0xec, 0xa9, 0, 0, 0, 0, 0x17, 0x3a, 0x5, 0x30, 0xc, 0x6, 0x36, 0xc, 0, 0, 0x34, 0x13, 0x16, 0xf6, 0x91, 0xc3, 0x79, 0x5b, 0x80, 0xed, 0xc9, 0xd9, 0xdb,
    0xcb, 0, 0, 0x31, 0xa9, 0, 0, 0x6, 0xf7, 0x57, 0xc, 0xbd, 0x88, 0x5, 0xc6, 0xb6, 0xed, 0x61, 0, 0x8, 0x64, 0x5, 0x2, 0xbf, 0x9d, 0x4, 0x3a, 0, 0x1, 0xbe,
    0xa2, 0, 0, 0x3d, 0xd1, 0xfc, 0x41, 0x2, 0xfc, 0x41, 0, 0, 0, 0, 0xc9, 0xa9, 0x1f, 0, 0, 0, 0, 0, 0, 0x90, 0xbf, 0x23, 0xfe, 0x27, 0, 0x9b, 0xaf, 0x2b,
    0xfc, 0x21, 0, 0xa9, 0xb1, 0, 0, 0, 0x1, 0x50, 0xce, 0, 0, 0, 0, 0, 0, 0x6f, 0x98, 0x1e, 0, 0, 0, 0, 0, 0x7c, 0xc1, 0, 0xea, 0x6d, 0, 0, 0, 0, 0xea, 0x6a,
    0, 0, 0, 0, 0, 0, 0x85, 0xf6, 0xdf, 0xbf, 0xf9, 0x39, 0x1d, 0xff, 0x38, 0, 0, 0xf6, 0x5c, 0x30, 0xff, 0x55, 0xa7, 0, 0, 0x6, 0x21, 0xdd, 0xeb, 0x45, 0xc,
    0, 0, 0x3a, 0xc8, 0, 0, 0x35, 0xfd, 0x9, 0, 0x77, 0xc1, 0, 0x44, 0x3, 0xee, 0x61, 0, 0, 0, 0, 0, 0xa6, 0xa9, 0, 0, 0, 0x9, 0xdb, 0x79, 0, 0xd, 0xdc, 0x43,
    0xff, 0x41, 0x13, 0xff, 0x38, 0xf, 0, 0, 0x36, 0xff, 0x34, 0x26, 0x5, 0, 0, 0, 0, 0x12, 0xf3, 0x4e, 0xa, 0xf1, 0x6b, 0x16, 0xce, 0x80, 0x6e, 0xda, 0, 0,
    0x58, 0xe9, 0, 0x1, 0x50, 0xce, 0xbc, 0x44, 0x49, 0xab, 0xab, 0xab, 0xab, 0xa8, 0xf, 0x78, 0xe1, 0x98, 0x1f, 0, 0, 0x1, 0xdb, 0x62, 0, 0xea, 0x6d, 0, 0, 0,
    0, 0xea, 0x6a, 0, 0, 0, 0, 0, 0xe, 0xfa, 0x58, 0, 0x6f, 0xff, 0x39, 0x1d, 0xff, 0x38, 0, 0, 0xf6, 0x5c, 0, 0x90, 0xfc, 0xdd, 0x46, 0, 0, 0x8b, 0xbb, 0x75,
    0xce, 0x6, 0x41, 0x8d, 0xa7, 0xe7, 0x8d, 0x8d, 0x6b, 0xe3, 0, 0, 0x52, 0xf5, 0, 0, 0, 0xf0, 0x61, 0, 0, 0, 0, 0x31, 0xf8, 0x3f, 0, 0x33, 0xcb, 0xf8, 0x85,
    0x5, 0, 0x9c, 0x8e, 0x8, 0xff, 0x41, 0x1f, 0xfd, 0xfe, 0xfa, 0xb8, 0x2d, 0x5b, 0xf2, 0xc9, 0xe6, 0xe7, 0x5a, 0, 0, 0, 0x7c, 0xd9, 0x2, 0, 0x37, 0xfb, 0xf8,
    0xa4, 0x2, 0x2e, 0xfd, 0x49, 0x26, 0xbd, 0xfe, 0x35, 0xce, 0xac, 0x34, 0, 0, 0x24, 0x55, 0x55, 0x55, 0x55, 0x53, 0, 0, 0x8, 0x68, 0xd8, 0x98, 0, 0x3b,
    0xf4, 0xd, 0, 0xea, 0xe8, 0xd6, 0xd6, 0x6f, 0, 0xea, 0xb6, 0x82, 0x82, 0x42, 0, 0, 0x4f, 0xfb, 0xc, 0, 0x23, 0xff, 0x39, 0x1d, 0xff, 0x38, 0, 0, 0xf6,
    0x5c, 0, 0, 0x3a, 0xe4, 0xf4, 0x89, 0, 0x24, 0x30, 0xc, 0x48, 0, 0x34, 0x72, 0x92, 0xe1, 0x72, 0x72, 0x5e, 0xea, 0, 0, 0x59, 0xee, 0, 0, 0, 0xf0, 0x61, 0,
    0, 0, 0x22, 0xe6, 0x76, 0, 0, 0x15, 0x57, 0x82, 0xe7, 0x79, 0x4d, 0xd5, 0x9, 0x8, 0xff, 0x41, 0, 0x13, 0x3, 0x2c, 0xcd, 0xa5, 0x6f, 0xfc, 0x30, 0x1, 0x81,
    0xd3, 0, 0, 0x9, 0xe9, 0x6a, 0, 0xf, 0xc5, 0xb2, 0x78, 0xed, 0x63, 0, 0x7f, 0xe4, 0xe4, 0xa5, 0xeb, 0x34, 0xc4, 0xcd, 0x64, 0xa, 0, 0x2e, 0x6c, 0x6c, 0x6c,
    0x6c, 0x6a, 0, 0, 0x29, 0x92, 0xe4, 0x93, 0, 0x9a, 0xa3, 0, 0, 0xea, 0x96, 0x48, 0x48, 0x25, 0, 0xea, 0xc8, 0xa1, 0xa1, 0x52, 0, 0, 0x43, 0xfe, 0x12, 0,
    0x23, 0xff, 0x39, 0x1c, 0xff, 0x3d, 0, 0x1c, 0xff, 0x5c, 0x6, 0, 0x17, 0xa7, 0x8a, 0xdd, 0, 0, 0, 0, 0, 0, 0, 0, 0x3a, 0xc8, 0, 0, 0x25, 0xff, 0xf, 0,
    0x7f, 0xb8, 0, 0, 0, 0xf0, 0x61, 0, 0, 0x20, 0xe0, 0x7d, 0, 0, 0, 0, 0, 0, 0x84, 0xe4, 0xc4, 0xf0, 0xe2, 0xe3, 0xff, 0xe9, 0, 0, 0, 0, 0x7b, 0xdd, 0x4b,
    0xf3, 0x7, 0, 0x47, 0xf8, 0, 0, 0x68, 0xed, 0xb, 0, 0x5f, 0xe3, 0x2, 0, 0x5d, 0xeb, 0, 0, 0x1, 0, 0xaa, 0xbd, 0, 0, 0x35, 0xa5, 0xea, 0x90, 0x3f, 0x92,
    0x92, 0x92, 0x92, 0x90, 0x34, 0xbe, 0xe0, 0x74, 0x10, 0, 0x9, 0xf0, 0x44, 0, 0, 0xea, 0x6d, 0, 0, 0, 0, 0xea, 0x6a, 0, 0, 0, 0, 0, 0x7, 0xf5, 0x7b, 0x10,
    0x8b, 0xff, 0x39, 0x2, 0xe4, 0x8c, 0x10, 0x86, 0xff, 0x5c, 0x60, 0xd2, 0xa9, 0xea, 0xfd, 0x77, 0, 0, 0, 0, 0, 0, 0, 0, 0x2a, 0x91, 0, 0, 0x1, 0xea, 0x87,
    0x2f, 0xd5, 0x81, 0, 0, 0, 0xf0, 0x61, 0, 0x1f, 0xdf, 0x9f, 0x29, 0x29, 0x29, 0x36, 0x28, 0x4, 0x29, 0xcd, 0x9c, 0x2a, 0x35, 0x35, 0x3b, 0xff, 0x69, 0x28,
    0x37, 0x9, 0x32, 0xd3, 0x81, 0x16, 0xef, 0x7f, 0x18, 0xa7, 0xaf, 0, 0x3, 0xdb, 0x87, 0, 0, 0x3e, 0xf5, 0x26, 0x5, 0x88, 0xc7, 0, 0x6, 0x1b, 0x75, 0xf4,
    0x48, 0, 0, 0, 0, 0x1b, 0x87, 0, 0, 0, 0, 0, 0, 0x4d, 0x56, 0x4, 0, 0, 0, 0x5a, 0xe2, 0x2, 0, 0, 0xea, 0x81, 0x24, 0x24, 0x1a, 0, 0xea, 0x6a, 0, 0, 0, 0,
    0, 0, 0x5c, 0xda, 0xda, 0x71, 0xff, 0x39, 0, 0x68, 0xe4, 0xfa, 0xa6, 0xd0, 0x5c, 0x9, 0x38, 0x74, 0xc9, 0x22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0x32, 0xb5, 0xef, 0x82, 0x10, 0, 0, 0, 0xf0, 0x61, 0, 0x7e, 0xff, 0xff, 0xff, 0xff, 0xff, 0x5f, 0xdc, 0xfc, 0xe9, 0xa6, 0x22, 0, 0, 0, 0x8, 0xff, 0x41,
    0x3e, 0xd7, 0xfd, 0xe4, 0x95, 0x17, 0, 0x47, 0xdc, 0xfb, 0xb2, 0x30, 0, 0x55, 0xfa, 0x1b, 0, 0, 0, 0x90, 0xe3, 0xf2, 0xbf, 0x3c, 0, 0xec, 0xfc, 0xde, 0x57,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xb9, 0x85, 0, 0, 0, 0xea, 0xff, 0xff, 0xff, 0xba, 0, 0xea, 0x6a, 0, 0, 0, 0, 0, 0, 0, 0x3, 0x4,
    0x1a, 0xff, 0x39, 0, 0, 0x2, 0xa, 0, 0, 0, 0, 0, 0x10, 0x70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0xd, 0x1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xc, 0x1, 0, 0, 0, 0, 0, 0xa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xb, 0, 0, 0, 0xf, 0xd, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1a, 0xff, 0x39, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x14, 0xc2, 0x2b, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x31,
    0x17, 0, 0, 0, 0, 0, 0x15, 0x43, 0x14, 0, 0x2d, 0x35, 0x35, 0x35, 0x35, 0x35, 0x32, 0x1b, 0, 0, 0, 0x26, 0x1c, 0x35, 0x35, 0x35, 0x35, 0x35, 0, 0, 0x1d,
    0x17, 0, 0, 0, 0, 0x8c, 0xa9, 0x1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xb, 0xbc, 0x2f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0x3f, 0x7a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x31, 0x35, 0x35, 0x35, 0x2b, 0xd, 0x35, 0x4, 0x35, 0xe, 0, 0x12, 0x41, 0xe, 0, 0x2f, 0x14,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xea, 0x6d, 0, 0, 0, 0, 0xa1, 0xf9, 0xe4, 0xfe, 0x7c, 0xc9, 0xf0, 0xf6, 0xfe, 0xf0, 0xf0, 0x9e, 0xd1, 0x2, 0,
    0x14, 0xf1, 0x7c, 0xf0, 0xf0, 0xf0, 0xf8, 0xff, 0, 0, 0xbd, 0xae, 0, 0, 0, 0, 0xb, 0xb2, 0x74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0xe, 0xff, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x58, 0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xb5, 0x60, 0x60, 0x60, 0xaa,
    0x33, 0xfc, 0x2, 0xf9, 0x36, 0x9f, 0xeb, 0xd6, 0xf7, 0x5d, 0xa5, 0x99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xea, 0x6d, 0, 0, 0, 0x2b, 0xfe, 0x2d,
    0, 0x1a, 0x15, 0, 0, 0x6c, 0xea, 0, 0, 0x1d, 0xf7, 0x58, 0, 0x8a, 0xda, 0, 0, 0, 0x8, 0xd5, 0x9c, 0, 0x35, 0xc9, 0xd2, 0x34, 0, 0, 0, 0, 0x1, 0x6, 0, 0,
    0x3, 0x35, 0x42, 0x9, 0, 0, 0, 0x7, 0x38, 0x3, 0, 0, 0, 0x28, 0x48, 0x36, 0x36, 0xe, 0xff, 0x40, 0, 0xe, 0x36, 0x34, 0x17, 0, 0, 0xf, 0x36, 0x25, 0x2c, 0,
    0, 0x1a, 0x35, 0x33, 0x1a, 0, 0, 0x10, 0x36, 0, 0, 0x58, 0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x95, 0, 0, 0, 0x95, 0x1f, 0xea, 0, 0xe6, 0x22, 0x1c, 0x4, 0,
    0x7a, 0xcf, 0x46, 0xef, 0x9, 0, 0, 0, 0x4, 0x3e, 0x21, 0, 0x4, 0x36, 0x34, 0x6, 0, 0, 0, 0xea, 0x6d, 0, 0, 0, 0x24, 0xff, 0x4c, 0, 0, 0, 0, 0, 0x6c, 0xea,
    0, 0, 0, 0x8b, 0xd9, 0x1d, 0xf5, 0x54, 0, 0, 0, 0x8b, 0xe0, 0xe, 0, 0xac, 0x58, 0x54, 0xb7, 0, 0, 0, 0, 0, 0, 0, 0, 0xa7, 0xd5, 0xc6, 0xf3, 0x42, 0, 0x6d,
    0xe1, 0xc7, 0xe1, 0x49, 0x2, 0xca, 0xbd, 0xb6, 0xff, 0xb2, 0xe, 0xff, 0x40, 0xc, 0xc7, 0xae, 0xba, 0xa4, 0, 0, 0x7f, 0xdf, 0x46, 0xfb, 0x38, 0x8, 0xd8,
    0x97, 0xb6, 0xb1, 0, 0, 0x81, 0xe2, 0, 0, 0x58, 0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x95, 0, 0, 0, 0x95, 0xa, 0xa3, 0, 0xa1, 0xd, 0, 0, 0x2, 0xa9, 0xb9, 0x3,
    0xe3, 0x58, 0, 0, 0x70, 0xe7, 0xdd, 0xf7, 0x11, 0xe9, 0xe3, 0xd8, 0xcf, 0, 0, 0, 0xea, 0x6d, 0, 0, 0, 0, 0x84, 0xfb, 0xb6, 0x33, 0, 0, 0, 0x6c, 0xea, 0, 0,
    0, 0x12, 0xef, 0xd9, 0xc8, 0x1, 0, 0, 0x3b, 0xfb, 0x42, 0, 0x26, 0xdf, 0x4, 0x2, 0xd1, 0x3d, 0, 0, 0, 0, 0, 0, 0, 0xa, 0, 0, 0xa7, 0xa5, 0x7, 0xf1, 0x3f,
    0, 0x77, 0xba, 0x45, 0xf4, 0x6, 0x1, 0xe9, 0x66, 0xe, 0xff, 0x45, 0xbc, 0xbb, 0x7, 0x59, 0xf3, 0xa, 0, 0xd8, 0x80, 0, 0x95, 0xd5, 0x8f, 0xdb, 0xb, 0x50,
    0xf9, 0x13, 0, 0xd8, 0x85, 0, 0, 0x58, 0xaa, 0, 0, 0x2c, 0xb1, 0xcf, 0x88, 0x4c, 0x89, 0, 0x95, 0, 0, 0, 0x95, 0, 0, 0, 0, 0, 0, 0x1, 0x9a, 0xd6, 0x1d, 0,
    0x86, 0xb7, 0, 0x7, 0xf1, 0x62, 0, 0x6, 0x58, 0xf8, 0x20, 0, 0x8, 0, 0, 0, 0xea, 0x6d, 0, 0, 0, 0, 0, 0x33, 0xbe, 0xfe, 0x5a, 0, 0, 0x6c, 0xea, 0, 0, 0, 0,
    0x79, 0xff, 0x40, 0, 0, 0xa, 0xda, 0x92, 0, 0, 0x9a, 0x75, 0, 0, 0x53, 0xc0, 0, 0, 0, 0, 0, 0, 0, 0x44, 0x8b, 0xbf, 0xe6, 0xc5, 0x49, 0xfe, 0xc3, 0xc2,
    0xd2, 0xfc, 0x36, 0xfe, 0x39, 0x2b, 0xf8, 0x29, 0xe, 0xff, 0xc7, 0xf9, 0x18, 0, 0x9, 0xef, 0x57, 0x32, 0xfd, 0x20, 0, 0xb, 0xe0, 0xff, 0x3d, 0, 0x4, 0xe5,
    0x69, 0x31, 0xfe, 0x25, 0, 0, 0x58, 0xaa, 0, 0, 0x49, 0x3a, 0x47, 0x91, 0xbd, 0x67, 0, 0x95, 0, 0, 0, 0x95, 0, 0, 0, 0, 0, 0, 0x43, 0xe6, 0x15, 0, 0, 0x28,
    0xfb, 0x1a, 0x4a, 0xfb, 0xd, 0, 0, 0x11, 0xc2, 0xf4, 0x90, 0x19, 0, 0, 0, 0xea, 0x6d, 0, 0, 0, 0, 0, 0, 0, 0xaf, 0xbe, 0, 0, 0x6c, 0xea, 0, 0, 0, 0, 0x47,
    0xff, 0x12, 0, 0, 0x91, 0xda, 0xa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x27, 0xfc, 0x6a, 0x1e, 0x9a, 0xc5, 0x3b, 0xfd, 0x42, 0x31, 0x31, 0x31, 0,
    0x9f, 0xe5, 0xb3, 0x5d, 0, 0xe, 0xff, 0xac, 0xd2, 0xa4, 0, 0, 0x97, 0xac, 0x88, 0xbc, 0, 0, 0x2b, 0xf7, 0xee, 0x75, 0, 0, 0x82, 0xbf, 0x86, 0xc3, 0, 0, 0,
    0x58, 0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x95, 0, 0, 0, 0x95, 0, 0, 0, 0, 0, 0, 0x34, 0x5b, 0, 0, 0, 0, 0xc7, 0x76, 0x40, 0xfe, 0x15, 0, 0, 0, 0x1, 0x4f,
    0xd1, 0xdf, 0, 0, 0, 0xea, 0x83, 0x27, 0x27, 0x23, 0x39, 0x44, 0x14, 0x2b, 0xd1, 0x7b, 0, 0, 0x6c, 0xea, 0, 0, 0, 0, 0x47, 0xff, 0x12, 0, 0x40, 0xfd, 0x60,
    0x27, 0x27, 0x27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x5f, 0xf6, 0xf, 0x1f, 0xe4, 0xc5, 0x1, 0xe0, 0x89, 0xe, 0x7, 0x31, 0x8, 0xfa, 0x4e, 0x24, 0xa, 0,
    0xe, 0xff, 0x40, 0x2b, 0xf5, 0x65, 0, 0x36, 0xef, 0xd1, 0x5b, 0, 0x5, 0xcd, 0x9e, 0x4b, 0xf8, 0x2d, 0, 0x1e, 0xf6, 0xda, 0x63, 0, 0, 0, 0x58, 0xaa, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0x95, 0, 0, 0, 0x95, 0, 0, 0, 0, 0, 0, 0x10, 0x5f, 0, 0, 0, 0, 0x68, 0xd4, 0x4, 0xec, 0x8a, 0x14, 0x3f, 0x27, 0x2c, 0x1, 0x54, 0xee,
    0, 0, 0, 0xea, 0xff, 0xff, 0xff, 0xe6, 0x51, 0xdb, 0xfd, 0xea, 0xa3, 0x11, 0, 0, 0x6c, 0xea, 0, 0, 0, 0, 0x47, 0xff, 0x12, 0, 0xa0, 0xff, 0xff, 0xff, 0xff,
    0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xf, 0xc8, 0xf7, 0xdf, 0x83, 0xc5, 0, 0x3f, 0xae, 0xf4, 0xfc, 0x9f, 0xa, 0xd2, 0xfc, 0xfb, 0xf7, 0x90, 0xe, 0xff,
    0x40, 0, 0x65, 0xf6, 0, 0, 0xd4, 0xf0, 0x9, 0, 0x83, 0xe5, 0x10, 0, 0xa2, 0xd0, 0, 0, 0xb4, 0xf4, 0xd, 0, 0, 0, 0x58, 0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0xc6, 0x95, 0x95, 0x95, 0xb6, 0, 0, 0, 0, 0, 0, 0x82, 0xe8, 0, 0, 0, 0, 0x11, 0xf7, 0, 0x4f, 0xbf, 0xfe, 0xf1, 0x57, 0xe9, 0xf3, 0xde, 0x67, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0xc, 0x2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xe, 0, 0, 0, 0, 0, 0, 0xd,
    0xa, 0, 0xaa, 0x94, 0x8, 0x2, 0x4e, 0xf8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x6, 0xbf, 0xa0, 0, 0, 0, 0, 0x58, 0xaa, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x3, 0xf, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10, 0x5, 0, 0, 0xf, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xa0, 0xae, 0x37, 0x49, 0xae,
    0x9e, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x42, 0xc3, 0xf1, 0x1e, 0, 0, 0, 0, 0x58, 0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x18, 0x79, 0xb0, 0x96, 0x5c, 0x10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0x89, 0x9f, 0x3a, 0, 0, 0, 0, 0, 0x3d, 0x77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x5a, 0xff, 0x5a, 0xff, 0x8f, 0, 0, 0, 0xff, 0x8f, 0, 0, 0, 0x8f, 0xff, 0xff,
    0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x3c, 0xaa, 0x3c, 0x8f, 0xff, 0x8f, 0, 0, 0x8f, 0xff, 0x8f, 0, 0x8f, 0xff, 0x8f, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0x8f, 0xff, 0x8f, 0, 0, 0x8f, 0xff, 0x8f, 0xff, 0x8f, 0, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x14, 0x5c, 0, 0, 0, 0, 0, 0, 0x5c, 0x14, 0, 0, 0, 0, 0, 0, 0x6e, 0xaa, 0, 0x3c, 0xaa, 0x3c, 0, 0, 0x8f, 0xff, 0x8f,
    0, 0, 0x8f, 0xff, 0x8f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x18, 0x86, 0xbd, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10,
    0x17, 0, 0, 0, 0xa, 0x35, 0x13, 0, 0, 0x30, 0x20, 0x1d, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x31, 0x17, 0, 0x33, 0x15, 0x7, 0x35, 0x35,
    0x26, 0x35, 0x31, 0x3, 0x7f, 0x18, 0x3, 0x7f, 0x18, 0xb, 0xbc, 0x31, 0, 0, 0xd, 0x35, 0, 0, 0, 0x5e, 0xff, 0x70, 0, 0, 0, 0, 0x70, 0xff, 0x5e, 0, 0, 0, 0,
    0, 0, 0xa5, 0xff, 0, 0x5a, 0xff, 0x5a, 0, 0x8f, 0xff, 0x8f, 0, 0, 0, 0, 0x8f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0x81, 0xd8, 0x56, 0, 0, 0, 0, 0, 0x4f, 0x3b, 0, 0, 0, 0xb5, 0xf5, 0xb0, 0xe7, 0x28, 0, 0x26, 0xff, 0x55, 0, 0x3e, 0xe7, 0x4c, 0xdc, 0x6, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xea, 0x6d, 0, 0xf4, 0x64, 0x22, 0xff, 0xcb, 0x8e, 0xd6, 0xeb, 0xb, 0xc5, 0x2f, 0xb, 0xc5, 0x2f, 0xe, 0xff, 0x43, 0, 0, 0x33,
    0xfc, 0, 0, 0, 0, 0x70, 0xff, 0x70, 0, 0, 0x70, 0xff, 0x70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x8f, 0xff, 0x8f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0x17, 0x36, 0x36, 0x36, 0x36, 0, 0, 0, 0, 0x7, 0xcd, 0xa8, 0x36, 0x3, 0x35, 0, 0x24, 0xb, 0xcf, 0x85, 0x36, 0, 0x32, 0xfd, 0x1b,
    0x1, 0xbb, 0x85, 0, 0x19, 0xff, 0x48, 0x1, 0xca, 0x70, 0x1, 0xca, 0x6f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x2c, 0x9, 0, 0x2c, 0x8, 0, 0xea, 0x6d, 0, 0xf4,
    0x64, 0x22, 0xff, 0x12, 0, 0x46, 0xeb, 0x3, 0x36, 0xe, 0x3, 0x36, 0xe, 0xe, 0xff, 0x43, 0, 0, 0x1f, 0xea, 0, 0, 0, 0, 0, 0x70, 0xff, 0x70, 0x70, 0xff,
    0x70, 0, 0, 0, 0, 0, 0x55, 0x37, 0, 0x37, 0x55, 0, 0x1e, 0x55, 0x1e, 0xff, 0x8f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x57,
    0xcc, 0xcc, 0xe8, 0xf8, 0, 0, 0, 0, 0x98, 0xf3, 0xe6, 0xc9, 0xe, 0xff, 0x85, 0xfc, 0x93, 0xfb, 0xde, 0xc9, 0, 0x41, 0xf6, 0, 0, 0x9e, 0x99, 0, 0xc, 0xff,
    0x3c, 0x2c, 0xff, 0xf, 0, 0x69, 0xd0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1e, 0xf8, 0x54, 0x20, 0xf8, 0x51, 0, 0xea, 0x6d, 0, 0xf4, 0x64, 0x22, 0xff, 0x12, 0,
    0x46, 0xeb, 0xe, 0xff, 0x43, 0xe, 0xff, 0x43, 0xe, 0xff, 0x43, 0, 0, 0xa, 0xa3, 0, 0, 0, 0, 0, 0, 0x70, 0xff, 0xff, 0x70, 0, 0, 0, 0, 0, 0, 0xff, 0xa5, 0,
    0xa5, 0xff, 0, 0x5a, 0xff, 0x5a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x14, 0xe6, 0x6f, 0, 0, 0, 0, 0, 0xc7,
    0x8b, 0, 0xe, 0xff, 0xc7, 0x1f, 0, 0xee, 0x64, 0, 0, 0x64, 0xe7, 0, 0, 0x8f, 0xba, 0, 0x1, 0xfd, 0x2f, 0x54, 0xe5, 0, 0, 0x41, 0xf6, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0x2, 0x45, 0x9, 0x3, 0x46, 0x9, 0, 0xea, 0x6d, 0, 0xf4, 0x64, 0x22, 0xff, 0x12, 0, 0x46, 0xeb, 0xe, 0xff, 0x43, 0xe, 0xff, 0x43, 0xe, 0xff, 0x43, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0x70, 0xff, 0xff, 0x70, 0, 0, 0, 0, 0, 0, 0x55, 0x37, 0, 0x37, 0x55, 0, 0x1e, 0x55, 0x1e, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1, 0xb1, 0xb6, 0x1, 0, 0, 0, 0, 0, 0xc7, 0x8b, 0, 0xe, 0xff, 0x5d, 0, 0, 0xee, 0x64, 0, 0x9b, 0xe8, 0x53, 0, 0,
    0x23, 0xc4, 0xd7, 0, 0xf1, 0x22, 0x7b, 0xbe, 0, 0, 0x1a, 0xff, 0, 0, 0, 0x62, 0xa4, 0xa4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xea, 0x6d, 0, 0xf4, 0x64, 0x22,
    0xff, 0x12, 0, 0x46, 0xeb, 0xe, 0xff, 0x43, 0xe, 0xff, 0x43, 0xe, 0xff, 0x43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x70, 0xff, 0x70, 0x70, 0xff, 0x70, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x69, 0xe8, 0x17, 0, 0, 0, 0, 0, 0, 0xc7, 0x8b, 0,
    0xe, 0xff, 0x43, 0, 0, 0xee, 0x64, 0, 0x52, 0xcc, 0xa7, 0, 0, 0x4e, 0xea, 0x76, 0, 0x75, 0xc, 0x6f, 0xca, 0, 0, 0x26, 0xff, 0, 0, 0, 0x49, 0x7a, 0x7a, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0xea, 0x6d, 0, 0xf4, 0x64, 0x22, 0xff, 0x12, 0, 0x46, 0xeb, 0xe, 0xff, 0x43, 0xe, 0xff, 0x43, 0xe, 0xff, 0x43, 0, 0, 0, 0, 0, 0,
    0, 0, 0x70, 0xff, 0x70, 0, 0, 0x70, 0xff, 0x70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0x2a, 0xf5, 0x4c, 0x3, 0x3, 0, 0, 0, 0, 0, 0xc7, 0x8b, 0, 0xe, 0xff, 0x43, 0, 0, 0xd0, 0x8f, 0x5, 0, 0x4a, 0xf5, 0, 0, 0x99, 0xa2, 0, 0x8, 0x75,
    0x17, 0x47, 0xf2, 0x1, 0, 0x4e, 0xeb, 0x9, 0x56, 0x17, 0, 0, 0, 0x8, 0x75, 0x17, 0x8, 0x75, 0x17, 0x9, 0x56, 0x17, 0, 0xea, 0x6d, 0, 0xf4, 0x64, 0x22,
    0xff, 0x12, 0, 0x46, 0xeb, 0xe, 0xff, 0x43, 0xe, 0xff, 0x43, 0xe, 0xff, 0x43, 0, 0, 0, 0, 0, 0, 0, 0x5e, 0xff, 0x70, 0, 0, 0, 0, 0x70, 0xff, 0x5e, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x9a, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0xc7,
    0x8b, 0, 0xe, 0xff, 0x43, 0, 0, 0x62, 0xee, 0xfc, 0, 0x41, 0xf7, 0, 0, 0x9f, 0x99, 0, 0x27, 0xfe, 0x4f, 0x1a, 0xfc, 0x27, 0, 0x82, 0xba, 0x3d, 0xf9, 0x14,
    0, 0, 0, 0x27, 0xfe, 0x4f, 0x27, 0xfe, 0x4f, 0x3d, 0xf9, 0x14, 0, 0xea, 0x6d, 0, 0xf6, 0x60, 0x22, 0xff, 0x12, 0, 0x46, 0xeb, 0xe, 0xff, 0x43, 0xe, 0xff,
    0x43, 0xe, 0xff, 0x43, 0, 0, 0, 0, 0, 0, 0, 0x14, 0x5c, 0, 0, 0, 0, 0, 0, 0x5c, 0x14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x6, 0xe, 0, 0x13, 0xf7, 0x74, 0x3a, 0xe1, 0x6d, 0, 0,
    0x12, 0, 0, 0x97, 0x9e, 0xd, 0xeb, 0x3d, 0x71, 0xaf, 0, 0, 0, 0, 0, 0x12, 0, 0, 0x12, 0, 0x71, 0xaf, 0, 0, 0, 0, 0x33, 0xff, 0x25, 0x22, 0xff, 0x3c, 0x20,
    0x67, 0xeb, 0, 0, 0, 0x17, 0xff, 0x42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xc1, 0xc1, 0xc1, 0xc1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x58, 0xa8, 0x8b,
    0x86, 0xd, 0, 0, 0, 0, 0, 0x16, 0xc8, 0x5c, 0x97, 0, 0x49, 0x33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x49, 0x33, 0, 0, 0, 0, 0xf9, 0xad, 0, 0x1c, 0xd1, 0xd1,
    0x95, 0xd1, 0xc0, 0, 0, 0, 0xba, 0xf7, 0xe, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static ATLAS: [Rect; 134] = [
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect {
            x: 88 as libc::c_int,
            y: 68 as libc::c_int,
            w: 16 as libc::c_int,
            h: 16 as libc::c_int,
        },
    Rect {
            x: 0 as libc::c_int,
            y: 0 as libc::c_int,
            w: 18 as libc::c_int,
            h: 18 as libc::c_int,
        },
    Rect {
            x: 113 as libc::c_int,
            y: 68 as libc::c_int,
            w: 5 as libc::c_int,
            h: 7 as libc::c_int,
        },
    Rect {
            x: 118 as libc::c_int,
            y: 68 as libc::c_int,
            w: 7 as libc::c_int,
            h: 5 as libc::c_int,
        },
    Rect {
            x: 125 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 3 as libc::c_int,
        },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect { x: 0, y: 0, w: 0, h: 0 },
    Rect {
            x: 84 as libc::c_int,
            y: 68 as libc::c_int,
            w: 2 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 39 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 114 as libc::c_int,
            y: 51 as libc::c_int,
            w: 5 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 34 as libc::c_int,
            y: 17 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 28 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 58 as libc::c_int,
            y: 0 as libc::c_int,
            w: 9 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 103 as libc::c_int,
            y: 0 as libc::c_int,
            w: 8 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 86 as libc::c_int,
            y: 68 as libc::c_int,
            w: 2 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 42 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 45 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 34 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 40 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 48 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 51 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 54 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 124 as libc::c_int,
            y: 34 as libc::c_int,
            w: 4 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 46 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 52 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 58 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 64 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 70 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 76 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 82 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 88 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 94 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 100 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 57 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 60 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 106 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 112 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 118 as libc::c_int,
            y: 34 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 119 as libc::c_int,
            y: 51 as libc::c_int,
            w: 5 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 18 as libc::c_int,
            y: 0 as libc::c_int,
            w: 10 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 41 as libc::c_int,
            y: 17 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 48 as libc::c_int,
            y: 17 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 55 as libc::c_int,
            y: 17 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 111 as libc::c_int,
            y: 0 as libc::c_int,
            w: 8 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 0 as libc::c_int,
            y: 35 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 6 as libc::c_int,
            y: 35 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 119 as libc::c_int,
            y: 0 as libc::c_int,
            w: 8 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 18 as libc::c_int,
            y: 17 as libc::c_int,
            w: 8 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 63 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 66 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 62 as libc::c_int,
            y: 17 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 12 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 28 as libc::c_int,
            y: 0 as libc::c_int,
            w: 10 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 67 as libc::c_int,
            y: 0 as libc::c_int,
            w: 9 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 76 as libc::c_int,
            y: 0 as libc::c_int,
            w: 9 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 69 as libc::c_int,
            y: 17 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 85 as libc::c_int,
            y: 0 as libc::c_int,
            w: 9 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 76 as libc::c_int,
            y: 17 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 18 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 24 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 26 as libc::c_int,
            y: 17 as libc::c_int,
            w: 8 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 83 as libc::c_int,
            y: 17 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 38 as libc::c_int,
            y: 0 as libc::c_int,
            w: 10 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 90 as libc::c_int,
            y: 17 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 30 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 36 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 69 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 124 as libc::c_int,
            y: 51 as libc::c_int,
            w: 4 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 72 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 42 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 15 as libc::c_int,
            y: 68 as libc::c_int,
            w: 4 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 48 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 54 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 97 as libc::c_int,
            y: 17 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 0 as libc::c_int,
            y: 52 as libc::c_int,
            w: 5 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 104 as libc::c_int,
            y: 17 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 60 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 19 as libc::c_int,
            y: 68 as libc::c_int,
            w: 4 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 66 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 111 as libc::c_int,
            y: 17 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 75 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 78 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 72 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 81 as libc::c_int,
            y: 68 as libc::c_int,
            w: 3 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 48 as libc::c_int,
            y: 0 as libc::c_int,
            w: 10 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 118 as libc::c_int,
            y: 17 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 0 as libc::c_int,
            y: 18 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 7 as libc::c_int,
            y: 18 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 14 as libc::c_int,
            y: 34 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 23 as libc::c_int,
            y: 68 as libc::c_int,
            w: 4 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 5 as libc::c_int,
            y: 52 as libc::c_int,
            w: 5 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 27 as libc::c_int,
            y: 68 as libc::c_int,
            w: 4 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 21 as libc::c_int,
            y: 34 as libc::c_int,
            w: 7 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 78 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 94 as libc::c_int,
            y: 0 as libc::c_int,
            w: 9 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 84 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 90 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 10 as libc::c_int,
            y: 68 as libc::c_int,
            w: 5 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 31 as libc::c_int,
            y: 68 as libc::c_int,
            w: 4 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 96 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 35 as libc::c_int,
            y: 68 as libc::c_int,
            w: 4 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 102 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
    Rect {
            x: 108 as libc::c_int,
            y: 51 as libc::c_int,
            w: 6 as libc::c_int,
            h: 17 as libc::c_int,
        },
];
static mut tex_buf: [GLfloat; 131072] = [0.; 131072];
static mut vert_buf: [GLfloat; 131072] = [0.; 131072];
static mut color_buf: [GLubyte; 262144] = [0; 262144];
static mut index_buf: [GLuint; 98304] = [0; 98304];
static mut width: libc::c_int = 800 as libc::c_int;
static mut height: libc::c_int = 600 as libc::c_int;
static mut buf_idx: libc::c_int = 0;
static mut window: *mut SDL_Window = 0 as *const SDL_Window as *mut SDL_Window;

#[no_mangle]
pub unsafe extern "C" fn r_init() {
    window = SDL_CreateWindow(
        0 as *const libc::c_char,
        (0x1fff0000 as libc::c_uint | 0 as libc::c_int as libc::c_uint) as libc::c_int,
        (0x1fff0000 as libc::c_uint | 0 as libc::c_int as libc::c_uint) as libc::c_int,
        width,
        height,
        SDL_WINDOW_OPENGL as libc::c_int as Uint32,
    );
    SDL_GL_CreateContext(window);
    glEnable(0xbe2 as libc::c_int as GLenum);
    glBlendFunc(0x302 as libc::c_int as GLenum, 0x303 as libc::c_int as GLenum);
    glDisable(0xb44 as libc::c_int as GLenum);
    glDisable(0xb71 as libc::c_int as GLenum);
    glEnable(0xc11 as libc::c_int as GLenum);
    glEnable(0xde1 as libc::c_int as GLenum);
    glEnableClientState(0x8074 as libc::c_int as GLenum);
    glEnableClientState(0x8078 as libc::c_int as GLenum);
    glEnableClientState(0x8076 as libc::c_int as GLenum);
    let mut id: GLuint = 0;
    glGenTextures(1 as libc::c_int, &mut id);
    glBindTexture(0xde1 as libc::c_int as GLenum, id);
    glTexImage2D(
        0xde1 as libc::c_int as GLenum,
        0 as libc::c_int,
        0x1906 as libc::c_int,
        ATLAS_WIDTH as libc::c_int,
        ATLAS_HEIGHT as libc::c_int,
        0 as libc::c_int,
        0x1906 as libc::c_int as GLenum,
        0x1401 as libc::c_int as GLenum,
        atlas_texture.as_mut_ptr() as *const libc::c_void,
    );
    glTexParameteri(0xde1 as libc::c_int as GLenum, 0x2801 as libc::c_int as GLenum, 0x2600 as libc::c_int);
    glTexParameteri(0xde1 as libc::c_int as GLenum, 0x2800 as libc::c_int as GLenum, 0x2600 as libc::c_int);
    if glGetError() == 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(
            b"glGetError() == 0\0" as *const u8 as *const libc::c_char,
            b"renderer.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"void r_init(void)\0")).as_ptr(),
        );
    };
}

unsafe extern "C" fn flush() {
    if buf_idx == 0 as libc::c_int {
        return;
    }
    glViewport(0 as libc::c_int, 0 as libc::c_int, width, height);
    glMatrixMode(0x1701 as libc::c_int as GLenum);
    glPushMatrix();
    glLoadIdentity();
    glOrtho(
        0.0f32 as GLdouble,
        width as GLdouble,
        height as GLdouble,
        0.0f32 as GLdouble,
        -1.0f32 as GLdouble,
        1.0f32 as GLdouble,
    );
    glMatrixMode(0x1700 as libc::c_int as GLenum);
    glPushMatrix();
    glLoadIdentity();
    glTexCoordPointer(
        2 as libc::c_int,
        0x1406 as libc::c_int as GLenum,
        0 as libc::c_int,
        tex_buf.as_mut_ptr() as *const libc::c_void,
    );
    glVertexPointer(
        2 as libc::c_int,
        0x1406 as libc::c_int as GLenum,
        0 as libc::c_int,
        vert_buf.as_mut_ptr() as *const libc::c_void,
    );
    glColorPointer(
        4 as libc::c_int,
        0x1401 as libc::c_int as GLenum,
        0 as libc::c_int,
        color_buf.as_mut_ptr() as *const libc::c_void,
    );
    glDrawElements(
        0x4 as libc::c_int as GLenum,
        buf_idx * 6 as libc::c_int,
        0x1405 as libc::c_int as GLenum,
        index_buf.as_mut_ptr() as *const libc::c_void,
    );
    glMatrixMode(0x1700 as libc::c_int as GLenum);
    glPopMatrix();
    glMatrixMode(0x1701 as libc::c_int as GLenum);
    glPopMatrix();
    buf_idx = 0 as libc::c_int;
}

unsafe extern "C" fn push_quad(mut dst: Rect, mut src: Rect, mut color: Color) {
    if buf_idx == 16384 as libc::c_int {
        flush();
    }
    let mut texvert_idx: libc::c_int = buf_idx * 8 as libc::c_int;
    let mut color_idx: libc::c_int = buf_idx * 16 as libc::c_int;
    let mut element_idx: libc::c_int = buf_idx * 4 as libc::c_int;
    let mut index_idx: libc::c_int = buf_idx * 6 as libc::c_int;
    buf_idx += 1;
    let mut x: libc::c_float = src.x as libc::c_float / ATLAS_WIDTH as libc::c_int as libc::c_float;
    let mut y: libc::c_float = src.y as libc::c_float / ATLAS_HEIGHT as libc::c_int as libc::c_float;
    let mut w: libc::c_float = src.w as libc::c_float / ATLAS_WIDTH as libc::c_int as libc::c_float;
    let mut h: libc::c_float = src.h as libc::c_float / ATLAS_HEIGHT as libc::c_int as libc::c_float;
    tex_buf[(texvert_idx + 0 as libc::c_int) as usize] = x;
    tex_buf[(texvert_idx + 1 as libc::c_int) as usize] = y;
    tex_buf[(texvert_idx + 2 as libc::c_int) as usize] = x + w;
    tex_buf[(texvert_idx + 3 as libc::c_int) as usize] = y;
    tex_buf[(texvert_idx + 4 as libc::c_int) as usize] = x;
    tex_buf[(texvert_idx + 5 as libc::c_int) as usize] = y + h;
    tex_buf[(texvert_idx + 6 as libc::c_int) as usize] = x + w;
    tex_buf[(texvert_idx + 7 as libc::c_int) as usize] = y + h;
    vert_buf[(texvert_idx + 0 as libc::c_int) as usize] = dst.x as GLfloat;
    vert_buf[(texvert_idx + 1 as libc::c_int) as usize] = dst.y as GLfloat;
    vert_buf[(texvert_idx + 2 as libc::c_int) as usize] = (dst.x + dst.w) as GLfloat;
    vert_buf[(texvert_idx + 3 as libc::c_int) as usize] = dst.y as GLfloat;
    vert_buf[(texvert_idx + 4 as libc::c_int) as usize] = dst.x as GLfloat;
    vert_buf[(texvert_idx + 5 as libc::c_int) as usize] = (dst.y + dst.h) as GLfloat;
    vert_buf[(texvert_idx + 6 as libc::c_int) as usize] = (dst.x + dst.w) as GLfloat;
    vert_buf[(texvert_idx + 7 as libc::c_int) as usize] = (dst.y + dst.h) as GLfloat;
    memcpy(
        color_buf.as_mut_ptr().offset(color_idx as isize).offset(0 as libc::c_int as isize) as *mut libc::c_void,
        &mut color as *mut Color as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        color_buf.as_mut_ptr().offset(color_idx as isize).offset(4 as libc::c_int as isize) as *mut libc::c_void,
        &mut color as *mut Color as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        color_buf.as_mut_ptr().offset(color_idx as isize).offset(8 as libc::c_int as isize) as *mut libc::c_void,
        &mut color as *mut Color as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        color_buf.as_mut_ptr().offset(color_idx as isize).offset(12 as libc::c_int as isize) as *mut libc::c_void,
        &mut color as *mut Color as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    index_buf[(index_idx + 0 as libc::c_int) as usize] = (element_idx + 0 as libc::c_int) as GLuint;
    index_buf[(index_idx + 1 as libc::c_int) as usize] = (element_idx + 1 as libc::c_int) as GLuint;
    index_buf[(index_idx + 2 as libc::c_int) as usize] = (element_idx + 2 as libc::c_int) as GLuint;
    index_buf[(index_idx + 3 as libc::c_int) as usize] = (element_idx + 2 as libc::c_int) as GLuint;
    index_buf[(index_idx + 4 as libc::c_int) as usize] = (element_idx + 3 as libc::c_int) as GLuint;
    index_buf[(index_idx + 5 as libc::c_int) as usize] = (element_idx + 1 as libc::c_int) as GLuint;
}

#[no_mangle]
pub unsafe extern "C" fn r_draw_rect(mut rect: Rect, mut color: Color) {
    push_quad(rect, ATLAS[ATLAS_WHITE as libc::c_int as usize], color);
}

#[no_mangle]
pub unsafe extern "C" fn r_draw_text(text: &str, mut pos: Vec2i, mut color: Color) {
    let mut dst: Rect = Rect {
            x: pos.x,
            y: pos.y,
            w: 0 as libc::c_int,
            h: 0 as libc::c_int,
        };
    for p in text.chars() {
        if !(p as libc::c_int & 0xc0 as libc::c_int == 0x80 as libc::c_int) {
            let mut chr: libc::c_int = i32::min(p as i32, 127);
            let mut src: Rect = ATLAS[(ATLAS_FONT as libc::c_int + chr) as usize];
            dst.w = src.w;
            dst.h = src.h;
            push_quad(dst, src, color);
            dst.x += dst.w;
        }
    }
}

pub fn r_draw_icon(mut id: Icon, mut r: Rect, mut color: Color) {
    let mut src: Rect = ATLAS[id as usize];
    let mut x: libc::c_int = r.x + (r.w - src.w) / 2 as libc::c_int;
    let mut y: libc::c_int = r.y + (r.h - src.h) / 2 as libc::c_int;
    unsafe {
        push_quad(rect(x, y, src.w, src.h), src, color);
    }
}

pub fn r_get_char_width(_font: Font, c: char) -> usize {
    unsafe { ATLAS[ATLAS_FONT as usize + c as usize].w as usize }
}
pub fn r_get_font_height(_font: Font) -> usize {
    18
}

pub fn r_set_clip_rect(mut rect: Rect) {
    unsafe {
        flush();
        glScissor(rect.x, height - (rect.y + rect.h), rect.w, rect.h);
    }
}

pub fn r_clear(mut clr: Color) {
    unsafe {
        flush();
        glClearColor(
            (clr.r as libc::c_int as libc::c_double / 255.0f64) as GLclampf,
            (clr.g as libc::c_int as libc::c_double / 255.0f64) as GLclampf,
            (clr.b as libc::c_int as libc::c_double / 255.0f64) as GLclampf,
            (clr.a as libc::c_int as libc::c_double / 255.0f64) as GLclampf,
        );
        glClear(0x4000 as libc::c_int as GLbitfield);
    }
}

#[no_mangle]
pub unsafe extern "C" fn r_present() {
    flush();
    SDL_GL_SwapWindow(window);
}
