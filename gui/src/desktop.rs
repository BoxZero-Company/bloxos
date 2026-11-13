
use crate::renderer::framebuffer::Framebuffer;
use crate::renderer::font::draw_text;

const BG: u32 = 0xFF87CEEB;
const SKY_LIGHT: u32 = 0xFFBFE8FF;
const SKY_PALE: u32 = 0xFFDDF4FF;
const SKY_DARK: u32 = 0xFF70B9E0;

const PANEL: u32 = 0xFF17191F;
const PANEL2: u32 = 0xFF22252D;
const SURFACE: u32 = 0xFF1D2028;
const SURFACE_HOVER: u32 = 0xFF292D38;

const GREEN: u32 = 0xFFB7FF36;
const PURPLE: u32 = 0xFF9A6BFF;
const WHITE: u32 = 0xFFF2F4F8;
const MUTED: u32 = 0xFF858B98;
const BORDER: u32 = 0xFF343943;
const RED: u32 = 0xFF703C50;
const SHADOW: u32 = 0xFF050609;
const CURSOR_OUTLINE: u32 = 0xFF101820;

const TASKBAR_HEIGHT: usize = 56;
const TOPBAR_HEIGHT: usize = 32;

const WINDOW_ANIMATION_MS: u32 = 160;
const MENU_ANIMATION_MS: u32 = 130;

// BAF application IDs
pub const FILE_EXPLORER_APP_ID: u32 = 1;

#[derive(Clone, Copy)]
struct Window {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    open: bool,
}

pub struct Desktop {
    width: usize,
    height: usize,

    start_open: bool,
    locked: bool,

    window: Window,
    dragging: bool,

    mouse_x: usize,
    mouse_y: usize,

    previous_left: bool,

    window_progress: u32,
    start_progress: u32,

    // Application Framework integration
    window_app_id: Option<u32>,
    window_title: &'static str,
    last_closed_app_id: Option<u32>,
}

impl Desktop {
    pub fn new(width: usize, height: usize) -> Self {
        let window_width = 420.min(width.saturating_sub(40));

        let window_height = 280.min(
            height.saturating_sub(
                TASKBAR_HEIGHT + TOPBAR_HEIGHT + 40
            )
        );

        Self {
            width,
            height,

            start_open: false,
            locked: false,

            window: Window {
                x: width.saturating_sub(window_width) / 2,
                y: height.saturating_sub(window_height) / 2,
                width: window_width,
                height: window_height,
                open: false,
            },

            dragging: false,

            mouse_x: width / 2,
            mouse_y: height / 2,

            previous_left: false,

            window_progress: 0,
            start_progress: 0,

            window_app_id: None,
            window_title: "FILE EXPLORER",
            last_closed_app_id: None,
        }
    }

    // ========================================================
    // APPLICATION WINDOW MANAGEMENT
    // ========================================================

    pub fn open_application_window(
        &mut self,
        app_id: u32,
        title: &'static str,
    ) {
        self.window_app_id = Some(app_id);
        self.window_title = title;

        self.window.open = true;
        self.dragging = false;
    }

    pub fn close_application_window(&mut self) {
        if !self.window.open {
            return;
        }

        self.last_closed_app_id = self.window_app_id.take();

        self.window.open = false;
        self.dragging = false;
    }

    pub fn active_application_id(&self) -> Option<u32> {
        if self.window.open {
            self.window_app_id
        } else {
            None
        }
    }

    pub fn take_closed_application_id(&mut self) -> Option<u32> {
        self.last_closed_app_id.take()
    }

    fn open_file_explorer(&mut self) {
        self.open_application_window(
            FILE_EXPLORER_APP_ID,
            "FILE EXPLORER",
        );
    }

    // ========================================================
    // GEOMETRY
    // ========================================================

    fn inside(
        x: usize,
        y: usize,
        rx: usize,
        ry: usize,
        rw: usize,
        rh: usize,
    ) -> bool {
        x >= rx
            && y >= ry
            && x < rx.saturating_add(rw)
            && y < ry.saturating_add(rh)
    }

    fn scaled(value: usize, progress: u32) -> usize {
        ((value as u64 * progress as u64) / 1000) as usize
    }

