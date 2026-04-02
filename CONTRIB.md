# Contributing to dotfiles

Thanks for contributing.

This repo is intentionally small: a Rust CLI, docs-first planning, GitHub issue templates, and CI centered on `cargo fmt`, `cargo clippy`, and `cargo test`.

We absolutely welcome contributions. The easiest way to help is to keep changes focused and grounded in what the repo supports today.

## Start with the current reality

Before changing behavior, skim the sources of truth that already define the project:

- `docs/PRD.md` — product intent and boundaries
- `docs/ROADMAP.md` — milestone status
- `docs/decisions.md` and `docs/adrs/` — architecture decisions
- `.github/workflows/ci.yml` — checks that actually run in CI
- `.github/workflows/release.yml` — release pipeline shape
- `install.sh` — current installation and distribution path

If your change affects user-visible behavior, update the relevant docs in the same pass.

## Open an issue the practical way

Blank issues are disabled. Use the templates in `.github/ISSUE_TEMPLATE/`:

- **Bug Report** — broken behavior, regressions, crashes
- **Feature** — net-new user-facing functionality
- **Enhancement** — improvements to existing behavior or contributor workflow

The templates already ask for the useful stuff: the problem, scope, acceptance/DoD, references, environment, user impact, and any alternatives you've already considered. Lean on them — they are there to make review easier, not to make contribution harder.

## Label vocabulary already in use

The issue templates apply type labels automatically. Maintainers apply and adjust the rest during triage.

- type labels:
  - `type: bug`
  - `type: feature`
  - `type: enhancement`

- priority labels:
  - `priority: critical`
  - `priority: high`
  - `priority: medium`
  - `priority: low`

If you discuss scope in an issue or PR, reuse the same vocabulary instead of inventing parallel labels.

## Keep scope tight

Small, reviewable changes are the default:

- one problem per PR
- link the PR to a real issue when possible
- don't mix docs cleanup, refactors, and feature work unless they truly belong together

The PR template expects an issue link, a change summary, test steps, DoD, and technical notes. Please use it.

## Pull request expectations

The template at `.github/PULL_REQUEST_TEMPLATE.md` is the quality bar. A solid PR includes:

- the related issue
- a concise outcome-first summary
- explicit validation steps
- scope limits, tradeoffs, or follow-ups

Before opening a PR, check that:

- scope still matches the linked issue
- the checks relevant to your change pass locally
- docs or ADRs were updated when behavior or decisions changed
- no accidental breaking change slipped in

## Local checks that matter

These match the repo's actual CI checks:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked
```

If you are touching release or distribution behavior, also inspect:

```bash
cargo build --release --locked
```

Don't run unrelated heavy validation just to look busy. Run the checks that map to your change.

## Documentation rules

- document the repo as it exists now, not as a future wish
- when code and docs disagree, fix the mismatch explicitly
- keep README-level docs practical for first-time users
- put long-form rationale in ADRs and decision docs, not in every user-facing document

## Practical codebase map

- main entrypoint: `src/main.rs`
- interaction flow: `src/tui.rs`
- detection and font/theme scanning: `src/scanner.rs`, `src/font.rs`
- rendering and apply logic: `src/engine.rs`, `src/template.rs`, `src/symlink.rs`
- templates: `modules/`
- built-in themes: `themes/`

Current implemented surface area is smaller than the PRD's full target. Today the code supports Bash, Zsh, Kitty, Alacritty, theme seeding, and Nerd Font scanning. That is not a limitation on contribution — it is simply the best place to anchor discussions and changes.

## When docs or ADRs should move with the code

Update docs when you change:

- supported platforms
- supported tools
- installation flow
- generated file locations
- CI or release behavior
- contributor workflow

Add or update an ADR when the change is architectural, cross-cutting, or establishes a long-term convention.

## A good contribution flow

1. Open or pick the right issue template.
2. Confirm scope and acceptance criteria.
3. Make the smallest coherent change.
4. Run the relevant local checks.
5. Update docs if behavior or workflow changed.
6. Open a PR using the repository template.

That's it. Keep contributions honest, focused, and easy to review.
