pub mod pagination;

pub trait FieldNamesExt {
    const FIELDS: &'static [&'static str];
}
