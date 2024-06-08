use crate::{other::OtherFields, TransactionRequest};
use alloy_consensus::{TxEnvelope, TypedTransaction};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::{Deref, DerefMut};

/// Wrapper allowing to catch all fields missing on the inner struct while
/// deserialize.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WithOtherFields<T> {
    /// The inner struct.
    pub inner: T,
    /// All fields not present in the inner struct.
    pub other: OtherFields,
}

impl<T> WithOtherFields<T> {
    /// Create a new `Extra`.
    pub fn new(inner: T) -> Self {
        Self { inner, other: Default::default() }
    }
}

impl From<TypedTransaction> for WithOtherFields<TransactionRequest> {
    fn from(tx: TypedTransaction) -> Self {
        Self::new(tx.into())
    }
}

impl From<TxEnvelope> for WithOtherFields<TransactionRequest> {
    fn from(envelope: TxEnvelope) -> Self {
        Self::new(envelope.into())
    }
}

impl<T> Deref for WithOtherFields<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for WithOtherFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Default> Default for WithOtherFields<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<'de, T> Deserialize<'de> for WithOtherFields<T>
where
    T: Deserialize<'de> + Serialize,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        #[derive(Deserialize)]
        struct WithOtherFieldsHelper<T> {
            inner: T,
            other: OtherFields,
        }

        let mut helper = WithOtherFieldsHelper::deserialize(deserializer)?;
        // remove all fields present in the inner struct from the other fields, this is to avoid
        // duplicate fields in the catch all other fields because serde flatten does not exclude
        // already deserialized fields when deserializing the other fields.
        if let Value::Object(map) = serde_json::to_value(&helper.inner).map_err(D::Error::custom)? {
            for key in map.keys() {
                helper.other.remove(key);
            }
        }

        Ok(Self { inner: helper.inner, other: helper.other })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use serde_json::json;

    #[derive(Serialize, Deserialize)]
    struct Inner {
        a: u64,
    }

    #[derive(Serialize, Deserialize)]
    struct InnerWrapper {
        inner: Inner,
    }

    #[test]
    fn test_correct_other() {
        let with_other: WithOtherFields<InnerWrapper> =
            serde_json::from_str("{\"a\": 1, \"b\": 2}").unwrap();
        assert_eq!(with_other.inner.inner.a, 1);
        assert_eq!(
            with_other.other,
            OtherFields::new(BTreeMap::from_iter(vec![("b".to_string(), json!(2))]))
        );
    }
}
