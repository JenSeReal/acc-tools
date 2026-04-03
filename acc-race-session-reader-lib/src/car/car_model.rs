use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// ACC car model IDs mapped to human-readable names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u32", into = "u32")]
pub enum CarModel {
  Porsche991GT3R,
  MercedesAMGGT3,
  Ferrari488GT3,
  AudiR8LMS,
  LamborghiniHuracanGT3,
  McLaren650SGT3,
  NissanGTRNismoGT3,
  BMWM6GT3,
  BentleyContinentalGT32018,
  Porsche991IIGT3Cup,
  NissanGTRNismoGT32018,
  BentleyContinentalGT32016,
  AMRV12VantageGT3,
  ReiterEngineeringREXGT3,
  EmilFreyJaguarG3,
  LexusRCFGT3,
  LamborghiniHuracanGT3Evo,
  HondaNSXGT3,
  LamborghiniHuracanSuperTrofeo,
  AudiR8LMSEvo,
  AMRV8Vantage,
  HondaNSXGT3Evo,
  McLaren720SGT3,
  Porsche991IIGT3R,
  Ferrari488GT3Evo,
  MercedesAMGGT32020,
  Ferrari488ChallengeEvo,
  BMWM2Cup,
  Porsche992GT3Cup,
  LamborghiniHuracanSuperTrofeoEvo2,
  BMWM4GT3,
  AudiR8LMSEvoII,
  Ferrari296GT3,
  LamborghiniHuracanGT3Evo2,
  Porsche992GT3R,
  McLaren720SGT3Evo,
  FordMustangGT3,
  // GT2 cars
  PorscheGT2RS,
  AudiR8LMSGT2,
  KTMXBowGT2,
  MaseratiMCGT2,
  MercedesAMGGT2,
  // GT4 cars
  AlpineA110GT4,
  AMRV8VantageGT4,
  AudiR8LMSGT4,
  BMWM4GT4,
  ChevroletCamaroGT4R,
  GinettaG55GT4,
  KTMXBowGT4,
  MaseratiGranTurismoMCSport,
  McLaren570SGT4,
  MercedesAMGGT4,
  Porsche718CaymanGT4MR,
  ToyotaGRSupraGT4,
  // Unknown car model
  Unknown(u32),
}

impl CarModel {
  pub fn from_id(id: u32) -> Self {
    match id {
      0 => CarModel::Porsche991GT3R,
      1 => CarModel::MercedesAMGGT3,
      2 => CarModel::Ferrari488GT3,
      3 => CarModel::AudiR8LMS,
      4 => CarModel::LamborghiniHuracanGT3,
      5 => CarModel::McLaren650SGT3,
      6 => CarModel::NissanGTRNismoGT3,
      7 => CarModel::BMWM6GT3,
      8 => CarModel::BentleyContinentalGT32018,
      9 => CarModel::Porsche991IIGT3Cup,
      10 => CarModel::NissanGTRNismoGT32018,
      11 => CarModel::BentleyContinentalGT32016,
      12 => CarModel::AMRV12VantageGT3,
      13 => CarModel::ReiterEngineeringREXGT3,
      14 => CarModel::EmilFreyJaguarG3,
      15 => CarModel::LexusRCFGT3,
      16 => CarModel::LamborghiniHuracanGT3Evo,
      17 => CarModel::HondaNSXGT3,
      18 => CarModel::LamborghiniHuracanSuperTrofeo,
      19 => CarModel::AudiR8LMSEvo,
      20 => CarModel::AMRV8Vantage,
      21 => CarModel::HondaNSXGT3Evo,
      22 => CarModel::McLaren720SGT3,
      23 => CarModel::Porsche991IIGT3R,
      24 => CarModel::Ferrari488GT3Evo,
      25 => CarModel::MercedesAMGGT32020,
      26 => CarModel::Ferrari488ChallengeEvo,
      27 => CarModel::BMWM2Cup,
      28 => CarModel::Porsche992GT3Cup,
      29 => CarModel::LamborghiniHuracanSuperTrofeoEvo2,
      30 => CarModel::BMWM4GT3,
      31 => CarModel::AudiR8LMSEvoII,
      32 => CarModel::Ferrari296GT3,
      33 => CarModel::LamborghiniHuracanGT3Evo2,
      34 => CarModel::Porsche992GT3R,
      35 => CarModel::McLaren720SGT3Evo,
      36 => CarModel::FordMustangGT3,
      // GT2 cars (50-59)
      50 => CarModel::PorscheGT2RS,
      51 => CarModel::AudiR8LMSGT2,
      52 => CarModel::KTMXBowGT2,
      53 => CarModel::MaseratiMCGT2,
      55 => CarModel::MercedesAMGGT2,
      // GT4 cars (60-79)
      60 => CarModel::AlpineA110GT4,
      61 => CarModel::AMRV8VantageGT4,
      62 => CarModel::AudiR8LMSGT4,
      63 => CarModel::BMWM4GT4,
      64 => CarModel::ChevroletCamaroGT4R,
      65 => CarModel::GinettaG55GT4,
      66 => CarModel::KTMXBowGT4,
      67 => CarModel::MaseratiGranTurismoMCSport,
      68 => CarModel::McLaren570SGT4,
      69 => CarModel::MercedesAMGGT4,
      70 => CarModel::Porsche718CaymanGT4MR,
      71 => CarModel::ToyotaGRSupraGT4,
      id => CarModel::Unknown(id),
    }
  }

