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
#[repr(u8)]
pub enum MoonPhase {
    #[default]
    New = 0,
    Waxing_Crescent = 1,
    First_Quarter = 2,
    Waxing_Gibbous = 3,
    Full = 4,
    Waning_Gibbous = 5,
    Third_Quarter = 6,
    Waning_Crescent = 7,
}

impl<'a> FromSql<'a> for MoonPhase {
    fn from_sql(
        _ty: &postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<
        MoonPhase,
        Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>,
    > {
        let text = String::from_utf8_lossy(raw);
        Ok(MoonPhase::from_str(&text).unwrap())
    }

    fn accepts(ty: &postgres::types::Type) -> bool {
        ty == &postgres::types::Type::VARCHAR
    }
}

impl ToSql for MoonPhase {
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
