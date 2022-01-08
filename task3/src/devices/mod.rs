mod socket;
mod therm;

pub use self::socket::Socket;
pub use self::therm::Therm;

#[allow(dead_code)]
pub struct Device {
    _name: String,
    _device_type: DeviceType,
}

#[allow(dead_code)]
pub enum DeviceType {
    Thermometer(Therm),
    SmartSocket(Socket),
}

impl Device {
    fn _name(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Устройство имеет уникальное в рамках помещения название, тип и описание.
    fn unique_device_name() {
        todo!()
    }

    #[test]
    // Библтотека позволяет добавлять, получать и удалять любое устройство в доме. Получать список устройств в помещении
    fn request_add_remove_devices() {
        todo!()
    }

    #[test]
    // Типы устройств: термометр, умная розетка
    fn create_therm() {
        todo!()
    }

    #[test]
    // Типы устройств: термометр, умная розетка
    fn create_socket() {
        todo!()
    }
}
