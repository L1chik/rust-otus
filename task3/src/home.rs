use crate::space::Space;

#[allow(dead_code)]
pub struct Home {
    name: String,
    spaces: Vec<Space>,
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

    pub fn _request_list(&self) -> &Vec<Space> {
        &self.spaces
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Дом имеет название и содержит несколько помещений
    fn home_has_name_and_spaces() {
        todo!()
    }

    #[test]
    // Библтотека позволяет запросить список помещений, добавлять и удалять помещения в доме
    fn request_add_remove_spaces() {
        todo!()
    }

    #[test]
    // Умная розетка позволяет включать и выключать себя. Предоставляет информацию о текущем состоянии и потребляемой мощности
    fn full_report() {
        todo!()
    }
}
