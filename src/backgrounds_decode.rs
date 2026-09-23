use gba_emulator::memory::memory_map::{MemoryMap, PALETTE_RAM_START, PALETTE_RAM_SIZE};
use gba_emulator::memory::lcd_io_registers::PixelFormat;
use gba_emulator::gpu::gpu::GPU;
use gba_emulator::gpu::rgb15::Rgb15;

pub const GBA_WIDTH: usize = 240;
pub const GBA_HEIGHT: usize = 160;

pub fn decode_background(gpu: &GPU, mem_map: &MemoryMap, bg_number: usize, mode: u8) -> Vec<Rgb15> {
    if mode >= 3 {
        return decode_bitmap(gpu, mem_map, mode);
    }

    let is_affine = (mode == 1 && bg_number == 2) || (mode == 2 && (bg_number == 2 || bg_number == 3));
    if is_affine {
        decode_affine_text(gpu, mem_map, bg_number)
    } else {
        decode_text_mode(gpu, mem_map, bg_number)
    }
}

pub fn decode_text_mode(gpu: &GPU, mem_map: &MemoryMap, bg_number: usize) -> Vec<Rgb15> {
    let mut frame = vec![Rgb15::new(0x8000); GBA_WIDTH * GBA_HEIGHT];
    let background = &gpu.backgrounds[bg_number];
    let (vertical_offset, horizontal_offset) = background.get_offsets();
    let tileset_location = background.control.get_tileset_location();
    let tilemap_location = background.control.get_tilemap_location();
    let (background_width, background_height) = background.control.get_background_dimensions();
    let pixel_format = background.control.get_pixel_format();
    let tile_size = background.control.get_tilesize();

    for screen_y in 0..GBA_HEIGHT as u32 {
        let background_y = (screen_y + vertical_offset) % background_height;
        for screen_x in 0..GBA_WIDTH as u32 {
            let background_x = (screen_x + horizontal_offset) % background_width;

            let mut sbb: u32 = 0;
            if background_width == 512 && background_height == 256 {
                sbb = background_x / 256;
            } else if background_width == 256 && background_height == 512 {
                sbb = background_y / 256;
            } else if background_width == 512 && background_height == 512 {
                sbb = 2 * (background_y / 256) + (background_x / 256);
            }

            let se_row = (background_x / 8) % 32;
            let se_column = (background_y / 8) % 32;
            let tile_px = background_x % 8;
            let tile_py = background_y % 8;

            let map_address = tilemap_location + 0x800 * sbb + 2 * (32 * se_column + se_row);
            let entry = read_u16(mem_map, map_address);
            let tile_index = (entry & 0x3FF) as u32;
            let horizontal_flip = ((entry >> 10) & 1) != 0;
            let vertical_flip = ((entry >> 11) & 1) != 0;
            let palette_bank = (entry >> 12) as u32;

            let pixel_x = if horizontal_flip { 7 - tile_px } else { tile_px };
            let pixel_y = if vertical_flip { 7 - tile_py } else { tile_py };
            let tile_address = tileset_location + tile_index * tile_size;

            let color = sample_tile_pixel(mem_map, &pixel_format, tile_address, pixel_x, pixel_y, palette_bank);
            frame[(screen_y as usize) * GBA_WIDTH + (screen_x as usize)] = color;
        }
    }
    frame
}

