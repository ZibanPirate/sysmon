pub fn bytes_to_string(bytes: u64, suffix: &str) -> String {
    format!(
        "{} {}",
        match bytes {
            0..=1023 => "00.00 K".to_string(),
            1024..=1_048_575 => format!("{:0>5.2} K", bytes as f64 / 1024.0),
            1_048_576..=1_073_741_824 => format!("{:0>5.2} M", bytes as f64 / 1024.0 / 1024.0),
            _ => format!("{:0>5.2} G", bytes as f64 / 1024.0 / 1024.0 / 1024.0),
        },
        suffix
    )
}
