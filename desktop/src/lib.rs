
#![no_std]

use bloxos_gui::renderer::framebuffer::Framebuffer;
use bloxos_gui::gui_rect::Rect;

// =====================================================
// BLOXOS 27H1 HORIZON - DESKTOP ENVIRONMENT
// =====================================================

pub const COLOR_BACKGROUND: u32 = 0xFF08090D;
pub const COLOR_PANEL: u32 = 0xFF17191F;
pub const COLOR_PANEL_LIGHT: u32 = 0xFF22252D;
pub const COLOR_ACCENT: u32 = 0xFFB7FF36;
pub const COLOR_TEXT: u32 = 0xFFF4F5F7;
pub const COLOR_MUTED: u32 = 0xFF9297A3;
pub const COLOR_PURPLE: u32 = 0xFF9A6BFF;
pub const COLOR_BLUE: u32 = 0xFF62B8FF;

pub const TOPBAR_HEIGHT: usize = 34;
pub const TASKBAR_HEIGHT: usize = 58;

// =====================================================
// APPLICATIONS
// =====================================================

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum App {
    None,
    Settings,
    FileManager,
    Terminal,
    Calculator,
    TaskManager,
    Browser,
}

impl App {
    pub fn title(self) -> &'static str {
        match self {
            Self::None => "DESKTOP",
            Self::Settings => "SETTINGS",
            Self::FileManager => "FILES",
            Self::Terminal => "TERMINAL",
            Self::Calculator => "CALCULATOR",
            Self::TaskManager => "TASK MANAGER",
            Self::Browser => "BROWSER",
        }
    }
}

// =====================================================
// CLOCK
// =====================================================

pub struct Clock {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Clock {
    pub const fn new() -> Self {
        Self {
            hour: 12,
            minute: 0,
            second: 0,
        }
    }

    pub fn set(&mut self, hour: u8, minute: u8, second: u8) {
        self.hour = hour.min(23);
        self.minute = minute.min(59);
        self.second = second.min(59);
    }
}

// =====================================================
// SEARCH
// =====================================================

pub struct Search {
    pub opened: bool,
    pub query: [u8; 48],
    pub length: usize,
}

impl Search {
    pub const fn new() -> Self {
        Self {
            opened: false,
            query: [0; 48],
            length: 0,
        }
    }

    pub fn clear(&mut self) {
        self.query = [0; 48];
        self.length = 0;
    }

    pub fn push(&mut self, character: u8) {
        if self.length < self.query.len()
            && character.is_ascii_graphic()
        {
            self.query[self.length] = character.to_ascii_uppercase();
            self.length += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.length > 0 {
            self.length -= 1;
            self.query[self.length] = 0;
        }
    }
}

// =====================================================
// WIDGETS
// =====================================================

pub struct Widgets {
    pub clock_visible: bool,
    pub system_visible: bool,
    pub calendar_visible: bool,
}

impl Widgets {
    pub const fn new() -> Self {
        Self {
            clock_visible: true,
            system_visible: true,
            calendar_visible: false,
        }
    }
}

// =====================================================
// DESKTOP
// =====================================================

pub struct Desktop {
    pub visible: bool,
    pub start_menu_open: bool,
    pub taskbar_visible: bool,
    pub topbar_visible: bool,

    pub active_app: App,

    pub clock: Clock,
    pub search: Search,
    pub widgets: Widgets,

    pub mouse_x: usize,
    pub mouse_y: usize,

