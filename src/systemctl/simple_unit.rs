use super::types::*;

#[derive(Debug)]
pub struct SimpleUnit {
  pub(super) name: String,
  pub(super) load: Load,
  pub(super) unit_type: Type,
  pub(super) active: bool,
  pub(super) description: Option<String>,
}

impl SimpleUnit {
  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn unit_type(&self) -> Type {
    self.unit_type.clone()
  }
}
