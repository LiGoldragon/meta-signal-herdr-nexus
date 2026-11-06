//! The meta herdr-nexus contract: binary archives round-trip in every build, and datom text
//! reads, prints and refuses wrong shapes when the `datom` feature is on.

use meta_signal_herdr_nexus::{Query, Response};
use rkyv::rancor::Error;

#[cfg(feature = "datom")]
mod text {
    use datom_codec::{Actualizing, Budget, Composing, Datomizable, Path, Potential};
    use meta_signal_herdr_nexus::{Query, Response};
    use protos::{Compactable, Protosizable, ReaderBudget};

    fn budget() -> Budget {
        Budget {
            remaining: 4096,
            reader: ReaderBudget::try_from(4096).expect("the reader budget is within range"),
            depth: 0,
            maximum_depth: 4096,
        }
    }

    fn read<T: Composing>(source: &str) -> Result<T, datom_codec::Error> {
        Potential::<T>::from(source).actualize(&mut budget())
    }

    fn print<T: Datomizable>(value: &T) -> String {
        value.datomize(Path::new()).protosize().compact()
    }

    fn round_trip<T: Composing + Datomizable + std::fmt::Debug + PartialEq>(source: &str) {
        let value = read::<T>(source).unwrap_or_else(|error| panic!("{source}: {error:?}"));
        assert_eq!(
            read::<T>(&print(&value)).expect("the printing reads back"),
            value,
            "{source}"
        );
    }

    #[test]
    fn every_configure_shape_round_trips() {
        round_trip::<Query>("Configure.HerdrPath.«/run/user/1001/herdr.sock»");
        round_trip::<Query>("Configure.CallTimeout.5000");
        round_trip::<Query>("Configure.TextBudget.{ 1024 16 }");
        round_trip::<Query>("Configure.SubscriberCapacity.8");
        round_trip::<Query>("Configure.Peer.{ peerOne { 1000 4242 } }");
        round_trip::<Query>("Configure.RemovePeer.peerOne");
    }

    #[test]
    fn responses_round_trip() {
        round_trip::<Response>("Configured.{ CallTimeout.5000 Nexus Ready }");
        round_trip::<Response>("Configured.{ Polling.100 Translator Awaiting.[ HerdrPath ] }");
        round_trip::<Response>("ConfigureRefused.UnknownPeer.peerOne");
    }

    #[test]
    fn wrong_shapes_and_unknown_variants_are_refused() {
        assert!(read::<Query>("Configure.Frobnicate.1").is_err());
        assert!(read::<Query>("Configure.TextBudget.{ 1024 }").is_err());
        assert!(read::<Query>("Configure.CallTimeout.fast").is_err());
        assert!(read::<Response>("Configured.{ CallTimeout.5000 Nobody Ready }").is_err());
    }
}

fn archive_round_trip<T>(value: &T)
where
    T: rkyv::Archive
        + PartialEq
        + std::fmt::Debug
        + for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                Error,
            >,
        >,
    T::Archived: rkyv::Deserialize<T, rkyv::rancor::Strategy<rkyv::de::Pool, Error>>
        + for<'a> rkyv::bytecheck::CheckBytes<rkyv::api::high::HighValidator<'a, Error>>,
{
    let bytes = rkyv::to_bytes::<Error>(value).expect("archive");
    assert_eq!(
        &rkyv::from_bytes::<T, Error>(&bytes).expect("restore"),
        value
    );
    assert!(rkyv::from_bytes::<T, Error>(&bytes[..bytes.len() - 1]).is_err() || bytes.len() < 2);
}

#[test]
fn queries_and_responses_survive_the_binary_archive() {
    use meta_signal_herdr_nexus::{
        ConfigurationRefusal, Configured_Data, Owner, Readiness, Set, Timeout,
    };
    archive_round_trip(&Query::Configure(Set::CallTimeout(Timeout(5000))));
    archive_round_trip(&Response::Configured(Configured_Data {
        set: Set::Polling(Timeout(100)),
        owner: Owner::Translator,
        readiness: Readiness::Ready,
    }));
}
