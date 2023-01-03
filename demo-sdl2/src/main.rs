extern crate sdl2;
mod renderer;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use crate::renderer::Renderer;
use microui::*;

pub fn r_get_char_width(_font: FontId, c: char) -> usize {
    ATLAS[ATLAS_FONT as usize + c as usize].w as usize
}

pub fn r_get_font_height(_font: FontId) -> usize {
    18
}

struct State<'a> {
    label_colors: [LabelColor<'a>; 15],
    bg: [Real; 3],
    logbuf: FixedString<65536>,
    logbuf_updated: bool,
    submit_buf: FixedString<128>,
    ctx: microui::Context,
    checks: [bool; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LabelColor<'a> {
    pub label: &'a str,
    pub idx: ControlColor,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        let mut ctx = microui::Context::new();

        ctx.char_width = Some(r_get_char_width);
        ctx.font_height = Some(r_get_font_height);

        Self {
            label_colors: [
                LabelColor { label: "text", idx: ControlColor::Text },
                LabelColor {
                    label: "border:",
                    idx: ControlColor::Border,
                },
                LabelColor {
                    label: "windowbg:",
                    idx: ControlColor::WindowBG,
                },
                LabelColor {
                    label: "titlebg:",
                    idx: ControlColor::TitleBG,
                },
                LabelColor {
                    label: "titletext:",
                    idx: ControlColor::TitleText,
                },
                LabelColor {
                    label: "panelbg:",
                    idx: ControlColor::PanelBG,
                },
                LabelColor {
                    label: "button:",
                    idx: ControlColor::Button,
                },
                LabelColor {
                    label: "buttonhover:",
                    idx: ControlColor::ButtonHover,
                },
                LabelColor {
                    label: "buttonfocus:",
                    idx: ControlColor::ButtonFocus,
                },
                LabelColor { label: "base:", idx: ControlColor::Base },
                LabelColor {
                    label: "basehover:",
                    idx: ControlColor::BaseHover,
                },
                LabelColor {
                    label: "basefocus:",
                    idx: ControlColor::BaseFocus,
                },
                LabelColor {
                    label: "scrollbase:",
                    idx: ControlColor::ScrollBase,
                },
                LabelColor {
                    label: "scrollthumb:",
                    idx: ControlColor::ScrollThumb,
                },
                LabelColor { label: "", idx: ControlColor::Text },
            ],
            bg: [90.0, 95.0, 100.0],
            logbuf: FixedString::new(),
            logbuf_updated: false,
            submit_buf: FixedString::new(),
            ctx,
            checks: [false, true, false],
        }
    }

    fn write_log(&mut self, text: &str) {
        if self.logbuf.len() != 0 {
            self.logbuf.push('\n');
        }
        for c in text.chars() {
            self.logbuf.push(c);
        }
        self.logbuf_updated = true;
    }

    fn test_window(&mut self) {
        if !self.ctx.begin_window_ex("Demo Window", rect(40, 40, 300, 450), WidgetOption::NONE).is_none() {
            let mut win = self.ctx.get_current_container_rect();
            win.w = if win.w > 240 { win.w } else { 240 };
            win.h = if win.h > 300 { win.h } else { 300 };

            self.ctx.set_current_container_rect(&win);

            let mut buff = FixedString::<128>::new();

            if !self.ctx.header_ex("Window Info", WidgetOption::NONE).is_none() {
                let win_0 = self.ctx.get_current_container_rect();
                self.ctx.layout_row(&[54, -1], 0);
                self.ctx.label("Position:");

                buff.clear();
                buff.append_int("%d", win_0.x);
                buff.add_str(", ");
                buff.append_int("%d", win_0.y);

                self.ctx.label(buff.as_str());
                buff.clear();
                self.ctx.label("Size:");

                buff.append_int("%d", win_0.w);
                buff.add_str(", ");
                buff.append_int("%d", win_0.h);

                self.ctx.label(buff.as_str());
            }
            if !self.ctx.header_ex("Test Buttons", WidgetOption::EXPANDED).is_none() {
                self.ctx.layout_row(&[86, -110, -1], 0);
                self.ctx.label("Test buttons 1:");
                if !self.ctx.button_ex("Button 1", Icon::None, WidgetOption::ALIGN_CENTER).is_none() {
                    self.write_log("Pressed button 1");
                }
                if !self.ctx.button_ex("Button 2", Icon::None, WidgetOption::ALIGN_CENTER).is_none() {
                    self.write_log("Pressed button 2");
                }
                self.ctx.label("Test buttons 2:");
                if !self.ctx.button_ex("Button 3", Icon::None, WidgetOption::ALIGN_CENTER).is_none() {
                    self.write_log("Pressed button 3");
                }
                if !self.ctx.button_ex("Popup", Icon::None, WidgetOption::ALIGN_CENTER).is_none() {
                    self.ctx.open_popup("Test Popup");
                }
                if !self.ctx.begin_popup("Test Popup").is_none() {
                    if !self.ctx.button_ex("Hello", Icon::None, WidgetOption::ALIGN_CENTER).is_none() {
                        self.write_log("Hello")
                    }
                    if !self.ctx.button_ex("World", Icon::None, WidgetOption::ALIGN_CENTER).is_none() {
                        self.write_log("World")
                    }
                    self.ctx.end_popup();
                }
            }
            if !self.ctx.header_ex("Tree and Text", WidgetOption::EXPANDED).is_none() {
                self.ctx.layout_row(&[140, -1], 0);
                self.ctx.layout_begin_column();
                if !self.ctx.begin_treenode_ex("Test 1", WidgetOption::NONE).is_none() {
                    if !self.ctx.begin_treenode_ex("Test 1a", WidgetOption::NONE).is_none() {
                        self.ctx.label("Hello");
                        self.ctx.label("world");
                        self.ctx.end_treenode();
                    }
                    if !self.ctx.begin_treenode_ex("Test 1b", WidgetOption::NONE).is_none() {
                        if !self.ctx.button_ex("Button 1", Icon::None, WidgetOption::ALIGN_CENTER).is_none() {
                            self.write_log("Pressed button 1");
                        }
                        if !self.ctx.button_ex("Button 2", Icon::None, WidgetOption::ALIGN_CENTER).is_none() {
                            self.write_log("Pressed button 2");
                        }
                        self.ctx.end_treenode();
                    }
                    self.ctx.end_treenode();
                }
                if !self.ctx.begin_treenode_ex("Test 2", WidgetOption::NONE).is_none() {
                    self.ctx.layout_row(&[54, 54], 0);
                    if !self.ctx.button_ex("Button 3", Icon::None, WidgetOption::ALIGN_CENTER).is_none() {
                        self.write_log("Pressed button 3");
                    }
                    if !self.ctx.button_ex("Button 4", Icon::None, WidgetOption::ALIGN_CENTER).is_none() {
                        self.write_log("Pressed button 4");
                    }
                    if !self.ctx.button_ex("Button 5", Icon::None, WidgetOption::ALIGN_CENTER).is_none() {
                        self.write_log("Pressed button 5");
                    }
                    if !self.ctx.button_ex("Button 6", Icon::None, WidgetOption::ALIGN_CENTER).is_none() {
                        self.write_log("Pressed button 6");
                    }
                    self.ctx.end_treenode();
                }
                if !self.ctx.begin_treenode_ex("Test 3", WidgetOption::NONE).is_none() {
                    self.ctx.checkbox("Checkbox 1", &mut self.checks[0]);
                    self.ctx.checkbox("Checkbox 2", &mut self.checks[1]);
                    self.ctx.checkbox("Checkbox 3", &mut self.checks[2]);
                    self.ctx.end_treenode();
                }
                self.ctx.layout_end_column();
                self.ctx.layout_begin_column();
                self.ctx.layout_row(&[-1], 0);
                self.ctx.text(
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas lacinia, sem eu lacinia molestie, mi risus faucibus ipsum, eu varius magna felis a nulla."
                    ,
                );
                self.ctx.layout_end_column();
            }
            if !self.ctx.header_ex("Background Color", WidgetOption::EXPANDED).is_none() {
                self.ctx.layout_row(&[-78, -1], 74);
                self.ctx.layout_begin_column();
                self.ctx.layout_row(&[46, -1], 0);
                self.ctx.label("Red:");
                self.ctx
                    .slider_ex(&mut self.bg[0], 0 as Real, 255 as Real, 0 as Real, "%.2", WidgetOption::ALIGN_CENTER);
                self.ctx.label("Green:");
                self.ctx
                    .slider_ex(&mut self.bg[1], 0 as Real, 255 as Real, 0 as Real, "%.2", WidgetOption::ALIGN_CENTER);
                self.ctx.label("Blue:");
                self.ctx
                    .slider_ex(&mut self.bg[2], 0 as Real, 255 as Real, 0 as Real, "%.2", WidgetOption::ALIGN_CENTER);
                self.ctx.layout_end_column();
                let r: Rect = self.ctx.layout_next();
                self.ctx.draw_rect(r, color(self.bg[0] as u8, self.bg[1] as u8, self.bg[2] as u8, 255));
                let mut buff = FixedString::<128>::new();
                buff.add_str("#");
                buff.append_int("%02X", self.bg[0] as _);
                buff.append_int("%02X", self.bg[1] as _);
                buff.append_int("%02X", self.bg[2] as _);
                self.ctx.draw_control_text(buff.as_str(), r, ControlColor::Text, WidgetOption::ALIGN_CENTER);
            }
            self.ctx.end_window();
        }
    }

    fn log_window(&mut self) {
        if !self.ctx.begin_window_ex("Log Window", rect(350, 40, 300, 200), WidgetOption::NONE).is_none() {
            self.ctx.layout_row(&[-1], -25);
            self.ctx.begin_panel_ex("Log Output", WidgetOption::NONE);
            let mut scroll = self.ctx.get_current_container_scroll();
            let content_size = self.ctx.get_current_container_content_size();
            self.ctx.layout_row(&[-1], -1);
            self.ctx.text(self.logbuf.as_str());
            if self.logbuf_updated {
                scroll.y = content_size.y;
                self.ctx.set_current_container_scroll(&scroll);
                self.logbuf_updated = false;
            }
            self.ctx.end_panel();

            let mut submitted = false;
            self.ctx.layout_row(&[-70, -1], 0);
            if self.ctx.textbox_ex(&mut self.submit_buf, WidgetOption::NONE).is_submitted() {
                self.ctx.set_focus(self.ctx.last_id);
                submitted = true;
            }
            if !self.ctx.button_ex("Submit", Icon::None, WidgetOption::ALIGN_CENTER).is_none() {
                submitted = true;
            }
            if submitted {
                let mut buf = FixedString::<128>::new();
                buf.add_str(self.submit_buf.as_str());
                self.write_log(buf.as_str());
                self.submit_buf.clear();
            }
            self.ctx.end_window();
        }
    }
    fn uint8_slider(&mut self, value: &mut u8, low: i32, high: i32) -> ResourceState {
        let mut tmp = *value as f32;
        self.ctx.push_id_from_ptr(value);
        let res = self
            .ctx
            .slider_ex(&mut tmp, low as Real, high as Real, 0 as Real, "%.2f", WidgetOption::ALIGN_CENTER);
        *value = tmp as u8;
        self.ctx.pop_id();
        return res;
    }
    fn style_window(&mut self) {
        if !self.ctx.begin_window_ex("Style Editor", rect(350, 250, 300, 240), WidgetOption::NONE).is_none() {
            let sw = (self.ctx.get_current_container_body().w as f64 * 0.14) as i32;
            self.ctx.layout_row(&[80, sw, sw, sw, sw, -1], 0);
            let mut i = 0;
            while self.label_colors[i].label.len() > 0 {
                self.ctx.label(self.label_colors[i].label);
                unsafe {
                    let color = self.ctx.style.colors.as_mut_ptr().offset(i as isize);
                    self.uint8_slider(&mut (*color).r, 0, 255);
                    self.uint8_slider(&mut (*color).g, 0, 255);
                    self.uint8_slider(&mut (*color).b, 0, 255);
                    self.uint8_slider(&mut (*color).a, 0, 255);
                }
                let next_layout = self.ctx.layout_next();
                self.ctx.draw_rect(next_layout, self.ctx.style.colors[i]);
                i += 1;
            }
            self.ctx.end_window();
        }
    }

    fn process_frame(&mut self) {
        self.ctx.begin();
        self.style_window();
        self.log_window();
        self.test_window();
        self.ctx.end();
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video_subsystem.window("Window", 800, 600).opengl().build().unwrap();

    // Unlike the other example above, nobody created a context for your window, so you need to create one.

    // TODO: the rust compiler optimizes this out
    let _x_ = window.gl_create_context().unwrap();
    let gl = unsafe { glow::Context::from_loader_function(|s| video_subsystem.gl_get_proc_address(s) as *const _) };

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::GLES);
    debug_assert_eq!(gl_attr.context_version(), (3, 0));

    let mut event_pump = sdl_context.event_pump().unwrap();
    let (width, height) = window.size();
    let mut rd = Renderer::new(&gl, &microui::ATLAS_TEXTURE, width, height);

    let mut state = State::new();

    'running: loop {
        let (width, height) = window.size();

        rd.clear(
            &gl,
            width as i32,
            height as i32,
            color(state.bg[0] as u8, state.bg[1] as u8, state.bg[2] as u8, 255),
        );

        fn map_mouse_button(sdl_mb: sdl2::mouse::MouseButton) -> microui::MouseButton {
            match sdl_mb {
                sdl2::mouse::MouseButton::Left => microui::MouseButton::LEFT,
                sdl2::mouse::MouseButton::Right => microui::MouseButton::RIGHT,
                sdl2::mouse::MouseButton::Middle => microui::MouseButton::MIDDLE,
                _ => microui::MouseButton::NONE,
            }
        }

        fn map_keymode(sdl_km: sdl2::keyboard::Mod, sdl_kc: Option<sdl2::keyboard::Keycode>) -> microui::KeyMode {
            match (sdl_km, sdl_kc) {
                (sdl2::keyboard::Mod::LALTMOD, _) | (sdl2::keyboard::Mod::RALTMOD, _) => microui::KeyMode::ALT,
                (sdl2::keyboard::Mod::LCTRLMOD, _) | (sdl2::keyboard::Mod::RCTRLMOD, _) => microui::KeyMode::CTRL,
                (sdl2::keyboard::Mod::LSHIFTMOD, _) | (sdl2::keyboard::Mod::RSHIFTMOD, _) => microui::KeyMode::SHIFT,
                (_, Some(sdl2::keyboard::Keycode::Backspace)) => microui::KeyMode::BACKSPACE,
                (_, Some(sdl2::keyboard::Keycode::Return)) => microui::KeyMode::RETURN,
                _ => microui::KeyMode::NONE,
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::Window { win_event: WindowEvent::Close, .. } => break 'running,
                Event::MouseMotion { x, y, .. } => state.ctx.input_mousemove(x, y),
                Event::MouseWheel { y, .. } => state.ctx.input_scroll(0, y * -30),
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    let mb = map_mouse_button(mouse_btn);
                    state.ctx.input_mousedown(x, y, mb);
                }
                Event::MouseButtonUp { x, y, mouse_btn, .. } => {
                    let mb = map_mouse_button(mouse_btn);
                    state.ctx.input_mouseup(x, y, mb);
                }
                Event::KeyDown { keymod, keycode, .. } => {
                    let km = map_keymode(keymod, keycode);
                    state.ctx.input_keydown(km);
                }
                Event::KeyUp { keymod, keycode, .. } => {
                    let km = map_keymode(keymod, keycode);
                    state.ctx.input_keyup(km);
                }
                Event::TextInput { text, .. } => {
                    state.ctx.input_text(text.as_str());
                }

                _ => {}
            }
        }

        state.process_frame();

        let mut cmd_id = 0;
        loop {
            match state.ctx.mu_next_command(cmd_id) {
                Some((command, id)) => {
                    match command {
                        Command::Text { str_start, str_len, pos, color, .. } => {
                            let str = &state.ctx.text_stack[str_start..str_start + str_len];
                            rd.draw_text(&gl, str, pos, color);
                        }
                        Command::Rect { rect, color } => {
                            rd.draw_rect(&gl, rect, color);
                        }
                        Command::Icon { id, rect, color } => {
                            rd.draw_icon(&gl, id, rect, color);
                        }
                        Command::Clip { rect } => {
                            rd.set_clip_rect(&gl, 800, 600, rect);
                        }
                        _ => {}
                    }
                    cmd_id = id;
                }
                None => break,
            }
        }

        rd.flush(&gl);
        window.gl_swap_window();

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
