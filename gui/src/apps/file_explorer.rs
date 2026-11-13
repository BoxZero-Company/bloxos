
use crate::renderer::framebuffer::Framebuffer;
use crate::renderer::font::draw_text;
use crate::renderer::primitives::Rect;

const BG: u32 = 0xFF08090D;
const PANEL: u32 = 0xFF17191F;
const SURFACE: u32 = 0xFF22252D;
const HOVER: u32 = 0xFF292D38;
const GREEN: u32 = 0xFFB7FF36;
const PURPLE: u32 = 0xFF9A6BFF;
const WHITE: u32 = 0xFFF2F4F8;
const MUTED: u32 = 0xFF858B98;
const BORDER: u32 = 0xFF343943;

pub const MAX_VISIBLE_ENTRIES: usize = 12;

const HEADER_HEIGHT: usize = 38;
const NAV_HEIGHT: usize = 38;
const COLUMN_HEIGHT: usize = 28;
const ROW_HEIGHT: usize = 32;
const START_Y: usize = 106;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
}

#[derive(Clone, Copy)]
pub struct ExplorerEntry<'a> {
    pub name: &'a str,
    pub kind: EntryKind,
    pub size: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExplorerAction {
    None,
    OpenDirectory(usize),
    OpenFile(usize),
    Back,
}

pub struct FileExplorer {
    pub selected: Option<usize>,
    pub scroll: usize,
    pub width: usize,
    pub height: usize,
}

impl FileExplorer {
    pub const fn new(width: usize, height: usize) -> Self {
        Self {
            selected: None,
            scroll: 0,
            width,
            height,
        }
    }

    fn rect(
        fb: &mut Framebuffer,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) {
        fb.rectangle(x, y, width, height, color);
    }

    fn text(
        fb: &mut Framebuffer,
        x: usize,
        y: usize,
        value: &str,
        color: u32,
    ) {
        draw_text(fb, x, y, value, color, 1);
    }

    pub fn render(
        &self,
        fb: &mut Framebuffer,
        entries: &[ExplorerEntry<'_>],
        current_path: &str,
    ) {
        let width = fb.width.min(self.width);
        let height = fb.height.min(self.height);

        if width < 180 || height < 130 {
            return;
        }

        // Background
        // Note: this version is for a standalone framebuffer.
        // Do not use full-screen clear when rendering inside Desktop.
        fb.clear(BG);

        // Header
        Self::rect(fb, 0, 0, width, HEADER_HEIGHT, PANEL);
        Self::rect(fb, 0, 36, width, 2, PURPLE);

        Self::text(fb, 14, 15, "BLOXOS FILE EXPLORER", GREEN);

        // Navigation bar
        Self::rect(fb, 0, HEADER_HEIGHT, width, NAV_HEIGHT, SURFACE);

        Self::rect(fb, 10, 45, 30, 25, HOVER);
        Self::text(fb, 20, 54, "<", WHITE);

        Self::rect(
            fb,
            48,
            45,
            width.saturating_sub(60),
            25,
            PANEL,
        );

        Self::text(fb, 58, 54, current_path, WHITE);

        // Column header
        Self::rect(
            fb,
            0,
            HEADER_HEIGHT + NAV_HEIGHT,
            width,
            COLUMN_HEIGHT,
            PANEL,
        );

        Self::text(fb, 16, 87, "NAME", MUTED);
        Self::text(fb, width.saturating_sub(100), 87, "TYPE", MUTED);

        // Entries
        let visible = entries
            .len()
            .saturating_sub(self.scroll)
            .min(MAX_VISIBLE_ENTRIES);

        for visible_index in 0..visible {
            let index = self.scroll + visible_index;
            let entry = entries[index];
            let y = START_Y + visible_index * ROW_HEIGHT;

            if y + ROW_HEIGHT > height.saturating_sub(27) {
                break;
            }

            // Selected row
            if self.selected == Some(index) {
                Self::rect(
                    fb,
                    8,
                    y,
                    width.saturating_sub(16),
                    29,
                    HOVER,
                );

                Self::rect(fb, 8, y, 3, 29, GREEN);
            }

            // Entry icon and name
            match entry.kind {
                EntryKind::Directory => {
                    Self::rect(fb, 19, y + 7, 17, 13, GREEN);
                    Self::text(fb, 45, y + 10, entry.name, WHITE);
                    Self::text(
                        fb,
                        width.saturating_sub(100),
                        y + 10,
                        "FOLDER",
                        GREEN,
                    );
                }

                EntryKind::File => {
                    Self::rect(fb, 22, y + 5, 12, 17, PURPLE);
                    Self::text(fb, 45, y + 10, entry.name, WHITE);
                    Self::text(
                        fb,
                        width.saturating_sub(100),
                        y + 10,
                        "FILE",
                        MUTED,
                    );
                }
            }
        }

        // Empty directory
        if entries.is_empty() {
            Self::text(
                fb,
                20,
                145,
                "THIS DIRECTORY IS EMPTY",
                MUTED,
            );
        }

        // Footer
        let footer_y = height.saturating_sub(27);

        Self::rect(
            fb,
            0,
            footer_y,
            width,
            27,
            PANEL,
        );

        Self::rect(
            fb,
            0,
            footer_y,
            width,
            1,
            BORDER,
        );

        Self::text(
            fb,
            12,
            footer_y + 10,
            "BLOXOS STORAGE",
            GREEN,
        );

        Self::text(
            fb,
            width.saturating_sub(100),
            footer_y + 10,
            "HORIZON",
            MUTED,
        );
    }

    pub fn handle_click(
        &mut self,
        x: usize,
        y: usize,
        entries: &[ExplorerEntry<'_>],
    ) -> ExplorerAction {
        // Back button
        if (Rect {
            x: 10,
            y: 45,
            width: 30,
            height: 25,
        })
        .contains(x, y)
        {
            self.selected = None;
            return ExplorerAction::Back;
        }

        // Ignore clicks outside entry area
        if y < START_Y {
            return ExplorerAction::None;
        }

        let visible_index = (y - START_Y) / ROW_HEIGHT;

        if visible_index >= MAX_VISIBLE_ENTRIES {
            return ExplorerAction::None;
        }

        let index = self.scroll + visible_index;

        if index >= entries.len() {
            return ExplorerAction::None;
        }

        self.selected = Some(index);

        match entries[index].kind {
            EntryKind::Directory => {
                ExplorerAction::OpenDirectory(index)
            }

            EntryKind::File => {
                ExplorerAction::OpenFile(index)
            }
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected = None;
    }

    pub fn scroll_down(&mut self, entry_count: usize) {
        if self.scroll + MAX_VISIBLE_ENTRIES < entry_count {
            self.scroll += 1;
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn reset_scroll(&mut self) {
        self.scroll = 0;
        self.selected = None;
    }

    pub fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }
}
