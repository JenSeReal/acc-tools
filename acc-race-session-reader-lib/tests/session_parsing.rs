use acc_race_session_reader_lib::{
  session::model::Session, session_type::model::SessionType, track::model::Track,
};

#[test]
fn test_parse_fp_session_from_bytes() {
  // Include the JSON file at compile time as bytes (handles UTF-16)
  const FP_SESSION_JSON_BYTES: &[u8] = include_bytes!("fixtures/251022_212652_FP.json");

  let session = Session::try_from(FP_SESSION_JSON_BYTES).expect("Failed to parse JSON");

  session
    .session_result
    .leader_board_lines
    .iter()
    .enumerate()
    .for_each(|(index, line)| {
      println!("Line {index}: {line:#?}");
    });

  // Basic assertions to verify the session was parsed correctly
  assert_eq!(session.session_type, SessionType::FreePractice);
  assert_eq!(session.track_name, Track::RedBullRing);
  assert_eq!(session.session_index, 0);
  assert_eq!(session.race_weekend_index, 35);
  assert_eq!(session.server_name, "ApexPredators Trainingsserver");
}
