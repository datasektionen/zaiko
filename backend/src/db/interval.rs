use std::mem;

use serde::{Deserialize, Serialize};
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{types::PgInterval, PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Interval {
    pub months: i32,
    pub days: i32,
    pub microseconds: i64,
}

impl From<PgInterval> for Interval {
    fn from(value: PgInterval) -> Self {
        Self {
            months: value.months,
            days: value.days,
            microseconds: value.microseconds,
        }
    }
}

impl Into<PgInterval> for Interval {
    fn into(self) -> PgInterval {
        PgInterval {
            months: self.months,
            days: self.days,
            microseconds: self.microseconds,
        }
    }
}

impl Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Self {
            months,
            days,
            microseconds,
        } = self.clone();
        let pgi = pg_interval::Interval {
            months,
            days,
            microseconds,
        };
        serializer.serialize_str(&pgi.to_iso_8601())
    }
}

impl<'de> Deserialize<'de> for Interval {
    fn deserialize<D>(deserializer: D) -> Result<Interval, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let pgi = pg_interval::Interval::from_iso(&s).map_err(|error| {
            serde::de::Error::custom(match error {
                pg_interval::ParseError::ParseIntErr(parse_int_error) => {
                    parse_int_error.to_string()
                }
                pg_interval::ParseError::ParseFloatErr(parse_float_error) => {
                    parse_float_error.to_string()
                }
                pg_interval::ParseError::InvalidYearMonth(invalid_year_month) => invalid_year_month,
                pg_interval::ParseError::InvalidTime(invalid_time) => invalid_time,
                pg_interval::ParseError::InvalidInterval(invalid_interval) => invalid_interval,
            })
        })?;
        Ok(Interval {
            months: pgi.months,
            days: pgi.days,
            microseconds: pgi.microseconds,
        })
    }
}

impl Type<Postgres> for Interval {
    fn type_info() -> PgTypeInfo {
        PgInterval::type_info()
    }
}

impl PgHasArrayType for Interval {
    fn array_type_info() -> PgTypeInfo {
        PgInterval::array_type_info()
    }
}

impl<'de> Decode<'de, Postgres> for Interval {
    fn decode(value: PgValueRef<'de>) -> Result<Self, BoxDynError> {
        let PgInterval {
            months,
            days,
            microseconds,
        } = PgInterval::decode(value)?;
        Ok(Interval {
            months,
            days,
            microseconds,
        })
    }
}

impl Encode<'_, Postgres> for Interval {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let Self {
            months,
            days,
            microseconds,
        } = self.clone();
        let pg_interval = PgInterval {
            months,
            days,
            microseconds,
        };
        pg_interval.encode_by_ref(buf)
    }

    fn size_hint(&self) -> usize {
        2 * mem::size_of::<i64>()
    }
}

impl utoipa::ToSchema for Interval {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("Interval")
    }
}
impl utoipa::PartialSchema for Interval {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                utoipa::openapi::KnownFormat::Duration,
            )))
            .into()
    }
}
