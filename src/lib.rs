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
use core::ptr;

mod fixed_collections;
pub use crate::fixed_collections::*;

mod atlas;
pub use atlas::*;

use bitflags::*;

#[derive(Copy, Clone)]
pub struct Pool<const N: usize> {
    vec: [PoolItem; N],
}

impl<const N: usize> Pool<N> {
    pub fn alloc(&mut self, id: Id, frame: usize) -> usize {
        let mut res = None;
        let mut latest_update = frame;
        for i in 0..N {
            if self.vec[i].last_update < latest_update {
                latest_update = self.vec[i].last_update;
                res = Some(i);
            }
        }

        assert!(res.is_some());
        self.vec[res.unwrap()].id = id;
        self.update(res.unwrap(), frame);
        return res.unwrap();
    }

    pub fn get(&self, id: Id) -> Option<usize> {
        for i in 0..N {
            if self.vec[i].id == id {
                return Some(i);
            }
        }
        None
    }

    pub fn update(&mut self, idx: usize, frame: usize) {
        self.vec[idx].last_update = frame;
    }

    pub fn reset(&mut self, idx: usize) {
        self.vec[idx] = PoolItem::default();
    }
}

impl<const N: usize> Default for Pool<N> {
    fn default() -> Self {
        Self { vec: [PoolItem::default(); N] }
    }
}

#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Clip {
    None = 0,
    Part = 1,
    All = 2,
}

#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum ControlColor {
    Max = 14,
    ScrollThumb = 13,
    ScrollBase = 12,
    BaseFocus = 11,
    BaseHover = 10,
    Base = 9,
    ButtonFocus = 8,
    ButtonHover = 7,
    Button = 6,
    PanelBG = 5,
    TitleText = 4,
    TitleBG = 3,
    WindowBG = 2,
    Border = 1,
    Text = 0,
}

impl ControlColor {
    pub fn hover(&mut self) {
        *self = match self {
            Self::Base => Self::BaseHover,
            Self::Button => Self::ButtonHover,
            _ => *self,
        }
    }

