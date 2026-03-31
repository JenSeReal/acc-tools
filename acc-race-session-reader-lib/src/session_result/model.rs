use crate::leaderboard::model::LeaderBoardLine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResult {
  #[serde(rename = "bestlap")]
  pub best_lap: u32,
  #[serde(rename = "bestSplits")]
  pub best_splits: Vec<u32>,
  #[serde(rename = "isWetSession")]
  pub is_wet_session: u32,
  #[serde(rename = "type")]
  pub session_type: u32,
  #[serde(rename = "leaderBoardLines")]
  pub leader_board_lines: Vec<LeaderBoardLine>,
}

impl SessionResult {
  pub fn is_wet(&self) -> bool {
    self.is_wet_session != 0
  }

  pub fn get_position(&self, car_id: u32) -> Option<usize> {
    self
      .leader_board_lines
      .iter()
      .position(|line| line.car.car_id == car_id)
      .map(|pos| pos + 1)
  }

  pub fn leader(&self) -> Option<&LeaderBoardLine> {
    self
      .leader_board_lines
      .iter()
      .min_by_key(|line| line.timing.best_lap)
  }

  pub fn total_participants(&self) -> usize {
    self.leader_board_lines.len()
  }

  pub fn get_line_by_position(&self, position: usize) -> Option<&LeaderBoardLine> {
    if position == 0 {
      return None;
    }
    self.leader_board_lines.get(position - 1)
  }

  pub fn sorted_by_best_lap(&self) -> Vec<&LeaderBoardLine> {
    let mut lines: Vec<&LeaderBoardLine> = self.leader_board_lines.iter().collect();
    lines.sort_by_key(|line| line.timing.best_lap);
    lines
  }

  pub fn get_by_car_id(&self, car_id: u32) -> Option<&LeaderBoardLine> {
    self
      .leader_board_lines
      .iter()
      .find(|line| line.car.car_id == car_id)
  }
}
