use std::{
  process::{Command, Stdio},
  str::Split,
};

use super::{simple_unit::SimpleUnit, types::*};

fn get_command_out_put(cmd: &str) -> Result<String, String> {
  let mut cmd = cmd.split(' ');
  let executable = cmd.next().ok_or("command invalid")?;
  let args: Vec<&str> = cmd.collect();
  let output = Command::new(executable)
    .args(args)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .stdin(Stdio::null())
    .output()
    .ok()
    .ok_or("Cannot create process")?;
  Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn get_simple_units() -> Vec<SimpleUnit> {
  let output = get_command_out_put("systemctl list-units").unwrap();
  let units_split: Vec<&str> = output.split('\n').collect();
  units_split[1..units_split.len() - 7]
    .iter()
    .map(|i| {
      let mut name = String::new();
      let mut load = String::new();
      let mut active = String::new();
      let mut sub = String::new();
      let mut description = String::new();
      let mut iter = (*i).chars();

      while let Some(i) = iter.next() {
        match i {
          ' ' => continue,
          _ => {
            name.push(i);
            break;
          },
        }
      }
      'name: while let Some(i) = iter.next() {
        match i {
          ' ' => {
            while let Some(i) = iter.next() {
              match i {
                ' ' => continue,
                _ => {
                  load.push(i);
                  break 'name;
                },
              }
            }
          },
          _ => name.push(i),
        }
      }

      'load: while let Some(i) = iter.next() {
        match i {
          ' ' => {
            while let Some(i) = iter.next() {
              match i {
                ' ' => continue,
                _ => {
                  active.push(i);
                  break 'load;
                },
              }
            }
          },
          _ => load.push(i),
        }
      }
      'active: while let Some(i) = iter.next() {
        match i {
          ' ' => {
            while let Some(i) = iter.next() {
              match i {
                ' ' => continue,
                _ => {
                  sub.push(i);
                  break 'active;
                },
              }
            }
          },
          _ => active.push(i),
        }
      }
      'sub: while let Some(i) = iter.next() {
        match i {
          ' ' => {
            while let Some(i) = iter.next() {
              match i {
                ' ' => continue,
                _ => {
                  description.push(i);
                  break 'sub;
                },
              }
            }
          },
          _ => sub.push(i),
        }
      }
      while let Some(i) = iter.next() {
        description.push(i)
      }
      let (name, unit_type) = (|v: &mut Split<'_, char>| {
        (v.next().unwrap_or("").to_string(), v.next().unwrap_or("").to_string())
      })(&mut name.split('.'));

      let unit = SimpleUnit {
        name: name.to_string(),
        load: match load.as_str() {
          "loaded" => Load::Loaded,
          "masked" => Load::Masked,
          _ => return None,
        },
        active: match active.as_str() {
          "active" => true,
          _ => false,
        },
        unit_type: Type::from(unit_type),
        description: if description.is_empty() { None } else { Some(description) },
      };
      Some(unit)
    })
    .filter(|v| {
      match v {
        Some(_) => true,
        None => false,
      }
    })
    .map(|v| v.unwrap())
    .collect()
}

pub fn get_service_status(name: &str) -> String {
  get_command_out_put(format!("systemctl status {}", name).as_str()).unwrap()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_command_output() {
    let output = match get_command_out_put("systemctl status dbus.service") {
      Ok(o) => o,
      Err(o) => o,
    };
    println!("{}", output);
  }

  #[test]
  fn test_get_simple_units() {
    println!("{:?}", get_simple_units());
  }
}