    fn animate_value(
        current: u32,
        target: u32,
        delta_ms: u32,
        duration: u32,
    ) -> u32 {
        if current == target {
            return current;
        }

        let duration = duration.max(1);

        let step = (
            (delta_ms as u64 * 1000) / duration as u64
        )
            .max(1) as u32;

        if current < target {
            current.saturating_add(step).min(target)
        } else {
            current.saturating_sub(step).max(target)
        }
    }

    pub fn tick_animations(&mut self, delta_ms: u32) {
        let window_target = if self.window.open {
            1000
        } else {
            0
        };

        let menu_target = if self.start_open {
            1000
        } else {
            0
        };

        self.window_progress = Self::animate_value(
            self.window_progress,
            window_target,
            delta_ms,
            WINDOW_ANIMATION_MS,
        );

        self.start_progress = Self::animate_value(
            self.start_progress,
            menu_target,
            delta_ms,
            MENU_ANIMATION_MS,
        );
    }

    pub fn animations_active(&self) -> bool {
        let window_target = if self.window.open {
            1000
        } else {
            0
        };

        let menu_target = if self.start_open {
            1000
        } else {
            0
        };

        self.window_progress != window_target
            || self.start_progress != menu_target
    }

    fn round_rect(
        fb: &mut Framebuffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        radius: usize,
        color: u32,
    ) {
        if width == 0 || height == 0 {
            return;
        }

        let r = radius.min(width / 2).min(height / 2);

        if r == 0 {
            fb.rectangle(x, y, width, height, color);
            return;
        }

        for row in 0..height {
            let edge = if row < r {
                r - row
            } else if row >= height - r {
                row - (height - r - 1)
            } else {
                0
            };

            let inset = if edge == 0 {
                0
            } else {
                let horizontal = r * r - edge * edge;
                let mut result = 0;

                while (result + 1) * (result + 1) <= horizontal {
                    result += 1;
                }

                r.saturating_sub(result)
            };

            let draw_width = width.saturating_sub(inset * 2);

            if draw_width > 0 {
                fb.rectangle(
                    x + inset,
                    y + row,
                    draw_width,
                    1,
                    color,
                );
            }
        }
    }

    fn menu_geometry(&self) -> (usize, usize, usize) {
        let bottom = self.height.saturating_sub(
            TASKBAR_HEIGHT + 12
        );

        let progress = if self.start_open && self.start_progress == 0 {
            1000
        } else {
            self.start_progress
        };

        let height = Self::scaled(218, progress);
        let y = bottom.saturating_sub(height);

        (16, y, height)
    }

    // ========================================================
    // POINTER INPUT
    // ========================================================

    pub fn pointer_move(&mut self, x: i32, y: i32) {
        self.mouse_x = (x.max(0) as usize).min(
            self.width.saturating_sub(1)
        );

        self.mouse_y = (y.max(0) as usize).min(
            self.height.saturating_sub(1)
        );

        if self.dragging {
            self.window.x = self.mouse_x.saturating_sub(
                self.window.width / 2
            );

            self.window.y = self.mouse_y.saturating_sub(15);

            self.window.x = self.window.x.min(
                self.width.saturating_sub(self.window.width)
            );

            self.window.y = self.window.y.min(
                self.height.saturating_sub(TASKBAR_HEIGHT + 30)
            );
        }
    }

    pub fn pointer_button(&mut self, left: bool) {
        let pressed = left && !self.previous_left;
        let released = !left && self.previous_left;

        self.previous_left = left;

        if released {
            self.dragging = false;
        }

        if !pressed || self.locked {
            return;
        }

        let x = self.mouse_x;
        let y = self.mouse_y;

        let taskbar_y = self.height.saturating_sub(
            TASKBAR_HEIGHT
        );

        // START button
        if Self::inside(
            x,
            y,
            12,
            taskbar_y + 8,
            90,
            40,
        ) {
            self.start_open = !self.start_open;
            return;
        }

        // START menu
        if self.start_open {
            let (menu_x, menu_y, menu_height) =
                self.menu_geometry();

            // FILE EXPLORER
            if menu_height >= 164
                && Self::inside(
                    x,
                    y,
                    menu_x + 12,
                    menu_y + 62,
                    236,
                    42,
                )
            {
                self.open_file_explorer();

                self.start_open = false;
                self.dragging = false;

                return;
            }

            // Click outside the menu closes it.
            self.start_open = false;
        }

        // Window controls
        if self.window.open {
            let close_x = self.window.x
                + self.window.width
                - 30;

            let close_y = self.window.y + 8;

            // Close button
            if Self::inside(
                x,
                y,
                close_x,
                close_y,
                20,
                20,
            ) {
                self.close_application_window();
                return;
            }

            // Title bar drag
            if Self::inside(
                x,
                y,
                self.window.x,
                self.window.y,
                self.window.width,
                36,
            ) {
                if x < self.window.x + self.window.width - 95 {
                    self.dragging = true;
                }

                return;
            }
        }

        // Desktop icons
        if Self::inside(x, y, 28, 70, 66, 82)
            || Self::inside(x, y, 28, 156, 66, 82)
        {
            self.open_file_explorer();
        }
    }

