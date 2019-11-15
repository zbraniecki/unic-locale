pub const CHARACTER_DIRECTION_RTL: [u64; 6] = [24934, 25715, 25960, 29281, 29301, 29552];
pub fn is_rtl(subtag: u64) -> bool {
    CHARACTER_DIRECTION_RTL.binary_search(&subtag).is_ok()
}
