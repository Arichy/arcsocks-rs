pub fn hex_string(data: &[u8], n: usize) -> String {
    let mut result = "".to_string();
    for byte in &data[0..n] {
        result.push_str(&format!("{:02X} ", byte));
    }
    result
}
