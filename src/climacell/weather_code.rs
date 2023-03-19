use postgres::types::FromSql;
use postgres_types::{to_sql_checked, IsNull, ToSql, Type};
use serde::Serialize;
use serde_repr::Deserialize_repr;
use std::error::Error;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(
    Default, Debug, Clone, PartialEq, Serialize, Deserialize_repr, Copy, EnumString, Display,
)]
#[repr(i16)]
pub enum WeatherCode {
    #[default]
    Unknown = 0,
    Clear = 1000,
    Cloudy = 1001,
    #[serde(rename = "Mostly_Clear")]
    Mostly_Clear = 1100,
    #[serde(rename = "Partly_Cloudy")]
    Partly_Cloudy = 1101,
    #[serde(rename = "Mostly_Cloudy")]
    Mostly_Cloudy = 1102,
    Fog = 2000,
    #[serde(rename = "Light_Fog")]
    Light_Fog = 2100,
    #[serde(rename = "Light_Wind")]
    Light_Wind = 3000,
    Wind = 3001,
    #[serde(rename = "Strong_Wind")]
    Strong_Wind = 3002,
    Drizzle = 4000,
    Rain = 4001,
    #[serde(rename = "Light_Rain")]
    Light_Rain = 4200,
    #[serde(rename = "Heavy_Rain")]
    Heavy_Rain = 4201,
    Snow = 5000,
    Flurries = 5001,
    #[serde(rename = "Light_Snow")]
    Light_Snow = 5100,
    #[serde(rename = "Heavy_Snow")]
    Heavy_Snow = 5101,
    #[serde(rename = "Freezing_Drizzle")]
    Freezing_Drizzle = 6000,
    #[serde(rename = "Freezing_Rain")]
    Freezing_Rain = 6001,
    #[serde(rename = "Light_Freezing_Rain")]
    Light_Freezing_Rain = 6200,
    #[serde(rename = "Heavy_Freezing_Rain")]
    Heavy_Freezing_Rain = 6201,
    #[serde(rename = "Ice_Pellets")]
    Ice_Pellets = 7000,
    #[serde(rename = "Heavy_Ice_Pellets")]
    Heavy_Ice_Pellets = 7101,
    #[serde(rename = "Light_Ice_Pellets")]
    Light_Ice_Pellets = 7102,
    Thunderstorm = 8000,
}

impl<'a> FromSql<'a> for WeatherCode {
    fn from_sql(
        _ty: &postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<
        WeatherCode,
        Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>,
    > {
        let text = String::from_utf8_lossy(raw);
        Ok(WeatherCode::from_str(&text).unwrap())
    }

    fn accepts(ty: &postgres::types::Type) -> bool {
        ty == &postgres::types::Type::VARCHAR
    }
}

impl ToSql for WeatherCode {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        return self.to_string().to_sql(ty, out);
    }

    to_sql_checked!();

    fn accepts(ty: &Type) -> bool {
        return ty.name() == "varchar";
    }
}
