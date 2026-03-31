use std::str::FromStr;

use crate::track::error::ParseError;

use super::model::Track;

impl FromStr for Track {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "red_bull_ring" => Ok(Track::RedBullRing),
      "monza" => Ok(Track::Monza),
      "barcelona" => Ok(Track::Barcelona),
      "brands_hatch" => Ok(Track::BrandsHatch),
      "hungaroring" => Ok(Track::Hungaroring),
      "imola" => Ok(Track::Imola),
      "kyalami" => Ok(Track::Kyalami),
      "laguna_seca" => Ok(Track::LagunaSeca),
      "misano" => Ok(Track::Misano),
      "mount_panorama" => Ok(Track::MountPanorama),
      "nurburgring" => Ok(Track::Nurburgring),
      "paul_ricard" => Ok(Track::PaulRicard),
      "silverstone" => Ok(Track::Silverstone),
      "spa" => Ok(Track::Spa),
      "suzuka" => Ok(Track::Suzuka),
      "zandvoort" => Ok(Track::Zandvoort),
      "zolder" => Ok(Track::Zolder),
      "cota" => Ok(Track::Cota),
      "indianapolis" => Ok(Track::Indianapolis),
      "watkins_glen" => Ok(Track::WatkinsGlen),
      "valencia" => Ok(Track::Valencia),
      _ => Err(ParseError::InvalidTrackName(s.to_string())),
    }
  }
}