pub fn decode_affine_text(gpu: &GPU, mem_map: &MemoryMap, bg_number: usize) -> Vec<Rgb15> {
    let mut frame = vec![Rgb15::new(0x8000); GBA_WIDTH * GBA_HEIGHT];
    let component_index = bg_number - 2;
    let component = &gpu.bg_affine_components[component_index];
    let texture_size = 128i32 << gpu.backgrounds[bg_number].control.get_screen_size();
    let wraparound = gpu.backgrounds[bg_number].control.get_display_area_overflow();
    let screen_block = gpu.backgrounds[bg_number].control.get_tilemap_location();
    let char_block = gpu.backgrounds[bg_number].control.get_tileset_location();

    let pa = i32::from(&component.rotation_scaling_param_a);
    let pb = i32::from(&component.rotation_scaling_param_b);
    let pc = i32::from(&component.rotation_scaling_param_c);
    let pd = i32::from(&component.rotation_scaling_param_d);

    let mut row_ref_x = component.refrence_point_x_external.get_register() as i32;
    let mut row_ref_y = component.refrence_point_y_external.get_register() as i32;
    row_ref_x = sign_extend_28(row_ref_x);
    row_ref_y = sign_extend_28(row_ref_y);

    for screen_y in 0..GBA_HEIGHT {
        for screen_x in 0..GBA_WIDTH as i32 {
            let mut point_x = (row_ref_x + screen_x * pa) >> 8;
            let mut point_y = (row_ref_y + screen_x * pc) >> 8;

            if !(point_x >= 0 && point_x < texture_size && point_y >= 0 && point_y < texture_size) {
                if wraparound != 0 {
                    point_x = point_x.rem_euclid(texture_size);
                    point_y = point_y.rem_euclid(texture_size);
                } else {
                    continue;
                }
            }

            let map_addr = screen_block + ((texture_size as u32 / 8) * (point_y as u32 / 8) + (point_x as u32 / 8));
            let tile_index = mem_map.memory[map_addr as usize].get() as u32;
            let tile_addr = char_block + tile_index * 0x40;
            let pixel_index_address = tile_addr + (8 * ((point_y % 8) as u32) + ((point_x % 8) as u32));
            let pixel_index = mem_map.memory[pixel_index_address as usize].get() as u32;

            let color = if pixel_index == 0 {
                Rgb15::new(0x8000)
            } else {
                read_palette(mem_map, 2 * pixel_index)
            };
            frame[screen_y * GBA_WIDTH + (screen_x as usize)] = color;
        }
        row_ref_x += pb;
        row_ref_y += pd;
    }
    frame
}

pub fn decode_bitmap(gpu: &GPU, mem_map: &MemoryMap, mode: u8) -> Vec<Rgb15> {
    let mut frame = vec![Rgb15::new(0x8000); GBA_WIDTH * GBA_HEIGHT];
    let component = &gpu.bg_affine_components[0];
    let pa = i32::from(&component.rotation_scaling_param_a);
    let pb = i32::from(&component.rotation_scaling_param_b);
    let pc = i32::from(&component.rotation_scaling_param_c);
    let pd = i32::from(&component.rotation_scaling_param_d);

    let mut row_ref_x = sign_extend_28(component.refrence_point_x_external.get_register() as i32);
    let mut row_ref_y = sign_extend_28(component.refrence_point_y_external.get_register() as i32);

    let (width, height) = if mode == 5 { (160, 128) } else { (240, 160) };
    let page_ofs: u32 = if mode == 3 {
        0x0600_0000
    } else {
        match gpu.display_control.get_display_frame_select() {
            0 => 0x0600_0000,
            _ => 0x0600_A000,
        }
    };

    for screen_y in 0..height {
        for screen_x in 0..width {
            let pixel_x = (row_ref_x + (screen_x as i32) * pa) >> 8;
            let pixel_y = (row_ref_y + (screen_x as i32) * pc) >> 8;
            if pixel_x < 0 || pixel_x >= width || pixel_y < 0 || pixel_y >= height {
                continue;
            }

            let color = match mode {
                3 => {
                    let bitmap_index = (width as u32) * (pixel_y as u32) + (pixel_x as u32);
                    read_u16_raw(mem_map, page_ofs + 2 * bitmap_index)
                }
                4 => {
                    let bitmap_index = (width as u32) * (pixel_y as u32) + (pixel_x as u32);
                    let index = mem_map.memory[(page_ofs + bitmap_index) as usize].get() as u32;
                    if index == 0 { Rgb15::new(0x8000) } else { read_palette(mem_map, 2 * index) }
                }
                _ => {
                    let bitmap_index = (width as u32) * (pixel_y as u32) + (pixel_x as u32);
                    read_u16_raw(mem_map, page_ofs + 2 * bitmap_index)
                }
            };
            frame[(screen_y as usize) * GBA_WIDTH + (screen_x as usize)] = color;
        }
        row_ref_x += pb;
        row_ref_y += pd;
    }
    frame
}

