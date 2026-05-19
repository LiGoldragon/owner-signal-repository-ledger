# owner-signal-repository-ledger Architecture

`owner-signal-repository-ledger` is the owner-only authority contract for
`repository-ledger`.

It exists because privileged repository policy must be separated by contract
and socket from ordinary repository event/query traffic. The daemon has one
owner listener actor for this contract and one ordinary listener actor for
`signal-repository-ledger`.

## Owns

- Repository registration and retirement requests.
- Spool directory policy.
- Future mirror target policy records.
- Owner-only request/reply variants declared with `signal_channel!`.

## Does Not Own

- The ordinary receive-hook notification relation.
- The Gitolite server installation.
- Runtime actors or sema-engine storage.

## Constraints

- Owner-only mutations are `Mutate`.
- Owner-only removals are `Retract`.
- This crate may reuse ordinary contract nouns when the same typed noun crosses
  the owner boundary, but owner authority lives here.
