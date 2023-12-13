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
