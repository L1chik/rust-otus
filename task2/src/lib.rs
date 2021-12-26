#[allow(dead_code)]
pub struct Home {
    pub name: String,
    spaces: Vec<Space>,
}

#[allow(dead_code)]
pub struct Space {
    name: String,
    devices: Vec<Device>,
}

#[allow(dead_code)]
pub enum DeviceType {
    Thermometer(Therm),
    SmartSocket(Socket),
}
сд
pub struct Device {
    _name: String,
    _device_type: DeviceType,
}

#[allow(dead_code)]
pub struct Therm {
    name: String,
    temperature: i16,
}

#[allow(dead_code)]
pub struct Socket {
    name: String,
    state: bool,
    power_consumption: u32,
}

impl Home {
    pub fn _new() -> Home {
        Home {
            name: String::new(),
            spaces: Vec::new(),
        }
    }

    pub fn _change_name(&mut self, _name: String) -> Result<(), String> {
        // self.name.push_str(&name)

        todo!()
    }

    pub fn _add_space(&mut self, _space: Space) -> Result<(), String> {
        // self.spaces.push(space);

        todo!()
    }

    pub fn _remove_space(&mut self, _space_name: String) -> Result<(), String> {
        // let it = self
        //     .spaces
        //     .iter()
        //     .position(|i| *i.name == space_name)
        //     .unwrap();
        //
        // self.spaces.remove(it);

        todo!()
    }

    pub fn request_list(&self) -> &Vec<Space> {
        &self.spaces
    }
}

impl Space {
    pub fn _change_name(&mut self, _name: String) -> Result<(), String> {
        // self.name.push_str(&name);

        todo!()
    }

    pub fn _add_device(&mut self, _device: Device) -> Result<(), String> {
        todo!()
    }

    pub fn _remove_device(&mut self, _device_name: String) -> Result<(), String> {
        // let it = self
        //     .devices
        //     .iter()
        //     .position(|i| *i.name == device_name)
        //     .unwrap();
        //
        // self.devices.remove(it);

        todo!()
    }

    pub fn request_list(&self) -> &Vec<Device> {
        &self.devices
    }
}

impl Device {
    fn _name(&self) -> String {
        todo!()
    }
}

impl Therm {
    fn _new(_name: String, _temperature: i16) -> Self {
        todo!()
    }

    fn _get_temp(&self) -> i16 {
        todo!()
    }
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
