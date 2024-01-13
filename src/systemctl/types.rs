use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  AutoMount,
  Device,
  Mount,
  Service,
  Scope,
  Socket,
  Slice,
  Timer,
  Path,
  Target,
  Other,
}

impl From<&str> for Type {
  fn from(value: &str) -> Self {
    match value {
      "automount" => Self::AutoMount,
      "device" => Self::Device,
      "mount" => Self::Mount,
      "service" => Self::Service,
      "scope" => Self::Scope,
      "socket" => Self::Socket,
      "slice" => Self::Slice,
      "timer" => Self::Timer,
      "path" => Self::Path,
      "target" => Self::Target,
      _ => Self::Other,
    }
  }
}

impl From<String> for Type {
  fn from(value: String) -> Self {
    match value.as_str() {
      "automount" => Self::AutoMount,
      "device" => Self::Device,
      "mount" => Self::Mount,
      "service" => Self::Service,
      "scope" => Self::Scope,
      "socket" => Self::Socket,
      "slice" => Self::Slice,
      "timer" => Self::Timer,
      "path" => Self::Path,
      "target" => Self::Target,
      _ => Self::Other,
    }
  }
}

#[derive(Debug)]
pub enum Load {
  Masked,
  Loaded,
}
