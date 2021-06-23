pub mod legacy_system;

#[cfg(test)]
mod tests {
    use super::legacy_system::{Adapter, Target, LegacySystem};

    #[test]
    fn test_adapter() {
        let system: LegacySystem = LegacySystem {};

        let adapter = Adapter::new(system);

        assert_eq!(adapter.new_process_a(), "ふるーい処理A");
        assert_eq!(adapter.new_process_b(), "ふるーい処理B");
    }
}