  pub fn to_id(&self) -> u32 {
    match self {
      CarModel::Porsche991GT3R => 0,
      CarModel::MercedesAMGGT3 => 1,
      CarModel::Ferrari488GT3 => 2,
      CarModel::AudiR8LMS => 3,
      CarModel::LamborghiniHuracanGT3 => 4,
      CarModel::McLaren650SGT3 => 5,
      CarModel::NissanGTRNismoGT3 => 6,
      CarModel::BMWM6GT3 => 7,
      CarModel::BentleyContinentalGT32018 => 8,
      CarModel::Porsche991IIGT3Cup => 9,
      CarModel::NissanGTRNismoGT32018 => 10,
      CarModel::BentleyContinentalGT32016 => 11,
      CarModel::AMRV12VantageGT3 => 12,
      CarModel::ReiterEngineeringREXGT3 => 13,
      CarModel::EmilFreyJaguarG3 => 14,
      CarModel::LexusRCFGT3 => 15,
      CarModel::LamborghiniHuracanGT3Evo => 16,
      CarModel::HondaNSXGT3 => 17,
      CarModel::LamborghiniHuracanSuperTrofeo => 18,
      CarModel::AudiR8LMSEvo => 19,
      CarModel::AMRV8Vantage => 20,
      CarModel::HondaNSXGT3Evo => 21,
      CarModel::McLaren720SGT3 => 22,
      CarModel::Porsche991IIGT3R => 23,
      CarModel::Ferrari488GT3Evo => 24,
      CarModel::MercedesAMGGT32020 => 25,
      CarModel::Ferrari488ChallengeEvo => 26,
      CarModel::BMWM2Cup => 27,
      CarModel::Porsche992GT3Cup => 28,
      CarModel::LamborghiniHuracanSuperTrofeoEvo2 => 29,
      CarModel::BMWM4GT3 => 30,
      CarModel::AudiR8LMSEvoII => 31,
      CarModel::Ferrari296GT3 => 32,
      CarModel::LamborghiniHuracanGT3Evo2 => 33,
      CarModel::Porsche992GT3R => 34,
      CarModel::McLaren720SGT3Evo => 35,
      CarModel::FordMustangGT3 => 36,
      CarModel::PorscheGT2RS => 50,
      CarModel::AudiR8LMSGT2 => 51,
      CarModel::KTMXBowGT2 => 52,
      CarModel::MaseratiMCGT2 => 53,
      CarModel::MercedesAMGGT2 => 55,
      CarModel::AlpineA110GT4 => 60,
      CarModel::AMRV8VantageGT4 => 61,
      CarModel::AudiR8LMSGT4 => 62,
      CarModel::BMWM4GT4 => 63,
      CarModel::ChevroletCamaroGT4R => 64,
      CarModel::GinettaG55GT4 => 65,
      CarModel::KTMXBowGT4 => 66,
      CarModel::MaseratiGranTurismoMCSport => 67,
      CarModel::McLaren570SGT4 => 68,
      CarModel::MercedesAMGGT4 => 69,
      CarModel::Porsche718CaymanGT4MR => 70,
      CarModel::ToyotaGRSupraGT4 => 71,
      CarModel::Unknown(id) => *id,
    }
  }

