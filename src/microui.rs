use ::libc;
pub type _IO_wide_data = libc::c_int;
pub type _IO_codecvt = libc::c_int;
pub type _IO_marker = libc::c_int;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;

pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;

#[derive(PartialEq)]
pub enum Clip {
    None = 0,
    All = 2,
    Part = 1,
}

pub type C2RustUnnamed_0 = libc::c_uint;
pub const MU_COMMAND_MAX: C2RustUnnamed_0 = 6;
pub const MU_COMMAND_ICON: C2RustUnnamed_0 = 5;
pub const MU_COMMAND_TEXT: C2RustUnnamed_0 = 4;
pub const MU_COMMAND_RECT: C2RustUnnamed_0 = 3;
pub const MU_COMMAND_CLIP: C2RustUnnamed_0 = 2;
pub const MU_COMMAND_JUMP: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MU_COLOR_MAX: C2RustUnnamed_1 = 14;
pub const MU_COLOR_SCROLLTHUMB: C2RustUnnamed_1 = 13;
pub const MU_COLOR_SCROLLBASE: C2RustUnnamed_1 = 12;
pub const MU_COLOR_BASEFOCUS: C2RustUnnamed_1 = 11;
pub const MU_COLOR_BASEHOVER: C2RustUnnamed_1 = 10;
pub const MU_COLOR_BASE: C2RustUnnamed_1 = 9;
pub const MU_COLOR_BUTTONFOCUS: C2RustUnnamed_1 = 8;
pub const MU_COLOR_BUTTONHOVER: C2RustUnnamed_1 = 7;
pub const MU_COLOR_BUTTON: C2RustUnnamed_1 = 6;
pub const MU_COLOR_PANELBG: C2RustUnnamed_1 = 5;
pub const MU_COLOR_TITLETEXT: C2RustUnnamed_1 = 4;
pub const MU_COLOR_TITLEBG: C2RustUnnamed_1 = 3;
pub const MU_COLOR_WINDOWBG: C2RustUnnamed_1 = 2;
pub const MU_COLOR_BORDER: C2RustUnnamed_1 = 1;
pub const MU_COLOR_TEXT: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MU_ICON_MAX: C2RustUnnamed_2 = 5;
pub const MU_ICON_EXPANDED: C2RustUnnamed_2 = 4;
pub const MU_ICON_COLLAPSED: C2RustUnnamed_2 = 3;
pub const MU_ICON_CHECK: C2RustUnnamed_2 = 2;
pub const MU_ICON_CLOSE: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const MU_RES_CHANGE: C2RustUnnamed_3 = 4;
pub const MU_RES_SUBMIT: C2RustUnnamed_3 = 2;
pub const MU_RES_ACTIVE: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const MU_OPT_EXPANDED: C2RustUnnamed_4 = 4096;
pub const MU_OPT_CLOSED: C2RustUnnamed_4 = 2048;
pub const MU_OPT_POPUP: C2RustUnnamed_4 = 1024;
pub const MU_OPT_AUTOSIZE: C2RustUnnamed_4 = 512;
pub const MU_OPT_HOLDFOCUS: C2RustUnnamed_4 = 256;
pub const MU_OPT_NOTITLE: C2RustUnnamed_4 = 128;
pub const MU_OPT_NOCLOSE: C2RustUnnamed_4 = 64;
pub const MU_OPT_NOSCROLL: C2RustUnnamed_4 = 32;
pub const MU_OPT_NORESIZE: C2RustUnnamed_4 = 16;
pub const MU_OPT_NOFRAME: C2RustUnnamed_4 = 8;
pub const MU_OPT_NOINTERACT: C2RustUnnamed_4 = 4;
pub const MU_OPT_ALIGNRIGHT: C2RustUnnamed_4 = 2;
pub const MU_OPT_ALIGNCENTER: C2RustUnnamed_4 = 1;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const MU_MOUSE_MIDDLE: C2RustUnnamed_5 = 4;
pub const MU_MOUSE_RIGHT: C2RustUnnamed_5 = 2;
pub const MU_MOUSE_LEFT: C2RustUnnamed_5 = 1;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const MU_KEY_RETURN: C2RustUnnamed_6 = 16;
pub const MU_KEY_BACKSPACE: C2RustUnnamed_6 = 8;
pub const MU_KEY_ALT: C2RustUnnamed_6 = 4;
pub const MU_KEY_CTRL: C2RustUnnamed_6 = 2;
pub const MU_KEY_SHIFT: C2RustUnnamed_6 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_Context {
    pub text_width: Option::<
        unsafe extern "C" fn(mu_Font, *const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub text_height: Option::<unsafe extern "C" fn(mu_Font) -> libc::c_int>,
    pub draw_frame: Option::<
        unsafe extern "C" fn(*mut mu_Context, mu_Rect, libc::c_int) -> (),
    >,
    pub _style: mu_Style,
    pub style: *mut mu_Style,
    pub hover: mu_Id,
    pub focus: mu_Id,
    pub last_id: mu_Id,
    pub last_rect: mu_Rect,
    pub last_zindex: libc::c_int,
    pub updated_focus: libc::c_int,
    pub frame: libc::c_int,
    pub hover_root: *mut mu_Container,
    pub next_hover_root: *mut mu_Container,
    pub scroll_target: *mut mu_Container,
    pub number_edit_buf: [libc::c_char; 127],
    pub number_edit: mu_Id,
    pub command_list: C2RustUnnamed_13,
    pub root_list: C2RustUnnamed_12,
    pub container_stack: C2RustUnnamed_11,
    pub clip_stack: C2RustUnnamed_10,
    pub id_stack: C2RustUnnamed_9,
    pub layout_stack: C2RustUnnamed_8,
    pub text_stack: C2RustUnnamed_7,
    pub container_pool: [mu_PoolItem; 48],
    pub containers: [mu_Container; 48],
    pub treenode_pool: [mu_PoolItem; 48],
    pub mouse_pos: mu_Vec2,
    pub last_mouse_pos: mu_Vec2,
    pub mouse_delta: mu_Vec2,
    pub scroll_delta: mu_Vec2,
    pub mouse_down: libc::c_int,
    pub mouse_pressed: libc::c_int,
    pub key_down: libc::c_int,
    pub key_pressed: libc::c_int,
    pub input_text: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_Vec2 {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_PoolItem {
    pub id: mu_Id,
    pub last_update: libc::c_int,
}
pub type mu_Id = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_Container {
    pub head_idx: libc::c_int,
    pub tail_idx: libc::c_int,
    pub rect: mu_Rect,
    pub body: mu_Rect,
    pub content_size: mu_Vec2,
    pub scroll: mu_Vec2,
    pub zindex: libc::c_int,
    pub open: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_Rect {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub idx: libc::c_int,
    pub items: [libc::c_char; 65536],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub idx: libc::c_int,
    pub items: [mu_Layout; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_Layout {
    pub body: mu_Rect,
    pub next: mu_Rect,
    pub position: mu_Vec2,
    pub size: mu_Vec2,
    pub max: mu_Vec2,
    pub widths: [libc::c_int; 16],
    pub items: libc::c_int,
    pub item_index: libc::c_int,
    pub next_row: libc::c_int,
    pub next_type: libc::c_int,
    pub indent: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub idx: libc::c_int,
    pub items: [mu_Id; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub idx: libc::c_int,
    pub items: [mu_Rect; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub idx: libc::c_int,
    pub items: [*mut mu_Container; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub idx: libc::c_int,
    pub items: [*mut mu_Container; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub idx: libc::c_int,
    pub items: [mu_Command; 4096],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union mu_Command {
    pub type_0: libc::c_int,
    pub base: mu_BaseCommand,
    pub jump: mu_JumpCommand,
    pub clip: mu_ClipCommand,
    pub rect: mu_RectCommand,
    pub text: mu_TextCommand,
    pub icon: mu_IconCommand,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_IconCommand {
    pub base: mu_BaseCommand,
    pub rect: mu_Rect,
    pub id: libc::c_int,
    pub color: mu_Color,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_Color {
    pub r: libc::c_uchar,
    pub g: libc::c_uchar,
    pub b: libc::c_uchar,
    pub a: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_BaseCommand {
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_TextCommand {
    pub base: mu_BaseCommand,
    pub font: mu_Font,
    pub pos: mu_Vec2,
    pub color: mu_Color,
    pub str_0: *mut libc::c_char,
}
pub type mu_Font = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_RectCommand {
    pub base: mu_BaseCommand,
    pub rect: mu_Rect,
    pub color: mu_Color,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_ClipCommand {
    pub base: mu_BaseCommand,
    pub rect: mu_Rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_JumpCommand {
    pub base: mu_BaseCommand,
    pub dst_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mu_Style {
    pub font: mu_Font,
    pub size: mu_Vec2,
    pub padding: libc::c_int,
    pub spacing: libc::c_int,
    pub indent: libc::c_int,
    pub title_height: libc::c_int,
    pub scrollbar_size: libc::c_int,
    pub thumb_size: libc::c_int,
    pub colors: [mu_Color; 14],
}
pub type mu_Real = libc::c_float;
pub const ABSOLUTE: C2RustUnnamed_14 = 2;
pub const RELATIVE: C2RustUnnamed_14 = 1;
pub type C2RustUnnamed_14 = libc::c_uint;
static mut unclipped_rect: mu_Rect = {
    let mut init = mu_Rect {
        x: 0 as libc::c_int,
        y: 0 as libc::c_int,
        w: 0x1000000 as libc::c_int,
        h: 0x1000000 as libc::c_int,
    };
    init
};
static mut default_style: mu_Style = {
    let mut init = mu_Style {
        font: 0 as *const libc::c_void as *mut libc::c_void,
        size: {
            let mut init = mu_Vec2 {
                x: 68 as libc::c_int,
                y: 10 as libc::c_int,
            };
            init
        },
        padding: 5 as libc::c_int,
        spacing: 4 as libc::c_int,
        indent: 24 as libc::c_int,
        title_height: 24 as libc::c_int,
        scrollbar_size: 12 as libc::c_int,
        thumb_size: 8 as libc::c_int,
        colors: [
            {
                let mut init = mu_Color {
                    r: 230 as libc::c_int as libc::c_uchar,
                    g: 230 as libc::c_int as libc::c_uchar,
                    b: 230 as libc::c_int as libc::c_uchar,
                    a: 255 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = mu_Color {
                    r: 25 as libc::c_int as libc::c_uchar,
                    g: 25 as libc::c_int as libc::c_uchar,
                    b: 25 as libc::c_int as libc::c_uchar,
                    a: 255 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = mu_Color {
                    r: 50 as libc::c_int as libc::c_uchar,
                    g: 50 as libc::c_int as libc::c_uchar,
                    b: 50 as libc::c_int as libc::c_uchar,
                    a: 255 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = mu_Color {
                    r: 25 as libc::c_int as libc::c_uchar,
                    g: 25 as libc::c_int as libc::c_uchar,
                    b: 25 as libc::c_int as libc::c_uchar,
                    a: 255 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = mu_Color {
                    r: 240 as libc::c_int as libc::c_uchar,
                    g: 240 as libc::c_int as libc::c_uchar,
                    b: 240 as libc::c_int as libc::c_uchar,
                    a: 255 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = mu_Color {
                    r: 0 as libc::c_int as libc::c_uchar,
                    g: 0 as libc::c_int as libc::c_uchar,
                    b: 0 as libc::c_int as libc::c_uchar,
                    a: 0 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = mu_Color {
                    r: 75 as libc::c_int as libc::c_uchar,
                    g: 75 as libc::c_int as libc::c_uchar,
                    b: 75 as libc::c_int as libc::c_uchar,
                    a: 255 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = mu_Color {
                    r: 95 as libc::c_int as libc::c_uchar,
                    g: 95 as libc::c_int as libc::c_uchar,
                    b: 95 as libc::c_int as libc::c_uchar,
                    a: 255 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = mu_Color {
                    r: 115 as libc::c_int as libc::c_uchar,
                    g: 115 as libc::c_int as libc::c_uchar,
                    b: 115 as libc::c_int as libc::c_uchar,
                    a: 255 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = mu_Color {
                    r: 30 as libc::c_int as libc::c_uchar,
                    g: 30 as libc::c_int as libc::c_uchar,
                    b: 30 as libc::c_int as libc::c_uchar,
                    a: 255 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = mu_Color {
                    r: 35 as libc::c_int as libc::c_uchar,
                    g: 35 as libc::c_int as libc::c_uchar,
                    b: 35 as libc::c_int as libc::c_uchar,
                    a: 255 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = mu_Color {
                    r: 40 as libc::c_int as libc::c_uchar,
                    g: 40 as libc::c_int as libc::c_uchar,
                    b: 40 as libc::c_int as libc::c_uchar,
                    a: 255 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = mu_Color {
                    r: 43 as libc::c_int as libc::c_uchar,
                    g: 43 as libc::c_int as libc::c_uchar,
                    b: 43 as libc::c_int as libc::c_uchar,
                    a: 255 as libc::c_int as libc::c_uchar,
                };
                init
            },
            {
                let mut init = mu_Color {
                    r: 30 as libc::c_int as libc::c_uchar,
                    g: 30 as libc::c_int as libc::c_uchar,
                    b: 30 as libc::c_int as libc::c_uchar,
                    a: 255 as libc::c_int as libc::c_uchar,
                };
                init
            },
        ],
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn mu_vec2(mut x: libc::c_int, mut y: libc::c_int) -> mu_Vec2 {
    let mut res: mu_Vec2 = mu_Vec2 { x: 0, y: 0 };
    res.x = x;
    res.y = y;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mu_rect(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> mu_Rect {
    let mut res: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
    res.x = x;
    res.y = y;
    res.w = w;
    res.h = h;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mu_color(
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
    mut a: libc::c_int,
) -> mu_Color {
    let mut res: mu_Color = mu_Color { r: 0, g: 0, b: 0, a: 0 };
    res.r = r as libc::c_uchar;
    res.g = g as libc::c_uchar;
    res.b = b as libc::c_uchar;
    res.a = a as libc::c_uchar;
    return res;
}
unsafe extern "C" fn expand_rect(mut rect: mu_Rect, mut n: libc::c_int) -> mu_Rect {
    return mu_rect(
        rect.x - n,
        rect.y - n,
        rect.w + n * 2 as libc::c_int,
        rect.h + n * 2 as libc::c_int,
    );
}
unsafe extern "C" fn intersect_rects(mut r1: mu_Rect, mut r2: mu_Rect) -> mu_Rect {
    let mut x1: libc::c_int = if r1.x > r2.x { r1.x } else { r2.x };
    let mut y1: libc::c_int = if r1.y > r2.y { r1.y } else { r2.y };
    let mut x2: libc::c_int = if r1.x + r1.w < r2.x + r2.w {
        r1.x + r1.w
    } else {
        r2.x + r2.w
    };
    let mut y2: libc::c_int = if r1.y + r1.h < r2.y + r2.h {
        r1.y + r1.h
    } else {
        r2.y + r2.h
    };
    if x2 < x1 {
        x2 = x1;
    }
    if y2 < y1 {
        y2 = y1;
    }
    return mu_rect(x1, y1, x2 - x1, y2 - y1);
}
unsafe extern "C" fn rect_overlaps_vec2(mut r: mu_Rect, mut p: mu_Vec2) -> libc::c_int {
    return (p.x >= r.x && p.x < r.x + r.w && p.y >= r.y && p.y < r.y + r.h)
        as libc::c_int;
}
unsafe extern "C" fn draw_frame(
    mut ctx: *mut mu_Context,
    mut rect: mu_Rect,
    mut colorid: libc::c_int,
) {
    mu_draw_rect(ctx, rect, (*(*ctx).style).colors[colorid as usize]);
    if colorid == MU_COLOR_SCROLLBASE as libc::c_int
        || colorid == MU_COLOR_SCROLLTHUMB as libc::c_int
        || colorid == MU_COLOR_TITLEBG as libc::c_int
    {
        return;
    }
    if (*(*ctx).style).colors[MU_COLOR_BORDER as libc::c_int as usize].a != 0 {
        mu_draw_box(
            ctx,
            expand_rect(rect, 1 as libc::c_int),
            (*(*ctx).style).colors[MU_COLOR_BORDER as libc::c_int as usize],
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn mu_init(mut ctx: *mut mu_Context) {
    memset(
        ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mu_Context>() as libc::c_ulong,
    );
    (*ctx)
        .draw_frame = Some(
        draw_frame as unsafe extern "C" fn(*mut mu_Context, mu_Rect, libc::c_int) -> (),
    );
    (*ctx)._style = default_style;
    (*ctx).style = &mut (*ctx)._style;
}
#[no_mangle]
pub unsafe extern "C" fn mu_begin(mut ctx: *mut mu_Context) {
    assert!(((*ctx).text_width).is_some() && ((*ctx).text_height).is_some());
    (*ctx).command_list.idx = 0 as libc::c_int;
    (*ctx).root_list.idx = 0 as libc::c_int;
    (*ctx).text_stack.idx = 0 as libc::c_int;
    (*ctx).scroll_target = 0 as *mut mu_Container;
    (*ctx).hover_root = (*ctx).next_hover_root;
    (*ctx).next_hover_root = 0 as *mut mu_Container;
    (*ctx).mouse_delta.x = (*ctx).mouse_pos.x - (*ctx).last_mouse_pos.x;
    (*ctx).mouse_delta.y = (*ctx).mouse_pos.y - (*ctx).last_mouse_pos.y;
    (*ctx).frame += 1;
}
unsafe extern "C" fn compare_zindex(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return (**(a as *mut *mut mu_Container)).zindex
        - (**(b as *mut *mut mu_Container)).zindex;
}
#[no_mangle]
pub unsafe extern "C" fn mu_end(mut ctx: *mut mu_Context) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    assert!((*ctx).container_stack.idx == 0 as libc::c_int);
    assert!((*ctx).clip_stack.idx == 0 as libc::c_int);
    assert!((*ctx).id_stack.idx == 0 as libc::c_int);
    assert!((*ctx).layout_stack.idx == 0 as libc::c_int);
    if !((*ctx).scroll_target).is_null() {
        (*(*ctx).scroll_target).scroll.x += (*ctx).scroll_delta.x;
        (*(*ctx).scroll_target).scroll.y += (*ctx).scroll_delta.y;
    }
    if (*ctx).updated_focus == 0 {
        (*ctx).focus = 0 as libc::c_int as mu_Id;
    }
    (*ctx).updated_focus = 0 as libc::c_int;
    if (*ctx).mouse_pressed != 0 && !((*ctx).next_hover_root).is_null()
        && (*(*ctx).next_hover_root).zindex < (*ctx).last_zindex
        && (*(*ctx).next_hover_root).zindex >= 0 as libc::c_int
    {
        mu_bring_to_front(ctx, (*ctx).next_hover_root);
    }
    (*ctx).key_pressed = 0 as libc::c_int;
    (*ctx).input_text[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    (*ctx).mouse_pressed = 0 as libc::c_int;
    (*ctx).scroll_delta = mu_vec2(0 as libc::c_int, 0 as libc::c_int);
    (*ctx).last_mouse_pos = (*ctx).mouse_pos;
    n = (*ctx).root_list.idx;
    qsort(
        ((*ctx).root_list.items).as_mut_ptr() as *mut libc::c_void,
        n as size_t,
        ::core::mem::size_of::<*mut mu_Container>() as libc::c_ulong,
        Some(
            compare_zindex
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < n {
        let mut cnt: *mut mu_Container = (*ctx).root_list.items[i as usize];
        if i == 0 as libc::c_int {
            let mut cmd: *mut mu_Command = ((*ctx).command_list.items).as_mut_ptr();
            assert!((*cmd).type_0 == MU_COMMAND_JUMP as libc::c_int);
            (*cmd).jump.dst_idx = (*cnt).head_idx + 1 as libc::c_int;
            assert!((*cmd).jump.dst_idx < 4096 as libc::c_int);
        } else {
            let mut prev: *mut mu_Container = (*ctx)
                .root_list
                .items[(i - 1 as libc::c_int) as usize];
            (*ctx)
                .command_list
                .items[(*prev).tail_idx as usize]
                .jump
                .dst_idx = (*cnt).head_idx + 1 as libc::c_int;
        }
        if i == n - 1 as libc::c_int {
            assert!((*cnt).tail_idx < 4096 as libc::c_int);
            assert!((*ctx).command_list.items[(*cnt).tail_idx as usize].type_0
                == MU_COMMAND_JUMP as libc::c_int);
            (*ctx)
                .command_list
                .items[(*cnt).tail_idx as usize]
                .jump
                .dst_idx = (*ctx).command_list.idx;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mu_set_focus(mut ctx: *mut mu_Context, mut id: mu_Id) {
    (*ctx).focus = id;
    (*ctx).updated_focus = 1 as libc::c_int;
}
unsafe extern "C" fn hash(
    mut hash_0: *mut mu_Id,
    mut data: *const libc::c_void,
    mut size: libc::c_int,
) {
    let mut p: *const libc::c_uchar = data as *const libc::c_uchar;
    loop {
        let fresh0 = size;
        size = size - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = p;
        p = p.offset(1);
        *hash_0 = (*hash_0 ^ *fresh1 as libc::c_uint)
            .wrapping_mul(16777619 as libc::c_int as libc::c_uint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mu_get_id(
    mut ctx: *mut mu_Context,
    mut data: *const libc::c_void,
    mut size: libc::c_int,
) -> mu_Id {
    let mut idx: libc::c_int = (*ctx).id_stack.idx;
    let mut res: mu_Id = (if idx > 0 as libc::c_int {
        (*ctx).id_stack.items[(idx - 1 as libc::c_int) as usize] as libc::c_long
    } else {
        2166136261 as libc::c_long
    }) as mu_Id;
    hash(&mut res, data, size);
    (*ctx).last_id = res;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mu_push_id(
    mut ctx: *mut mu_Context,
    mut data: *const libc::c_void,
    mut size: libc::c_int,
) {
    assert!((*ctx).id_stack.idx
        < (::core::mem::size_of::<[mu_Id; 32]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<mu_Id>() as libc::c_ulong)
            as libc::c_int);
    (*ctx).id_stack.items[(*ctx).id_stack.idx as usize] = mu_get_id(ctx, data, size);
    (*ctx).id_stack.idx += 1;
}
#[no_mangle]
pub unsafe extern "C" fn mu_pop_id(mut ctx: *mut mu_Context) {
    assert!((*ctx).id_stack.idx > 0 as libc::c_int);
    (*ctx).id_stack.idx -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn mu_push_clip_rect(mut ctx: *mut mu_Context, mut rect: mu_Rect) {
    let mut last: mu_Rect = mu_get_clip_rect(ctx);
    assert!((*ctx).clip_stack.idx
        < (::core::mem::size_of::<[mu_Rect; 32]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<mu_Rect>() as libc::c_ulong)
            as libc::c_int);
    (*ctx)
        .clip_stack
        .items[(*ctx).clip_stack.idx as usize] = intersect_rects(rect, last);
    (*ctx).clip_stack.idx += 1;
}
#[no_mangle]
pub unsafe extern "C" fn mu_pop_clip_rect(mut ctx: *mut mu_Context) {
    assert!((*ctx).clip_stack.idx > 0 as libc::c_int);
    (*ctx).clip_stack.idx -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn mu_get_clip_rect(mut ctx: *mut mu_Context) -> mu_Rect {
    assert!((*ctx).clip_stack.idx > 0 as libc::c_int);
    return (*ctx).clip_stack.items[((*ctx).clip_stack.idx - 1 as libc::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn mu_check_clip(
    mut ctx: *mut mu_Context,
    mut r: mu_Rect,
) -> Clip {
    let mut cr: mu_Rect = mu_get_clip_rect(ctx);
    if r.x > cr.x + cr.w || r.x + r.w < cr.x || r.y > cr.y + cr.h || r.y + r.h < cr.y {
        return Clip::All;
    }
    if r.x >= cr.x && r.x + r.w <= cr.x + cr.w && r.y >= cr.y && r.y + r.h <= cr.y + cr.h
    {
        return Clip::None;
    }
    return Clip::Part;
}
unsafe extern "C" fn push_layout(
    mut ctx: *mut mu_Context,
    mut body: mu_Rect,
    mut scroll: mu_Vec2,
) {
    let mut layout: mu_Layout = mu_Layout {
        body: mu_Rect { x: 0, y: 0, w: 0, h: 0 },
        next: mu_Rect { x: 0, y: 0, w: 0, h: 0 },
        position: mu_Vec2 { x: 0, y: 0 },
        size: mu_Vec2 { x: 0, y: 0 },
        max: mu_Vec2 { x: 0, y: 0 },
        widths: [0; 16],
        items: 0,
        item_index: 0,
        next_row: 0,
        next_type: 0,
        indent: 0,
    };
    let mut width: libc::c_int = 0 as libc::c_int;
    memset(
        &mut layout as *mut mu_Layout as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mu_Layout>() as libc::c_ulong,
    );
    layout.body = mu_rect(body.x - scroll.x, body.y - scroll.y, body.w, body.h);
    layout.max = mu_vec2(-(0x1000000 as libc::c_int), -(0x1000000 as libc::c_int));
    assert!((*ctx).layout_stack.idx
        < (::core::mem::size_of::<[mu_Layout; 16]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<mu_Layout>() as libc::c_ulong)
            as libc::c_int);
    (*ctx).layout_stack.items[(*ctx).layout_stack.idx as usize] = layout;
    (*ctx).layout_stack.idx += 1;
    mu_layout_row(ctx, 1 as libc::c_int, &mut width, 0 as libc::c_int);
}
unsafe extern "C" fn get_layout(mut ctx: *mut mu_Context) -> *mut mu_Layout {
    return &mut *((*ctx).layout_stack.items)
        .as_mut_ptr()
        .offset(((*ctx).layout_stack.idx - 1 as libc::c_int) as isize) as *mut mu_Layout;
}
unsafe extern "C" fn pop_container(mut ctx: *mut mu_Context) {
    let mut cnt: *mut mu_Container = mu_get_current_container(ctx);
    let mut layout: *mut mu_Layout = get_layout(ctx);
    (*cnt).content_size.x = (*layout).max.x - (*layout).body.x;
    (*cnt).content_size.y = (*layout).max.y - (*layout).body.y;
    assert!((*ctx).container_stack.idx > 0 as libc::c_int);
    (*ctx).container_stack.idx -= 1;
    assert!((*ctx).layout_stack.idx > 0 as libc::c_int);
    (*ctx).layout_stack.idx -= 1;
    mu_pop_id(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn mu_get_current_container(
    mut ctx: *mut mu_Context,
) -> *mut mu_Container {
    assert!((*ctx).container_stack.idx > 0 as libc::c_int);
    return (*ctx)
        .container_stack
        .items[((*ctx).container_stack.idx - 1 as libc::c_int) as usize];
}
unsafe extern "C" fn get_container(
    mut ctx: *mut mu_Context,
    mut id: mu_Id,
    mut opt: libc::c_int,
) -> *mut mu_Container {
    let mut cnt: *mut mu_Container = 0 as *mut mu_Container;
    let mut idx: libc::c_int = mu_pool_get(
        ctx,
        ((*ctx).container_pool).as_mut_ptr(),
        48 as libc::c_int,
        id,
    );
    if idx >= 0 as libc::c_int {
        if (*ctx).containers[idx as usize].open != 0
            || !opt & MU_OPT_CLOSED as libc::c_int != 0
        {
            mu_pool_update(ctx, ((*ctx).container_pool).as_mut_ptr(), idx);
        }
        return &mut *((*ctx).containers).as_mut_ptr().offset(idx as isize)
            as *mut mu_Container;
    }
    if opt & MU_OPT_CLOSED as libc::c_int != 0 {
        return 0 as *mut mu_Container;
    }
    idx = mu_pool_init(ctx, ((*ctx).container_pool).as_mut_ptr(), 48 as libc::c_int, id);
    cnt = &mut *((*ctx).containers).as_mut_ptr().offset(idx as isize)
        as *mut mu_Container;
    memset(
        cnt as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mu_Container>() as libc::c_ulong,
    );
    (*cnt).head_idx = -(1 as libc::c_int);
    (*cnt).tail_idx = -(1 as libc::c_int);
    (*cnt).open = 1 as libc::c_int;
    mu_bring_to_front(ctx, cnt);
    return cnt;
}
#[no_mangle]
pub unsafe extern "C" fn mu_get_container(
    mut ctx: *mut mu_Context,
    mut name: *const libc::c_char,
) -> *mut mu_Container {
    let mut id: mu_Id = mu_get_id(
        ctx,
        name as *const libc::c_void,
        strlen(name) as libc::c_int,
    );
    return get_container(ctx, id, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn mu_bring_to_front(
    mut ctx: *mut mu_Context,
    mut cnt: *mut mu_Container,
) {
    (*ctx).last_zindex += 1;
    (*cnt).zindex = (*ctx).last_zindex;
}
#[no_mangle]
pub unsafe extern "C" fn mu_pool_init(
    mut ctx: *mut mu_Context,
    mut items: *mut mu_PoolItem,
    mut len: libc::c_int,
    mut id: mu_Id,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = -(1 as libc::c_int);
    let mut f: libc::c_int = (*ctx).frame;
    i = 0 as libc::c_int;
    while i < len {
        if (*items.offset(i as isize)).last_update < f {
            f = (*items.offset(i as isize)).last_update;
            n = i;
        }
        i += 1;
    }
    assert!(n > -(1 as libc::c_int));
    (*items.offset(n as isize)).id = id;
    mu_pool_update(ctx, items, n);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn mu_pool_get(
    mut ctx: *mut mu_Context,
    mut items: *mut mu_PoolItem,
    mut len: libc::c_int,
    mut id: mu_Id,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        if (*items.offset(i as isize)).id == id {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn mu_pool_update(
    mut ctx: *mut mu_Context,
    mut items: *mut mu_PoolItem,
    mut idx: libc::c_int,
) {
    (*items.offset(idx as isize)).last_update = (*ctx).frame;
}
#[no_mangle]
pub unsafe extern "C" fn mu_input_mousemove(
    mut ctx: *mut mu_Context,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    (*ctx).mouse_pos = mu_vec2(x, y);
}
#[no_mangle]
pub unsafe extern "C" fn mu_input_mousedown(
    mut ctx: *mut mu_Context,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut btn: libc::c_int,
) {
    mu_input_mousemove(ctx, x, y);
    (*ctx).mouse_down |= btn;
    (*ctx).mouse_pressed |= btn;
}
#[no_mangle]
pub unsafe extern "C" fn mu_input_mouseup(
    mut ctx: *mut mu_Context,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut btn: libc::c_int,
) {
    mu_input_mousemove(ctx, x, y);
    (*ctx).mouse_down &= !btn;
}
#[no_mangle]
pub unsafe extern "C" fn mu_input_scroll(
    mut ctx: *mut mu_Context,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    (*ctx).scroll_delta.x += x;
    (*ctx).scroll_delta.y += y;
}
#[no_mangle]
pub unsafe extern "C" fn mu_input_keydown(
    mut ctx: *mut mu_Context,
    mut key: libc::c_int,
) {
    (*ctx).key_pressed |= key;
    (*ctx).key_down |= key;
}
#[no_mangle]
pub unsafe extern "C" fn mu_input_keyup(mut ctx: *mut mu_Context, mut key: libc::c_int) {
    (*ctx).key_down &= !key;
}
#[no_mangle]
pub unsafe extern "C" fn mu_input_text(
    mut ctx: *mut mu_Context,
    mut text: *const libc::c_char,
) {
    let mut len: libc::c_int = strlen(((*ctx).input_text).as_mut_ptr()) as libc::c_int;
    let mut size: libc::c_int = (strlen(text))
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    assert!(len + size
        <= ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int);
    memcpy(
        ((*ctx).input_text).as_mut_ptr().offset(len as isize) as *mut libc::c_void,
        text as *const libc::c_void,
        size as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mu_push_command(
    mut ctx: *mut mu_Context,
    mut type_0: libc::c_int,
) -> *mut mu_Command {
    let mut cmd: *mut mu_Command = &mut *((*ctx).command_list.items)
        .as_mut_ptr()
        .offset((*ctx).command_list.idx as isize) as *mut mu_Command;
    assert!((*ctx).command_list.idx < 4096 as libc::c_int);
    (*cmd).base.type_0 = type_0;
    (*ctx).command_list.idx += 1 as libc::c_int;
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn mu_push_text(
    mut ctx: *mut mu_Context,
    mut str: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut str_start: *mut libc::c_char = &mut *((*ctx).text_stack.items)
        .as_mut_ptr()
        .offset((*ctx).text_stack.idx as isize) as *mut libc::c_char;
    assert!(((*ctx).text_stack.idx as libc::c_ulong)
        .wrapping_add(len)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        < 65536 as libc::c_int as libc::c_ulong);
    memcpy(str_start as *mut libc::c_void, str as *const libc::c_void, len);
    *str_start.offset(len as isize) = '\0' as i32 as libc::c_char;
    (*ctx)
        .text_stack
        .idx = ((*ctx).text_stack.idx as libc::c_ulong)
        .wrapping_add(len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as libc::c_int
        as libc::c_int;
    return str_start;
}
#[no_mangle]
pub unsafe extern "C" fn mu_next_command(
    mut ctx: *mut mu_Context,
    mut cmd: *mut *mut mu_Command,
) -> libc::c_int {
    if !(*cmd).is_null() {
        *cmd = (*cmd).offset(1 as libc::c_int as isize);
    } else {
        *cmd = ((*ctx).command_list.items).as_mut_ptr();
    }
    while *cmd
        != &mut *((*ctx).command_list.items)
            .as_mut_ptr()
            .offset((*ctx).command_list.idx as isize) as *mut mu_Command
    {
        if (**cmd).type_0 != MU_COMMAND_JUMP as libc::c_int {
            return 1 as libc::c_int;
        }
        *cmd = &mut *((*ctx).command_list.items)
            .as_mut_ptr()
            .offset((**cmd).jump.dst_idx as isize) as *mut mu_Command;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn push_jump(
    mut ctx: *mut mu_Context,
    mut dst_idx: libc::c_int,
) -> libc::c_int {
    let mut cmd: *mut mu_Command = 0 as *mut mu_Command;
    cmd = mu_push_command(ctx, MU_COMMAND_JUMP as libc::c_int);
    (*cmd).jump.dst_idx = dst_idx;
    assert!(cmd
        == &mut *((*ctx).command_list.items)
            .as_mut_ptr()
            .offset(((*ctx).command_list.idx - 1 as libc::c_int) as isize)
            as *mut mu_Command);
    return (*ctx).command_list.idx - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mu_set_clip(mut ctx: *mut mu_Context, mut rect: mu_Rect) {
    let mut cmd: *mut mu_Command = 0 as *mut mu_Command;
    cmd = mu_push_command(ctx, MU_COMMAND_CLIP as libc::c_int);
    (*cmd).clip.rect = rect;
}
#[no_mangle]
pub unsafe extern "C" fn mu_draw_rect(
    mut ctx: *mut mu_Context,
    mut rect: mu_Rect,
    mut color: mu_Color,
) {
    let mut cmd: *mut mu_Command = 0 as *mut mu_Command;
    rect = intersect_rects(rect, mu_get_clip_rect(ctx));
    if rect.w > 0 as libc::c_int && rect.h > 0 as libc::c_int {
        cmd = mu_push_command(ctx, MU_COMMAND_RECT as libc::c_int);
        (*cmd).rect.rect = rect;
        (*cmd).rect.color = color;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mu_draw_box(
    mut ctx: *mut mu_Context,
    mut rect: mu_Rect,
    mut color: mu_Color,
) {
    mu_draw_rect(
        ctx,
        mu_rect(
            rect.x + 1 as libc::c_int,
            rect.y,
            rect.w - 2 as libc::c_int,
            1 as libc::c_int,
        ),
        color,
    );
    mu_draw_rect(
        ctx,
        mu_rect(
            rect.x + 1 as libc::c_int,
            rect.y + rect.h - 1 as libc::c_int,
            rect.w - 2 as libc::c_int,
            1 as libc::c_int,
        ),
        color,
    );
    mu_draw_rect(ctx, mu_rect(rect.x, rect.y, 1 as libc::c_int, rect.h), color);
    mu_draw_rect(
        ctx,
        mu_rect(rect.x + rect.w - 1 as libc::c_int, rect.y, 1 as libc::c_int, rect.h),
        color,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mu_draw_text(
    mut ctx: *mut mu_Context,
    mut font: mu_Font,
    mut str: *const libc::c_char,
    mut len: libc::c_int,
    mut pos: mu_Vec2,
    mut color: mu_Color,
) {
    let mut cmd: *mut mu_Command = 0 as *mut mu_Command;
    let mut rect: mu_Rect = mu_rect(
        pos.x,
        pos.y,
        ((*ctx).text_width).expect("non-null function pointer")(font, str, len),
        ((*ctx).text_height).expect("non-null function pointer")(font),
    );
    let clipped = mu_check_clip(ctx, rect);
    match clipped {
        Clip::All => return,
        Clip::Part => mu_set_clip(ctx, mu_get_clip_rect(ctx)),
        _ => ()
    }

    if len < 0 as libc::c_int {
        len = strlen(str) as libc::c_int;
    }
    let mut str_start: *mut libc::c_char = mu_push_text(ctx, str, len as size_t);
    cmd = mu_push_command(ctx, MU_COMMAND_TEXT as libc::c_int);
    (*cmd).text.str_0 = str_start;
    (*cmd).text.pos = pos;
    (*cmd).text.color = color;
    (*cmd).text.font = font;
    if clipped != Clip::None {
        mu_set_clip(ctx, unclipped_rect);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mu_draw_icon(
    mut ctx: *mut mu_Context,
    mut id: libc::c_int,
    mut rect: mu_Rect,
    mut color: mu_Color,
) {
    let mut cmd: *mut mu_Command = 0 as *mut mu_Command;
    let clipped = mu_check_clip(ctx, rect);
    match clipped {
        Clip::All => return,
        Clip::Part => mu_set_clip(ctx, mu_get_clip_rect(ctx)),
        _ => (),
    }
    cmd = mu_push_command(ctx, MU_COMMAND_ICON as libc::c_int);
    (*cmd).icon.id = id;
    (*cmd).icon.rect = rect;
    (*cmd).icon.color = color;
    if clipped != Clip::None {
        mu_set_clip(ctx, unclipped_rect);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mu_layout_begin_column(mut ctx: *mut mu_Context) {
    push_layout(ctx, mu_layout_next(ctx), mu_vec2(0 as libc::c_int, 0 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn mu_layout_end_column(mut ctx: *mut mu_Context) {
    let mut a: *mut mu_Layout = 0 as *mut mu_Layout;
    let mut b: *mut mu_Layout = 0 as *mut mu_Layout;
    b = get_layout(ctx);
    assert!((*ctx).layout_stack.idx > 0 as libc::c_int);
    (*ctx).layout_stack.idx -= 1;
    a = get_layout(ctx);
    (*a)
        .position
        .x = if (*a).position.x > (*b).position.x + (*b).body.x - (*a).body.x {
        (*a).position.x
    } else {
        (*b).position.x + (*b).body.x - (*a).body.x
    };
    (*a)
        .next_row = if (*a).next_row > (*b).next_row + (*b).body.y - (*a).body.y {
        (*a).next_row
    } else {
        (*b).next_row + (*b).body.y - (*a).body.y
    };
    (*a).max.x = if (*a).max.x > (*b).max.x { (*a).max.x } else { (*b).max.x };
    (*a).max.y = if (*a).max.y > (*b).max.y { (*a).max.y } else { (*b).max.y };
}
#[no_mangle]
pub unsafe extern "C" fn mu_layout_row(
    mut ctx: *mut mu_Context,
    mut items: libc::c_int,
    mut widths: *const libc::c_int,
    mut height: libc::c_int,
) {
    let mut layout: *mut mu_Layout = get_layout(ctx);
    if !widths.is_null() {
        assert!(items <= 16 as libc::c_int);
        memcpy(
            ((*layout).widths).as_mut_ptr() as *mut libc::c_void,
            widths as *const libc::c_void,
            (items as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
    }
    (*layout).items = items;
    (*layout).position = mu_vec2((*layout).indent, (*layout).next_row);
    (*layout).size.y = height;
    (*layout).item_index = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mu_layout_width(
    mut ctx: *mut mu_Context,
    mut width: libc::c_int,
) {
    (*get_layout(ctx)).size.x = width;
}
#[no_mangle]
pub unsafe extern "C" fn mu_layout_height(
    mut ctx: *mut mu_Context,
    mut height: libc::c_int,
) {
    (*get_layout(ctx)).size.y = height;
}
#[no_mangle]
pub unsafe extern "C" fn mu_layout_set_next(
    mut ctx: *mut mu_Context,
    mut r: mu_Rect,
    mut relative: libc::c_int,
) {
    let mut layout: *mut mu_Layout = get_layout(ctx);
    (*layout).next = r;
    (*layout)
        .next_type = if relative != 0 {
        RELATIVE as libc::c_int
    } else {
        ABSOLUTE as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn mu_layout_next(mut ctx: *mut mu_Context) -> mu_Rect {
    let mut layout: *mut mu_Layout = get_layout(ctx);
    let mut style: *mut mu_Style = (*ctx).style;
    let mut res: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
    if (*layout).next_type != 0 {
        let mut type_0: libc::c_int = (*layout).next_type;
        (*layout).next_type = 0 as libc::c_int;
        res = (*layout).next;
        if type_0 == ABSOLUTE as libc::c_int {
            (*ctx).last_rect = res;
            return (*ctx).last_rect;
        }
    } else {
        if (*layout).item_index == (*layout).items {
            mu_layout_row(
                ctx,
                (*layout).items,
                0 as *const libc::c_int,
                (*layout).size.y,
            );
        }
        res.x = (*layout).position.x;
        res.y = (*layout).position.y;
        res
            .w = if (*layout).items > 0 as libc::c_int {
            (*layout).widths[(*layout).item_index as usize]
        } else {
            (*layout).size.x
        };
        res.h = (*layout).size.y;
        if res.w == 0 as libc::c_int {
            res.w = (*style).size.x + (*style).padding * 2 as libc::c_int;
        }
        if res.h == 0 as libc::c_int {
            res.h = (*style).size.y + (*style).padding * 2 as libc::c_int;
        }
        if res.w < 0 as libc::c_int {
            res.w += (*layout).body.w - res.x + 1 as libc::c_int;
        }
        if res.h < 0 as libc::c_int {
            res.h += (*layout).body.h - res.y + 1 as libc::c_int;
        }
        (*layout).item_index += 1;
    }
    (*layout).position.x += res.w + (*style).spacing;
    (*layout)
        .next_row = if (*layout).next_row > res.y + res.h + (*style).spacing {
        (*layout).next_row
    } else {
        res.y + res.h + (*style).spacing
    };
    res.x += (*layout).body.x;
    res.y += (*layout).body.y;
    (*layout)
        .max
        .x = if (*layout).max.x > res.x + res.w {
        (*layout).max.x
    } else {
        res.x + res.w
    };
    (*layout)
        .max
        .y = if (*layout).max.y > res.y + res.h {
        (*layout).max.y
    } else {
        res.y + res.h
    };
    (*ctx).last_rect = res;
    return (*ctx).last_rect;
}
unsafe extern "C" fn in_hover_root(mut ctx: *mut mu_Context) -> libc::c_int {
    let mut i: libc::c_int = (*ctx).container_stack.idx;
    loop {
        let fresh2 = i;
        i = i - 1;
        if !(fresh2 != 0) {
            break;
        }
        if (*ctx).container_stack.items[i as usize] == (*ctx).hover_root {
            return 1 as libc::c_int;
        }
        if (*(*ctx).container_stack.items[i as usize]).head_idx != -(1 as libc::c_int) {
            break;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mu_draw_control_frame(
    mut ctx: *mut mu_Context,
    mut id: mu_Id,
    mut rect: mu_Rect,
    mut colorid: libc::c_int,
    mut opt: libc::c_int,
) {
    if opt & MU_OPT_NOFRAME as libc::c_int != 0 {
        return;
    }
    colorid
        += if (*ctx).focus == id {
            2 as libc::c_int
        } else if (*ctx).hover == id {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    ((*ctx).draw_frame).expect("non-null function pointer")(ctx, rect, colorid);
}
#[no_mangle]
pub unsafe extern "C" fn mu_draw_control_text(
    mut ctx: *mut mu_Context,
    mut str: *const libc::c_char,
    mut rect: mu_Rect,
    mut colorid: libc::c_int,
    mut opt: libc::c_int,
) {
    let mut pos: mu_Vec2 = mu_Vec2 { x: 0, y: 0 };
    let mut font: mu_Font = (*(*ctx).style).font;
    let mut tw: libc::c_int = ((*ctx).text_width)
        .expect("non-null function pointer")(font, str, -(1 as libc::c_int));
    mu_push_clip_rect(ctx, rect);
    pos
        .y = rect.y
        + (rect.h - ((*ctx).text_height).expect("non-null function pointer")(font))
            / 2 as libc::c_int;
    if opt & MU_OPT_ALIGNCENTER as libc::c_int != 0 {
        pos.x = rect.x + (rect.w - tw) / 2 as libc::c_int;
    } else if opt & MU_OPT_ALIGNRIGHT as libc::c_int != 0 {
        pos.x = rect.x + rect.w - tw - (*(*ctx).style).padding;
    } else {
        pos.x = rect.x + (*(*ctx).style).padding;
    }
    mu_draw_text(
        ctx,
        font,
        str,
        -(1 as libc::c_int),
        pos,
        (*(*ctx).style).colors[colorid as usize],
    );
    mu_pop_clip_rect(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn mu_mouse_over(
    mut ctx: *mut mu_Context,
    mut rect: mu_Rect,
) -> libc::c_int {
    return (rect_overlaps_vec2(rect, (*ctx).mouse_pos) != 0
        && rect_overlaps_vec2(mu_get_clip_rect(ctx), (*ctx).mouse_pos) != 0
        && in_hover_root(ctx) != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mu_update_control(
    mut ctx: *mut mu_Context,
    mut id: mu_Id,
    mut rect: mu_Rect,
    mut opt: libc::c_int,
) {
    let mut mouseover: libc::c_int = mu_mouse_over(ctx, rect);
    if (*ctx).focus == id {
        (*ctx).updated_focus = 1 as libc::c_int;
    }
    if opt & MU_OPT_NOINTERACT as libc::c_int != 0 {
        return;
    }
    if mouseover != 0 && (*ctx).mouse_down == 0 {
        (*ctx).hover = id;
    }
    if (*ctx).focus == id {
        if (*ctx).mouse_pressed != 0 && mouseover == 0 {
            mu_set_focus(ctx, 0 as libc::c_int as mu_Id);
        }
        if (*ctx).mouse_down == 0 && !opt & MU_OPT_HOLDFOCUS as libc::c_int != 0 {
            mu_set_focus(ctx, 0 as libc::c_int as mu_Id);
        }
    }
    if (*ctx).hover == id {
        if (*ctx).mouse_pressed != 0 {
            mu_set_focus(ctx, id);
        } else if mouseover == 0 {
            (*ctx).hover = 0 as libc::c_int as mu_Id;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mu_text(
    mut ctx: *mut mu_Context,
    mut text: *const libc::c_char,
) {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = text;
    let mut width: libc::c_int = -(1 as libc::c_int);
    let mut font: mu_Font = (*(*ctx).style).font;
    let mut color: mu_Color = (*(*ctx).style)
        .colors[MU_COLOR_TEXT as libc::c_int as usize];
    mu_layout_begin_column(ctx);
    mu_layout_row(
        ctx,
        1 as libc::c_int,
        &mut width,
        ((*ctx).text_height).expect("non-null function pointer")(font),
    );
    loop {
        let mut r: mu_Rect = mu_layout_next(ctx);
        let mut w: libc::c_int = 0 as libc::c_int;
        end = p;
        start = end;
        loop {
            let mut word: *const libc::c_char = p;
            while *p as libc::c_int != 0 && *p as libc::c_int != ' ' as i32
                && *p as libc::c_int != '\n' as i32
            {
                p = p.offset(1);
            }
            w
                += ((*ctx).text_width)
                    .expect(
                        "non-null function pointer",
                    )(font, word, p.offset_from(word) as libc::c_long as libc::c_int);
            if w > r.w && end != start {
                break;
            }
            w
                += ((*ctx).text_width)
                    .expect("non-null function pointer")(font, p, 1 as libc::c_int);
            let fresh3 = p;
            p = p.offset(1);
            end = fresh3;
            if !(*end as libc::c_int != 0 && *end as libc::c_int != '\n' as i32) {
                break;
            }
        }
        mu_draw_text(
            ctx,
            font,
            start,
            end.offset_from(start) as libc::c_long as libc::c_int,
            mu_vec2(r.x, r.y),
            color,
        );
        p = end.offset(1 as libc::c_int as isize);
        if !(*end != 0) {
            break;
        }
    }
    mu_layout_end_column(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn mu_label(
    mut ctx: *mut mu_Context,
    mut text: *const libc::c_char,
) {
    mu_draw_control_text(
        ctx,
        text,
        mu_layout_next(ctx),
        MU_COLOR_TEXT as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mu_button_ex(
    mut ctx: *mut mu_Context,
    mut label: *const libc::c_char,
    mut icon: libc::c_int,
    mut opt: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut id: mu_Id = if !label.is_null() {
        mu_get_id(ctx, label as *const libc::c_void, strlen(label) as libc::c_int)
    } else {
        mu_get_id(
            ctx,
            &mut icon as *mut libc::c_int as *const libc::c_void,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        )
    };
    let mut r: mu_Rect = mu_layout_next(ctx);
    mu_update_control(ctx, id, r, opt);
    if (*ctx).mouse_pressed == MU_MOUSE_LEFT as libc::c_int && (*ctx).focus == id {
        res |= MU_RES_SUBMIT as libc::c_int;
    }
    mu_draw_control_frame(ctx, id, r, MU_COLOR_BUTTON as libc::c_int, opt);
    if !label.is_null() {
        mu_draw_control_text(ctx, label, r, MU_COLOR_TEXT as libc::c_int, opt);
    }
    if icon != 0 {
        mu_draw_icon(
            ctx,
            icon,
            r,
            (*(*ctx).style).colors[MU_COLOR_TEXT as libc::c_int as usize],
        );
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mu_checkbox(
    mut ctx: *mut mu_Context,
    mut label: *const libc::c_char,
    mut state: *mut libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut id: mu_Id = mu_get_id(
        ctx,
        &mut state as *mut *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong as libc::c_int,
    );
    let mut r: mu_Rect = mu_layout_next(ctx);
    let mut box_0: mu_Rect = mu_rect(r.x, r.y, r.h, r.h);
    mu_update_control(ctx, id, r, 0 as libc::c_int);
    if (*ctx).mouse_pressed == MU_MOUSE_LEFT as libc::c_int && (*ctx).focus == id {
        res |= MU_RES_CHANGE as libc::c_int;
        *state = (*state == 0) as libc::c_int;
    }
    mu_draw_control_frame(
        ctx,
        id,
        box_0,
        MU_COLOR_BASE as libc::c_int,
        0 as libc::c_int,
    );
    if *state != 0 {
        mu_draw_icon(
            ctx,
            MU_ICON_CHECK as libc::c_int,
            box_0,
            (*(*ctx).style).colors[MU_COLOR_TEXT as libc::c_int as usize],
        );
    }
    r = mu_rect(r.x + box_0.w, r.y, r.w - box_0.w, r.h);
    mu_draw_control_text(ctx, label, r, MU_COLOR_TEXT as libc::c_int, 0 as libc::c_int);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mu_textbox_raw(
    mut ctx: *mut mu_Context,
    mut buf: *mut libc::c_char,
    mut bufsz: libc::c_int,
    mut id: mu_Id,
    mut r: mu_Rect,
    mut opt: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    mu_update_control(ctx, id, r, opt | MU_OPT_HOLDFOCUS as libc::c_int);
    if (*ctx).focus == id {
        let mut len: libc::c_int = strlen(buf) as libc::c_int;
        let mut n: libc::c_int = if (bufsz - len - 1 as libc::c_int)
            < strlen(((*ctx).input_text).as_mut_ptr()) as libc::c_int
        {
            bufsz - len - 1 as libc::c_int
        } else {
            strlen(((*ctx).input_text).as_mut_ptr()) as libc::c_int
        };
        if n > 0 as libc::c_int {
            memcpy(
                buf.offset(len as isize) as *mut libc::c_void,
                ((*ctx).input_text).as_mut_ptr() as *const libc::c_void,
                n as libc::c_ulong,
            );
            len += n;
            *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
            res |= MU_RES_CHANGE as libc::c_int;
        }
        if (*ctx).key_pressed & MU_KEY_BACKSPACE as libc::c_int != 0
            && len > 0 as libc::c_int
        {
            loop {
                len -= 1;
                if !(*buf.offset(len as isize) as libc::c_int & 0xc0 as libc::c_int
                    == 0x80 as libc::c_int && len > 0 as libc::c_int)
                {
                    break;
                }
            }
            *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
            res |= MU_RES_CHANGE as libc::c_int;
        }
        if (*ctx).key_pressed & MU_KEY_RETURN as libc::c_int != 0 {
            mu_set_focus(ctx, 0 as libc::c_int as mu_Id);
            res |= MU_RES_SUBMIT as libc::c_int;
        }
    }
    mu_draw_control_frame(ctx, id, r, MU_COLOR_BASE as libc::c_int, opt);
    if (*ctx).focus == id {
        let mut color: mu_Color = (*(*ctx).style)
            .colors[MU_COLOR_TEXT as libc::c_int as usize];
        let mut font: mu_Font = (*(*ctx).style).font;
        let mut textw: libc::c_int = ((*ctx).text_width)
            .expect("non-null function pointer")(font, buf, -(1 as libc::c_int));
        let mut texth: libc::c_int = ((*ctx).text_height)
            .expect("non-null function pointer")(font);
        let mut ofx: libc::c_int = r.w - (*(*ctx).style).padding - textw
            - 1 as libc::c_int;
        let mut textx: libc::c_int = r.x
            + (if ofx < (*(*ctx).style).padding {
                ofx
            } else {
                (*(*ctx).style).padding
            });
        let mut texty: libc::c_int = r.y + (r.h - texth) / 2 as libc::c_int;
        mu_push_clip_rect(ctx, r);
        mu_draw_text(ctx, font, buf, -(1 as libc::c_int), mu_vec2(textx, texty), color);
        mu_draw_rect(ctx, mu_rect(textx + textw, texty, 1 as libc::c_int, texth), color);
        mu_pop_clip_rect(ctx);
    } else {
        mu_draw_control_text(ctx, buf, r, MU_COLOR_TEXT as libc::c_int, opt);
    }
    return res;
}
unsafe extern "C" fn number_textbox(
    mut ctx: *mut mu_Context,
    mut value: *mut mu_Real,
    mut r: mu_Rect,
    mut id: mu_Id,
) -> libc::c_int {
    if (*ctx).mouse_pressed == MU_MOUSE_LEFT as libc::c_int
        && (*ctx).key_down & MU_KEY_SHIFT as libc::c_int != 0 && (*ctx).hover == id
    {
        (*ctx).number_edit = id;
        sprintf(
            ((*ctx).number_edit_buf).as_mut_ptr(),
            b"%.3g\0" as *const u8 as *const libc::c_char,
            *value as libc::c_double,
        );
    }
    if (*ctx).number_edit == id {
        let mut res: libc::c_int = mu_textbox_raw(
            ctx,
            ((*ctx).number_edit_buf).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 127]>() as libc::c_ulong
                as libc::c_int,
            id,
            r,
            0 as libc::c_int,
        );
        if res & MU_RES_SUBMIT as libc::c_int != 0 || (*ctx).focus != id {
            *value = strtod(
                ((*ctx).number_edit_buf).as_mut_ptr(),
                0 as *mut *mut libc::c_char,
            ) as mu_Real;
            (*ctx).number_edit = 0 as libc::c_int as mu_Id;
        } else {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mu_textbox_ex(
    mut ctx: *mut mu_Context,
    mut buf: *mut libc::c_char,
    mut bufsz: libc::c_int,
    mut opt: libc::c_int,
) -> libc::c_int {
    let mut id: mu_Id = mu_get_id(
        ctx,
        &mut buf as *mut *mut libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong as libc::c_int,
    );
    let mut r: mu_Rect = mu_layout_next(ctx);
    return mu_textbox_raw(ctx, buf, bufsz, id, r, opt);
}
#[no_mangle]
pub unsafe extern "C" fn mu_slider_ex(
    mut ctx: *mut mu_Context,
    mut value: *mut mu_Real,
    mut low: mu_Real,
    mut high: mu_Real,
    mut step: mu_Real,
    mut fmt: *const libc::c_char,
    mut opt: libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut thumb: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut x: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut last: mu_Real = *value;
    let mut v: mu_Real = last;
    let mut id: mu_Id = mu_get_id(
        ctx,
        &mut value as *mut *mut mu_Real as *const libc::c_void,
        ::core::mem::size_of::<*mut mu_Real>() as libc::c_ulong as libc::c_int,
    );
    let mut base: mu_Rect = mu_layout_next(ctx);
    if number_textbox(ctx, &mut v, base, id) != 0 {
        return res;
    }
    mu_update_control(ctx, id, base, opt);
    if (*ctx).focus == id
        && (*ctx).mouse_down | (*ctx).mouse_pressed == MU_MOUSE_LEFT as libc::c_int
    {
        v = low
            + ((*ctx).mouse_pos.x - base.x) as libc::c_float * (high - low)
                / base.w as libc::c_float;
        if step != 0. {
            v = (v + step / 2 as libc::c_int as libc::c_float) / step * step;
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
        res |= MU_RES_CHANGE as libc::c_int;
    }
    mu_draw_control_frame(ctx, id, base, MU_COLOR_BASE as libc::c_int, opt);
    w = (*(*ctx).style).thumb_size;
    x = ((v - low) * (base.w - w) as libc::c_float / (high - low)) as libc::c_int;
    thumb = mu_rect(base.x + x, base.y, w, base.h);
    mu_draw_control_frame(ctx, id, thumb, MU_COLOR_BUTTON as libc::c_int, opt);
    sprintf(buf.as_mut_ptr(), fmt, v as libc::c_double);
    mu_draw_control_text(ctx, buf.as_mut_ptr(), base, MU_COLOR_TEXT as libc::c_int, opt);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mu_number_ex(
    mut ctx: *mut mu_Context,
    mut value: *mut mu_Real,
    mut step: mu_Real,
    mut fmt: *const libc::c_char,
    mut opt: libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut id: mu_Id = mu_get_id(
        ctx,
        &mut value as *mut *mut mu_Real as *const libc::c_void,
        ::core::mem::size_of::<*mut mu_Real>() as libc::c_ulong as libc::c_int,
    );
    let mut base: mu_Rect = mu_layout_next(ctx);
    let mut last: mu_Real = *value;
    if number_textbox(ctx, value, base, id) != 0 {
        return res;
    }
    mu_update_control(ctx, id, base, opt);
    if (*ctx).focus == id && (*ctx).mouse_down == MU_MOUSE_LEFT as libc::c_int {
        *value += (*ctx).mouse_delta.x as libc::c_float * step;
    }
    if *value != last {
        res |= MU_RES_CHANGE as libc::c_int;
    }
    mu_draw_control_frame(ctx, id, base, MU_COLOR_BASE as libc::c_int, opt);
    sprintf(buf.as_mut_ptr(), fmt, *value as libc::c_double);
    mu_draw_control_text(ctx, buf.as_mut_ptr(), base, MU_COLOR_TEXT as libc::c_int, opt);
    return res;
}
unsafe extern "C" fn header(
    mut ctx: *mut mu_Context,
    mut label: *const libc::c_char,
    mut istreenode: libc::c_int,
    mut opt: libc::c_int,
) -> libc::c_int {
    let mut r: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut active: libc::c_int = 0;
    let mut expanded: libc::c_int = 0;
    let mut id: mu_Id = mu_get_id(
        ctx,
        label as *const libc::c_void,
        strlen(label) as libc::c_int,
    );
    let mut idx: libc::c_int = mu_pool_get(
        ctx,
        ((*ctx).treenode_pool).as_mut_ptr(),
        48 as libc::c_int,
        id,
    );
    let mut width: libc::c_int = -(1 as libc::c_int);
    mu_layout_row(ctx, 1 as libc::c_int, &mut width, 0 as libc::c_int);
    active = (idx >= 0 as libc::c_int) as libc::c_int;
    expanded = if opt & MU_OPT_EXPANDED as libc::c_int != 0 {
        (active == 0) as libc::c_int
    } else {
        active
    };
    r = mu_layout_next(ctx);
    mu_update_control(ctx, id, r, 0 as libc::c_int);
    active
        ^= ((*ctx).mouse_pressed == MU_MOUSE_LEFT as libc::c_int && (*ctx).focus == id)
            as libc::c_int;
    if idx >= 0 as libc::c_int {
        if active != 0 {
            mu_pool_update(ctx, ((*ctx).treenode_pool).as_mut_ptr(), idx);
        } else {
            memset(
                &mut *((*ctx).treenode_pool).as_mut_ptr().offset(idx as isize)
                    as *mut mu_PoolItem as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<mu_PoolItem>() as libc::c_ulong,
            );
        }
    } else if active != 0 {
        mu_pool_init(ctx, ((*ctx).treenode_pool).as_mut_ptr(), 48 as libc::c_int, id);
    }
    if istreenode != 0 {
        if (*ctx).hover == id {
            ((*ctx).draw_frame)
                .expect(
                    "non-null function pointer",
                )(ctx, r, MU_COLOR_BUTTONHOVER as libc::c_int);
        }
    } else {
        mu_draw_control_frame(
            ctx,
            id,
            r,
            MU_COLOR_BUTTON as libc::c_int,
            0 as libc::c_int,
        );
    }
    mu_draw_icon(
        ctx,
        if expanded != 0 {
            MU_ICON_EXPANDED as libc::c_int
        } else {
            MU_ICON_COLLAPSED as libc::c_int
        },
        mu_rect(r.x, r.y, r.h, r.h),
        (*(*ctx).style).colors[MU_COLOR_TEXT as libc::c_int as usize],
    );
    r.x += r.h - (*(*ctx).style).padding;
    r.w -= r.h - (*(*ctx).style).padding;
    mu_draw_control_text(ctx, label, r, MU_COLOR_TEXT as libc::c_int, 0 as libc::c_int);
    return if expanded != 0 { MU_RES_ACTIVE as libc::c_int } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn mu_header_ex(
    mut ctx: *mut mu_Context,
    mut label: *const libc::c_char,
    mut opt: libc::c_int,
) -> libc::c_int {
    return header(ctx, label, 0 as libc::c_int, opt);
}
#[no_mangle]
pub unsafe extern "C" fn mu_begin_treenode_ex(
    mut ctx: *mut mu_Context,
    mut label: *const libc::c_char,
    mut opt: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = header(ctx, label, 1 as libc::c_int, opt);
    if res & MU_RES_ACTIVE as libc::c_int != 0 {
        (*get_layout(ctx)).indent += (*(*ctx).style).indent;
        assert!((*ctx).id_stack.idx
            < (::core::mem::size_of::<[mu_Id; 32]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<mu_Id>() as libc::c_ulong)
                as libc::c_int);
        (*ctx).id_stack.items[(*ctx).id_stack.idx as usize] = (*ctx).last_id;
        (*ctx).id_stack.idx += 1;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mu_end_treenode(mut ctx: *mut mu_Context) {
    (*get_layout(ctx)).indent -= (*(*ctx).style).indent;
    mu_pop_id(ctx);
}
unsafe extern "C" fn scrollbars(
    mut ctx: *mut mu_Context,
    mut cnt: *mut mu_Container,
    mut body: *mut mu_Rect,
) {
    let mut sz: libc::c_int = (*(*ctx).style).scrollbar_size;
    let mut cs: mu_Vec2 = (*cnt).content_size;
    cs.x += (*(*ctx).style).padding * 2 as libc::c_int;
    cs.y += (*(*ctx).style).padding * 2 as libc::c_int;
    mu_push_clip_rect(ctx, *body);
    if cs.y > (*cnt).body.h {
        (*body).w -= sz;
    }
    if cs.x > (*cnt).body.w {
        (*body).h -= sz;
    }
    let mut maxscroll: libc::c_int = cs.y - (*body).h;
    if maxscroll > 0 as libc::c_int && (*body).h > 0 as libc::c_int {
        let mut base: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
        let mut thumb: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
        let mut id: mu_Id = mu_get_id(
            ctx,
            b"!scrollbary\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            11 as libc::c_int,
        );
        base = *body;
        base.x = (*body).x + (*body).w;
        base.w = (*(*ctx).style).scrollbar_size;
        mu_update_control(ctx, id, base, 0 as libc::c_int);
        if (*ctx).focus == id && (*ctx).mouse_down == MU_MOUSE_LEFT as libc::c_int {
            (*cnt).scroll.y += (*ctx).mouse_delta.y * cs.y / base.h;
        }
        (*cnt)
            .scroll
            .y = if maxscroll
            < (if 0 as libc::c_int > (*cnt).scroll.y {
                0 as libc::c_int
            } else {
                (*cnt).scroll.y
            })
        {
            maxscroll
        } else if 0 as libc::c_int > (*cnt).scroll.y {
            0 as libc::c_int
        } else {
            (*cnt).scroll.y
        };
        ((*ctx).draw_frame)
            .expect(
                "non-null function pointer",
            )(ctx, base, MU_COLOR_SCROLLBASE as libc::c_int);
        thumb = base;
        thumb
            .h = if (*(*ctx).style).thumb_size > base.h * (*body).h / cs.y {
            (*(*ctx).style).thumb_size
        } else {
            base.h * (*body).h / cs.y
        };
        thumb.y += (*cnt).scroll.y * (base.h - thumb.h) / maxscroll;
        ((*ctx).draw_frame)
            .expect(
                "non-null function pointer",
            )(ctx, thumb, MU_COLOR_SCROLLTHUMB as libc::c_int);
        if mu_mouse_over(ctx, *body) != 0 {
            (*ctx).scroll_target = cnt;
        }
    } else {
        (*cnt).scroll.y = 0 as libc::c_int;
    }
    let mut maxscroll_0: libc::c_int = cs.x - (*body).w;
    if maxscroll_0 > 0 as libc::c_int && (*body).w > 0 as libc::c_int {
        let mut base_0: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
        let mut thumb_0: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
        let mut id_0: mu_Id = mu_get_id(
            ctx,
            b"!scrollbarx\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            11 as libc::c_int,
        );
        base_0 = *body;
        base_0.y = (*body).y + (*body).h;
        base_0.h = (*(*ctx).style).scrollbar_size;
        mu_update_control(ctx, id_0, base_0, 0 as libc::c_int);
        if (*ctx).focus == id_0 && (*ctx).mouse_down == MU_MOUSE_LEFT as libc::c_int {
            (*cnt).scroll.x += (*ctx).mouse_delta.x * cs.x / base_0.w;
        }
        (*cnt)
            .scroll
            .x = if maxscroll_0
            < (if 0 as libc::c_int > (*cnt).scroll.x {
                0 as libc::c_int
            } else {
                (*cnt).scroll.x
            })
        {
            maxscroll_0
        } else if 0 as libc::c_int > (*cnt).scroll.x {
            0 as libc::c_int
        } else {
            (*cnt).scroll.x
        };
        ((*ctx).draw_frame)
            .expect(
                "non-null function pointer",
            )(ctx, base_0, MU_COLOR_SCROLLBASE as libc::c_int);
        thumb_0 = base_0;
        thumb_0
            .w = if (*(*ctx).style).thumb_size > base_0.w * (*body).w / cs.x {
            (*(*ctx).style).thumb_size
        } else {
            base_0.w * (*body).w / cs.x
        };
        thumb_0.x += (*cnt).scroll.x * (base_0.w - thumb_0.w) / maxscroll_0;
        ((*ctx).draw_frame)
            .expect(
                "non-null function pointer",
            )(ctx, thumb_0, MU_COLOR_SCROLLTHUMB as libc::c_int);
        if mu_mouse_over(ctx, *body) != 0 {
            (*ctx).scroll_target = cnt;
        }
    } else {
        (*cnt).scroll.x = 0 as libc::c_int;
    }
    mu_pop_clip_rect(ctx);
}
unsafe extern "C" fn push_container_body(
    mut ctx: *mut mu_Context,
    mut cnt: *mut mu_Container,
    mut body: mu_Rect,
    mut opt: libc::c_int,
) {
    if !opt & MU_OPT_NOSCROLL as libc::c_int != 0 {
        scrollbars(ctx, cnt, &mut body);
    }
    push_layout(ctx, expand_rect(body, -(*(*ctx).style).padding), (*cnt).scroll);
    (*cnt).body = body;
}
unsafe extern "C" fn begin_root_container(
    mut ctx: *mut mu_Context,
    mut cnt: *mut mu_Container,
) {
    assert!((*ctx).container_stack.idx
        < (::core::mem::size_of::<[*mut mu_Container; 32]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut mu_Container>() as libc::c_ulong)
            as libc::c_int);
    (*ctx).container_stack.items[(*ctx).container_stack.idx as usize] = cnt;
    (*ctx).container_stack.idx += 1;
    assert!((*ctx).root_list.idx
        < (::core::mem::size_of::<[*mut mu_Container; 32]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut mu_Container>() as libc::c_ulong)
            as libc::c_int);
    (*ctx).root_list.items[(*ctx).root_list.idx as usize] = cnt;
    (*ctx).root_list.idx += 1;
    (*cnt).head_idx = push_jump(ctx, -(1 as libc::c_int));
    if rect_overlaps_vec2((*cnt).rect, (*ctx).mouse_pos) != 0
        && (((*ctx).next_hover_root).is_null()
            || (*cnt).zindex > (*(*ctx).next_hover_root).zindex)
    {
        (*ctx).next_hover_root = cnt;
    }
    assert!((*ctx).clip_stack.idx
        < (::core::mem::size_of::<[mu_Rect; 32]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<mu_Rect>() as libc::c_ulong)
            as libc::c_int);
    (*ctx).clip_stack.items[(*ctx).clip_stack.idx as usize] = unclipped_rect;
    (*ctx).clip_stack.idx += 1;
}
unsafe extern "C" fn end_root_container(mut ctx: *mut mu_Context) {
    let mut cnt: *mut mu_Container = mu_get_current_container(ctx);
    (*cnt).tail_idx = push_jump(ctx, -(1 as libc::c_int));
    (*ctx)
        .command_list
        .items[(*cnt).head_idx as usize]
        .jump
        .dst_idx = (*ctx).command_list.idx;
    mu_pop_clip_rect(ctx);
    pop_container(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn mu_begin_window_ex(
    mut ctx: *mut mu_Context,
    mut title: *const libc::c_char,
    mut rect: mu_Rect,
    mut opt: libc::c_int,
) -> libc::c_int {
    let mut body: mu_Rect = mu_Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut id: mu_Id = mu_get_id(
        ctx,
        title as *const libc::c_void,
        strlen(title) as libc::c_int,
    );
    let mut cnt: *mut mu_Container = get_container(ctx, id, opt);
    if cnt.is_null() || (*cnt).open == 0 {
        return 0 as libc::c_int;
    }
    assert!((*ctx).id_stack.idx
        < (::core::mem::size_of::<[mu_Id; 32]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<mu_Id>() as libc::c_ulong)
            as libc::c_int);
    (*ctx).id_stack.items[(*ctx).id_stack.idx as usize] = id;
    (*ctx).id_stack.idx += 1;
    if (*cnt).rect.w == 0 as libc::c_int {
        (*cnt).rect = rect;
    }
    begin_root_container(ctx, cnt);
    body = (*cnt).rect;
    rect = body;
    if !opt & MU_OPT_NOFRAME as libc::c_int != 0 {
        ((*ctx).draw_frame)
            .expect(
                "non-null function pointer",
            )(ctx, rect, MU_COLOR_WINDOWBG as libc::c_int);
    }
    if !opt & MU_OPT_NOTITLE as libc::c_int != 0 {
        let mut tr: mu_Rect = rect;
        tr.h = (*(*ctx).style).title_height;
        ((*ctx).draw_frame)
            .expect(
                "non-null function pointer",
            )(ctx, tr, MU_COLOR_TITLEBG as libc::c_int);
        if !opt & MU_OPT_NOTITLE as libc::c_int != 0 {
            let mut id_0: mu_Id = mu_get_id(
                ctx,
                b"!title\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                6 as libc::c_int,
            );
            mu_update_control(ctx, id_0, tr, opt);
            mu_draw_control_text(ctx, title, tr, MU_COLOR_TITLETEXT as libc::c_int, opt);
            if id_0 == (*ctx).focus && (*ctx).mouse_down == MU_MOUSE_LEFT as libc::c_int
            {
                (*cnt).rect.x += (*ctx).mouse_delta.x;
                (*cnt).rect.y += (*ctx).mouse_delta.y;
            }
            body.y += tr.h;
            body.h -= tr.h;
        }
        if !opt & MU_OPT_NOCLOSE as libc::c_int != 0 {
            let mut id_1: mu_Id = mu_get_id(
                ctx,
                b"!close\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                6 as libc::c_int,
            );
            let mut r: mu_Rect = mu_rect(tr.x + tr.w - tr.h, tr.y, tr.h, tr.h);
            tr.w -= r.w;
            mu_draw_icon(
                ctx,
                MU_ICON_CLOSE as libc::c_int,
                r,
                (*(*ctx).style).colors[MU_COLOR_TITLETEXT as libc::c_int as usize],
            );
            mu_update_control(ctx, id_1, r, opt);
            if (*ctx).mouse_pressed == MU_MOUSE_LEFT as libc::c_int
                && id_1 == (*ctx).focus
            {
                (*cnt).open = 0 as libc::c_int;
            }
        }
    }
    push_container_body(ctx, cnt, body, opt);
    if !opt & MU_OPT_NORESIZE as libc::c_int != 0 {
        let mut sz: libc::c_int = (*(*ctx).style).title_height;
        let mut id_2: mu_Id = mu_get_id(
            ctx,
            b"!resize\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            7 as libc::c_int,
        );
        let mut r_0: mu_Rect = mu_rect(
            rect.x + rect.w - sz,
            rect.y + rect.h - sz,
            sz,
            sz,
        );
        mu_update_control(ctx, id_2, r_0, opt);
        if id_2 == (*ctx).focus && (*ctx).mouse_down == MU_MOUSE_LEFT as libc::c_int {
            (*cnt)
                .rect
                .w = if 96 as libc::c_int > (*cnt).rect.w + (*ctx).mouse_delta.x {
                96 as libc::c_int
            } else {
                (*cnt).rect.w + (*ctx).mouse_delta.x
            };
            (*cnt)
                .rect
                .h = if 64 as libc::c_int > (*cnt).rect.h + (*ctx).mouse_delta.y {
                64 as libc::c_int
            } else {
                (*cnt).rect.h + (*ctx).mouse_delta.y
            };
        }
    }
    if opt & MU_OPT_AUTOSIZE as libc::c_int != 0 {
        let mut r_1: mu_Rect = (*get_layout(ctx)).body;
        (*cnt).rect.w = (*cnt).content_size.x + ((*cnt).rect.w - r_1.w);
        (*cnt).rect.h = (*cnt).content_size.y + ((*cnt).rect.h - r_1.h);
    }
    if opt & MU_OPT_POPUP as libc::c_int != 0 && (*ctx).mouse_pressed != 0
        && (*ctx).hover_root != cnt
    {
        (*cnt).open = 0 as libc::c_int;
    }
    mu_push_clip_rect(ctx, (*cnt).body);
    return MU_RES_ACTIVE as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mu_end_window(mut ctx: *mut mu_Context) {
    mu_pop_clip_rect(ctx);
    end_root_container(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn mu_open_popup(
    mut ctx: *mut mu_Context,
    mut name: *const libc::c_char,
) {
    let mut cnt: *mut mu_Container = mu_get_container(ctx, name);
    (*ctx).next_hover_root = cnt;
    (*ctx).hover_root = (*ctx).next_hover_root;
    (*cnt)
        .rect = mu_rect(
        (*ctx).mouse_pos.x,
        (*ctx).mouse_pos.y,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    (*cnt).open = 1 as libc::c_int;
    mu_bring_to_front(ctx, cnt);
}
#[no_mangle]
pub unsafe extern "C" fn mu_begin_popup(
    mut ctx: *mut mu_Context,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut opt: libc::c_int = MU_OPT_POPUP as libc::c_int
        | MU_OPT_AUTOSIZE as libc::c_int | MU_OPT_NORESIZE as libc::c_int
        | MU_OPT_NOSCROLL as libc::c_int | MU_OPT_NOTITLE as libc::c_int
        | MU_OPT_CLOSED as libc::c_int;
    return mu_begin_window_ex(
        ctx,
        name,
        mu_rect(0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int),
        opt,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mu_end_popup(mut ctx: *mut mu_Context) {
    mu_end_window(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn mu_begin_panel_ex(
    mut ctx: *mut mu_Context,
    mut name: *const libc::c_char,
    mut opt: libc::c_int,
) {
    let mut cnt: *mut mu_Container = 0 as *mut mu_Container;
    mu_push_id(ctx, name as *const libc::c_void, strlen(name) as libc::c_int);
    cnt = get_container(ctx, (*ctx).last_id, opt);
    (*cnt).rect = mu_layout_next(ctx);
    if !opt & MU_OPT_NOFRAME as libc::c_int != 0 {
        ((*ctx).draw_frame)
            .expect(
                "non-null function pointer",
            )(ctx, (*cnt).rect, MU_COLOR_PANELBG as libc::c_int);
    }
    assert!((*ctx).container_stack.idx
        < (::core::mem::size_of::<[*mut mu_Container; 32]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut mu_Container>() as libc::c_ulong)
            as libc::c_int);
    (*ctx).container_stack.items[(*ctx).container_stack.idx as usize] = cnt;
    (*ctx).container_stack.idx += 1;
    push_container_body(ctx, cnt, (*cnt).rect, opt);
    mu_push_clip_rect(ctx, (*cnt).body);
}
#[no_mangle]
pub unsafe extern "C" fn mu_end_panel(mut ctx: *mut mu_Context) {
    mu_pop_clip_rect(ctx);
    pop_container(ctx);
}
