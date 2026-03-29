-- Colorscheme plugins
-- All three theme families supported by the dotfiles theme system.
-- The active theme is applied by lua/core/theme.lua (rendered from .tera).

return {
  -- ── Catppuccin ──────────────────────────────────────────────────────────────
  -- Soothing pastel theme; supports latte (light), frappé, macchiato, mocha.
  {
    "catppuccin/nvim",
    name     = "catppuccin",
    lazy     = true,
    priority = 1000,
    opts = {
      flavour            = "mocha", -- overridden at runtime by core.theme
      background         = { light = "latte", dark = "mocha" },
      transparent_background = false,
      integrations = {
        cmp        = true,
        gitsigns   = true,
        treesitter = true,
        telescope  = { enabled = true },
        mason      = true,
        lualine    = true,
        native_lsp = {
          enabled            = true,
          virtual_text       = { errors = { "italic" }, warnings = { "italic" } },
          underlines         = { errors = { "underline" }, warnings = { "underline" } },
          inlay_hints        = { background = true },
        },
      },
    },
  },

  -- ── Rosé Pine ───────────────────────────────────────────────────────────────
  -- Naturally muted tones; variants: main, moon (darker), dawn (light).
  {
    "rose-pine/neovim",
    name     = "rose-pine",
    lazy     = true,
    priority = 1000,
    opts = {
      variant      = "main",  -- overridden at runtime by core.theme
      dark_variant = "main",
      disable_background   = false,
      disable_float_background = false,
    },
  },

  -- ── Kanagawa ────────────────────────────────────────────────────────────────
  -- Inspired by the Great Wave; variants: wave (dark), dragon (darker), lotus (light).
  {
    "rebelot/kanagawa.nvim",
    name     = "kanagawa",
    lazy     = true,
    priority = 1000,
    opts = {
      theme    = "wave",  -- overridden at runtime by core.theme
      undercurl = true,
      commentStyle = { italic = true },
      functionStyle = {},
      keywordStyle  = { italic = true },
      statementStyle = { bold = true },
      transparent = false,
      dimInactive = false,
    },
  },
}
