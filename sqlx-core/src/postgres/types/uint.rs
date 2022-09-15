use byteorder::{BigEndian, ByteOrder};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::postgres::{
    PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres,
};
use crate::types::Type;

impl Type<Postgres> for u8 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::UINT1
    }
}

impl Type<Postgres> for u16 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::UINT2
    }
}

impl PgHasArrayType for u16 {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::UINT2_ARRAY
    }
}

impl Encode<'_, Postgres> for u16 {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        buf.extend(&self.to_be_bytes());

        IsNull::No
    }
}

impl Decode<'_, Postgres> for u16 {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_u16(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Type<Postgres> for u32 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::UINT4
    }
}

impl PgHasArrayType for u32 {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::UINT4_ARRAY
    }
}

impl Encode<'_, Postgres> for u32 {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        buf.extend(&self.to_be_bytes());

        IsNull::No
    }
}

impl Decode<'_, Postgres> for u32 {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_u32(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Type<Postgres> for u64 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::UINT8
    }
}

impl PgHasArrayType for u64 {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::UINT8_ARRAY
    }
}

impl Encode<'_, Postgres> for u64 {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        buf.extend(&self.to_be_bytes());

        IsNull::No
    }
}

impl Decode<'_, Postgres> for u64 {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_u64(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}
