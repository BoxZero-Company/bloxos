
#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::read_unaligned;

use bloxos_gui::desktop::{
    Desktop,
    FILE_EXPLORER_APP_ID,
};

use bloxos_gui::renderer::framebuffer::{
    Framebuffer,
    PixelFormat,
};

// BloxOS Application Framework
#[path = "../apps/mod.rs"]
mod apps;

const MULTIBOOT2_BOOT_MAGIC: u32 = 0x36D76289;
const MULTIBOOT2_FRAMEBUFFER_TAG: u32 = 8;
const DEBUG_PORT: u16 = 0xE9;

#[inline(always)]
unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;

    asm!(
        "in al, dx",
        in("dx") port,
        out("al") value,
        options(nomem, nostack, preserves_flags)
    );

    value
}

fn debug_byte(value: u8) {
    unsafe {
        outb(DEBUG_PORT, value);
    }
}

fn debug_text(text: &[u8]) {
    for &byte in text {
        debug_byte(byte);
    }
}

// ============================================================
// FRAMEBUFFER INFORMATION
// ============================================================

#[derive(Clone, Copy)]
struct FramebufferInfo {
    address: u64,
    pitch: u32,
    width: u32,
    height: u32,
    bpp: u8,
    pixel_type: u8,

    red_position: u8,
    red_size: u8,

    green_position: u8,
    green_size: u8,

    blue_position: u8,
    blue_size: u8,
}

impl FramebufferInfo {
    fn valid(&self) -> bool {
        if self.address == 0
            || self.width == 0
            || self.height == 0
            || self.bpp != 32
            || self.pixel_type != 1
        {
            return false;
        }

        let minimum_pitch = self.width.saturating_mul(4);

        if self.pitch < minimum_pitch || self.pitch % 4 != 0 {
            return false;
        }

        let end = self.address.saturating_add(
            (self.pitch as u64)
                .saturating_mul(self.height as u64)
        );

        end > self.address && end <= 0x1_0000_0000
    }
}

fn find_framebuffer(
    info_address: u64,
) -> Option<FramebufferInfo> {
    if info_address == 0 {
        return None;
    }

    unsafe {
        let base = info_address as *const u8;

        let total_size =
            read_unaligned(base as *const u32) as usize;

        if total_size < 16 || total_size > 1024 * 1024 {
            return None;
        }

        let mut offset = 8usize;

        while offset + 8 <= total_size {
            let tag = base.add(offset);

            let tag_type =
                read_unaligned(tag as *const u32);

            let tag_size =
                read_unaligned(tag.add(4) as *const u32)
                    as usize;

            if tag_size < 8
                || offset.saturating_add(tag_size) > total_size
            {
                return None;
            }

            if tag_type == MULTIBOOT2_FRAMEBUFFER_TAG {
                if tag_size < 32 {
                    return None;
                }

                let address =
                    read_unaligned(tag.add(8) as *const u64);

                let pitch =
                    read_unaligned(tag.add(16) as *const u32);

                let width =
                    read_unaligned(tag.add(20) as *const u32);

                let height =
                    read_unaligned(tag.add(24) as *const u32);

                let bpp = *tag.add(28);
                let pixel_type = *tag.add(29);

                let mut red_position = 0;
                let mut red_size = 0;

                let mut green_position = 0;
                let mut green_size = 0;

                let mut blue_position = 0;
                let mut blue_size = 0;

                if pixel_type == 1 && tag_size >= 38 {
                    red_position = *tag.add(32);
                    red_size = *tag.add(33);

                    green_position = *tag.add(34);
                    green_size = *tag.add(35);

                    blue_position = *tag.add(36);
                    blue_size = *tag.add(37);
                }

                let fb = FramebufferInfo {
                    address,
                    pitch,
                    width,
                    height,
                    bpp,
                    pixel_type,

                    red_position,
                    red_size,

                    green_position,
                    green_size,

                    blue_position,
                    blue_size,
                };

                if fb.valid() {
                    return Some(fb);
                }

                return None;
            }

            if tag_type == 0 {
                break;
            }

            offset = offset.saturating_add(
                (tag_size + 7) & !7
            );
        }
    }

    None
}

// ============================================================
// PS/2 INPUT
// ============================================================

