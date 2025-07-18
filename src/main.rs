mod processor;
mod cpu;
mod memory;
mod opcodes;
mod gpu;
mod util;
mod args;
mod mbc;

use cpu::Cpu;
use gpu::{Ppu, update_ppu};
use memory::{Memory, update_timer};
use processor::{run, handle_interrupts};
use util::INTERRUPT_F_ADDRESS;

use clap::Parser;
use macroquad::prelude::*;

/// this function is useful for debugging purposes
/// any keybindings which I want to use will be done in this function
fn misc_inputs(speed_up_factor: &mut usize) {
    if is_key_down(KeyCode::LeftControl) {
        if is_key_pressed(KeyCode::Left) {
            if *speed_up_factor == 1 {
                return;
            }
            *speed_up_factor -= 1;
        }
        if is_key_pressed(KeyCode::Right) {
            if *speed_up_factor == 5 {
                return;
            }
            *speed_up_factor += 1;
        }
    }
}

pub fn joypad_interrupt(mem: &mut Memory) {
    let mut interrupt = false;
    interrupt |= is_key_pressed(KeyCode::A);
    interrupt |= is_key_pressed(KeyCode::D);
    interrupt |= is_key_pressed(KeyCode::Enter);
    interrupt |= is_key_pressed(KeyCode::Space);
    interrupt |= is_key_pressed(KeyCode::Down);
    interrupt |= is_key_pressed(KeyCode::Left);
    interrupt |= is_key_pressed(KeyCode::Right);
    interrupt |= is_key_pressed(KeyCode::Up);

    if interrupt {
        let interrupt = mem.read(INTERRUPT_F_ADDRESS);
        mem.write(INTERRUPT_F_ADDRESS, interrupt|0b0001_0000);
    }
}

pub fn joypad(joypad: u8) -> u8 {
    let mut upper_joypad = joypad & 0b1111_0000;
    // neither buttons nor d-pad is selected
    if upper_joypad & 0x30 == 0x30 {
        return upper_joypad | 0xF;
    }
    if upper_joypad & 0b0001_00000 == 0 {
        upper_joypad |= (!is_key_down(KeyCode::A) as u8) << 0;
        upper_joypad |= (!is_key_down(KeyCode::D) as u8) << 1;
        upper_joypad |= (!is_key_down(KeyCode::Space) as u8) << 2;
        upper_joypad |= (!is_key_down(KeyCode::Enter) as u8) << 3;
        return upper_joypad;
    }
    upper_joypad |= (!is_key_down(KeyCode::Right) as u8) << 0;
    upper_joypad |= (!is_key_down(KeyCode::Left) as u8) << 1;
    upper_joypad |= (!is_key_down(KeyCode::Up) as u8) << 2;
    upper_joypad |= (!is_key_down(KeyCode::Down) as u8) << 3;
    return upper_joypad;
}

fn serial_output(mem: &mut Memory) {
    if mem.read(0xFF02) == 0x81 {
        let c = mem.read(0xFF01) as char;
        print!("{c}");
        mem.write(0xFF02, 0);
    }
}

fn to_screen_pixel(p: u8) -> Color {
    match p {
        0 => WHITE,
        1 => LIGHTGRAY,
        2 => DARKGRAY,
        3 => BLACK,
        4 => WHITE,
        _ => unreachable!()
    }
}

fn get_rom(rom_path: &String) -> Vec<u8> {
    match std::fs::read(rom_path) {
        Err(e) => panic!("invalid file provided => {e:?}"),
        Ok(f) => f,
    }
}

const SCALE_FACTOR: i32 = 4;
fn window_conf() -> Conf {
    Conf {
        window_title: "gameboy emulator".to_owned(),
        window_resizable: false,
        window_height: 144*SCALE_FACTOR,
        window_width: 160*SCALE_FACTOR,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args = args::Args::parse();
    let rom = get_rom(&args.rom_name);

    let mut cpu = Cpu::new(args.booted);
    let mut memory = Memory::new(rom, args.booted);

    let mut ppu = Ppu::default();

    let mut pixel_buffer: Vec<u8> = Vec::new();
    let mut speed_up_factor = 1;
    'full: loop {
        joypad_interrupt(&mut memory);

        while pixel_buffer.len() != 23040*speed_up_factor {
            // fail-safe for the boot rom
            if cpu.regs.pc == 0x100 && !args.booted {
                break 'full;
            }

            let mut cycles = 4;
            cycles += handle_interrupts(&mut cpu, &mut memory);
            if !cpu.halt {
                // have to account for the previous 4 cycles 
                cycles += run(&mut cpu, &mut memory) - 4;
            }
            update_timer(&mut memory, cycles);
            serial_output(&mut memory);

            if let Some(line) = update_ppu(&mut ppu, &mut memory, cycles) {
                pixel_buffer.extend::<Vec<u8>>(line);
            }
        }
        misc_inputs(&mut speed_up_factor);
        // all of the actual rendering to the screen
        for (j, pixel) in pixel_buffer.iter().enumerate() {
            let pixel = to_screen_pixel(*pixel);
            draw_rectangle(
                ((j%160)*SCALE_FACTOR as usize) as f32, //X-pos
                ((j/160)*SCALE_FACTOR as usize) as f32, //Y-pos
                SCALE_FACTOR as f32, //width
                SCALE_FACTOR as f32, //height
                pixel // color
            );
        }
        next_frame().await;
        pixel_buffer.clear();
    }
}