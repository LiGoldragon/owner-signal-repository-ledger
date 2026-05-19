use owner_signal_repository_ledger::{Request, Retirement, SpoolDirectoryPolicy};
use signal_core::{RequestPayload, SignalVerb};
use signal_repository_ledger::Name;

#[test]
fn owner_request_variants_declare_expected_signal_verbs() {
    let retire = Request::Retirement(Retirement {
        repository_name: Name::new("repository-ledger"),
    });
    assert_eq!(retire.signal_verb(), SignalVerb::Retract);

    let spool = Request::SpoolDirectoryPolicy(SpoolDirectoryPolicy {
        path: owner_signal_repository_ledger::FilesystemPath::new(
            "/var/lib/repository-ledger/spool",
        ),
    });
    assert_eq!(spool.signal_verb(), SignalVerb::Mutate);
}
