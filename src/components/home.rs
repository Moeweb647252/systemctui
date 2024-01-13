use std::{collections::HashMap, time::Duration};

use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Constraint::Percentage, prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
  action::Action,
  config::{Config, KeyBindings},
  mode::Mode,
};

#[derive(Default)]
pub struct Home {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
  list_state: ListState,
  list_options: Vec<(String, Mode)>,
}

impl Home {
  pub fn new() -> Self {
    let mut ret = Self::default();
    ret.list_state.select(Some(0));
    ret.list_options.push(("List".to_string(), Mode::List));

    ret
  }
}

impl Component for Home {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    self.config = config;
    Ok(())
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => {},
      _ => {},
    }
    Ok(None)
  }

  fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    Ok(match key.code {
      KeyCode::Up => {
        let selected = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some(if selected == 0 {
          self.list_options.len() - 1
        } else {
          selected - 1
        }));
        None
      },
      KeyCode::Down => {
        let selected = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some(if selected == self.list_options.len() - 1 {
          0
        } else {
          selected + 1
        }));
        None
      },
      KeyCode::Enter => {
        if let Some(tx) = &self.command_tx {
          self
            .list_options
            .get(self.list_state.selected().unwrap_or(0))
            .map(|v| Action::ChangeMode(v.1))
        } else {
          None
        }
      },
      _ => None,
    })
  }

  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let vec_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![Percentage(10), Percentage(80), Percentage(10)])
      .split(area);
    let hor_layout = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![Percentage(25), Percentage(50), Percentage(25)])
      .split(vec_layout[1]);
    let block = Block::new().title("Index").borders(Borders::ALL);
    let list = List::new(self.list_options.iter().map(|v| v.0.as_str()).collect::<Vec<&str>>())
      .highlight_style(Style::new().bg(Color::White).black());
    f.render_stateful_widget(list, block.inner(hor_layout[1]), &mut self.list_state.clone());
    f.render_widget(block, hor_layout[1]);
    Ok(())
  }
}
