# ADR-003: Shell RC Injection Strategy

- **Status:** Accepted
- **Date:** 2026-03-29
- **Deciders:** Dotfiles project team

---

## Context

The dotfiles installer must configure the shell prompt for Zsh and Bash. The prompt configuration requires setting up `vcs_info` / `PROMPT_COMMAND`, exporting `DOTFILES_*` variables, and defining helper functions.

The naive approach — replacing `~/.zshrc` and `~/.bashrc` with installer-owned symlinks — destroys any content the user already has in those files: existing aliases, exports, path entries, environment variables, tool initialisation blocks (nvm, conda, rbenv, etc.).

This is unacceptable for a tool whose core value proposition is zero side-effects on the user's environment.

Three strategies were evaluated.

---

## Options Considered

### Option A — Source Guard (append a guarded source line)

Render the prompt configuration to a dedicated fragment file at `~/.config/dotfiles/rendered/prompt.{zsh,bash}`. Append a single guarded line to the user's existing rc file:

```bash
# dotfiles managed — do not remove
[[ -f "$HOME/.config/dotfiles/rendered/prompt.zsh" ]] && source "$HOME/.config/dotfiles/rendered/prompt.zsh"
```

The inject step is idempotent: if the fragment path is already present in the rc file, nothing is appended.

**Pros:**
- Completely non-destructive. The user's existing rc file is never overwritten or replaced.
- Industry standard pattern: nvm, conda, rbenv, Homebrew, Starship all use this approach.
- Idempotent: re-running the installer does not duplicate the source line.
- The fragment file is still managed (rendered, versioned) by the installer. Updating the prompt only requires re-rendering the fragment — no rc file modification needed.

**Cons:**
- The installer does not control the full rc file. Load order depends on the user's existing content.
- If the user deletes or renames the rc file, the fragment is orphaned (though the installer will re-inject on next run).

**Verdict:** ✅ Recommended.

---

### Option B — Managed Block (delimited replace block)

Inject a delimited block bounded by `# BEGIN dotfiles managed` and `# END dotfiles managed` markers. On re-runs, locate and replace the block rather than appending.

**Pros:**
- More idempotent than Option A in theory: the entire block is replaced atomically.
- Can update injected content without touching the surrounding rc file.

**Cons:**
- Requires regex or line-by-line parsing of the rc file — fragile if the user edits the markers.
- Significantly more implementation complexity than Option A.
- No meaningful advantage over Option A since the fragment file is already managed externally — only the source line needs to be injected once.

**Verdict:** Rejected. Adds complexity without adding value over Option A.

---

### Option C — Full Ownership with escape hatch

Remain the owner of `~/.zshrc` (symlink-based, as before). The template sources a user-owned `~/.zshrc.local` for custom content:

```bash
# User customizations
[[ -f ~/.zshrc.local ]] && source ~/.zshrc.local
```

**Pros:**
- Full control over the shell environment.
- Theme variables, prompt, and tool integrations are all in one managed file.

**Cons:**
- Destructive on first installation: overwrites the user's existing rc file.
- Requires the user to migrate existing aliases and exports to a new file (`~/.zshrc.local`).
- Contradicts the PRD non-goal of modifying the system beyond placing configuration files.

**Verdict:** Rejected. Fails the zero-side-effects requirement.

---

## Decision

**Use Option A — Source Guard.**

The installer renders the prompt configuration to `~/.config/dotfiles/rendered/prompt.{zsh,bash}` and appends a single idempotent source line to the user's existing rc file.

The template files are renamed from `.zshrc.tera` / `.bashrc.tera` to `prompt.zsh.tera` / `prompt.bash.tera` to accurately reflect their scope: they are prompt fragments, not full rc replacements.

The `inject_source_line` function in `src/symlink.rs` handles the append logic with idempotency and rc file creation if the file does not yet exist.

---

## Consequences

- The user's existing `~/.zshrc` and `~/.bashrc` are never modified beyond the single appended source line.
- Updating the prompt in a subsequent run only requires re-rendering the fragment — no further rc modification.
- `create_symlink` remains in `src/symlink.rs` for use by other modules (Kitty, Alacritty, Zellij, etc.) that own their target config files outright.
- The backup-on-conflict strategy (`D7`) still applies to those other modules. It does not apply to shell rc files under this decision.
