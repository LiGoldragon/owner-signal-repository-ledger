# owner-signal-repository-ledger Architecture

`owner-signal-repository-ledger` is the owner-only authority contract for
`repository-ledger`.

It exists because privileged repository policy must be separated by contract
and socket from ordinary repository event/query traffic. The daemon has one
owner listener actor for this contract and one ordinary listener actor for
`signal-repository-ledger`.

## MUST IMPLEMENT — three-layer migration

This contract is migrating to the three-layer model affirmed
2026-05-20 per
`primary/reports/designer/246-v4-bundled-fix-deep-design-with-examples.md`
and `primary/reports/designer/248-three-layer-changes-for-operators.md`.

**Layer 1 — Contract Operations on the wire (this crate).** Drop the
`Mutate Configure` / `Retract Retire` style wrapping entirely (per
`intent/component-shape.nota` 19:30Z, `Mutate Configure` and
`Retract Retire` are grammatically wrong — you mutate or retract
nouns, not verbs). Use contract-local owner verbs directly:
`Configure` (for daemon configuration mutation), `Register` (for
repository registration), `Retire` (for repository retirement),
`Start` / `Stop` (for lifecycle), and any other owner-domain verbs in
verb-form. Payloads stay nouns: `Configure` carries `Configuration`,
`Register` carries `Registration`, `Retire` carries the identifier of
the repository being retired.

**Layer 2 — Component Commands.** Lowering from contract operations
to typed Component Commands (`Configure` →
`LedgerCommand::MutatePolicyState`, `Register` →
`LedgerCommand::AssertRepositoryRecord` plus
`LedgerCommand::MutateRepositoryPolicy`, `Retire` →
`LedgerCommand::RetractRepositoryRecord`, etc.) lives in the
`repository-ledger` daemon.

**Layer 3 — Sema classification.** Each Component Command projects to
a payloadless Sema class label via `ToSemaOperation` for observation.

**Frame layer.** The dependency on `signal-core` shifts to
`signal-frame`.

References:
- `primary/reports/designer/246-v4-bundled-fix-deep-design-with-examples.md`
- `primary/reports/designer/248-three-layer-changes-for-operators.md`
- `primary/skills/component-triad.md` §"Verbs come in three layers"
- `primary/skills/contract-repo.md` §"Public contracts use contract-local operation verbs"

**Note to remover:** when the refactor lands, remove this section and
add a `## Migration history — three-layer model (2026-05-XX)`
paragraph noting the shape change.

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

- Owner-only operations (after migration) are contract-local verbs in
  verb form on the wire (Layer 1). The daemon owns the typed
  Component Commands (Layer 2) that project to Sema classes
  (Layer 3 — `Mutate` / `Retract` / `Assert`) for observation only.
- This crate may reuse ordinary contract nouns when the same typed
  noun crosses the owner boundary, but owner authority lives here.
