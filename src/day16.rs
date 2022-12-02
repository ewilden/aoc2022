fn unpack_hex(hex: &str) -> Vec<bool> {
  hex.chars().flat_map(|c| {
    format!("{}", u8::from_str_radix(std::iter::once(c).collect(), 16).unwrap())
  })
}