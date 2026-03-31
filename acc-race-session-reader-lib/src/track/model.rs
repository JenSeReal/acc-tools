use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Track {
  RedBullRing,
  Monza,
  Barcelona,
  BrandsHatch,
  Hungaroring,
  Imola,
  Kyalami,
  LagunaSeca,
  Misano,
  MountPanorama,
  Nurburgring,
  PaulRicard,
  Silverstone,
  Spa,
  Suzuka,
  Zandvoort,
  Zolder,
  Cota,
  Indianapolis,
  WatkinsGlen,
  Valencia,
}