    pub fn focus(&mut self) {
        *self = match self {
            Self::Base => Self::BaseFocus,
            Self::Button => Self::ButtonFocus,
            Self::BaseHover => Self::BaseFocus,
            Self::ButtonHover => Self::ButtonFocus,
            _ => *self,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Icon {
    Max = 5,
    Expanded = 4,
    Collapsed = 3,
    Check = 2,
    Close = 1,
    None = 0,
}

bitflags! {
    pub struct ResourceState : u32 {
        const CHANGE = 4;
        const SUBMIT = 2;
        const ACTIVE = 1;
        const NONE = 0;
    }
}

impl ResourceState {
    pub fn is_changed(&self) -> bool {
        self.intersects(Self::CHANGE)
    }
    pub fn is_submitted(&self) -> bool {
        self.intersects(Self::SUBMIT)
    }
    pub fn is_active(&self) -> bool {
        self.intersects(Self::ACTIVE)
    }
    pub fn is_none(&self) -> bool {
        self.bits == 0
    }
}

bitflags! {
    pub struct WidgetOption : u32 {
        const EXPANDED = 4096;
        const CLOSED = 2048;
        const POPUP= 1024;
        const AUTO_SIZE = 512;
        const HOLD_FOCUS = 256;
        const NO_TITLE = 128;
        const NO_CLOSE = 64;
        const NO_SCROLL = 32;
        const NO_RESIZE = 16;
        const NO_FRAME = 8;
        const NO_INTERACT = 4;
        const ALIGN_RIGHT = 2;
        const ALIGN_CENTER = 1;
        const NONE = 0;
    }
}

impl WidgetOption {
    pub fn is_expanded(&self) -> bool {
        self.intersects(WidgetOption::EXPANDED)
    }
    pub fn is_closed(&self) -> bool {
        self.intersects(WidgetOption::CLOSED)
    }
    pub fn is_popup(&self) -> bool {
        self.intersects(WidgetOption::POPUP)
    }
    pub fn is_auto_sizing(&self) -> bool {
        self.intersects(WidgetOption::AUTO_SIZE)
    }
    pub fn is_holding_focus(&self) -> bool {
        self.intersects(WidgetOption::HOLD_FOCUS)
    }
    pub fn has_no_title(&self) -> bool {
        self.intersects(WidgetOption::NO_TITLE)
    }
    pub fn has_no_close(&self) -> bool {
        self.intersects(WidgetOption::NO_CLOSE)
    }
    pub fn has_no_scroll(&self) -> bool {
        self.intersects(WidgetOption::NO_SCROLL)
    }
    pub fn is_fixed(&self) -> bool {
        self.intersects(WidgetOption::NO_RESIZE)
    }
    pub fn has_no_frame(&self) -> bool {
        self.intersects(WidgetOption::NO_FRAME)
    }
    pub fn is_not_interactive(&self) -> bool {
        self.intersects(WidgetOption::NO_INTERACT)
    }
    pub fn is_aligned_right(&self) -> bool {
        self.intersects(WidgetOption::ALIGN_RIGHT)
    }
    pub fn is_aligned_center(&self) -> bool {
        self.intersects(WidgetOption::ALIGN_CENTER)
    }
    pub fn is_none(&self) -> bool {
        self.bits == 0
    }
}

bitflags! {
    pub struct MouseButton : u32 {
        const MIDDLE = 4;
        const RIGHT = 2;
        const LEFT = 1;
        const NONE = 0;
    }
}

impl MouseButton {
    pub fn is_middle(&self) -> bool {
        self.intersects(Self::MIDDLE)
    }
    pub fn is_right(&self) -> bool {
        self.intersects(Self::RIGHT)
    }
    pub fn is_left(&self) -> bool {
        self.intersects(Self::LEFT)
    }
    pub fn is_none(&self) -> bool {
        self.bits == 0
    }
}

bitflags! {
    pub struct KeyMode : u32 {
        const RETURN = 16;
        const BACKSPACE = 8;
        const ALT = 4;
        const CTRL = 2;
        const SHIFT = 1;
        const NONE = 0;
    }
}

impl KeyMode {
    pub fn is_none(&self) -> bool {
        self.bits == 0
    }
    pub fn is_return(&self) -> bool {
        self.intersects(Self::RETURN)
    }
    pub fn is_backspace(&self) -> bool {
        self.intersects(Self::BACKSPACE)
    }
    pub fn is_alt(&self) -> bool {
        self.intersects(Self::ALT)
    }
    pub fn is_ctrl(&self) -> bool {
        self.intersects(Self::CTRL)
    }
    pub fn is_shift(&self) -> bool {
        self.intersects(Self::SHIFT)
    }
}

#[repr(C)]
pub struct Context {
    pub char_width: Option<fn(FontId, char) -> usize>,
    pub font_height: Option<fn(FontId) -> usize>,
    pub draw_frame: Option<fn(&mut Context, Rect, ControlColor) -> ()>,
    pub style: Style,
    pub hover: Option<Id>,
    pub focus: Option<Id>,
    pub last_id: Option<Id>,
    pub last_rect: Rect,
    pub last_zindex: i32,
    pub updated_focus: bool,
    pub frame: usize,
    pub hover_root: Option<usize>,
    pub next_hover_root: Option<usize>,
    pub scroll_target: Option<usize>,
    pub number_edit_buf: FixedString<127>,
    pub number_edit: Option<Id>,
    pub command_list: FixedVec<Command, 4096>,
    pub root_list: FixedVec<usize, 32>,
    pub container_stack: FixedVec<usize, 32>,
    pub clip_stack: FixedVec<Rect, 32>,
    pub id_stack: FixedVec<Id, 32>,
    pub layout_stack: FixedVec<Layout, 16>,
    pub text_stack: FixedString<65536>,
    pub container_pool: Pool<48>,
    pub containers: [Container; 48],
    pub treenode_pool: Pool<48>,
    pub mouse_pos: Vec2i,
    pub last_mouse_pos: Vec2i,
    pub mouse_delta: Vec2i,
    pub scroll_delta: Vec2i,
    pub mouse_down: MouseButton,
    pub mouse_pressed: MouseButton,
    pub key_down: KeyMode,
    pub key_pressed: KeyMode,
    pub input_text: FixedString<32>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            char_width: None,
            font_height: None,
            draw_frame: None,
            style: Style::default(),
            hover: None,
            focus: None,
            last_id: None,
            last_rect: Rect::default(),
            last_zindex: 0,
            updated_focus: false,
            frame: 0,
            hover_root: None,
            next_hover_root: None,
            scroll_target: None,
            number_edit_buf: FixedString::default(),
            number_edit: None,
            command_list: FixedVec::default(),
            root_list: FixedVec::default(),
            container_stack: FixedVec::default(),
            clip_stack: FixedVec::default(),
            id_stack: FixedVec::default(),
            layout_stack: FixedVec::default(),
            text_stack: FixedString::default(),
            container_pool: Pool::default(),
            containers: [Container::default(); 48],
            treenode_pool: Pool::default(),
            mouse_pos: Vec2i::default(),
            last_mouse_pos: Vec2i::default(),
            mouse_delta: Vec2i::default(),
            scroll_delta: Vec2i::default(),
            mouse_down: MouseButton::NONE,
            mouse_pressed: MouseButton::NONE,
            key_down: KeyMode::NONE,
            key_pressed: KeyMode::NONE,
            input_text: FixedString::default(),
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Copy, Clone)]
struct PoolItem {
    pub id: Id,
    pub last_update: usize,
}

#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct Id(u32);

#[derive(Default, Copy, Clone)]
pub struct Container {
    pub head_idx: Option<usize>,
    pub tail_idx: Option<usize>,
    pub rect: Rect,
    pub body: Rect,
    pub content_size: Vec2i,
    pub scroll: Vec2i,
    pub zindex: i32,
    pub open: bool,
}

#[derive(Default, Copy, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Default, Copy, Clone)]
pub struct Layout {
    pub body: Rect,
    pub next: Rect,
    pub position: Vec2i,
    pub size: Vec2i,
    pub max: Vec2i,
    pub widths: [i32; 16],
    pub items: usize,
    pub item_index: usize,
    pub next_row: i32,
    pub next_type: LayoutPosition,
    pub indent: i32,
}

#[derive(Copy, Clone)]
pub enum Command {
    Jump {
        dst_idx: Option<usize>,
    },
    Clip {
        rect: Rect,
    },
    Rect {
        rect: Rect,
        color: Color,
    },
    Text {
        font: FontId,
        pos: Vec2i,
        color: Color,
        str_start: usize,
        str_len: usize,
    },
    Icon {
        rect: Rect,
        id: Icon,
        color: Color,
    },
    None,
}

impl Default for Command {
    fn default() -> Self {
        Command::None
    }
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub trait Font {
    fn name(&self) -> &str;
    fn get_size(&self) -> usize;
    fn get_char_size(&self, c: char) -> (usize, usize);
}

#[derive(Copy, Clone)]
pub struct FontId(pub usize);

#[derive(Copy, Clone)]
pub struct Style {
    pub font: FontId,
    pub size: Vec2i,
    pub padding: i32,
    pub spacing: i32,
    pub indent: i32,
    pub title_height: i32,
    pub scrollbar_size: i32,
    pub thumb_size: i32,
    pub colors: [Color; 14],
}

pub type Real = f32;

#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum LayoutPosition {
    Absolute = 2,
    Relative = 1,
    None = 0,
}

impl Default for LayoutPosition {
    fn default() -> Self {
        LayoutPosition::None
    }
}

static UNCLIPPED_RECT: Rect = Rect { x: 0, y: 0, w: 0x1000000, h: 0x1000000 };

impl Default for Style {
    fn default() -> Self {
        Self {
            font: FontId(0),
            size: Vec2i { x: 68, y: 10 },
            padding: 5,
            spacing: 4,
            indent: 24,
            title_height: 24,
            scrollbar_size: 12,
            thumb_size: 8,
            colors: [
                Color { r: 230, g: 230, b: 230, a: 255 },
                Color { r: 25, g: 25, b: 25, a: 255 },
                Color { r: 50, g: 50, b: 50, a: 255 },
                Color { r: 25, g: 25, b: 25, a: 255 },
                Color { r: 240, g: 240, b: 240, a: 255 },
                Color { r: 0, g: 0, b: 0, a: 0 },
                Color { r: 75, g: 75, b: 75, a: 255 },
                Color { r: 95, g: 95, b: 95, a: 255 },
                Color { r: 115, g: 115, b: 115, a: 255 },
                Color { r: 30, g: 30, b: 30, a: 255 },
                Color { r: 35, g: 35, b: 35, a: 255 },
                Color { r: 40, g: 40, b: 40, a: 255 },
                Color { r: 43, g: 43, b: 43, a: 255 },
                Color { r: 30, g: 30, b: 30, a: 255 },
            ],
        }
    }
}

pub fn vec2(x: i32, y: i32) -> Vec2i {
    Vec2i { x, y }
}

pub fn rect(x: i32, y: i32, w: i32, h: i32) -> Rect {
    Rect { x, y, w, h }
}

pub fn color(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
}

pub fn expand_rect(r: Rect, n: i32) -> Rect {
    rect(r.x - n, r.y - n, r.w + n * 2, r.h + n * 2)
}

pub fn intersect_rects(r1: Rect, r2: Rect) -> Rect {
    let x1 = if r1.x > r2.x { r1.x } else { r2.x };
    let y1 = if r1.y > r2.y { r1.y } else { r2.y };
    let mut x2 = if r1.x + r1.w < r2.x + r2.w { r1.x + r1.w } else { r2.x + r2.w };
    let mut y2 = if r1.y + r1.h < r2.y + r2.h { r1.y + r1.h } else { r2.y + r2.h };
    if x2 < x1 {
        x2 = x1;
    }
    if y2 < y1 {
        y2 = y1;
    }
    return rect(x1, y1, x2 - x1, y2 - y1);
}

pub fn rect_overlaps_vec2(r: Rect, p: Vec2i) -> bool {
    p.x >= r.x && p.x < r.x + r.w && p.y >= r.y && p.y < r.y + r.h
}

pub fn draw_frame(ctx: &mut Context, rect: Rect, colorid: ControlColor) {
    ctx.draw_rect(rect, ctx.style.colors[colorid as usize]);
    if colorid == ControlColor::ScrollBase || colorid == ControlColor::ScrollThumb || colorid == ControlColor::TitleBG {
        return;
    }
    if ctx.style.colors[ControlColor::Border as usize].a != 0 {
        ctx.draw_box(expand_rect(rect, 1), ctx.style.colors[ControlColor::Border as usize]);
    }
}

fn hash_u32(hash_0: &mut Id, orig_id: u32) {
    let bytes = orig_id.to_be_bytes();
    for b in bytes {
        let fresh1 = b;
        *hash_0 = Id((hash_0.0 ^ fresh1 as u32).wrapping_mul(16777619 as u32));
    }
}

fn hash_str(hash_0: &mut Id, s: &str) {
    for c in s.chars() {
        let fresh1 = c as i32;
        *hash_0 = Id((hash_0.0 ^ fresh1 as u32).wrapping_mul(16777619 as u32));
    }
}

fn hash_bytes(hash_0: &mut Id, s: &[u8]) {
    for c in s {
        let fresh1 = *c as i32;
        *hash_0 = Id((hash_0.0 ^ fresh1 as u32).wrapping_mul(16777619 as u32));
    }
}

impl Context {
    pub fn new() -> Self {
        let mut s = Self::default();
        s.draw_frame = Some(draw_frame as fn(&mut Context, Rect, ControlColor) -> ());
        s.style = Style::default();
        s
    }

    pub fn begin(&mut self) {
        assert!((self.char_width).is_some() && (self.font_height).is_some());
        self.root_list.clear();
        self.text_stack.clear();
        self.scroll_target = None;
        self.hover_root = self.next_hover_root;
        self.next_hover_root = None;
        self.mouse_delta.x = self.mouse_pos.x - self.last_mouse_pos.x;
        self.mouse_delta.y = self.mouse_pos.y - self.last_mouse_pos.y;
        self.command_list.clear();
        self.frame += 1;
    }

    pub fn end(&mut self) {
        assert_eq!(self.container_stack.len(), 0);
        assert_eq!(self.clip_stack.len(), 0);
        assert_eq!(self.id_stack.len(), 0);
        assert_eq!(self.layout_stack.len(), 0);
        if !self.scroll_target.is_none() {
            self.containers[self.scroll_target.unwrap()].scroll.x += self.scroll_delta.x;
            self.containers[self.scroll_target.unwrap()].scroll.y += self.scroll_delta.y;
        }
        if !self.updated_focus {
            self.focus = None;
        }
        self.updated_focus = false;
        if !self.mouse_pressed.is_none()
            && !self.next_hover_root.is_none()
            && self.containers[self.next_hover_root.unwrap()].zindex < self.last_zindex
            && self.containers[self.next_hover_root.unwrap()].zindex >= 0
        {
            self.bring_to_front(self.next_hover_root.unwrap());
        }
        self.key_pressed = KeyMode::NONE;
        self.input_text.clear();
        self.mouse_pressed = MouseButton::NONE;
        self.scroll_delta = vec2(0, 0);
        self.last_mouse_pos = self.mouse_pos;
        let n = self.root_list.len();
        quick_sort_by(self.root_list.as_slice_mut(), |a, b| {
            self.containers[*a].zindex.cmp(&self.containers[*b].zindex)
        });

        for i in 0..n {
            if i == 0 {
                // root container!
                // if this is the first container then make the first command jump to it.
                // otherwise set the previous container's tail to jump to this one

                let cmd = &mut self.command_list[0];
                assert!(match cmd {
                    Command::Jump { .. } => true,
                    _ => false,
                });
                let dst_idx = self.containers[self.root_list[i as usize]].head_idx.unwrap() + 1;
                *cmd = Command::Jump { dst_idx: Some(dst_idx) };
                assert!(dst_idx < self.command_list.len());
            } else {
                let prev = &self.containers[self.root_list[i - 1]];
                self.command_list[prev.tail_idx.unwrap()] = Command::Jump {
                    dst_idx: Some(self.containers[self.root_list[i as usize]].head_idx.unwrap() + 1),
                };
            }
            if i == n - 1 {
                assert!(self.containers[self.root_list[i as usize]].tail_idx.unwrap() < self.command_list.len());
                assert!(match self.command_list[self.containers[self.root_list[i as usize]].tail_idx.unwrap()] {
                    Command::Jump { .. } => true,
                    _ => false,
                });
                self.command_list[self.containers[self.root_list[i as usize]].tail_idx.unwrap()] = Command::Jump { dst_idx: Some(self.command_list.len()) };
                // the snake eats its tail
            }
        }
    }

    pub fn set_focus(&mut self, id: Option<Id>) {
        self.focus = id;
        self.updated_focus = true;
    }

    pub fn get_id_u32(&mut self, orig_id: u32) -> Id {
        let mut res: Id = match self.id_stack.top() {
            Some(id) => *id,
            None => Id(2166136261),
        };
        hash_u32(&mut res, orig_id);
        self.last_id = Some(res);
        return res;
    }

    pub fn get_id_from_ptr<T: ?Sized>(&mut self, orig_id: &T) -> Id {
        let mut res: Id = match self.id_stack.top() {
            Some(id) => *id,
            None => Id(2166136261),
        };
        let ptr = orig_id as *const T as *const u8 as usize;
        let bytes = ptr.to_le_bytes();
        hash_bytes(&mut res, &bytes);
        self.last_id = Some(res);
        return res;
    }

    pub fn get_id_from_str(&mut self, s: &str) -> Id {
        let mut res: Id = match self.id_stack.top() {
            Some(id) => *id,
            None => Id(2166136261),
        };
        hash_str(&mut res, s);
        self.last_id = Some(res);
        return res;
    }

    pub fn push_id_from_ptr<T>(&mut self, orig_id: &T) {
        let id = self.get_id_from_ptr(orig_id);
        self.id_stack.push(id);
    }

    pub fn push_id_from_str(&mut self, s: &str) {
        let id = self.get_id_from_str(s);
        self.id_stack.push(id);
    }

    pub fn pop_id(&mut self) {
        self.id_stack.pop();
    }

    pub fn push_clip_rect(&mut self, rect: Rect) {
        let last = self.get_clip_rect();
        self.clip_stack.push(intersect_rects(rect, last));
    }

    pub fn pop_clip_rect(&mut self) {
        self.clip_stack.pop();
    }

    pub fn get_clip_rect(&mut self) -> Rect {
        *self.clip_stack.top().unwrap()
    }

    pub fn check_clip(&mut self, r: Rect) -> Clip {
        let cr = self.get_clip_rect();
        if r.x > cr.x + cr.w || r.x + r.w < cr.x || r.y > cr.y + cr.h || r.y + r.h < cr.y {
            return Clip::All;
        }
        if r.x >= cr.x && r.x + r.w <= cr.x + cr.w && r.y >= cr.y && r.y + r.h <= cr.y + cr.h {
            return Clip::None;
        }
        return Clip::Part;
    }

    fn push_layout(&mut self, body: Rect, scroll: Vec2i) {
        let mut layout: Layout = Layout {
            body: Rect { x: 0, y: 0, w: 0, h: 0 },
            next: Rect { x: 0, y: 0, w: 0, h: 0 },
            position: Vec2i { x: 0, y: 0 },
            size: Vec2i { x: 0, y: 0 },
            max: Vec2i { x: 0, y: 0 },
            widths: [0; 16],
            items: 0,
            item_index: 0,
            next_row: 0,
            next_type: LayoutPosition::None,
            indent: 0,
        };
        layout.body = rect(body.x - scroll.x, body.y - scroll.y, body.w, body.h);
        layout.max = vec2(-0x1000000, -0x1000000);
        self.layout_stack.push(layout);
        self.layout_row(&[0], 0);
    }

    fn get_layout(&self) -> &Layout {
        return self.layout_stack.top().unwrap();
    }

    fn get_layout_mut(&mut self) -> &mut Layout {
        return self.layout_stack.top_mut().unwrap();
    }

    fn pop_container(&mut self) {
        let cnt = self.get_current_container();
        let layout = *self.get_layout();
        self.containers[cnt].content_size.x = layout.max.x - layout.body.x;
        self.containers[cnt].content_size.y = layout.max.y - layout.body.y;

        self.container_stack.pop();
        self.layout_stack.pop();
        self.pop_id();
    }

    pub fn get_current_container(&self) -> usize {
        *self.container_stack.top().unwrap()
    }

    pub fn get_current_container_rect(&self) -> Rect {
        self.containers[*self.container_stack.top().unwrap()].rect
    }

    pub fn set_current_container_rect(&mut self, rect: &Rect) {
        self.containers[*self.container_stack.top().unwrap()].rect = *rect;
    }

    pub fn get_current_container_scroll(&self) -> Vec2i {
        self.containers[*self.container_stack.top().unwrap()].scroll
    }

    pub fn set_current_container_scroll(&mut self, scroll: &Vec2i) {
        self.containers[*self.container_stack.top().unwrap()].scroll = *scroll;
    }

    pub fn get_current_container_content_size(&self) -> Vec2i {
        self.containers[*self.container_stack.top().unwrap()].content_size
    }

    pub fn get_current_container_body(&self) -> Rect {
        self.containers[*self.container_stack.top().unwrap()].body
    }

    fn get_container_index_intern(&mut self, id: Id, opt: WidgetOption) -> Option<usize> {
        let idx = self.container_pool.get(id);
        if idx.is_some() {
            if self.containers[idx.unwrap()].open || !opt.is_closed() {
                self.container_pool.update(idx.unwrap(), self.frame);
            }
            return idx;
        }
        if opt.is_closed() {
            return None;
        }
        let idx = self.container_pool.alloc(id, self.frame);
        self.containers[idx] = Container::default();
        self.containers[idx].head_idx = None;
        self.containers[idx].tail_idx = None;
        self.containers[idx].open = true;
        self.bring_to_front(idx);
        Some(idx)
    }

    fn get_container_index(&mut self, name: &str) -> Option<usize> {
        let id = self.get_id_from_str(name);
        self.get_container_index_intern(id, WidgetOption::NONE)
    }

    pub fn bring_to_front(&mut self, cnt: usize) {
        self.last_zindex += 1;
        self.containers[cnt].zindex = self.last_zindex;
    }

    pub fn input_mousemove(&mut self, x: i32, y: i32) {
        self.mouse_pos = vec2(x, y);
    }

    pub fn input_mousedown(&mut self, x: i32, y: i32, btn: MouseButton) {
        self.input_mousemove(x, y);
        self.mouse_down |= btn;
        self.mouse_pressed |= btn;
    }

    pub fn input_mouseup(&mut self, x: i32, y: i32, btn: MouseButton) {
        self.input_mousemove(x, y);
        self.mouse_down &= !btn;
    }

    pub fn input_scroll(&mut self, x: i32, y: i32) {
        self.scroll_delta.x += x;
        self.scroll_delta.y += y;
    }

    pub fn input_keydown(&mut self, key: KeyMode) {
        self.key_pressed |= key;
        self.key_down |= key;
    }

    pub fn input_keyup(&mut self, key: KeyMode) {
        self.key_down &= !key;
    }

    pub fn input_text(&mut self, text: &str) {
        self.input_text += text;
    }

    pub fn push_command(&mut self, cmd: Command) -> (&mut Command, usize) {
        self.command_list.push(cmd)
    }

    pub fn push_text(&mut self, str: &str) -> usize {
        let str_start = self.text_stack.len();
        for c in str.chars() {
            self.text_stack.push(c);
        }
        return str_start;
    }

    ///
    /// returns the next command to execute and the next index to use
    ///
    pub fn mu_next_command(&mut self, mut cmd_id: usize) -> Option<(Command, usize)> {
        if cmd_id >= self.command_list.len() {
            cmd_id = 0
        }

        while cmd_id != self.command_list.len() {
            match self.command_list[cmd_id] {
                Command::Jump { dst_idx } => cmd_id = dst_idx.unwrap(),
                _ => return Some((self.command_list[cmd_id], cmd_id + 1)),
            }
        }
        None
    }

    fn push_jump(&mut self, dst_idx: Option<usize>) -> usize {
        let (_, pos) = self.push_command(Command::Jump { dst_idx });
        pos
    }

    pub fn set_clip(&mut self, rect: Rect) {
        self.push_command(Command::Clip { rect });
    }

    pub fn draw_rect(&mut self, mut rect: Rect, color: Color) {
        rect = intersect_rects(rect, self.get_clip_rect());
        if rect.w > 0 && rect.h > 0 {
            self.push_command(Command::Rect { rect, color });
        }
    }

    pub fn draw_box(&mut self, r: Rect, color: Color) {
        self.draw_rect(rect(r.x + 1, r.y, r.w - 2, 1), color);
        self.draw_rect(rect(r.x + 1, r.y + r.h - 1, r.w - 2, 1), color);
        self.draw_rect(rect(r.x, r.y, 1, r.h), color);
        self.draw_rect(rect(r.x + r.w - 1, r.y, 1, r.h), color);
    }

    pub fn draw_text(&mut self, font: FontId, str: &str, pos: Vec2i, color: Color) {
        let rect: Rect = rect(pos.x, pos.y, self.get_text_width(font, str), self.get_text_height(font, str));
        let clipped = self.check_clip(rect);
        match clipped {
            Clip::All => return,
            Clip::Part => {
                let clip = self.get_clip_rect();
                self.set_clip(clip)
            }
            _ => (),
        }

        let str_start = self.push_text(str);
        self.push_command(Command::Text {
            str_start,
            str_len: str.len(),
            pos,
            color,
            font,
        });
        if clipped != Clip::None {
            self.set_clip(UNCLIPPED_RECT);
        }
    }

    pub fn draw_icon(&mut self, id: Icon, rect: Rect, color: Color) {
        let clipped = self.check_clip(rect);
        match clipped {
            Clip::All => return,
            Clip::Part => {
                let clip = self.get_clip_rect();
                self.set_clip(clip)
            }
            _ => (),
        }
        self.push_command(Command::Icon { id, rect, color });
        if clipped != Clip::None {
            self.set_clip(UNCLIPPED_RECT);
        }
    }

    pub fn layout_begin_column(&mut self) {
        let layout = self.layout_next();
        self.push_layout(layout, vec2(0, 0));
    }

    pub fn layout_end_column(&mut self) {
        let b = self.get_layout().clone();
        self.layout_stack.pop();

        let a = self.get_layout_mut();
        a.position.x = if a.position.x > b.position.x + b.body.x - a.body.x {
            a.position.x
        } else {
            b.position.x + b.body.x - a.body.x
        };
        a.next_row = if a.next_row > b.next_row + b.body.y - a.body.y {
            a.next_row
        } else {
            b.next_row + b.body.y - a.body.y
        };
        a.max.x = i32::max(a.max.x, b.max.x);
        a.max.y = i32::max(a.max.y, b.max.y);
    }

    pub fn layout_row_for_layout(layout: &mut Layout, widths: &[i32], height: i32) {
        layout.items = widths.len();
        assert!(widths.len() <= 16);
        for i in 0..widths.len() {
            layout.widths[i] = widths[i];
        }
        layout.position = vec2(layout.indent, layout.next_row);
        layout.size.y = height;
        layout.item_index = 0;
    }

    pub fn layout_row(&mut self, widths: &[i32], height: i32) {
        let layout = self.get_layout_mut();
        Self::layout_row_for_layout(layout, widths, height);
    }

    pub fn layout_width(&mut self, width: i32) {
        self.get_layout_mut().size.x = width;
    }

    pub fn layout_height(&mut self, height: i32) {
        self.get_layout_mut().size.y = height;
    }

    pub fn layout_set_next(&mut self, r: Rect, position: LayoutPosition) {
        let layout = self.get_layout_mut();
        layout.next = r;
        layout.next_type = position;
    }

    pub fn layout_next(&mut self) -> Rect {
        let style = self.style;
        let layout = self.get_layout_mut();
        let mut res: Rect = Rect { x: 0, y: 0, w: 0, h: 0 };
        if layout.next_type != LayoutPosition::None {
            let type_0 = layout.next_type;
            layout.next_type = LayoutPosition::None;
            res = layout.next;
            if type_0 == LayoutPosition::Absolute {
                self.last_rect = res;
                return self.last_rect;
            }
        } else {
            let litems = layout.items;
            let lsize_y = layout.size.y;
            let mut undefined_widths = [0; 16];
            undefined_widths[0..litems as usize].copy_from_slice(&layout.widths[0..litems as usize]);
            if layout.item_index == layout.items {
                Self::layout_row_for_layout(layout, &undefined_widths[0..litems as usize], lsize_y);
            }
            res.x = layout.position.x;
            res.y = layout.position.y;
            res.w = if layout.items > 0 {
                layout.widths[layout.item_index as usize]
            } else {
                layout.size.x
            };
            res.h = layout.size.y;
            if res.w == 0 {
                res.w = style.size.x + style.padding * 2;
            }
            if res.h == 0 {
                res.h = style.size.y + style.padding * 2;
            }
            if res.w < 0 {
                res.w += layout.body.w - res.x + 1;
            }
            if res.h < 0 {
                res.h += layout.body.h - res.y + 1;
            }
            layout.item_index += 1;
        }
        layout.position.x += res.w + style.spacing;
        layout.next_row = if layout.next_row > res.y + res.h + style.spacing {
            layout.next_row
        } else {
            res.y + res.h + style.spacing
        };
        res.x += layout.body.x;
        res.y += layout.body.y;
        layout.max.x = if layout.max.x > res.x + res.w { layout.max.x } else { res.x + res.w };
        layout.max.y = if layout.max.y > res.y + res.h { layout.max.y } else { res.y + res.h };
        self.last_rect = res;
        return self.last_rect;
    }

    fn in_hover_root(&mut self) -> bool {
        match self.hover_root {
            Some(hover_root) => {
                let len = self.container_stack.len();
                for i in 0..len {
                    if self.container_stack[len - i - 1] == hover_root {
                        return true;
                    }
                    if self.containers[self.container_stack[len - i - 1]].head_idx.is_some() {
                        break;
                    }
                }
                false
            }
            None => false,
        }
    }

    pub fn draw_control_frame(&mut self, id: Id, rect: Rect, mut colorid: ControlColor, opt: WidgetOption) {
        if opt.has_no_frame() {
            return;
        }

        if self.focus == Some(id) {
            colorid.focus()
        } else if self.hover == Some(id) {
            colorid.hover()
        }
        (self.draw_frame).expect("non-null function pointer")(self, rect, colorid);
    }

    pub fn draw_control_text(&mut self, str: &str, rect: Rect, colorid: ControlColor, opt: WidgetOption) {
        let mut pos: Vec2i = Vec2i { x: 0, y: 0 };
        let font = self.style.font;
        let tw = self.get_text_width(font, str);
        self.push_clip_rect(rect);
        pos.y = rect.y + (rect.h - self.get_text_height(font, str)) / 2;
        if opt.is_aligned_center() {
            pos.x = rect.x + (rect.w - tw) / 2;
        } else if opt.is_aligned_right() {
            pos.x = rect.x + rect.w - tw - self.style.padding;
        } else {
            pos.x = rect.x + self.style.padding;
        }
        self.draw_text(font, str, pos, self.style.colors[colorid as usize]);
        self.pop_clip_rect();
    }

    pub fn mouse_over(&mut self, rect: Rect) -> bool {
        rect_overlaps_vec2(rect, self.mouse_pos) && rect_overlaps_vec2(self.get_clip_rect(), self.mouse_pos) && self.in_hover_root()
    }

    pub fn update_control(&mut self, id: Id, rect: Rect, opt: WidgetOption) {
        let mouseover = self.mouse_over(rect);
        if self.focus == Some(id) {
            self.updated_focus = true;
        }
        if opt.is_not_interactive() {
            return;
        }
        if mouseover && self.mouse_down.is_none() {
            self.hover = Some(id);
        }
        if self.focus == Some(id) {
            if !self.mouse_pressed.is_none() && !mouseover {
                self.set_focus(None);
            }
            if self.mouse_down.is_none() && !opt.is_holding_focus() {
                self.set_focus(None);
            }
        }
        if self.hover == Some(id) {
            if !self.mouse_pressed.is_none() {
                self.set_focus(Some(id));
            } else if !mouseover {
                self.hover = None;
            }
        }
    }

    pub fn get_text_width(&self, font: FontId, text: &str) -> i32 {
        let mut res = 0;
        let mut acc = 0;
        for c in text.chars() {
            if c == '\n' {
                res = usize::max(res, acc);
                acc = 0;
            }
            acc += self.char_width.expect("non-null function pointer")(font, c);
        }
        res = usize::max(res, acc);
        res as i32
    }

    pub fn get_text_height(&self, font: FontId, text: &str) -> i32 {
        let font_height = self.font_height.expect("non-null function pointer")(font);
        let lc = text.lines().count();
        (lc * font_height) as i32
    }

    pub fn text(&mut self, text: &str) {
        let font = self.style.font;
        let color = self.style.colors[ControlColor::Text as usize];
        self.layout_begin_column();
        let h = self.font_height.expect("non-null function pointer")(font) as i32;
        self.layout_row(&[-1], h);
        let mut r = self.layout_next();
        for line in text.lines() {
            let mut rx = r.x;
            let words = line.split_inclusive(' ');
            for w in words {
                // TODO: split w when its width > w into many lines
                let tw = self.get_text_width(font, w);
                if tw + rx < r.x + r.w {
                    self.draw_text(font, w, vec2(rx, r.y), color);
                    rx += tw;
                } else {
                    r = self.layout_next();
                    rx = r.x;
                }
            }
            r = self.layout_next();
        }
        self.layout_end_column();
    }

    pub fn label(&mut self, text: &str) {
        let layout = self.layout_next();
        self.draw_control_text(text, layout, ControlColor::Text, WidgetOption::NONE);
    }

    pub fn button_ex(&mut self, label: &str, icon: Icon, opt: WidgetOption) -> ResourceState {
        let mut res = ResourceState::NONE;
        let id: Id = if label.len() > 0 {
            self.get_id_from_str(label)
        } else {
            self.get_id_u32(icon as u32)
        };
        let r: Rect = self.layout_next();
        self.update_control(id, r, opt);
        if self.mouse_pressed.is_left() && self.focus == Some(id) {
            res |= ResourceState::SUBMIT;
        }
        self.draw_control_frame(id, r, ControlColor::Button, opt);
        if label.len() > 0 {
            self.draw_control_text(label, r, ControlColor::Text, opt);
        }
        if icon != Icon::None {
            self.draw_icon(icon, r, self.style.colors[ControlColor::Text as usize]);
        }
        return res;
    }

    pub fn checkbox(&mut self, label: &str, state: &mut bool) -> ResourceState {
        let mut res = ResourceState::NONE;
        let id: Id = self.get_id_from_ptr(state);
        let mut r: Rect = self.layout_next();
        let box_0: Rect = rect(r.x, r.y, r.h, r.h);
        self.update_control(id, r, WidgetOption::NONE);
        if self.mouse_pressed.is_left() && self.focus == Some(id) {
            res |= ResourceState::CHANGE;
            *state = *state == false;
        }
        self.draw_control_frame(id, box_0, ControlColor::Base, WidgetOption::NONE);
        if *state {
            self.draw_icon(Icon::Check, box_0, self.style.colors[ControlColor::Text as usize]);
        }
        r = rect(r.x + box_0.w, r.y, r.w - box_0.w, r.h);
        self.draw_control_text(label, r, ControlColor::Text, WidgetOption::NONE);
        return res;
    }

    pub fn textbox_raw(&mut self, buf: &mut dyn IString, id: Id, r: Rect, opt: WidgetOption) -> ResourceState {
        let mut res = ResourceState::NONE;
        self.update_control(id, r, opt | WidgetOption::HOLD_FOCUS);
        if self.focus == Some(id) {
            let mut len = buf.len();

            if self.input_text.len() > 0 && self.input_text.len() + len < buf.capacity() {
                buf.add_str(self.input_text.as_str());
                len += self.input_text.len() as usize;
                res |= ResourceState::CHANGE
            }

            if self.key_pressed.is_backspace() && len > 0 {
                // skip utf-8 continuation bytes
                buf.pop();
                res |= ResourceState::CHANGE
            }
            if self.key_pressed.is_return() {
                self.set_focus(None);
                res |= ResourceState::SUBMIT;
            }
        }
        self.draw_control_frame(id, r, ControlColor::Base, opt);
        if self.focus == Some(id) {
            let color = self.style.colors[ControlColor::Text as usize];
            let font = self.style.font;
            let textw = self.get_text_width(font, buf.as_str());
            let texth = self.get_text_height(font, buf.as_str());
            let ofx = r.w - self.style.padding - textw - 1;
            let textx = r.x + (if ofx < self.style.padding { ofx } else { self.style.padding });
            let texty = r.y + (r.h - texth) / 2;
            self.push_clip_rect(r);
            self.draw_text(font, buf.as_str(), vec2(textx, texty), color);
            self.draw_rect(rect(textx + textw, texty, 1, texth), color);
            self.pop_clip_rect();
        } else {
            self.draw_control_text(buf.as_str(), r, ControlColor::Text, opt);
        }
        return res;
    }

    fn number_textbox(&mut self, value: &mut Real, r: Rect, id: Id) -> ResourceState {
        if self.mouse_pressed.is_left() && self.key_down.is_shift() && self.hover == Some(id) {
            self.number_edit = Some(id);
            self.number_edit_buf.clear();
            self.number_edit_buf.append_real("%.3f", *value);
        }

        if self.number_edit == Some(id) {
            let mut temp = self.number_edit_buf.clone();
            let res: ResourceState = self.textbox_raw(&mut temp, id, r, WidgetOption::NONE);
            self.number_edit_buf = temp;
            if res.is_submitted() || self.focus != Some(id) {
                let mut ascii = [0u8; 32];
                let mut i = 0;
                for c in self.number_edit_buf.as_str().chars() {
                    ascii[i] = c as u8;
                    i += 1;
                }
                ascii[i] = '\0' as u8;
                let v = unsafe { libc::strtod(ascii.as_ptr() as *const libc::c_char, ptr::null_mut() as *mut *mut libc::c_char) };
                *value = v as Real;
                self.number_edit = None;
            } else {
                return ResourceState::ACTIVE;
            }
        }
        return ResourceState::NONE;
    }

    pub fn textbox_ex(&mut self, buf: &mut dyn IString, opt: WidgetOption) -> ResourceState {
        let id: Id = self.get_id_from_ptr(buf);
        let r: Rect = self.layout_next();
        return self.textbox_raw(buf, id, r, opt);
    }

    pub fn slider_ex(&mut self, value: &mut Real, low: Real, high: Real, step: Real, fmt: &str, opt: WidgetOption) -> ResourceState {
        let mut res = ResourceState::NONE;
        let last = *value;
        let mut v = last;
        let id = self.get_id_from_ptr(value);
        let base = self.layout_next();
        if !self.number_textbox(&mut v, base, id).is_none() {
            return res;
        }
        self.update_control(id, base, opt);
        if self.focus == Some(id) && (!self.mouse_down.is_none() | self.mouse_pressed.is_left()) {
            v = low + (self.mouse_pos.x - base.x) as Real * (high - low) / base.w as Real;
            if step != 0. {
                v = (v + step / 2 as Real) / step * step;
            }
        }
        v = if high < (if low > v { low } else { v }) {
            high
        } else if low > v {
            low
        } else {
            v
        };
        *value = v;
        if last != v {
            res |= ResourceState::CHANGE;
        }
        self.draw_control_frame(id, base, ControlColor::Base, opt);
        let w = self.style.thumb_size;
        let x = ((v - low) * (base.w - w) as Real / (high - low)) as i32;
        let thumb = rect(base.x + x, base.y, w, base.h);
        self.draw_control_frame(id, thumb, ControlColor::Button, opt);
        let mut buff = FixedString::<64>::new();
        buff.append_real(fmt, *value);
        self.draw_control_text(buff.as_str(), base, ControlColor::Text, opt);
        return res;
    }

    pub fn number_ex(&mut self, value: &mut Real, step: Real, fmt: &str, opt: WidgetOption) -> ResourceState {
        let mut res = ResourceState::NONE;
        let id: Id = self.get_id_from_ptr(value);
        let base: Rect = self.layout_next();
        let last: Real = *value;
        if !self.number_textbox(value, base, id).is_none() {
            return res;
        }
        self.update_control(id, base, opt);
        if self.focus == Some(id) && self.mouse_down.is_left() {
            *value += self.mouse_delta.x as Real * step;
        }
        if *value != last {
            res |= ResourceState::CHANGE;
        }
        self.draw_control_frame(id, base, ControlColor::Base, opt);
        let mut buff = FixedString::<64>::new();
        buff.append_real(fmt, *value);
        self.draw_control_text(buff.as_str(), base, ControlColor::Text, opt);
        return res;
    }

    fn header(&mut self, label: &str, is_treenode: bool, opt: WidgetOption) -> ResourceState {
        let id: Id = self.get_id_from_str(label);
        let idx = self.treenode_pool.get(id);
        self.layout_row(&[-1], 0);
        let mut active = idx.is_some() as i32;
        let expanded = if opt.is_expanded() { (active == 0) as i32 } else { active };
        let mut r = self.layout_next();
        self.update_control(id, r, WidgetOption::NONE);
        active ^= (self.mouse_pressed.is_left() && self.focus == Some(id)) as i32;
        if idx.is_some() {
            if active != 0 {
                self.treenode_pool.update(idx.unwrap(), self.frame);
            } else {
                self.treenode_pool.reset(idx.unwrap());
            }
        } else if active != 0 {
            self.treenode_pool.alloc(id, self.frame);
        }

        if is_treenode {
            if self.hover == Some(id) {
                (self.draw_frame).expect("non-null function pointer")(self, r, ControlColor::ButtonHover);
            }
        } else {
            self.draw_control_frame(id, r, ControlColor::Button, WidgetOption::NONE);
        }
        self.draw_icon(
            if expanded != 0 { Icon::Expanded } else { Icon::Collapsed },
            rect(r.x, r.y, r.h, r.h),
            self.style.colors[ControlColor::Text as usize],
        );
        r.x += r.h - self.style.padding;
        r.w -= r.h - self.style.padding;
        self.draw_control_text(label, r, ControlColor::Text, WidgetOption::NONE);
        return if expanded != 0 { ResourceState::ACTIVE } else { ResourceState::NONE };
    }

    pub fn header_ex(&mut self, label: &str, opt: WidgetOption) -> ResourceState {
        return self.header(label, false, opt);
    }

    pub fn begin_treenode_ex(&mut self, label: &str, opt: WidgetOption) -> ResourceState {
        let res = self.header(label, true, opt);
        if res.is_active() && self.last_id.is_some() {
            self.get_layout_mut().indent += self.style.indent;
            self.id_stack.push(self.last_id.unwrap());
        }
        return res;
    }

    pub fn end_treenode(&mut self) {
        self.get_layout_mut().indent -= self.style.indent;
        self.pop_id();
    }

    fn clamp(x: i32, a: i32, b: i32) -> i32 {
        i32::min(b, i32::max(a, x))
    }

    fn scrollbars(&mut self, cnt_id: usize, body: &mut Rect) {
        let sz = self.style.scrollbar_size;
        let mut cs: Vec2i = self.containers[cnt_id].content_size;
        cs.x += self.style.padding * 2;
        cs.y += self.style.padding * 2;
        self.push_clip_rect(body.clone());
        if cs.y > self.containers[cnt_id].body.h {
            body.w -= sz;
        }
        if cs.x > self.containers[cnt_id].body.w {
            body.h -= sz;
        }
        let body = *body;
        let maxscroll = cs.y - body.h;
        if maxscroll > 0 && body.h > 0 {
            let id: Id = self.get_id_from_str("!scrollbary");
            let mut base = body;
            base.x = body.x + body.w;
            base.w = self.style.scrollbar_size;
            self.update_control(id, base, WidgetOption::NONE);
            if self.focus == Some(id) && self.mouse_down.is_left() {
                self.containers[cnt_id].scroll.y += self.mouse_delta.y * cs.y / base.h;
            }
            self.containers[cnt_id].scroll.y = Self::clamp(self.containers[cnt_id].scroll.y, 0, maxscroll);

            (self.draw_frame).expect("non-null function pointer")(self, base, ControlColor::ScrollBase);
            let mut thumb = base;
            thumb.h = if self.style.thumb_size > base.h * body.h / cs.y {
                self.style.thumb_size
            } else {
                base.h * body.h / cs.y
            };
            thumb.y += self.containers[cnt_id].scroll.y * (base.h - thumb.h) / maxscroll;
            (self.draw_frame).expect("non-null function pointer")(self, thumb, ControlColor::ScrollThumb);
            if self.mouse_over(body) {
                self.scroll_target = Some(cnt_id);
            }
        } else {
            self.containers[cnt_id].scroll.y = 0;
        }
        let maxscroll_0 = cs.x - body.w;
        if maxscroll_0 > 0 && body.w > 0 {
            let id_0: Id = self.get_id_from_str("!scrollbarx");
            let mut base_0 = body;
            base_0.y = body.y + body.h;
            base_0.h = self.style.scrollbar_size;
            self.update_control(id_0, base_0, WidgetOption::NONE);
            if self.focus == Some(id_0) && self.mouse_down.is_left() {
                self.containers[cnt_id].scroll.x += self.mouse_delta.x * cs.x / base_0.w;
            }
            self.containers[cnt_id].scroll.x = Self::clamp(self.containers[cnt_id].scroll.x, 0, maxscroll_0);

            (self.draw_frame).expect("non-null function pointer")(self, base_0, ControlColor::ScrollBase);
            let mut thumb_0 = base_0;
            thumb_0.w = if self.style.thumb_size > base_0.w * body.w / cs.x {
                self.style.thumb_size
            } else {
                base_0.w * body.w / cs.x
            };
            thumb_0.x += self.containers[cnt_id].scroll.x * (base_0.w - thumb_0.w) / maxscroll_0;
            (self.draw_frame).expect("non-null function pointer")(self, thumb_0, ControlColor::ScrollThumb);
            if self.mouse_over(body) {
                self.scroll_target = Some(cnt_id);
            }
        } else {
            self.containers[cnt_id].scroll.x = 0;
        }
        self.pop_clip_rect();
    }

    fn push_container_body(&mut self, cnt_idx: usize, body: Rect, opt: WidgetOption) {
        let mut body = body;
        if !opt.has_no_scroll() {
            self.scrollbars(cnt_idx, &mut body);
        }
        self.push_layout(expand_rect(body, -self.style.padding), self.containers[cnt_idx].scroll);
        self.containers[cnt_idx].body = body;
    }

    fn begin_root_container(&mut self, cnt: usize) {
        self.container_stack.push(cnt);

        self.root_list.push(cnt);
        self.containers[cnt].head_idx = Some(self.push_jump(None));
        if rect_overlaps_vec2(self.containers[cnt].rect, self.mouse_pos)
            && (self.next_hover_root.is_none() || self.containers[cnt].zindex > self.containers[self.next_hover_root.unwrap()].zindex)
        {
            self.next_hover_root = Some(cnt);
        }
        self.clip_stack.push(UNCLIPPED_RECT);
    }

    fn end_root_container(&mut self) {
        let cnt = self.get_current_container();
        self.containers[cnt].tail_idx = Some(self.push_jump(None));
        self.command_list[self.containers[cnt].head_idx.unwrap()] = Command::Jump { dst_idx: Some(self.command_list.len()) };
        self.pop_clip_rect();
        self.pop_container();
    }

    pub fn begin_window_ex(&mut self, title: &str, mut r: Rect, opt: WidgetOption) -> ResourceState {
        let id = self.get_id_from_str(title);
        let cnt_id = self.get_container_index_intern(id, opt);
        if cnt_id.is_none() || !self.containers[cnt_id.unwrap()].open {
            return ResourceState::NONE;
        }
        self.id_stack.push(id);

        if self.containers[cnt_id.unwrap()].rect.w == 0 {
            self.containers[cnt_id.unwrap()].rect = r;
        }
        self.begin_root_container(cnt_id.unwrap());
        let mut body = self.containers[cnt_id.unwrap()].rect;
        r = body;
        if !opt.has_no_frame() {
            (self.draw_frame).expect("non-null function pointer")(self, r, ControlColor::WindowBG);
        }
        if !opt.has_no_title() {
            let mut tr: Rect = r;
            tr.h = self.style.title_height;
            (self.draw_frame).expect("non-null function pointer")(self, tr, ControlColor::TitleBG);

            // TODO: Is this necessary?
            if !opt.has_no_title() {
                let id = self.get_id_from_str("!title");
                self.update_control(id, tr, opt);
                self.draw_control_text(title, tr, ControlColor::TitleText, opt);
                if Some(id) == self.focus && self.mouse_down.is_left() {
                    self.containers[cnt_id.unwrap()].rect.x += self.mouse_delta.x;
                    self.containers[cnt_id.unwrap()].rect.y += self.mouse_delta.y;
                }
                body.y += tr.h;
                body.h -= tr.h;
            }
            if !opt.has_no_close() {
                let id = self.get_id_from_str("!close");
                let r: Rect = rect(tr.x + tr.w - tr.h, tr.y, tr.h, tr.h);
                tr.w -= r.w;
                self.draw_icon(Icon::Close, r, self.style.colors[ControlColor::TitleText as usize]);
                self.update_control(id, r, opt);
                if self.mouse_pressed.is_left() && Some(id) == self.focus {
                    self.containers[cnt_id.unwrap()].open = false;
                }
            }
        }
        self.push_container_body(cnt_id.unwrap(), body, opt);
        if !opt.is_auto_sizing() {
            let sz = self.style.title_height;
            let id_2 = self.get_id_from_str("!resize");
            let r_0 = rect(r.x + r.w - sz, r.y + r.h - sz, sz, sz);
            self.update_control(id_2, r_0, opt);
            if Some(id_2) == self.focus && self.mouse_down.is_left() {
                self.containers[cnt_id.unwrap()].rect.w = if 96 > self.containers[cnt_id.unwrap()].rect.w + self.mouse_delta.x {
                    96
                } else {
                    self.containers[cnt_id.unwrap()].rect.w + self.mouse_delta.x
                };
                self.containers[cnt_id.unwrap()].rect.h = if 64 > self.containers[cnt_id.unwrap()].rect.h + self.mouse_delta.y {
                    64
                } else {
                    self.containers[cnt_id.unwrap()].rect.h + self.mouse_delta.y
                };
            }
        }
        if opt.is_auto_sizing() {
            let r_1 = self.get_layout().body;
            self.containers[cnt_id.unwrap()].rect.w = self.containers[cnt_id.unwrap()].content_size.x + (self.containers[cnt_id.unwrap()].rect.w - r_1.w);
            self.containers[cnt_id.unwrap()].rect.h = self.containers[cnt_id.unwrap()].content_size.y + (self.containers[cnt_id.unwrap()].rect.h - r_1.h);
        }

        if opt.is_popup() && !self.mouse_pressed.is_none() && self.hover_root != cnt_id {
            self.containers[cnt_id.unwrap()].open = false;
        }
        self.push_clip_rect(self.containers[cnt_id.unwrap()].body);
        return ResourceState::ACTIVE;
    }

    pub fn end_window(&mut self) {
        self.pop_clip_rect();
        self.end_root_container();
    }

    pub fn open_popup(&mut self, name: &str) {
        let cnt = self.get_container_index(name);
        self.next_hover_root = cnt;
        self.hover_root = self.next_hover_root;
        self.containers[cnt.unwrap()].rect = rect(self.mouse_pos.x, self.mouse_pos.y, 1, 1);
        self.containers[cnt.unwrap()].open = true;
        self.bring_to_front(cnt.unwrap());
    }

    pub fn begin_popup(&mut self, name: &str) -> ResourceState {
        let opt =
            WidgetOption::POPUP | WidgetOption::AUTO_SIZE | WidgetOption::NO_RESIZE | WidgetOption::NO_SCROLL | WidgetOption::NO_TITLE | WidgetOption::CLOSED;
        return self.begin_window_ex(name, rect(0, 0, 0, 0), opt);
    }

    pub fn end_popup(&mut self) {
        self.end_window();
    }

    pub fn begin_panel_ex(&mut self, name: &str, opt: WidgetOption) {
        self.push_id_from_str(name);
        let cnt_id = self.get_container_index_intern(self.last_id.unwrap(), opt);
        let rect = self.layout_next();
        self.containers[cnt_id.unwrap()].rect = rect;
        if !opt.has_no_frame() {
            (self.draw_frame).expect("non-null function pointer")(self, rect, ControlColor::PanelBG);
        }

        self.container_stack.push(cnt_id.unwrap());
        self.push_container_body(cnt_id.unwrap(), rect, opt);
        self.push_clip_rect(self.containers[cnt_id.unwrap()].body);
    }

    pub fn end_panel(&mut self) {
        self.pop_clip_rect();
        self.pop_container();
    }
}
