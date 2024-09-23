//! This module exposes traits that extend strum to provide crate compatible
//! [`Result`]

use strum::EnumProperty;

use crate::error::StrumError;

/// Helper extension for strum crate.
///
/// Getting string property returns an option and int property is not yet
/// stabilized. [`EnumPropertyExt`] trait provides additional wrappers over
/// [`EnumProperty`] trait to convert bad calls into internal [`StrumError`]
/// instead.
pub trait EnumPropertyExt {
    /// Gets an `&str` property from an enum with [`EnumProperty`] definition,
    /// by name, if it exists. Returns [`StrumError`] otherwise.
    fn get_property(&self, property: &str) -> Result<&str, StrumError>;
    /// Gets an `int` property from an enum with [`EnumProperty`] definition, by
    /// name, if it exists. Returns [`StrumError`] otherwise.
    fn get_int_property(&self, property: &str) -> Result<usize, StrumError>;
}

impl<T> EnumPropertyExt for T
where
    T: EnumProperty + ToString,
{
    #[inline]
    fn get_property(&self, property: &str) -> Result<&str, StrumError> {
        self.get_str(property)
            .ok_or_else(|| StrumError::PropertyNotFound {
                property: property.to_owned(),
                variant: self.to_string(),
            })
    }

    #[inline]
    fn get_int_property(&self, property: &str) -> Result<usize, StrumError> {
        Ok(str::parse::<usize>(self.get_property(property)?)?)
    }
}