    // ========================================================
    // KEYBOARD INPUT
    // ========================================================

    pub fn key_down(&mut self, scancode: u8) {
        // L: Lock screen
        if scancode == 0x26 {
            self.locked = true;
            self.start_open = false;
            self.dragging = false;
        }

        // Enter: Unlock
        if self.locked && scancode == 0x1C {
            self.locked = false;
        }

        // Escape: Close Start menu
        if scancode == 0x01 {
            self.start_open = false;
        }

        // E: Open File Explorer
        if !self.locked && scancode == 0x12 {
            self.open_file_explorer();
        }
    }

    // ========================================================
    // CURSOR
    // ========================================================

    fn draw_cursor(&self, fb: &mut Framebuffer) {
        const CURSOR: [&str; 15] = [
            "X...........",
            "XX..........",
            "XWX.........",
            "XWWX........",
            "XWWWX.......",
            "XWWWWX......",
            "XWWWWWX.....",
            "XWWWWWWX....",
            "XWWWWWWWX...",
            "XWWWWXXXX...",
            "XWWWX.......",
            "XWWXX.......",
            "XX.WX.......",
            "X..WWX......",
            "....XX......",
        ];

        for (row, pattern) in CURSOR.iter().enumerate() {
            for (column, pixel) in pattern.bytes().enumerate() {
                let color = match pixel {
                    b'X' => CURSOR_OUTLINE,
                    b'W' => WHITE,
                    b'G' => GREEN,
                    _ => continue,
                };

                fb.rectangle(
                    self.mouse_x.saturating_add(column),
                    self.mouse_y.saturating_add(row),
                    1,
                    1,
                    color,
                );
            }
        }
    }

    // ========================================================
    // WALLPAPER
    // ========================================================

    fn draw_wallpaper(&self, fb: &mut Framebuffer) {
        fb.clear(BG);

        Self::round_rect(
            fb,
            self.width.saturating_sub(190),
            65,
            130,
            38,
            19,
            SKY_LIGHT,
        );

        Self::round_rect(
            fb,
            self.width.saturating_sub(150),
            52,
            100,
            55,
            25,
            SKY_PALE,
        );

        Self::round_rect(
            fb,
            65,
            self.height / 2 + 65,
            150,
            28,
            14,
            SKY_LIGHT,
        );

        Self::round_rect(
            fb,
            95,
            self.height / 2 + 50,
            105,
            45,
            20,
            SKY_PALE,
        );

        fb.rectangle(
            0,
            self.height / 2,
            self.width,
            1,
            SKY_DARK,
        );

        fb.rectangle(
            0,
            self.height / 2 + 70,
            self.width,
            1,
            SKY_LIGHT,
        );

        let center = self.width / 2;

        fb.rectangle(
            center.saturating_sub(130),
            self.height / 2 + 30,
            260,
            2,
            PURPLE,
        );
    }

    // ========================================================
    // TOP BAR
    // ========================================================

    fn draw_topbar(&self, fb: &mut Framebuffer) {
        fb.rectangle(
            0,
            0,
            self.width,
            TOPBAR_HEIGHT,
            PANEL,
        );

        fb.rectangle(
            0,
            TOPBAR_HEIGHT - 2,
            self.width,
            2,
            GREEN,
        );

        draw_text(
            fb,
            14,
            10,
            "BLOXOS 27H1 HORIZON",
            WHITE,
            1,
        );

        draw_text(
            fb,
            self.width.saturating_sub(125),
            10,
            "SYSTEM READY",
            GREEN,
            1,
        );
    }

    // ========================================================
    // TASKBAR
    // ========================================================

