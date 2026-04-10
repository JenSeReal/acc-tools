use acc_session_viewer_lib::{
  car::car_model::CarModel,
  parser::{PdfParser, SessionInfo, SessionParser},
  session_type::model::SessionType,
  track::model::Track,
};
use std::str::FromStr;

const KYALAMI_PDF: &[u8] = include_bytes!("fixtures/kyalami.pdf");

#[test]
fn test_parse_kyalami_pdf() {
  let session = PdfParser::parse(KYALAMI_PDF).expect("Failed to parse PDF");

  assert_eq!(session.track_name, Track::Kyalami);
  assert_eq!(session.session_type, SessionType::Qualifying);
  assert_eq!(session.entries.len(), 27);
}

#[test]
fn test_first_entry() {
  let session = PdfParser::parse(KYALAMI_PDF).expect("Failed to parse PDF");
  let first = &session.entries[0];

  assert_eq!(first.position, 1);
  assert_eq!(first.race_number, 500);
  assert_eq!(first.driver_name, "Andi Lederer");
  assert_eq!(first.car_model, CarModel::FordMustangGT3);
  assert_eq!(first.timing.best_lap, Some(101115)); // 01:41.115
  assert_eq!(first.timing.average_lap, Some(101463)); // 01:41.463
  assert_eq!(first.timing.result_lap, Some(101289)); // 01:41.289
  assert!(!first.valid_consecutive_laps);
  assert_eq!(first.lap_count, 18);
  assert!(!first.qualified);
}

#[test]
fn test_qualified_entry() {
  let session = PdfParser::parse(KYALAMI_PDF).expect("Failed to parse PDF");

  // Entry #2 (Florian Baust) should be qualified
  let entry = &session.entries[1];
  assert_eq!(entry.position, 2);
  assert_eq!(entry.driver_name, "Florian Baust");
  assert_eq!(entry.car_model, CarModel::BMWM4GT3);
  assert!(entry.valid_consecutive_laps);
  assert!(entry.qualified);
}

#[test]
fn test_mclaren_multiword_car_name() {
  let session = PdfParser::parse(KYALAMI_PDF).expect("Failed to parse PDF");

  // Entry #4 (Yannick Petry) has McLaren 720S GT3 Evo 2023
  let entry = &session.entries[3];
  assert_eq!(entry.position, 4);
  assert_eq!(entry.driver_name, "Yannick Petry");
  assert_eq!(entry.car_model, CarModel::McLaren720SGT3Evo);
}

#[test]
fn test_no_valid_time_entry() {
  let session = PdfParser::parse(KYALAMI_PDF).expect("Failed to parse PDF");

  // Entry #27 (Celina Gauer) has "No valid time"
  let entry = &session.entries[26];
  assert_eq!(entry.position, 27);
  assert_eq!(entry.driver_name, "Celina Gauer");
  assert_eq!(entry.car_model, CarModel::BMWM4GT3);
  assert_eq!(entry.timing.best_lap, None);
  assert_eq!(entry.timing.average_lap, None);
  assert_eq!(entry.timing.result_lap, None);
  assert!(!entry.timing.has_valid_time());
  assert_eq!(entry.lap_count, 4);
  assert!(!entry.qualified);
}

#[test]
fn test_session_info_trait() {
  let session = PdfParser::parse(KYALAMI_PDF).expect("Failed to parse PDF");

  assert_eq!(*session.track(), Track::Kyalami);
  assert_eq!(session.session_type(), SessionType::Qualifying);
  assert_eq!(session.entry_count(), 27);
}

#[test]
fn test_qualified_entries_iterator() {
  let session = PdfParser::parse(KYALAMI_PDF).expect("Failed to parse PDF");

  let qualified_count = session.qualified_entries().count();
  // Based on the PDF: positions 2, 3, 5, 6, 7, 8, 9, 10, 11, 17, 19, 20 are qualified
  assert!(qualified_count > 0);
  assert!(qualified_count < 27);
}

#[test]
fn test_car_model_from_str() {
  assert_eq!(
    CarModel::from_str("BMW M4 GT3").unwrap(),
    CarModel::BMWM4GT3
  );
  assert_eq!(
    CarModel::from_str("Ford Mustang GT3").unwrap(),
    CarModel::FordMustangGT3
  );
  assert_eq!(
    CarModel::from_str("McLaren 720S GT3 Evo 2023").unwrap(),
    CarModel::McLaren720SGT3Evo
  );
  assert_eq!(
    CarModel::from_str("Porsche 992 GT3 R").unwrap(),
    CarModel::Porsche992GT3R
  );
  assert_eq!(
    CarModel::from_str("Ferrari 296 GT3").unwrap(),
    CarModel::Ferrari296GT3
  );
}

#[test]
fn test_car_model_from_str_case_insensitive() {
  assert_eq!(
    CarModel::from_str("bmw m4 gt3").unwrap(),
    CarModel::BMWM4GT3
  );
  assert_eq!(
    CarModel::from_str("BMW m4 GT3").unwrap(),
    CarModel::BMWM4GT3
  );
}

#[test]
fn test_car_model_from_str_unknown() {
  let result = CarModel::from_str("Unknown Car Model");
  assert!(result.is_err());
}

#[test]
fn test_time_parsing_accuracy() {
  let session = PdfParser::parse(KYALAMI_PDF).expect("Failed to parse PDF");

  // Entry #3 (Alex M): Best 01:40.755
  let entry = &session.entries[2];
  assert_eq!(entry.timing.best_lap, Some(100755)); // 1*60*1000 + 40*1000 + 755 = 100755
}
