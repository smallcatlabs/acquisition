use super::IntentValue;
use serde::Serialize;
use serde_json::Value;

macro_rules! loose_kind {
    ($name:ident, $kind:literal) => {
        #[derive(Debug, Clone, PartialEq, Serialize)]
        #[serde(transparent)]
        pub(crate) struct $name(pub Value);

        impl IntentValue for $name {
            const KIND: &'static str = $kind;
            const VERSION: i64 = 1;
            fn parse(value: &Value) -> Result<Self, String> {
                Ok($name(value.clone()))
            }
        }
    };
}

loose_kind!(Buyout, "buyout");
loose_kind!(Note, "note");
loose_kind!(Policy, "sync-policy");

/// A `note` that reads any shape but refuses, as a new write, a value
/// carrying `"retired": true` — the shape of a writer's rule.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(transparent)]
pub(crate) struct GuardedNote(pub Value);

impl IntentValue for GuardedNote {
    const KIND: &'static str = "note";
    const VERSION: i64 = 1;
    fn parse(value: &Value) -> Result<Self, String> {
        Ok(GuardedNote(value.clone()))
    }
    fn check_write(&self) -> Result<(), String> {
        match self.0.get("retired") {
            Some(Value::Bool(true)) => Err("names a retired thing".into()),
            _ => Ok(()),
        }
    }
}

/// The provenance every test write carries.
pub(crate) fn via_test() -> super::Provenance {
    super::Provenance::via("test")
}
