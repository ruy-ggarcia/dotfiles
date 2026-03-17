-- NeoVim entry point
-- Loads core settings, then bootstraps lazy.nvim and all plugins.

-- ─── Bootstrap lazy.nvim ─────────────────────────────────────────────────────
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({
    "git",
    "clone",
    "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git",
    "--branch=stable",
    lazypath,
  })
end
vim.opt.rtp:prepend(lazypath)

-- ─── Core settings (must run before plugins) ─────────────────────────────────
require("core.options")
require("core.keymaps")
require("core.autocmds")

-- ─── Plugin manager ───────────────────────────────────────────────────────────
-- Loads every file inside lua/plugins/ as a plugin spec.
require("lazy").setup("plugins", {
  change_detection = { notify = false },
  ui = { border = "rounded" },
})

-- ─── Theme (rendered from theme.lua.tera by the dotfiles installer) ──────────
-- This file is generated at install time; it sets up and activates the
-- colorscheme that matches the active dotfiles theme palette.
local ok, err = pcall(require, "core.theme")
if not ok then
  -- Graceful fallback when the installer hasn't rendered the template yet.
  vim.notify(
    "core.theme not found — run the dotfiles installer to apply a theme.\n" .. tostring(err),
    vim.log.levels.WARN
  )
end