const PS2_DATA: u16 = 0x60;
const PS2_STATUS: u16 = 0x64;
const PS2_COMMAND: u16 = 0x64;

struct MouseState {
    x: usize,
    y: usize,

    width: usize,
    height: usize,

    packet: [u8; 3],
    packet_index: usize,

    left_button: bool,
    right_button: bool,
    middle_button: bool,
}

impl MouseState {
    fn new(width: usize, height: usize) -> Self {
        Self {
            x: width / 2,
            y: height / 2,

            width,
            height,

            packet: [0; 3],
            packet_index: 0,

            left_button: false,
            right_button: false,
            middle_button: false,
        }
    }

    fn update(&mut self, byte: u8) {
        if self.packet_index == 0 && byte & 0x08 == 0 {
            return;
        }

        self.packet[self.packet_index] = byte;
        self.packet_index += 1;

        if self.packet_index < 3 {
            return;
        }

        self.packet_index = 0;

        let flags = self.packet[0];

        let dx = self.packet[1] as i8 as isize;
        let dy = self.packet[2] as i8 as isize;

        self.x = clamp_position(
            self.x as isize + dx,
            self.width,
        );

        self.y = clamp_position(
            self.y as isize - dy,
            self.height,
        );

        self.left_button = flags & 1 != 0;
        self.right_button = flags & 2 != 0;
        self.middle_button = flags & 4 != 0;
    }
}

fn clamp_position(
    value: isize,
    limit: usize,
) -> usize {
    if limit == 0 {
        return 0;
    }

    value.clamp(
        0,
        limit.saturating_sub(1) as isize,
    ) as usize
}

struct KeyboardState {
    last_scancode: u8,
    activity: u64,
}

impl KeyboardState {
    const fn new() -> Self {
        Self {
            last_scancode: 0,
            activity: 0,
        }
    }

    fn update(&mut self, byte: u8) {
        self.last_scancode = byte;
        self.activity = self.activity.wrapping_add(1);
    }
}

unsafe fn wait_ps2_input() {
    for _ in 0..100_000 {
        if inb(PS2_STATUS) & 2 == 0 {
            return;
        }
    }
}

unsafe fn wait_ps2_output() -> bool {
    for _ in 0..100_000 {
        if inb(PS2_STATUS) & 1 != 0 {
            return true;
        }
    }

    false
}

unsafe fn ps2_command(command: u8) {
    wait_ps2_input();
    outb(PS2_COMMAND, command);
}

unsafe fn ps2_write_device(value: u8) {
    wait_ps2_input();
    outb(PS2_COMMAND, 0xD4);

    wait_ps2_input();
    outb(PS2_DATA, value);
}

unsafe fn ps2_read_response() -> Option<u8> {
    if wait_ps2_output() {
        Some(inb(PS2_DATA))
    } else {
        None
    }
}

fn init_ps2() -> bool {
    unsafe {
        ps2_command(0xAD);
        ps2_command(0xA7);

        let _ = ps2_read_response();

        ps2_command(0x20);

        let config = match ps2_read_response() {
            Some(value) => value,
            None => return false,
        };

        ps2_command(0x60);

        wait_ps2_input();
        outb(PS2_DATA, (config | 0x03) & !0x30);

        ps2_command(0xAE);
        ps2_command(0xA8);

        ps2_command(0x20);

        let _ = ps2_read_response();

        ps2_write_device(0xF6);

        if ps2_read_response() != Some(0xFA) {
            return false;
        }

        ps2_write_device(0xF4);

        let mouse_ready =
            ps2_read_response() == Some(0xFA);

        ps2_command(0xAE);

        wait_ps2_input();
        outb(PS2_DATA, 0xF4);

        let keyboard_ready =
            ps2_read_response() == Some(0xFA);

        keyboard_ready && mouse_ready
    }
}

fn poll_input(
    mouse: &mut MouseState,
    keyboard: &mut KeyboardState,
) {
    for _ in 0..32 {
        let status = unsafe {
            inb(PS2_STATUS)
        };

        if status & 1 == 0 {
            break;
        }

        let byte = unsafe {
            inb(PS2_DATA)
        };

        if status & 0x20 != 0 {
            mouse.update(byte);
        } else {
            keyboard.update(byte);
        }
    }
}

