use std::{collections::HashMap, time::Duration};

use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Constraint::Percentage, prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
  action::Action,
  config::{Config, KeyBindings},
  mode::Mode,
  systemctl::{
    types::Type,
    utils::{get_service_status, get_simple_units},
  },
};

const DEFAULT_FILTER_TYPE: Type = Type::Service;

#[derive(Default)]
pub struct ServiceList {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
  list_state: ListState,
  list_options: Vec<String>,
  type_filter: Option<Type>,
  seleted_unit: Option<String>,
}

impl ServiceList {
  pub fn new() -> Self {
    let mut ret = Self::default();
    ret.list_state.select(Some(0));
    ret.list_options = get_simple_units().iter().map(|v| v.name()).collect();
    ret.type_filter = Some(DEFAULT_FILTER_TYPE);
    ret
  }

  fn refresh(&mut self) {
    self.list_options = get_simple_units()
      .iter()
      .filter(|v| {
        match &self.type_filter {
          Some(t) => *t == v.unit_type(),
          None => true,
        }
      })
      .map(|v| v.name())
      .collect();
  }
}

impl Component for ServiceList {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    self.config = config;
    self.refresh();
    match self.list_options.get(self.list_state.selected().unwrap_or(0)) {
      Some(unit) => self.seleted_unit = Some(unit.clone()),
      None => (),
    };
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
    let ret = Ok(match key.code {
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
      KeyCode::Char('a') => {
        self.type_filter = if self.type_filter != None { None } else { Some(DEFAULT_FILTER_TYPE) };
        self.refresh();
        None
      },
      KeyCode::Enter => None,
      _ => None,
    });
    match self.list_options.get(self.list_state.selected().unwrap_or(0)) {
      Some(unit) => self.seleted_unit = Some(unit.clone()),
      None => (),
    };
    ret
  }

  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let hor_layout = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![Percentage(30), Percentage(70)])
      .split(area);
    let block = Block::new().title("Units").borders(Borders::ALL);
    let selected = &self.list_state.selected().unwrap_or(0).to_string();
    let list =
      List::new(self.list_options.clone()).highlight_style(Style::new().bg(Color::White).black());
    f.render_stateful_widget(list, block.inner(hor_layout[0]), &mut self.list_state.clone());
    f.render_widget(block, hor_layout[0]);
    if let Some(unit) = &self.seleted_unit {
      let block = Block::new().title("Status").borders(Borders::ALL);
      let para = Paragraph::new(get_service_status(unit));
      f.render_widget(para, block.inner(hor_layout[1]));
      f.render_widget(block, hor_layout[1]);
    }
    Ok(())
  }
}