    pub wallpaper_style: u8,
}

impl Desktop {
    pub const fn new() -> Self {
        Self {
            visible: false,
            start_menu_open: false,
            taskbar_visible: true,
            topbar_visible: true,

            active_app: App::None,

            clock: Clock::new(),
            search: Search::new(),
            widgets: Widgets::new(),

            mouse_x: 0,
            mouse_y: 0,

            wallpaper_style: 0,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn toggle_start_menu(&mut self) {
        self.start_menu_open = !self.start_menu_open;

        if self.start_menu_open {
            self.search.opened = false;
        }
    }

    pub fn open_search(&mut self) {
        self.search.opened = true;
        self.start_menu_open = false;
        self.search.clear();
    }

    pub fn close_menus(&mut self) {
        self.start_menu_open = false;
        self.search.opened = false;
    }

    pub fn open_app(&mut self, app: App) {
        self.active_app = app;
        self.close_menus();
    }

    pub fn set_clock(
        &mut self,
        hour: u8,
        minute: u8,
        second: u8,
    ) {
        self.clock.set(hour, minute, second);
    }

    pub fn key_character(&mut self, character: u8) {
        if self.search.opened {
            self.search.push(character);
        }
    }

    pub fn key_backspace(&mut self) {
        if self.search.opened {
            self.search.backspace();
        }
    }

    pub fn key_escape(&mut self) {
        self.close_menus();
    }

    pub fn key_enter(&mut self) {
        if self.search.opened && self.search.length > 0 {
            self.open_app(App::None);
        }
    }

    pub fn mouse_move(&mut self, x: usize, y: usize) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    // -------------------------------------------------
    // MOUSE CLICK
    // -------------------------------------------------

    pub fn mouse_click(
        &mut self,
        x: usize,
        y: usize,
        screen_width: usize,
        screen_height: usize,
    ) {
        let taskbar_y = screen_height.saturating_sub(TASKBAR_HEIGHT);

        // START BUTTON
        if y >= taskbar_y
            && x >= 12
            && x <= 58
        {
            self.toggle_start_menu();
            return;
        }

        // SEARCH BUTTON
        if y >= taskbar_y
            && x >= 66
            && x <= 230
        {
            self.open_search();
            return;
        }

        // START MENU
        if self.start_menu_open {
            let menu_x = 16;
            let menu_y = screen_height.saturating_sub(420);
            let menu_width = 340;

            if (Rect {
                x: menu_x,
                y: menu_y,
                width: menu_width,
                height: 400,
            }).contains(x, y) {
                let local_y = y.saturating_sub(menu_y);

                if local_y >= 90 && local_y < 150 {
                    self.open_app(App::Settings);
                } else if local_y >= 150 && local_y < 210 {
                    self.open_app(App::FileManager);
                } else if local_y >= 210 && local_y < 270 {
                    self.open_app(App::Terminal);
                } else if local_y >= 270 && local_y < 330 {
                    self.open_app(App::Calculator);
                } else {
                    self.close_menus();
                }

                return;
            }

            self.close_menus();
            return;
        }

        // SEARCH PANEL
        if self.search.opened {
            let panel_y = screen_height.saturating_sub(470);

            if (Rect {
                x: 16,
                y: panel_y,
                width: 390,
                height: 440,
            }).contains(x, y) {
                return;
            }

            self.close_menus();
            return;
        }

        // WIDGET PANEL
        if self.widgets.calendar_visible {
            self.widgets.calendar_visible = false;
            return;
        }

        // SCREEN BOUNDS
        if x >= screen_width || y >= screen_height {
            return;
        }
    }

    // -------------------------------------------------
    // RENDER
    // -------------------------------------------------

    pub fn render(&self, fb: &mut Framebuffer) {
        if !self.visible {
            return;
        }

        self.render_wallpaper(fb);

        if self.topbar_visible {
            self.render_topbar(fb);
        }

        self.render_widgets(fb);

        if self.taskbar_visible {
            self.render_taskbar(fb);
        }

        if self.start_menu_open {
            self.render_start_menu(fb);
        }

        if self.search.opened {
            self.render_search(fb);
        }

        if self.active_app != App::None {
            self.render_app_window(fb);
        }
    }

    // -------------------------------------------------
    // WALLPAPER
    // -------------------------------------------------

    fn render_wallpaper(&self, fb: &mut Framebuffer) {
        fb.clear(COLOR_BACKGROUND);

        let width = fb.width;
        let height = fb.height;

        for i in 0..height {
            let color = if i < height / 2 {
                COLOR_BACKGROUND
            } else {
                0xFF0D1019
            };

            fb.horizontal_line(0, i, width, color);
        }

        // Decorative accents
        fb.rectangle(
            width.saturating_sub(260),
            120,
            3,
            180,
            COLOR_PURPLE,
        );

        fb.rectangle(
            width.saturating_sub(250),
            120,
            120,
            3,
            COLOR_ACCENT,
        );
    }

    // -------------------------------------------------
    // TOPBAR
    // -------------------------------------------------

    fn render_topbar(&self, fb: &mut Framebuffer) {
        fb.rectangle(
            0,
            0,
            fb.width,
            TOPBAR_HEIGHT,
            COLOR_PANEL,
        );

        fb.rectangle(
            0,
            TOPBAR_HEIGHT - 2,
            fb.width,
            2,
            COLOR_PANEL_LIGHT,
        );

        draw_text(
            fb,
            14,
            11,
            "BLOXOS 27H1 HORIZON",
            COLOR_ACCENT,
            2,
        );

        draw_text(
            fb,
            fb.width.saturating_sub(190),
            11,
            "SYSTEM DESKTOP",
            COLOR_MUTED,
            2,
        );
    }

    // -------------------------------------------------
    // TASKBAR
    // -------------------------------------------------

    fn render_taskbar(&self, fb: &mut Framebuffer) {
        let y = fb.height.saturating_sub(TASKBAR_HEIGHT);

        fb.rectangle(
            0,
            y,
            fb.width,
            TASKBAR_HEIGHT,
            COLOR_PANEL,
        );

        fb.rectangle(
            0,
            y,
            fb.width,
            1,
            COLOR_PANEL_LIGHT,
        );

        // Start icon
        fb.rectangle(
            18,
            y + 17,
            24,
            24,
            COLOR_ACCENT,
        );

        fb.rectangle(
            23,
            y + 22,
            6,
            6,
            COLOR_BACKGROUND,
        );

        fb.rectangle(
            32,
            y + 22,
            6,
            6,
            COLOR_BACKGROUND,
        );

        fb.rectangle(
            23,
            y + 31,
            6,
            6,
            COLOR_BACKGROUND,
        );

        fb.rectangle(
            32,
            y + 31,
            6,
            6,
            COLOR_BACKGROUND,
        );

        // Search button
        fb.rectangle(
            68,
            y + 12,
            150,
            34,
            COLOR_PANEL_LIGHT,
        );

        draw_text(
            fb,
            80,
            y + 24,
            "SEARCH",
            COLOR_MUTED,
            2,
        );

        // Active application
        if self.active_app != App::None {
            fb.rectangle(
                240,
                y + 12,
                110,
                34,
                COLOR_PANEL_LIGHT,
            );

            fb.rectangle(
                240,
                y + 43,
                110,
                3,
                COLOR_ACCENT,
            );

            draw_text(
                fb,
                250,
                y + 24,
                self.active_app.title(),
                COLOR_TEXT,
                1,
            );
        }

        // Clock
        let clock_text = [
            b'0' + self.clock.hour / 10,
            b'0' + self.clock.hour % 10,
            b':',
            b'0' + self.clock.minute / 10,
            b'0' + self.clock.minute % 10,
            0,
        ];

        draw_bytes(
            fb,
            fb.width.saturating_sub(90),
            y + 23,
            &clock_text,
            COLOR_TEXT,
            2,
        );
    }

    // -------------------------------------------------
    // START MENU
    // -------------------------------------------------

    fn render_start_menu(&self, fb: &mut Framebuffer) {
        let x = 16;
        let y = fb.height.saturating_sub(420);

        fb.rectangle(
            x,
            y,
            340,
            400,
            COLOR_PANEL,
        );

        fb.rectangle(
            x,
            y,
            340,
            3,
            COLOR_ACCENT,
        );

        draw_text(
            fb,
            x + 20,
            y + 22,
            "START MENU",
            COLOR_ACCENT,
            2,
        );

        draw_text(
            fb,
            x + 20,
            y + 55,
            "APPLICATIONS",
            COLOR_MUTED,
            1,
        );

        self.draw_menu_item(fb, x, y + 90, "SETTINGS");
        self.draw_menu_item(fb, x, y + 150, "FILE MANAGER");
        self.draw_menu_item(fb, x, y + 210, "TERMINAL");
        self.draw_menu_item(fb, x, y + 270, "CALCULATOR");

        draw_text(
            fb,
            x + 20,
            y + 360,
            "BLOXOS 27H1",
            COLOR_MUTED,
            1,
        );
    }

    fn draw_menu_item(
        &self,
        fb: &mut Framebuffer,
        x: usize,
        y: usize,
        label: &str,
    ) {
        fb.rectangle(
            x + 12,
            y,
            316,
            48,
            COLOR_PANEL_LIGHT,
        );

        fb.rectangle(
            x + 22,
            y + 13,
            20,
            20,
            COLOR_ACCENT,
        );

        draw_text(
            fb,
            x + 56,
            y + 20,
            label,
            COLOR_TEXT,
            1,
        );
    }

    // -------------------------------------------------
    // SEARCH
    // -------------------------------------------------

    fn render_search(&self, fb: &mut Framebuffer) {
        let x = 16;
        let y = fb.height.saturating_sub(470);

        fb.rectangle(
            x,
            y,
            390,
            440,
            COLOR_PANEL,
        );

        fb.rectangle(
            x,
            y,
            390,
            3,
            COLOR_PURPLE,
        );

        draw_text(
            fb,
            x + 20,
            y + 24,
            "SEARCH",
            COLOR_ACCENT,
            2,
        );

        fb.rectangle(
            x + 16,
            y + 65,
            358,
            42,
            COLOR_PANEL_LIGHT,
        );

        draw_text(
            fb,
            x + 28,
            y + 81,
            "TYPE TO SEARCH",
            COLOR_MUTED,
            1,
        );

        draw_bytes(
            fb,
            x + 28,
            y + 81,
            &self.search.query[..self.search.length],
            COLOR_TEXT,
            1,
        );

        draw_text(
            fb,
            x + 20,
            y + 135,
            "APPLICATIONS",
            COLOR_MUTED,
            1,
        );

        draw_text(
            fb,
            x + 20,
            y + 175,
            "SETTINGS",
            COLOR_TEXT,
            1,
        );

        draw_text(
            fb,
            x + 20,
            y + 215,
            "FILE MANAGER",
            COLOR_TEXT,
            1,
        );

        draw_text(
            fb,
            x + 20,
            y + 255,
            "TERMINAL",
            COLOR_TEXT,
            1,
        );
    }

    // -------------------------------------------------
    // WIDGETS
    // -------------------------------------------------

    fn render_widgets(&self, fb: &mut Framebuffer) {
        if self.widgets.clock_visible {
            let x = fb.width.saturating_sub(230);
            let y = 70;

            fb.rectangle(
                x,
                y,
                190,
                90,
                COLOR_PANEL,
            );

            fb.rectangle(
                x,
                y,
                3,
                90,
                COLOR_ACCENT,
            );

            draw_text(
                fb,
                x + 16,
                y + 15,
                "TIME",
                COLOR_MUTED,
                1,
            );

            let clock_text = [
                b'0' + self.clock.hour / 10,
                b'0' + self.clock.hour % 10,
                b':',
                b'0' + self.clock.minute / 10,
                b'0' + self.clock.minute % 10,
                0,
            ];

            draw_bytes(
                fb,
                x + 16,
                y + 42,
                &clock_text,
                COLOR_TEXT,
                3,
            );
        }

        if self.widgets.system_visible {
            let x = fb.width.saturating_sub(230);
            let y = 175;

            fb.rectangle(
                x,
                y,
                190,
                74,
                COLOR_PANEL,
            );

            draw_text(
                fb,
                x + 14,
                y + 14,
                "SYSTEM STATUS",
                COLOR_ACCENT,
                1,
            );

            draw_text(
                fb,
                x + 14,
                y + 42,
                "BLOXOS ACTIVE",
                COLOR_MUTED,
                1,
            );
        }
    }

    // -------------------------------------------------
    // APPLICATION WINDOW
    // -------------------------------------------------

    fn render_app_window(&self, fb: &mut Framebuffer) {
        let width = 540.min(fb.width.saturating_sub(40));
        let height = 340.min(fb.height.saturating_sub(120));

        let x = fb.width.saturating_sub(width) / 2;
        let y = fb.height.saturating_sub(height) / 2;

        fb.rectangle(
            x,
            y,
            width,
            height,
            COLOR_PANEL,
        );

        fb.rectangle(
            x,
            y,
            width,
            38,
            COLOR_PANEL_LIGHT,
        );

        fb.rectangle(
            x,
            y,
            width,
            2,
            COLOR_ACCENT,
        );

        draw_text(
            fb,
            x + 16,
            y + 15,
            self.active_app.title(),
            COLOR_TEXT,
            1,
        );

        // Close button
        fb.rectangle(
            x + width.saturating_sub(34),
            y + 10,
            16,
            16,
            0xFFFF5555,
        );

        draw_text(
            fb,
            x + 20,
            y + 70,
            "BLOXOS APPLICATION",
            COLOR_ACCENT,
            2,
        );

        draw_text(
            fb,
            x + 20,
            y + 110,
            "APPLICATION INTERFACE",
            COLOR_MUTED,
            1,
        );
    }
}

// =====================================================
// SIMPLE BITMAP FONT
// Supports English uppercase letters, numbers and basic
// punctuation. Persian font rendering will be added later.
// =====================================================

fn glyph(c: u8) -> [u8; 7] {
    match c.to_ascii_uppercase() {
        b'A' => [14,17,17,31,17,17,17],
        b'B' => [30,17,17,30,17,17,30],
        b'C' => [14,17,16,16,16,17,14],
        b'D' => [30,17,17,17,17,17,30],
        b'E' => [31,16,16,30,16,16,31],
        b'F' => [31,16,16,30,16,16,16],
        b'G' => [14,17,16,23,17,17,15],
        b'H' => [17,17,17,31,17,17,17],
        b'I' => [14,4,4,4,4,4,14],
        b'J' => [7,2,2,2,18,18,12],
        b'K' => [17,18,20,24,20,18,17],
        b'L' => [16,16,16,16,16,16,31],
        b'M' => [17,27,21,21,17,17,17],
        b'N' => [17,25,21,19,17,17,17],
        b'O' => [14,17,17,17,17,17,14],
        b'P' => [30,17,17,30,16,16,16],
        b'Q' => [14,17,17,17,21,18,13],
        b'R' => [30,17,17,30,20,18,17],
        b'S' => [15,16,16,14,1,1,30],
        b'T' => [31,4,4,4,4,4,4],
        b'U' => [17,17,17,17,17,17,14],
        b'V' => [17,17,17,17,17,10,4],
        b'W' => [17,17,17,21,21,21,10],
        b'X' => [17,17,10,4,10,17,17],
        b'Y' => [17,17,10,4,4,4,4],
        b'Z' => [31,1,2,4,8,16,31],

        b'0' => [14,17,19,21,25,17,14],
        b'1' => [4,12,4,4,4,4,14],
        b'2' => [14,17,1,2,4,8,31],
        b'3' => [30,1,1,14,1,1,30],
        b'4' => [2,6,10,18,31,2,2],
        b'5' => [31,16,16,30,1,1,30],
        b'6' => [14,16,16,30,17,17,14],
        b'7' => [31,1,2,4,8,8,8],
        b'8' => [14,17,17,14,17,17,14],
        b'9' => [14,17,17,15,1,1,14],

        b':' => [0,4,4,0,4,4,0],
        b'-' => [0,0,0,31,0,0,0],
        b'.' => [0,0,0,0,0,12,12],
        b'/' => [1,2,2,4,8,8,16],
        b'_' => [0,0,0,0,0,0,31],
        b' ' => [0; 7],

        _ => [31,17,1,2,4,0,4],
    }
}

fn draw_text(
    fb: &mut Framebuffer,
    x: usize,
    y: usize,
    text: &str,
    color: u32,
    scale: usize,
) {
    draw_bytes(
        fb,
        x,
        y,
        text.as_bytes(),
        color,
        scale,
    );
}

fn draw_bytes(
    fb: &mut Framebuffer,
    x: usize,
    y: usize,
    text: &[u8],
    color: u32,
    scale: usize,
) {
    let scale = scale.max(1);

    for (index, character) in text.iter().enumerate() {
        let pattern = glyph(*character);
        let origin_x = x + index * 6 * scale;

        for row in 0..7 {
            for column in 0..5 {
                if pattern[row] & (1 << (4 - column)) != 0 {
                    fb.rectangle(
                        origin_x + column * scale,
                        y + row * scale,
                        scale,
                        scale,
                        color,
                    );
                }
            }
        }
    }
}
