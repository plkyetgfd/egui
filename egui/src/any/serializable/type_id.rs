use std::any::Any;

/// We need this because `TypeId` can't be deserialized or serialized directly, but this can be done using hashing. However, there is a small possibility that different types will have intersection by hashes of their type ids.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TypeId(u64);

impl TypeId {
    #[inline]
    pub fn of<T: Any + 'static>() -> Self {
        std::any::TypeId::of::<T>().into()
    }

    #[inline(always)]
    pub(crate) fn value(&self) -> u64 {
        self.0
    }
}

impl From<std::any::TypeId> for TypeId {
    #[inline]
    fn from(id: std::any::TypeId) -> Self {
        Self(epaint::util::hash(id))
    }
}
