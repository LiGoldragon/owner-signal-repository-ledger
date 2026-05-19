# owner-signal-repository-ledger Architecture

`owner-signal-repository-ledger` is the owner-only authority contract for
`repository-ledger`.

It exists because privileged repository policy must be separated by contract
and socket from ordinary repository event/query traffic. The daemon has one
owner listener actor for this contract and one ordinary listener actor for
`signal-repository-ledger`.

## MUST IMPLEMENT — signal architecture migration

This contract is migrating to contract-local verbs per
`primary/reports/designer/238-signal-architecture-redirection-contract-local-verbs.md`
and `primary/reports/designer/239-signal-architecture-migration-plan.md`.

Drop the `Mutate Configure` / `Retract Retire` style wrapping (per
`intent/component-shape.nota` 19:30Z, `Mutate Configure` and
`Retract Retire` are grammatically wrong — you mutate or retract
nouns, not verbs). Use contract-local owner verbs directly:
`Configure` (for daemon configuration mutation), `Register` (for
repository registration), `Retire` (for repository retirement), `Start`
/ `Stop` (for lifecycle), and any other owner-domain verbs in
verb-form. Payloads stay nouns: `Configure` carries `Configuration`,
`Register` carries `Registration`, `Retire` carries the identifier of
the repository being retired. Move verb-to-Sema lowering
(`Configure` → `Mutate` policy state, `Register` → `Assert` plus
policy `Mutate`, `Retire` → `Retract` policy row) into the daemon.

References: `primary/reports/designer/238-signal-architecture-redirection-contract-local-verbs.md`,
`primary/reports/designer/239-signal-architecture-migration-plan.md`.

**Note to remover:** when the refactor lands, remove this section and
add a `## Migration history — contract-local verbs (2026-05-XX)`
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

- Owner-only mutations are `Mutate`.
- Owner-only removals are `Retract`.
- This crate may reuse ordinary contract nouns when the same typed noun crosses
  the owner boundary, but owner authority lives here.