  pub fn name(&self) -> &'static str {
    match self {
      CarModel::Porsche991GT3R => "Porsche 991 GT3 R",
      CarModel::MercedesAMGGT3 => "Mercedes-AMG GT3",
      CarModel::Ferrari488GT3 => "Ferrari 488 GT3",
      CarModel::AudiR8LMS => "Audi R8 LMS",
      CarModel::LamborghiniHuracanGT3 => "Lamborghini Huracán GT3",
      CarModel::McLaren650SGT3 => "McLaren 650S GT3",
      CarModel::NissanGTRNismoGT3 => "Nissan GT-R Nismo GT3",
      CarModel::BMWM6GT3 => "BMW M6 GT3",
      CarModel::BentleyContinentalGT32018 => "Bentley Continental GT3 2018",
      CarModel::Porsche991IIGT3Cup => "Porsche 991 II GT3 Cup",
      CarModel::NissanGTRNismoGT32018 => "Nissan GT-R Nismo GT3 2018",
      CarModel::BentleyContinentalGT32016 => "Bentley Continental GT3 2016",
      CarModel::AMRV12VantageGT3 => "AMR V12 Vantage GT3",
      CarModel::ReiterEngineeringREXGT3 => "Reiter Engineering R-EX GT3",
      CarModel::EmilFreyJaguarG3 => "Emil Frey Jaguar G3",
      CarModel::LexusRCFGT3 => "Lexus RC F GT3",
      CarModel::LamborghiniHuracanGT3Evo => "Lamborghini Huracán GT3 Evo",
      CarModel::HondaNSXGT3 => "Honda NSX GT3",
      CarModel::LamborghiniHuracanSuperTrofeo => "Lamborghini Huracán SuperTrofeo",
      CarModel::AudiR8LMSEvo => "Audi R8 LMS Evo",
      CarModel::AMRV8Vantage => "AMR V8 Vantage",
      CarModel::HondaNSXGT3Evo => "Honda NSX GT3 Evo",
      CarModel::McLaren720SGT3 => "McLaren 720S GT3",
      CarModel::Porsche991IIGT3R => "Porsche 991 II GT3 R",
      CarModel::Ferrari488GT3Evo => "Ferrari 488 GT3 Evo",
      CarModel::MercedesAMGGT32020 => "Mercedes-AMG GT3 2020",
      CarModel::Ferrari488ChallengeEvo => "Ferrari 488 Challenge Evo",
      CarModel::BMWM2Cup => "BMW M2 Cup",
      CarModel::Porsche992GT3Cup => "Porsche 992 GT3 Cup",
      CarModel::LamborghiniHuracanSuperTrofeoEvo2 => "Lamborghini Huracán SuperTrofeo Evo2",
      CarModel::BMWM4GT3 => "BMW M4 GT3",
      CarModel::AudiR8LMSEvoII => "Audi R8 LMS Evo II",
      CarModel::Ferrari296GT3 => "Ferrari 296 GT3",
      CarModel::LamborghiniHuracanGT3Evo2 => "Lamborghini Huracán GT3 Evo 2",
      CarModel::Porsche992GT3R => "Porsche 992 GT3 R",
      CarModel::McLaren720SGT3Evo => "McLaren 720S GT3 Evo",
      CarModel::FordMustangGT3 => "Ford Mustang GT3",
      CarModel::PorscheGT2RS => "Porsche GT2 RS",
      CarModel::AudiR8LMSGT2 => "Audi R8 LMS GT2",
      CarModel::KTMXBowGT2 => "KTM X-Bow GT2",
      CarModel::MaseratiMCGT2 => "Maserati MC GT2",
      CarModel::MercedesAMGGT2 => "Mercedes-AMG GT2",
      CarModel::AlpineA110GT4 => "Alpine A110 GT4",
      CarModel::AMRV8VantageGT4 => "AMR V8 Vantage GT4",
      CarModel::AudiR8LMSGT4 => "Audi R8 LMS GT4",
      CarModel::BMWM4GT4 => "BMW M4 GT4",
      CarModel::ChevroletCamaroGT4R => "Chevrolet Camaro GT4.R",
      CarModel::GinettaG55GT4 => "Ginetta G55 GT4",
      CarModel::KTMXBowGT4 => "KTM X-Bow GT4",
      CarModel::MaseratiGranTurismoMCSport => "Maserati GranTurismo MC Sport",
      CarModel::McLaren570SGT4 => "McLaren 570S GT4",
      CarModel::MercedesAMGGT4 => "Mercedes-AMG GT4",
      CarModel::Porsche718CaymanGT4MR => "Porsche 718 Cayman GT4 MR",
      CarModel::ToyotaGRSupraGT4 => "Toyota GR Supra GT4",
      CarModel::Unknown(_) => "Unknown",
    }
  }
}

