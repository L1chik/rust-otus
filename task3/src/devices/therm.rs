#[allow(dead_code)]
pub struct Therm {
    name: String,
    temperature: i16,
}

impl Therm {
    fn _new(_name: String, _temperature: i16) -> Self {
        todo!()
    }

    fn _get_temp(&self) -> i16 {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Термометр позволяет узнать температуру.
    fn therm_get_temp() {
        todo!()
    }
}