fn sample_tile_pixel(mem_map: &MemoryMap, pixel_format: &PixelFormat, tile_address: u32, pixel_x: u32, pixel_y: u32, palette_bank: u32) -> Rgb15 {
    let (pixel_index, bank) = match pixel_format {
        PixelFormat::EightBit => {
            let addr = tile_address + (8 * pixel_y + pixel_x);
            (mem_map.memory[addr as usize].get() as u32, 0)
        }
        PixelFormat::FourBit => {
            let addr = tile_address + (4 * pixel_y + (pixel_x / 2));
            let value = mem_map.memory[addr as usize].get();
            let raw = if pixel_x & 1 != 0 { value >> 4 } else { value & 0xf };
            (raw as u32, palette_bank)
        }
    };

    if pixel_index == 0 || (bank != 0 && pixel_index % 16 == 0) {
        Rgb15::new(0x8000)
    } else {
        read_palette(mem_map, 2 * pixel_index + 0x20 * bank)
    }
}

fn read_palette(mem_map: &MemoryMap, palette_ram_index: u32) -> Rgb15 {
    let raw_addr = palette_ram_index + 0x0500_0000;
    let masked_addr = (raw_addr & PALETTE_RAM_SIZE) + PALETTE_RAM_START;
    Rgb15::new(read_u16(mem_map, masked_addr))
}

fn read_u16(mem_map: &MemoryMap, addr: u32) -> u16 {
    let idx = addr as usize;
    u16::from_le_bytes([mem_map.memory[idx].get(), mem_map.memory[idx + 1].get()])
}

fn read_u16_raw(mem_map: &MemoryMap, addr: u32) -> Rgb15 {
    Rgb15::new(read_u16(mem_map, addr))
}

fn sign_extend_28(value: i32) -> i32 {
    (value << 4) >> 4
}

#[cfg(test)]
mod tests {
    use super::*;
    use gba_emulator::gamepak::GamePack;
    use gba_emulator::gba::GBA;

    fn write_u16(gba: &GBA, addr: u32, value: u16) {
        let bytes = value.to_le_bytes();
        gba.memory_bus.mem_map.memory[addr as usize].set(bytes[0]);
        gba.memory_bus.mem_map.memory[addr as usize + 1].set(bytes[1]);
    }

    #[test]
    fn decode_text_mode_reads_tile_and_palette_at_origin() {
        let game_pack = GamePack::default();
        let mut gba = GBA::new(0x08000000, &game_pack);

        gba.gpu.backgrounds[0].control.set_character_base_block(0);
        gba.gpu.backgrounds[0].control.set_screen_base_block(8);
        gba.gpu.backgrounds[0].control.set_colors(0);
        gba.gpu.backgrounds[0].control.set_screen_size(0);
        gba.gpu.backgrounds[0].horizontal_offset.set_offset(0);
        gba.gpu.backgrounds[0].vertical_offset.set_offset(0);

        let tilemap_location = gba.gpu.backgrounds[0].control.get_tilemap_location();
        write_u16(&gba, tilemap_location, 1);

        let tileset_location = gba.gpu.backgrounds[0].control.get_tileset_location();
        let tile_1_address = tileset_location + 0x20;
        gba.memory_bus.mem_map.memory[tile_1_address as usize].set(0x01);

        write_u16(&gba, 0x0500_0000 + 2, 0x1234);

        let frame = decode_text_mode(&gba.gpu, &gba.memory_bus.mem_map, 0);

        assert_eq!(frame[0].value, 0x1234);
    }

    #[test]
    fn decode_text_mode_leaves_untouched_pixels_transparent() {
        let game_pack = GamePack::default();
        let mut gba = GBA::new(0x08000000, &game_pack);

        gba.gpu.backgrounds[0].control.set_character_base_block(0);
        gba.gpu.backgrounds[0].control.set_screen_base_block(8);
        gba.gpu.backgrounds[0].control.set_colors(0);
        gba.gpu.backgrounds[0].control.set_screen_size(0);

        let frame = decode_text_mode(&gba.gpu, &gba.memory_bus.mem_map, 0);

        assert!(frame[0].is_transparent());
    }
}
