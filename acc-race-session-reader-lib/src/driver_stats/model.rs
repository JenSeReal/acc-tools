use crate::car::car_model::CarModel;
use crate::qualifying_session::QualifyingSession;
use crate::session::model::Session;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Common behavior for driver statistics from any source
pub trait DriverStatsCommon {
  fn race_number(&self) -> u32;
  fn car_model(&self) -> CarModel;
  fn driver_name(&self) -> &str;
  fn fastest_lap(&self) -> Option<u32>;
  fn average_lap(&self) -> Option<u32>;
  fn has_seven_consecutive_valid(&self) -> bool;
  fn total_laps(&self) -> usize;
  fn valid_laps(&self) -> usize;

  fn fastest_lap_formatted(&self) -> String {
    self
      .fastest_lap()
      .map(format_laptime)
      .unwrap_or_else(|| "-".to_string())
  }

  fn average_lap_formatted(&self) -> String {
    self
      .average_lap()
      .map(format_laptime)
      .unwrap_or_else(|| "-".to_string())
  }
}

/// Format a laptime in milliseconds to a human-readable string (M:SS.mmm)
pub fn format_laptime(ms: u32) -> String {
  let total_seconds = ms / 1000;
  let minutes = total_seconds / 60;
  let seconds = total_seconds % 60;
  let millis = ms % 1000;
  format!("{}:{:02}.{:03}", minutes, seconds, millis)
}

/// Driver statistics from JSON session files (full data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonDriverStats {
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

impl DriverStatsCommon for JsonDriverStats {
  fn race_number(&self) -> u32 {
    self.race_number
  }
  fn car_model(&self) -> CarModel {
    self.car_model
  }
  fn driver_name(&self) -> &str {
    &self.driver_name
  }
  fn fastest_lap(&self) -> Option<u32> {
    self.fastest_lap
  }
  fn average_lap(&self) -> Option<u32> {
    self.average_lap
  }
  fn has_seven_consecutive_valid(&self) -> bool {
    self.has_seven_consecutive_valid
  }
  fn total_laps(&self) -> usize {
    self.total_laps
  }
  fn valid_laps(&self) -> usize {
    self.valid_laps
  }
}

/// Driver statistics from PDF qualifying results (limited data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfDriverStats {
  pub position: u32,
  pub race_number: u32,
  pub car_model: CarModel,
  pub driver_name: String,
  pub fastest_lap: Option<u32>,
  pub average_lap: Option<u32>,
  pub result_lap: Option<u32>,
  pub has_seven_consecutive_valid: bool,
  pub total_laps: usize,
  pub qualified: bool,
}

impl DriverStatsCommon for PdfDriverStats {
  fn race_number(&self) -> u32 {
    self.race_number
  }
  fn car_model(&self) -> CarModel {
    self.car_model
  }
  fn driver_name(&self) -> &str {
    &self.driver_name
  }
  fn fastest_lap(&self) -> Option<u32> {
    self.fastest_lap
  }
  fn average_lap(&self) -> Option<u32> {
    self.average_lap
  }
  fn has_seven_consecutive_valid(&self) -> bool {
    self.has_seven_consecutive_valid
  }
  fn total_laps(&self) -> usize {
    self.total_laps
  }
  fn valid_laps(&self) -> usize {
    if self.qualified { self.total_laps } else { 0 }
  }
}

/// Unified enum for driver stats from any source
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DriverStats {
  Json(JsonDriverStats),
  Pdf(PdfDriverStats),
}

impl DriverStatsCommon for DriverStats {
  fn race_number(&self) -> u32 {
    match self {
      DriverStats::Json(s) => s.race_number(),
      DriverStats::Pdf(s) => s.race_number(),
    }
  }
  fn car_model(&self) -> CarModel {
    match self {
      DriverStats::Json(s) => s.car_model(),
      DriverStats::Pdf(s) => s.car_model(),
    }
  }
  fn driver_name(&self) -> &str {
    match self {
      DriverStats::Json(s) => s.driver_name(),
      DriverStats::Pdf(s) => s.driver_name(),
    }
  }
  fn fastest_lap(&self) -> Option<u32> {
    match self {
      DriverStats::Json(s) => s.fastest_lap(),
      DriverStats::Pdf(s) => s.fastest_lap(),
    }
  }
  fn average_lap(&self) -> Option<u32> {
    match self {
      DriverStats::Json(s) => s.average_lap(),
      DriverStats::Pdf(s) => s.average_lap(),
    }
  }
  fn has_seven_consecutive_valid(&self) -> bool {
    match self {
      DriverStats::Json(s) => s.has_seven_consecutive_valid(),
      DriverStats::Pdf(s) => s.has_seven_consecutive_valid(),
    }
  }
  fn total_laps(&self) -> usize {
    match self {
      DriverStats::Json(s) => s.total_laps(),
      DriverStats::Pdf(s) => s.total_laps(),
    }
  }
  fn valid_laps(&self) -> usize {
    match self {
      DriverStats::Json(s) => s.valid_laps(),
      DriverStats::Pdf(s) => s.valid_laps(),
    }
  }
}

