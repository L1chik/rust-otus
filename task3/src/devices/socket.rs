#[allow(dead_code)]
pub struct Socket {
    name: String,
    state: bool,
    power_consumption: u32,
}

impl Socket {
    fn _new(_name: String, _state: bool, _power: u32) -> Self {
        todo!()
    }

    fn _change_state(&self) {
        todo!()
    }

    fn _get_power(&self) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Умная розетка позволяет включать и выключать себя. Предоставляет информацию о текущем состоянии и потребляемой мощности.
    fn socket_change_state_get_info() {
        todo!()
    }
}
