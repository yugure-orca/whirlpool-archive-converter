use bigdecimal::BigDecimal;

// u64 to u64 string
pub fn serialize_u64<S>(data: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&data.to_string())
}

// u128 to u128 string
pub fn serialize_u128<S>(data: &u128, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&data.to_string())
}

// Option<u64> to u64 string (must be Some)
pub fn serialize_option_u64<S>(data: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&data.unwrap().to_string())
}

// BigDecimal to scientific notation number string
const DECIMAL_PRICE_PRECISION: u64 = 10;
pub fn serialize_decimal_price<S>(data: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(
        &data
            .with_prec(DECIMAL_PRICE_PRECISION)
            .to_scientific_notation(),
    )
}