impl fmt::Display for CarModel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name())
  }
}

impl From<u32> for CarModel {
  fn from(id: u32) -> Self {
    CarModel::from_id(id)
  }
}

impl From<CarModel> for u32 {
  fn from(model: CarModel) -> Self {
    model.to_id()
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarModelParseError {
  pub model_name: String,
}

impl fmt::Display for CarModelParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Unknown car model: {}", self.model_name)
  }
}

impl std::error::Error for CarModelParseError {}

impl FromStr for CarModel {
  type Err = CarModelParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // Normalize: lowercase, remove extra whitespace
    let normalized: String = s
      .split_whitespace()
      .collect::<Vec<_>>()
      .join(" ")
      .to_lowercase();

    match normalized.as_str() {
      // GT3 cars
      "porsche 991 gt3 r" => Ok(CarModel::Porsche991GT3R),
      "mercedes-amg gt3" | "mercedes amg gt3" => Ok(CarModel::MercedesAMGGT3),
      "ferrari 488 gt3" => Ok(CarModel::Ferrari488GT3),
      "audi r8 lms" => Ok(CarModel::AudiR8LMS),
      "lamborghini huracán gt3" | "lamborghini huracan gt3" => Ok(CarModel::LamborghiniHuracanGT3),
      "mclaren 650s gt3" => Ok(CarModel::McLaren650SGT3),
      "nissan gt-r nismo gt3" | "nissan gtr nismo gt3" => Ok(CarModel::NissanGTRNismoGT3),
      "bmw m6 gt3" => Ok(CarModel::BMWM6GT3),
      "bentley continental gt3 2018" => Ok(CarModel::BentleyContinentalGT32018),
      "porsche 991 ii gt3 cup" => Ok(CarModel::Porsche991IIGT3Cup),
      "nissan gt-r nismo gt3 2018" | "nissan gtr nismo gt3 2018" => {
        Ok(CarModel::NissanGTRNismoGT32018)
      }
      "bentley continental gt3 2016" => Ok(CarModel::BentleyContinentalGT32016),
      "amr v12 vantage gt3" => Ok(CarModel::AMRV12VantageGT3),
      "reiter engineering r-ex gt3" | "reiter engineering rex gt3" => {
        Ok(CarModel::ReiterEngineeringREXGT3)
      }
      "emil frey jaguar g3" => Ok(CarModel::EmilFreyJaguarG3),
      "lexus rc f gt3" => Ok(CarModel::LexusRCFGT3),
      "lamborghini huracán gt3 evo" | "lamborghini huracan gt3 evo" => {
        Ok(CarModel::LamborghiniHuracanGT3Evo)
      }
      "honda nsx gt3" => Ok(CarModel::HondaNSXGT3),
      "lamborghini huracán supertrofeo" | "lamborghini huracan supertrofeo" => {
        Ok(CarModel::LamborghiniHuracanSuperTrofeo)
      }
      "audi r8 lms evo" => Ok(CarModel::AudiR8LMSEvo),
      "amr v8 vantage" => Ok(CarModel::AMRV8Vantage),
      "honda nsx gt3 evo" => Ok(CarModel::HondaNSXGT3Evo),
      "mclaren 720s gt3" => Ok(CarModel::McLaren720SGT3),
      "porsche 991 ii gt3 r" => Ok(CarModel::Porsche991IIGT3R),
      "ferrari 488 gt3 evo" => Ok(CarModel::Ferrari488GT3Evo),
      "mercedes-amg gt3 2020" | "mercedes amg gt3 2020" => Ok(CarModel::MercedesAMGGT32020),
      "ferrari 488 challenge evo" => Ok(CarModel::Ferrari488ChallengeEvo),
      "bmw m2 cup" => Ok(CarModel::BMWM2Cup),
      "porsche 992 gt3 cup" => Ok(CarModel::Porsche992GT3Cup),
      "lamborghini huracán supertrofeo evo2" | "lamborghini huracan supertrofeo evo2" => {
        Ok(CarModel::LamborghiniHuracanSuperTrofeoEvo2)
      }
      "bmw m4 gt3" => Ok(CarModel::BMWM4GT3),
      "audi r8 lms evo ii" | "audi r8 lms evo 2" => Ok(CarModel::AudiR8LMSEvoII),
      "ferrari 296 gt3" => Ok(CarModel::Ferrari296GT3),
      "lamborghini huracán gt3 evo 2"
      | "lamborghini huracan gt3 evo 2"
      | "lamborghini huracán gt3 evo2"
      | "lamborghini huracan gt3 evo2" => Ok(CarModel::LamborghiniHuracanGT3Evo2),
      "porsche 992 gt3 r" => Ok(CarModel::Porsche992GT3R),
      "mclaren 720s gt3 evo" | "mclaren 720s gt3 evo 2023" => Ok(CarModel::McLaren720SGT3Evo),
      "ford mustang gt3" => Ok(CarModel::FordMustangGT3),
      // GT2 cars
      "porsche gt2 rs" | "porsche 911 gt2 rs" => Ok(CarModel::PorscheGT2RS),
      "audi r8 lms gt2" => Ok(CarModel::AudiR8LMSGT2),
      "ktm x-bow gt2" | "ktm xbow gt2" => Ok(CarModel::KTMXBowGT2),
      "maserati mc gt2" => Ok(CarModel::MaseratiMCGT2),
      "mercedes-amg gt2" | "mercedes amg gt2" => Ok(CarModel::MercedesAMGGT2),
      // GT4 cars
      "alpine a110 gt4" => Ok(CarModel::AlpineA110GT4),
      "amr v8 vantage gt4" => Ok(CarModel::AMRV8VantageGT4),
      "audi r8 lms gt4" => Ok(CarModel::AudiR8LMSGT4),
      "bmw m4 gt4" => Ok(CarModel::BMWM4GT4),
      "chevrolet camaro gt4.r" | "chevrolet camaro gt4 r" => Ok(CarModel::ChevroletCamaroGT4R),
      "ginetta g55 gt4" => Ok(CarModel::GinettaG55GT4),
      "ktm x-bow gt4" | "ktm xbow gt4" => Ok(CarModel::KTMXBowGT4),
      "maserati granturismo mc sport" => Ok(CarModel::MaseratiGranTurismoMCSport),
      "mclaren 570s gt4" => Ok(CarModel::McLaren570SGT4),
      "mercedes-amg gt4" | "mercedes amg gt4" => Ok(CarModel::MercedesAMGGT4),
      "porsche 718 cayman gt4 mr" => Ok(CarModel::Porsche718CaymanGT4MR),
      "toyota gr supra gt4" => Ok(CarModel::ToyotaGRSupraGT4),
      _ => Err(CarModelParseError {
        model_name: s.to_string(),
      }),
    }
  }
}
