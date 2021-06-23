pub struct LegacySystem {}

impl LegacySystem {
    pub fn old_process_a(&self) -> &str {
        "ふるーい処理A"
    }

    pub fn old_process_b(&self) -> &str {
        "ふるーい処理B"
    }
}

pub trait Target {
    fn new_process_a(&self) -> &str;
    fn new_process_b(&self) -> &str;
}

pub struct Adapter {
    legacy: LegacySystem,
}

impl Adapter {
    pub fn new(legacy: LegacySystem) -> Self {
        Self { legacy: legacy }
    }
}

impl Target for Adapter {
    fn new_process_a(&self) -> &str {
        self.legacy.old_process_a()
    }

    fn new_process_b(&self) -> &str {
        self.legacy.old_process_b()
    }
}