impl DriverStats {
  /// Get JSON-specific short_name if available
  pub fn short_name(&self) -> Option<&str> {
    match self {
      DriverStats::Json(s) => Some(&s.short_name),
      DriverStats::Pdf(_) => None,
    }
  }

  /// Get JSON-specific team_name if available
  pub fn team_name(&self) -> Option<&str> {
    match self {
      DriverStats::Json(s) => Some(&s.team_name),
      DriverStats::Pdf(_) => None,
    }
  }

  /// Get JSON-specific car_id if available
  pub fn car_id(&self) -> Option<u32> {
    match self {
      DriverStats::Json(s) => Some(s.car_id),
      DriverStats::Pdf(_) => None,
    }
  }

  /// Get PDF-specific result_lap if available
  pub fn result_lap(&self) -> Option<u32> {
    match self {
      DriverStats::Json(_) => None,
      DriverStats::Pdf(s) => s.result_lap,
    }
  }

  /// Get PDF-specific qualified status if available
  pub fn qualified(&self) -> Option<bool> {
    match self {
      DriverStats::Json(_) => None,
      DriverStats::Pdf(s) => Some(s.qualified),
    }
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

/// Trait for sources that can produce driver statistics
pub trait StatsSource {
  type Stats: DriverStatsCommon;
  fn calculate_stats(&self) -> Vec<Self::Stats>;
}

impl StatsSource for Session {
  type Stats = JsonDriverStats;

  fn calculate_stats(&self) -> Vec<Self::Stats> {
    let mut laps_by_driver: HashMap<(u32, u32), Vec<(u32, bool)>> = HashMap::new();

    for lap in &self.laps {
      laps_by_driver
        .entry((lap.car_id, lap.driver_index))
        .or_default()
        .push((lap.laptime, lap.is_valid_for_best));
    }

    let mut stats = Vec::new();

    for line in &self.session_result.leader_board_lines {
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

        stats.push(JsonDriverStats {
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

    stats.sort_by(|a, b| match (a.fastest_lap, b.fastest_lap) {
      (Some(a_lap), Some(b_lap)) => a_lap.cmp(&b_lap),
      (Some(_), None) => std::cmp::Ordering::Less,
      (None, Some(_)) => std::cmp::Ordering::Greater,
      (None, None) => std::cmp::Ordering::Equal,
    });

    stats
  }
}

impl StatsSource for QualifyingSession {
  type Stats = PdfDriverStats;

  fn calculate_stats(&self) -> Vec<Self::Stats> {
    self
      .entries
      .iter()
      .map(|entry| PdfDriverStats {
        position: entry.position,
        race_number: entry.race_number,
        car_model: entry.car_model,
        driver_name: entry.driver_name.clone(),
        fastest_lap: entry.timing.best_lap,
        average_lap: entry.timing.average_lap,
        result_lap: entry.timing.result_lap,
        has_seven_consecutive_valid: entry.valid_consecutive_laps,
        total_laps: entry.lap_count as usize,
        qualified: entry.qualified,
      })
      .collect()
  }
}

/// Calculate driver stats and wrap in the unified enum
pub fn calculate_driver_stats(session: &Session) -> Vec<DriverStats> {
  session
    .calculate_stats()
    .into_iter()
    .map(DriverStats::Json)
    .collect()
}

/// Calculate driver stats from qualifying and wrap in the unified enum
pub fn calculate_driver_stats_from_qualifying(qualifying: &QualifyingSession) -> Vec<DriverStats> {
  qualifying
    .calculate_stats()
    .into_iter()
    .map(DriverStats::Pdf)
    .collect()
}

/// Get unique car models from stats
pub fn get_unique_car_models(stats: &[DriverStats]) -> Vec<CarModel> {
  let mut models: Vec<CarModel> = stats.iter().map(|s| s.car_model()).collect();
  models.sort_by_key(|m| m.to_id());
  models.dedup();
  models
}

/// Filter driver stats by car model
pub fn filter_stats_by_car_model(stats: Vec<DriverStats>, car_model: CarModel) -> Vec<DriverStats> {
  stats
    .into_iter()
    .filter(|s| s.car_model() == car_model)
    .collect()
}

/// Convert driver stats to CSV format
pub fn stats_to_csv(stats: &[DriverStats]) -> String {
  let mut csv = String::new();

  csv.push_str(
    "Position,Race Number,Car Model,Team,Driver,Short Name,Fastest Lap,Average Lap,7 Valid Consecutive,Total Laps,Valid Laps\n",
  );

  for (i, stat) in stats.iter().enumerate() {
    let team_name = stat.team_name().unwrap_or("");
    let short_name = stat.short_name().unwrap_or("");

    csv.push_str(&format!(
      "{},{},\"{}\",\"{}\",\"{}\",\"{}\",{},{},{},{},{}\n",
      i + 1,
      stat.race_number(),
      stat.car_model().name(),
      team_name.replace('"', "\"\""),
      stat.driver_name().replace('"', "\"\""),
      short_name.replace('"', "\"\""),
      stat.fastest_lap_formatted(),
      stat.average_lap_formatted(),
      if stat.has_seven_consecutive_valid() {
        "Yes"
      } else {
        "No"
      },
      stat.total_laps(),
      stat.valid_laps(),
    ));
  }

  csv
}
