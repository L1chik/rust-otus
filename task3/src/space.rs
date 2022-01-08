use crate::devices::Device;

#[allow(dead_code)]
pub struct Space {
    name: String,
    devices: Vec<Device>,
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

    pub fn _request_list(&self) -> &Vec<Device> {
        &self.devices
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Помещение имеет уникальное название и содержит несколько устройств.
    fn unique_space_name() {
        todo!()
    }
}
