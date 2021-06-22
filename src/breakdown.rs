use super::*;

use std::str::FromStr;

#[derive(Serialize)]
pub struct LabeledCount {
    pub label: String,
    pub value: usize,
}

pub enum BreakdownType {
    WithComplications,
    WithEvent,
    PatientAge,
    PatientBmi,
    PatientSmoker,
}

impl FromStr for BreakdownType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "complications" => Ok(BreakdownType::WithComplications),
            "event" => Ok(BreakdownType::WithEvent),
            "age" => Ok(BreakdownType::PatientAge),
            "bmi" => Ok(BreakdownType::PatientBmi),
            "smoker" => Ok(BreakdownType::PatientSmoker),
            x => Err(crate::Error::ParseError {
                type_name: "BreakdownType",
                received: x.to_string(),
            }),
        }
    }
}