    fn draw_taskbar(&self, fb: &mut Framebuffer) {
        let y = self.height.saturating_sub(
            TASKBAR_HEIGHT
        );

        fb.rectangle(
            0,
            y,
            self.width,
            TASKBAR_HEIGHT,
            PANEL,
        );

        fb.rectangle(
            0,
            y,
            self.width,
            2,
            BORDER,
        );

        Self::round_rect(
            fb,
            12,
            y + 8,
            90,
            40,
            7,
            if self.start_open {
                GREEN
            } else {
                PANEL2
            },
        );

        draw_text(
            fb,
            28,
            y + 23,
            "START",
            if self.start_open {
                BG
            } else {
                WHITE
            },
            1,
        );

        if self.window_progress > 0 {
            Self::round_rect(
                fb,
                118,
                y + 8,
                130,
                40,
                6,
                PANEL2,
            );

            fb.rectangle(
                118,
                y + 8,
                3,
                40,
                PURPLE,
            );

            draw_text(
                fb,
                130,
                y + 23,
                self.window_title,
                WHITE,
                1,
            );
        }

        draw_text(
            fb,
            self.width.saturating_sub(85),
            y + 23,
            "HORIZON",
            GREEN,
            1,
        );
    }

    // ========================================================
    // DESKTOP ICONS
    // ========================================================

    fn draw_desktop_icons(&self, fb: &mut Framebuffer) {
        Self::round_rect(
            fb,
            35,
            72,
            42,
            42,
            7,
            PANEL2,
        );

        Self::round_rect(
            fb,
            42,
            79,
            28,
            28,
            5,
            GREEN,
        );

        draw_text(
            fb,
            28,
            122,
            "COMPUTER",
            PANEL,
            1,
        );

        Self::round_rect(
            fb,
            35,
            158,
            42,
            42,
            7,
            PANEL2,
        );

        Self::round_rect(
            fb,
            42,
            165,
            28,
            28,
            5,
            PURPLE,
        );

        draw_text(
            fb,
            28,
            208,
            "FILES",
            PANEL,
            1,
        );
    }

    // ========================================================
    // APPLICATION WINDOW
    // ========================================================

    fn draw_window(&self, fb: &mut Framebuffer) {
        if self.window_progress == 0 {
            return;
        }

        let w = self.window;
        let progress = self.window_progress;

        let width = Self::scaled(w.width, progress);
        let height = Self::scaled(w.height, progress);

        let x = w.x + (w.width - width) / 2;
        let y = w.y + (w.height - height) / 2;

        if width < 80 || height < 50 {
            return;
        }

        Self::round_rect(
            fb,
            x + 5,
            y + 6,
            width,
            height,
            12,
            SHADOW,
        );

        Self::round_rect(
            fb,
            x,
            y,
            width,
            height,
            12,
            PANEL,
        );

        Self::round_rect(
            fb,
            x,
            y,
            width,
            36,
            12,
            SURFACE,
        );

        fb.rectangle(
            x,
            y + 28,
            width,
            8,
            SURFACE,
        );

        fb.rectangle(
            x + 12,
            y,
            width.saturating_sub(24),
            2,
            PURPLE,
        );

        fb.rectangle(
            x + 12,
            y + height - 1,
            width.saturating_sub(24),
            1,
            BORDER,
        );

        // Dynamic application title
        draw_text(
            fb,
            x + 16,
            y + 14,
            self.window_title,
            WHITE,
            1,
        );

        // Minimize
        Self::round_rect(
            fb,
            x + width - 88,
            y + 9,
            20,
            18,
            5,
            PANEL2,
        );

        fb.rectangle(
            x + width - 83,
            y + 20,
            10,
            1,
            WHITE,
        );

        // Maximize
        Self::round_rect(
            fb,
            x + width - 57,
            y + 9,
            20,
            18,
            5,
            PANEL2,
        );

        fb.rectangle(
            x + width - 52,
            y + 13,
            10,
            9,
            GREEN,
        );

        // Close
        Self::round_rect(
            fb,
            x + width - 30,
            y + 8,
            20,
            20,
            5,
            RED,
        );

        draw_text(
            fb,
            x + width - 24,
            y + 14,
            "X",
            WHITE,
            1,
        );

        if progress < 850 || height < 150 {
            return;
        }

        draw_text(
            fb,
            x + 18,
            y + 65,
            "WELCOME TO BLOXOS",
            GREEN,
            1,
        );

        draw_text(
            fb,
            x + 18,
            y + 91,
            "HORIZON FILE MANAGER",
            WHITE,
            1,
        );

        fb.rectangle(
            x + 18,
            y + 116,
            width.saturating_sub(36),
            1,
            BORDER,
        );

        draw_text(
            fb,
            x + 18,
            y + 137,
            "QUICK ACCESS",
            MUTED,
            1,
        );

        let card_y = y + 160;
        let card_width = width.saturating_sub(48) / 2;

        Self::round_rect(
            fb,
            x + 18,
            card_y,
            card_width,
            48,
            7,
            SURFACE,
        );

        Self::round_rect(
            fb,
            x + 24,
            card_y + 9,
            25,
            25,
            5,
            GREEN,
        );

        draw_text(
            fb,
            x + 58,
            card_y + 20,
            "DOCUMENTS",
            WHITE,
            1,
        );

        Self::round_rect(
            fb,
            x + 30 + card_width,
            card_y,
            card_width,
            48,
            7,
            SURFACE,
        );

        Self::round_rect(
            fb,
            x + 36 + card_width,
            card_y + 9,
            25,
            25,
            5,
            PURPLE,
        );

        draw_text(
            fb,
            x + 70 + card_width,
            card_y + 20,
            "SYSTEM",
            WHITE,
            1,
        );
    }

