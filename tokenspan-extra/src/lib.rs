use bson::oid::ObjectId;
use serde::Serializer;

pub mod pagination;

pub fn serialize_oid<S: Serializer>(
    oid: impl Into<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let val: ObjectId = oid.into();

    serializer.serialize_str(val.to_hex().as_str())
}

pub fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        assert_eq!(round(0.699999988079071, 2), 0.7);
    }
}