// ============================================================
// TIMER - PIT CLOCK
// ============================================================

const PIT_FREQUENCY: u32 = 1_193_182;

const PIT_DIVISOR: u16 =
    (PIT_FREQUENCY / 1000) as u16;

const TARGET_FRAME_US: u64 = 16_667;

fn init_pit() {
    unsafe {
        let divisor = PIT_DIVISOR;

        outb(0x43, 0x34);
        outb(0x40, (divisor & 0xFF) as u8);
        outb(0x40, (divisor >> 8) as u8);
    }
}

fn pit_read_counter() -> u16 {
    unsafe {
        outb(0x43, 0x00);

        let low = inb(0x40);
        let high = inb(0x40);

        u16::from_le_bytes([low, high])
            .clamp(1, PIT_DIVISOR)
    }
}

struct PitClock {
    last_counter: u16,
    fractional: u64,
}

impl PitClock {
    fn new() -> Self {
        Self {
            last_counter: pit_read_counter(),
            fractional: 0,
        }
    }

    fn elapsed_us(&mut self) -> u64 {
        let current = pit_read_counter();

        let elapsed_counts =
            if current <= self.last_counter {
                self.last_counter - current
            } else {
                self.last_counter
                    .saturating_add(PIT_DIVISOR)
                    .saturating_sub(current)
            };

        self.last_counter = current;

        let numerator =
            elapsed_counts as u64 * 1_000_000
                + self.fractional;

        let elapsed =
            numerator / PIT_FREQUENCY as u64;

        self.fractional =
            numerator % PIT_FREQUENCY as u64;

        elapsed
    }
}

// ============================================================
// C APPLICATION SYSTEM
// ============================================================

fn initialize_app_system() {
    debug_byte(b'A');

    if !apps::app::initialize() {
        debug_byte(b'F');
        return;
    }

    let name = apps::app::name();
    let version = apps::app::version();
    let description = apps::app::description();

    let _metadata = (name, version, description);

    debug_byte(b'C');
}

// ============================================================
// APPLICATION MANAGER INITIALIZATION
// ============================================================

fn initialize_application_manager(
    manager: &mut apps::AppManager,
) -> bool {
    let file_explorer_id = match manager.register(
        apps::AppDescriptor {
            name: "FILE EXPLORER",
            version: "1.0.0",
            description: "BloxOS Native File Explorer",
            package_id: "com.boxzero.file-explorer",
        },
    ) {
        Ok(id) => id,
        Err(_) => {
            debug_byte(b'R');
            return false;
        }
    };

    if file_explorer_id != FILE_EXPLORER_APP_ID {
        debug_byte(b'R');
        return false;
    }

    debug_byte(b'B');

    true
}

// ============================================================
// DESKTOP
// ============================================================