    // ========================================================
    // START MENU
    // ========================================================

    fn draw_start_menu(&self, fb: &mut Framebuffer) {
        if self.start_progress == 0 && !self.start_open {
            return;
        }

        let (x, y, height) = self.menu_geometry();

        if height < 10 {
            return;
        }

        Self::round_rect(
            fb,
            x + 4,
            y + 5,
            260,
            height,
            12,
            SHADOW,
        );

        Self::round_rect(
            fb,
            x,
            y,
            260,
            height,
            12,
            PANEL,
        );

        fb.rectangle(
            x + 12,
            y,
            236,
            3,
            GREEN,
        );

        if self.start_progress < 750 && self.start_progress != 0 {
            return;
        }

        draw_text(
            fb,
            x + 16,
            y + 20,
            "BLOXOS MENU",
            WHITE,
            1,
        );

        fb.rectangle(
            x + 12,
            y + 48,
            236,
            1,
            BORDER,
        );

        Self::round_rect(
            fb,
            x + 12,
            y + 62,
            236,
            42,
            7,
            SURFACE_HOVER,
        );

        draw_text(
            fb,
            x + 24,
            y + 80,
            "FILE EXPLORER",
            GREEN,
            1,
        );

        draw_text(
            fb,
            x + 24,
            y + 130,
            "SYSTEM SETTINGS",
            WHITE,
            1,
        );

        draw_text(
            fb,
            x + 24,
            y + 180,
            "POWER OPTIONS",
            MUTED,
            1,
        );
    }

    // ========================================================
    // LOCK SCREEN
    // ========================================================

    fn draw_lock_screen(&self, fb: &mut Framebuffer) {
        fb.clear(BG);

        let center_x = self.width / 2;

        Self::round_rect(
            fb,
            center_x.saturating_sub(100),
            self.height / 2 - 105,
            200,
            45,
            20,
            SKY_LIGHT,
        );

        fb.rectangle(
            center_x.saturating_sub(80),
            self.height / 2 - 90,
            160,
            3,
            GREEN,
        );

        draw_text(
            fb,
            center_x.saturating_sub(66),
            self.height / 2 - 55,
            "BLOXOS",
            WHITE,
            2,
        );

        draw_text(
            fb,
            center_x.saturating_sub(60),
            self.height / 2 - 10,
            "27H1 HORIZON",
            PURPLE,
            1,
        );

        draw_text(
            fb,
            center_x.saturating_sub(75),
            self.height / 2 + 50,
            "PRESS ENTER TO UNLOCK",
            PANEL,
            1,
        );
    }

    // ========================================================
    // RENDER
    // ========================================================

    pub fn render(&self, fb: &mut Framebuffer) {
        if self.locked {
            self.draw_lock_screen(fb);
            return;
        }

        self.draw_wallpaper(fb);
        self.draw_desktop_icons(fb);
        self.draw_window(fb);
        self.draw_topbar(fb);
        self.draw_taskbar(fb);
        self.draw_start_menu(fb);
        self.draw_cursor(fb);
    }
}
