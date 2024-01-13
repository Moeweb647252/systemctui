use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::components::{self, fps::FpsCounter, home::Home, service_list::ServiceList, Component};

pub type Components = HashMap<Mode, Vec<Box<dyn Component>>>;
type ComponentsVec = Vec<Box<dyn Component>>;
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
  #[default]
  Home,
  List,
}

impl Mode {
  pub fn components() -> Components {
    let mut components: Components = HashMap::new();

    let mut home_components: ComponentsVec = Vec::new();
    let mut list_components: ComponentsVec = Vec::new();
    home_components.push(Box::new(Home::new()));
    list_components.push(Box::new(ServiceList::new()));
    components.insert(Self::Home, home_components);
    components.insert(Self::List, list_components);

    #[cfg(debug_assertions)]
    for (_, components_vec) in components.iter_mut() {
      components_vec.push(Box::new(FpsCounter::new()))
    }
    components
  }
}