fn run_desktop(
    fb: FramebufferInfo,
    app_manager: &mut apps::AppManager,
) -> ! {
    let width = fb.width as usize;
    let height = fb.height as usize;

    let mut mouse = MouseState::new(width, height);
    let mut keyboard = KeyboardState::new();

    let mut desktop = Desktop::new(width, height);

    let pixel_pitch = fb.pitch as usize / 4;

    let buffer_length =
        pixel_pitch.saturating_mul(height);

    let framebuffer_pointer =
        fb.address as *mut u32;

    let format = PixelFormat {
        red_position: fb.red_position,
        red_size: fb.red_size,

        green_position: fb.green_position,
        green_size: fb.green_size,

        blue_position: fb.blue_position,
        blue_size: fb.blue_size,
    };

    let mut previous_x = mouse.x;
    let mut previous_y = mouse.y;

    let mut previous_activity = keyboard.activity;
    let mut previous_left = false;

    let mut redraw = true;

    init_pit();

    let mut clock = PitClock::new();

    let mut animation_accumulator: u64 = 0;
    let mut frame_accumulator: u64 =
        TARGET_FRAME_US;

    loop {
        // ----------------------------------------------------
        // INPUT
        // ----------------------------------------------------

        poll_input(
            &mut mouse,
            &mut keyboard,
        );

        let pointer_changed =
            mouse.x != previous_x
                || mouse.y != previous_y;

        let button_changed =
            mouse.left_button != previous_left;

        if pointer_changed {
            desktop.pointer_move(
                mouse.x as i32,
                mouse.y as i32,
            );

            previous_x = mouse.x;
            previous_y = mouse.y;

            redraw = true;
        }

        if button_changed {
            previous_left = mouse.left_button;
            redraw = true;
        }

        desktop.pointer_button(
            mouse.left_button,
        );

        if keyboard.activity != previous_activity {
            previous_activity = keyboard.activity;

            let code = keyboard.last_scancode;

            // Ignore key-release scancodes.
            if code & 0x80 == 0 {
                desktop.key_down(code);
            }

            redraw = true;
        }

        // ----------------------------------------------------
        // APPLICATION MANAGER SYNCHRONIZATION
        // ----------------------------------------------------

        // Activate the currently visible application.
        if let Some(app_id) = desktop.active_application_id() {
            if app_manager.activate(app_id).is_err() {
                debug_byte(b'E');
            }
        }

        // Notify the manager when a window is closed.
        if let Some(app_id) = desktop.take_closed_application_id() {
            if app_manager.close(app_id).is_err() {
                debug_byte(b'F');
            }
        }

        // ----------------------------------------------------
        // TIME AND ANIMATIONS
        // ----------------------------------------------------

        let elapsed_us = clock.elapsed_us();

        animation_accumulator =
            animation_accumulator.saturating_add(
                elapsed_us
            );

        if animation_accumulator >= 1000 {
            let delta_ms = (
                animation_accumulator / 1000
            )
                .min(u32::MAX as u64) as u32;

            animation_accumulator %= 1000;

            desktop.tick_animations(delta_ms);

            if desktop.animations_active() {
                redraw = true;
            }
        }

        // ----------------------------------------------------
        // FRAME LIMITER AND RENDERING
        // ----------------------------------------------------

        frame_accumulator =
            frame_accumulator.saturating_add(
                elapsed_us
            );

        if frame_accumulator >= TARGET_FRAME_US {
            if redraw {
                let buffer = unsafe {
                    core::slice::from_raw_parts_mut(
                        framebuffer_pointer,
                        buffer_length,
                    )
                };

                let mut renderer =
                    Framebuffer::new_with_format(
                        buffer,
                        width,
                        height,
                        pixel_pitch,
                        format,
                    );

                desktop.render(&mut renderer);

                redraw = false;
            }

            frame_accumulator %= TARGET_FRAME_US;
        }

        core::hint::spin_loop();
    }
}

// ============================================================
// KERNEL ENTRY
// ============================================================

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(
    magic: u32,
    info_address: u64,
) -> ! {
    debug_byte(b'G');

    // Check Multiboot2 magic.
    if magic != MULTIBOOT2_BOOT_MAGIC {
        debug_byte(b'M');

        loop {
            unsafe {
                asm!(
                    "cli",
                    "hlt",
                    options(nomem, nostack)
                );
            }
        }
    }

    debug_byte(b'H');

    // Find framebuffer.
    let framebuffer =
        match find_framebuffer(info_address) {
            Some(value) => value,
            None => {
                debug_byte(b'N');

                loop {
                    unsafe {
                        asm!(
                            "cli",
                            "hlt",
                            options(nomem, nostack)
                        );
                    }
                }
            }
        };

    debug_byte(b'I');

    // Initialize the C application subsystem.
    initialize_app_system();

    // Initialize PS/2 mouse and keyboard.
    if init_ps2() {
        debug_byte(b'P');
    } else {
        debug_byte(b'X');
    }

    // Create the BloxOS Application Manager.
    let mut app_manager = apps::AppManager::new();

    // Register built-in File Explorer.
    if !initialize_application_manager(&mut app_manager) {
        loop {
            unsafe {
                asm!(
                    "cli",
                    "hlt",
                    options(nomem, nostack)
                );
            }
        }
    }

    debug_byte(b'D');

    // Start the desktop.
    run_desktop(
        framebuffer,
        &mut app_manager,
    );
}

// ============================================================
// PANIC HANDLER
// ============================================================

#[panic_handler]
fn panic(
    _info: &PanicInfo,
) -> ! {
    debug_byte(b'!');

    loop {
        unsafe {
            asm!(
                "cli",
                "hlt",
                options(nomem, nostack)
            );
        }
    }
}
