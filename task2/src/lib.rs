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
pub struct Device {
    name: String,
    device_type: DeviceType,
    description: String,
}

#[allow(dead_code)]
pub enum DeviceType {
    Thermometer(Therm),
    SmartSocket(Socket),
    Void,
}
#[allow(dead_code)]
pub struct Therm {
    temperature: i16,
}

#[allow(dead_code)]
pub struct Socket {
    state: bool,
    power_consumption: u32,
}

impl Home {
    pub fn new() -> Home {
        Home {
            name: String::new(),
            spaces: Vec::new(),
        }
    }

    pub fn change_name(&mut self, name: String){
        self.name.push_str(&name)
    }

    pub fn add_space(&mut self, space: Space) {
        self.spaces.push(space);
    }

    pub fn remove_space(&mut self, space_name: String) {
        let it = self
            .spaces
            .iter()
            .position(|i| *i.name == space_name)
            .unwrap();

        self.spaces.remove(it);
    }

    pub fn request_list(&self) -> &Vec<Space> {
        &self.spaces
    }
}

impl Space {
    pub fn new() -> Space {
        Space {
            name: String::new(),
            devices: Vec::new(),
        }
    }

    pub fn change_name(&mut self, name: String){
        self.name.push_str(&name);
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
    }

    pub fn remove_device(&mut self, device_name: String) {
        let it = self
            .devices
            .iter()
            .position(|i| *i.name == device_name)
            .unwrap();

        self.devices.remove(it);
    }

    pub fn request_list(&self) -> &Vec<Device> {
        &self.devices
    }
}

impl Device {
    pub fn new() -> Device {
        Device {
            name: String::new(),
            device_type: DeviceType::Void,
            description: String::new(),
        }
    }

    pub fn change_name(&mut self, name: String){
        self.name.push_str(&name)
    }
}

impl Default for Home {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Space {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Device {
    fn default() -> Self {
        Self::new()
    }
}
