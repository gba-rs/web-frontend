use gba_emulator::gba::GBA;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub const SAVE_STATE_SLOTS: u8 = 9;

pub fn rom_content_key(rom_bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    rom_bytes.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
pub fn save_state_key(rom_bytes: &[u8], slot: u8) -> String {
    save_state_key_from_id(rom_content_key(rom_bytes), slot)
}

#[cfg(test)]
pub fn battery_save_key(rom_bytes: &[u8]) -> String {
    battery_save_key_from_id(rom_content_key(rom_bytes))
}

pub fn save_state_key_from_id(id: u64, slot: u8) -> String {
    format!("save-state:{:x}:slot{}", id, slot)
}

pub fn battery_save_key_from_id(id: u64) -> String {
    format!("battery-save:{:x}", id)
}

pub fn serialize_gba(gba: &GBA) -> Result<Vec<u8>, bincode::Error> {
    bincode::serialize(gba)
}

pub fn deserialize_gba(bytes: &[u8], bios: &Vec<u8>, rom: &Vec<u8>) -> Result<GBA, bincode::Error> {
    let mut gba: GBA = bincode::deserialize(bytes)?;
    gba.register_memory();
    gba.load_bios(bios);
    gba.load_rom(rom);
    Ok(gba)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gba_emulator::gamepak::GamePack;

    #[test]
    fn cached_identifiers_keep_existing_storage_keys() {
        let rom = [1, 2, 3, 4];
        let id = rom_content_key(&rom);
        assert_eq!(battery_save_key(&rom), battery_save_key_from_id(id));
        assert_eq!(save_state_key(&rom, 3), save_state_key_from_id(id, 3));
    }

    #[test]
    fn rom_content_key_is_stable_for_the_same_bytes() {
        let rom = vec![1u8, 2, 3, 4, 5];
        assert_eq!(rom_content_key(&rom), rom_content_key(&rom));
    }

    #[test]
    fn rom_content_key_differs_for_different_bytes() {
        let rom_a = vec![1u8, 2, 3];
        let rom_b = vec![3u8, 2, 1];
        assert_ne!(rom_content_key(&rom_a), rom_content_key(&rom_b));
    }

    #[test]
    fn save_state_key_includes_slot_number() {
        let rom = vec![1u8, 2, 3];
        assert_ne!(save_state_key(&rom, 1), save_state_key(&rom, 2));
    }

    #[test]
    fn round_trip_preserves_cpu_registers_and_pc() {
        let game_pack = GamePack::default();
        let mut original = GBA::new(0x08000000, &game_pack);
        original.cpu.set_register(3, 0xDEADBEEF);
        original.cpu.set_register(7, 0x12345678);

        let bytes = serialize_gba(&original).expect("serialize should succeed");
        let restored = deserialize_gba(&bytes, &game_pack.bios, &game_pack.rom).expect("deserialize should succeed");

        assert_eq!(restored.cpu.get_pc(), original.cpu.get_pc());
        assert_eq!(restored.cpu.get_register_unsafe(3), 0xDEADBEEF);
        assert_eq!(restored.cpu.get_register_unsafe(7), 0x12345678);
    }

    #[test]
    fn round_trip_preserves_key_status() {
        let game_pack = GamePack::default();
        let mut original = GBA::new(0x08000000, &game_pack);
        original.key_status.set_register(0x1234);

        let bytes = serialize_gba(&original).expect("serialize should succeed");
        let restored = deserialize_gba(&bytes, &game_pack.bios, &game_pack.rom).expect("deserialize should succeed");

        assert_eq!(restored.key_status.get_register(), 0x1234);
    }
}
