use crate::car::car_model::CarModel;
use crate::session::model::Session;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverStats {
  pub car_id: u32,
  pub race_number: u32,
  pub car_model: CarModel,
  pub team_name: String,
  pub driver_index: u32,
  pub driver_name: String,
  pub short_name: String,
  pub fastest_lap: Option<u32>,
  pub average_lap: Option<u32>,
  pub has_seven_consecutive_valid: bool,
  pub total_laps: usize,
  pub valid_laps: usize,
}

impl DriverStats {
  /// Format a laptime in milliseconds to a human-readable string (M:SS.mmm)
  pub fn format_laptime(ms: u32) -> String {
    let total_seconds = ms / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    let millis = ms % 1000;
    format!("{}:{:02}.{:03}", minutes, seconds, millis)
  }

  /// Get the fastest lap formatted as a string, or "-" if no laps
  pub fn fastest_lap_formatted(&self) -> String {
    self
      .fastest_lap
      .map(Self::format_laptime)
      .unwrap_or_else(|| "-".to_string())
  }

  /// Get the average lap formatted as a string, or "-" if no laps
  pub fn average_lap_formatted(&self) -> String {
    self
      .average_lap
      .map(Self::format_laptime)
      .unwrap_or_else(|| "-".to_string())
  }
}

fn has_seven_consecutive_valid_laps(laps: &[(u32, bool)]) -> bool {
  let mut consecutive = 0;
  for (_, is_valid) in laps {
    if *is_valid {
      consecutive += 1;
      if consecutive >= 7 {
        return true;
      }
    } else {
      consecutive = 0;
    }
  }
  false
}

/// Calculate driver statistics from a session
pub fn calculate_driver_stats(session: &Session) -> Vec<DriverStats> {
  // Group laps by (car_id, driver_index)
  let mut laps_by_driver: HashMap<(u32, u32), Vec<(u32, bool)>> = HashMap::new();

  for lap in &session.laps {
    laps_by_driver
      .entry((lap.car_id, lap.driver_index))
      .or_default()
      .push((lap.laptime, lap.is_valid_for_best));
  }

  let mut stats = Vec::new();

  for line in &session.session_result.leader_board_lines {
    let car = &line.car;

    for (driver_index, driver) in car.drivers.iter().enumerate() {
      let driver_laps = laps_by_driver
        .get(&(car.car_id, driver_index as u32))
        .cloned()
        .unwrap_or_default();

      let valid_laptimes: Vec<u32> = driver_laps
        .iter()
        .filter(|(_, valid)| *valid)
        .map(|(time, _)| *time)
        .collect();

      let fastest_lap = valid_laptimes.iter().min().copied();
      let average_lap = if !valid_laptimes.is_empty() {
        Some(
          (valid_laptimes.iter().map(|&t| t as u64).sum::<u64>() / valid_laptimes.len() as u64)
            as u32,
        )
      } else {
        None
      };

      let has_seven = has_seven_consecutive_valid_laps(&driver_laps);

      stats.push(DriverStats {
        car_id: car.car_id,
        race_number: car.race_number,
        car_model: CarModel::from_id(car.car_model),
        team_name: car.team_name.clone(),
        driver_index: driver_index as u32,
        driver_name: format!("{} {}", driver.first_name, driver.last_name),
        short_name: driver.short_name.clone(),
        fastest_lap,
        average_lap,
        has_seven_consecutive_valid: has_seven,
        total_laps: driver_laps.len(),
        valid_laps: valid_laptimes.len(),
      });
    }
  }

  // Sort by fastest lap (drivers with no laps at the end)
  stats.sort_by(|a, b| match (a.fastest_lap, b.fastest_lap) {
    (Some(a_lap), Some(b_lap)) => a_lap.cmp(&b_lap),
    (Some(_), None) => std::cmp::Ordering::Less,
    (None, Some(_)) => std::cmp::Ordering::Greater,
    (None, None) => std::cmp::Ordering::Equal,
  });

  stats
}

/// Get unique car models present in a session
pub fn get_session_car_models(session: &Session) -> Vec<CarModel> {
  let mut models: Vec<CarModel> = session
    .session_result
    .leader_board_lines
    .iter()
    .map(|line| CarModel::from_id(line.car.car_model))
    .collect();

  models.sort_by_key(|m| m.to_id());
  models.dedup();
  models
}

/// Filter driver stats by car model
pub fn filter_stats_by_car_model(stats: Vec<DriverStats>, car_model: CarModel) -> Vec<DriverStats> {
  stats
    .into_iter()
    .filter(|s| s.car_model == car_model)
    .collect()
}
