//! Geth call tracer types.

use alloy_primitives::{Address, Bytes, B256, U256};
use serde::{Deserialize, Serialize};

/// The response object for `debug_traceTransaction` with `"tracer": "callTracer"`.
///
/// <https://github.com/ethereum/go-ethereum/blob/91cb6f863a965481e51d5d9c0e5ccd54796fd967/eth/tracers/native/call.go#L44>
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallFrame {
    /// The address of that initiated the call.
    pub from: Address,
    /// How much gas was left before the call.
    pub gas: U256,
    /// How much gas was used by the call.
    pub gas_used: U256,
    /// The address of the contract that was called.
    pub to: Option<Address>,
    /// Calldata input.
    pub input: Bytes,
    /// Output of the call, if any.
    pub output: Option<Bytes>,
    /// Error message, if any.
    pub error: Option<String>,
    /// Why this call reverted, if it reverted.
    pub revert_reason: Option<String>,
    /// Recorded child calls.
    pub calls: Vec<CallFrame>,
    /// Logs emitted by this call.
    pub logs: Vec<CallLogFrame>,
    /// Value transferred.
    pub value: Option<U256>,
    /// The type of the call.
    pub typ: String,
}

/// Represents a recorded call.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallLogFrame {
    /// The address of the contract that was called.
    pub address: Option<Address>,
    /// The topics of the log.
    pub topics: Option<Vec<B256>>,
    /// The data of the log.
    pub data: Option<Bytes>,
}

/// The configuration for the call tracer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallConfig {
    /// When set to true, this will only trace the primary (top-level) call and not any sub-calls.
    /// It eliminates the additional processing for each call frame.
    pub only_top_call: Option<bool>,
    /// When set to true, this will include the logs emitted by the call.
    pub with_log: Option<bool>,
}

impl CallConfig {
    /// Sets the only top call flag.
    pub const fn only_top_call(mut self) -> Self {
        self.only_top_call = Some(true);
        self
    }

    /// Sets the with log flag.
    pub const fn with_log(mut self) -> Self {
        self.with_log = Some(true);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geth::*;

    // See <https://github.com/ethereum/go-ethereum/tree/master/eth/tracers/internal/tracetest/testdata>
    const DEFAULT: &str = include_str!("../../test_data/call_tracer/default.json");
    const LEGACY: &str = include_str!("../../test_data/call_tracer/legacy.json");
    const ONLY_TOP_CALL: &str = include_str!("../../test_data/call_tracer/only_top_call.json");
    const WITH_LOG: &str = include_str!("../../test_data/call_tracer/with_log.json");

    #[test]
    fn test_serialize_call_trace() {
        let mut opts = GethDebugTracingCallOptions::default();
        opts.tracing_options.config.disable_storage = Some(false);
        opts.tracing_options.tracer =
            Some(GethDebugTracerType::BuiltInTracer(GethDebugBuiltInTracerType::CallTracer));
        opts.tracing_options.tracer_config =
            serde_json::to_value(CallConfig { only_top_call: Some(true), with_log: Some(true) })
                .unwrap()
                .into();

        assert_eq!(
            serde_json::to_string(&opts).unwrap(),
            r#"{"disableStorage":false,"tracer":"callTracer","tracerConfig":{"onlyTopCall":true,"withLog":true}}"#
        );
    }

    #[test]
    fn test_deserialize_call_trace() {
        let _trace: CallFrame = serde_json::from_str(DEFAULT).unwrap();
        let _trace: CallFrame = serde_json::from_str(LEGACY).unwrap();
        let _trace: CallFrame = serde_json::from_str(ONLY_TOP_CALL).unwrap();
        let _trace: CallFrame = serde_json::from_str(WITH_LOG).unwrap();
    }
}
