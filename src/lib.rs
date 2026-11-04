#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HerdrExecutable(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PollingMilliseconds(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CallTimeoutMilliseconds(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StartTimeoutMilliseconds(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FrameCapacityBytes(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SocketPath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Message(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BootstrapRefusal {
    pub socket_path: SocketPath,
    pub message: Message,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Configuration {
    pub herdr_executable: HerdrExecutable,
    pub polling_milliseconds: PollingMilliseconds,
    pub call_timeout_milliseconds: CallTimeoutMilliseconds,
    pub start_timeout_milliseconds: StartTimeoutMilliseconds,
    pub frame_capacity_bytes: FrameCapacityBytes,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ConfigurationError {
    EmptyExecutable,
    PollingOutOfRange(PollingMilliseconds),
    CallTimeoutOutOfRange(CallTimeoutMilliseconds),
    StartTimeoutOutOfRange(StartTimeoutMilliseconds),
    FrameCapacityOutOfRange(FrameCapacityBytes),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ConfigureOutcome {
    Succeeded,
    Invalid(std::vec::Vec<ConfigurationError>),
    Unavailable(BootstrapRefusal),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Configure(Configuration),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Configured(ConfigureOutcome),
}
/// The authored schema whose digest identifies this meta contract on the wire.
pub const ETHOS: &str = include_str!("../signal.ethos");

impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}

#[cfg(test)]
mod tests {
    use super::{
        CallTimeoutMilliseconds, Configuration, FrameCapacityBytes, HerdrExecutable,
        PollingMilliseconds, Query, StartTimeoutMilliseconds,
    };
    use signal::{ByteViewable, Restorable, Signal, Signalizable};

    #[test]
    fn numeric_configuration_restores_through_fixed_endian_validated_signal() {
        let query = Query::Configure(Configuration {
            herdr_executable: HerdrExecutable("/run/current-system/sw/bin/herdr".into()),
            polling_milliseconds: PollingMilliseconds(750),
            call_timeout_milliseconds: CallTimeoutMilliseconds(30_000),
            start_timeout_milliseconds: StartTimeoutMilliseconds(60_000),
            frame_capacity_bytes: FrameCapacityBytes(8 * 1024 * 1024),
        });
        let archived = query.signalize().expect("archive meta query");
        let restored = Signal::<Query>::from(archived.bytes().to_vec())
            .restore()
            .expect("validate and restore meta query");
        assert_eq!(restored, query);
    }
}
