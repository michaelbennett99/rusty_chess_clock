pub fn round(num: f64, places: u32) -> f64 {
    let factor = 10u32.pow(places);
    (num * factor as f64).round() / factor as f64
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        assert_eq!(round(1.2345, 2), 1.23);
        assert_eq!(round(1.2345, 3), 1.235);
        assert_eq!(round(1.2345, 4), 1.2345);
        assert_eq!(round(1.0, 0), 1.0);
        assert_eq!(round(1.5, 0), 2.0);
        assert_eq!(round(1.0, 5), 1.0);
    }
}
