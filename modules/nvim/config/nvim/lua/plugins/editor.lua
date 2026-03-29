-- Editor quality-of-life plugins
-- autopairs — auto-close brackets, quotes, and tags
-- nvim-surround — add / change / delete surrounding pairs
-- Comment.nvim — toggle line and block comments with a single binding

return {
  -- ── Auto-pairs ────────────────────────────────────────────────────────────
  -- Automatically inserts the closing bracket/quote when you type the opening one.
  {
    "windwp/nvim-autopairs",
    event = "InsertEnter",
    opts  = {
      check_ts                  = true,   -- Use Treesitter to avoid pairing inside strings/comments.
      ts_config                 = {
        lua  = { "string" },              -- Don't auto-pair inside Lua strings.
        javascript = { "template_string" },
      },
      disable_filetype          = { "TelescopePrompt", "spectre_panel" },
      fast_wrap                 = {
        -- <M-e> wraps the next token with the next typed pair character.
        map       = "<M-e>",
        chars     = { "{", "[", "(", '"', "'" },
        pattern   = [=[[%'%"%>%]%)%}%,]]=],
        end_key   = "$",
        keys      = "qwertyuiopzxcvbnmasdfghjkl",
        check_comma = true,
        highlight = "Search",
        highlight_grey = "Comment",
      },
    },
    config = function(_, opts)
      local autopairs = require("nvim-autopairs")
      autopairs.setup(opts)

      -- Integrate with nvim-cmp: press Enter on a completion item and the
      -- closing bracket is inserted at the right position automatically.
      local ok_cmp, cmp = pcall(require, "cmp")
      if ok_cmp then
        local cmp_autopairs = require("nvim-autopairs.completion.cmp")
        cmp.event:on("confirm_done", cmp_autopairs.on_confirm_done())
      end
    end,
  },

  -- ── nvim-surround ─────────────────────────────────────────────────────────
  -- Operators: ys (add), cs (change), ds (delete) around a motion or object.
  --   ysw"  → surround word with "..."
  --   cs"'  → change surrounding " to '
  --   ds)   → delete surrounding ()
  {
    "kylechui/nvim-surround",
    version = "*",    -- Use the latest stable release tag.
    event   = "VeryLazy",
    opts    = {},
  },

  -- ── Comment.nvim ──────────────────────────────────────────────────────────
  -- gcc  → toggle comment on current line
  -- gc   → toggle comment on a motion (e.g. gcap = comment a paragraph)
  -- gc   → toggle comment on a visual selection
  {
    "numToStr/Comment.nvim",
    event = { "BufReadPost", "BufNewFile" },
    opts  = {
      -- Allow padding between the comment marker and the text.
      padding   = true,
      -- Whether to create mappings in normal mode.
      sticky    = true,
      -- Ignore blank lines when commenting.
      ignore    = nil,
      toggler   = {
        line  = "gcc",   -- Toggle current line comment
        block = "gbc",   -- Toggle current block comment
      },
      opleader  = {
        line  = "gc",    -- Line comment in operator-pending mode
        block = "gb",    -- Block comment in operator-pending mode
      },
      mappings  = {
        basic     = true,   -- gcc / gbc / gc{motion} / gb{motion}
        extra     = true,   -- gco (add below), gcO (add above), gcA (end of line)
      },
      pre_hook  = nil,
      post_hook = nil,
    },
  },
}
