use owner_signal_repository_ledger::{
    OwnerRepositoryLedgerRequest, RetireRepository, SpoolDirectoryPolicy,
};
use signal_core::{RequestPayload, SignalVerb};
use signal_repository_ledger::RepositoryName;

#[test]
fn owner_request_variants_declare_expected_signal_verbs() {
    let retire = OwnerRepositoryLedgerRequest::RetireRepository(RetireRepository {
        repository_name: RepositoryName::new("repository-ledger"),
    });
    assert_eq!(retire.signal_verb(), SignalVerb::Retract);

    let spool = OwnerRepositoryLedgerRequest::SetSpoolDirectoryPolicy(SpoolDirectoryPolicy {
        path: owner_signal_repository_ledger::RepositoryLedgerPath::new(
            "/var/lib/repository-ledger/spool",
        ),
    });
    assert_eq!(spool.signal_verb(), SignalVerb::Mutate);
}
