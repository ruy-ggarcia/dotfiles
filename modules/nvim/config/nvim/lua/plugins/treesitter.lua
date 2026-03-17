-- Treesitter — syntax highlighting, indentation, and structural text objects.

return {
  {
    "nvim-treesitter/nvim-treesitter",
    build  = ":TSUpdate",   -- Keep parsers up to date after plugin updates.
    event  = { "BufReadPost", "BufNewFile" },
    dependencies = {
      -- Text-object motions powered by the AST (optional but highly recommended).
      "nvim-treesitter/nvim-treesitter-textobjects",
    },
    opts = {
      -- Parsers installed automatically on first use.
      ensure_installed = {
        -- Systems
        "c", "cpp", "rust",
        -- Web
        "html", "css", "javascript", "typescript", "tsx", "json", "jsonc",
        -- Scripting / config
        "lua", "python", "bash", "fish",
        -- Data formats
        "toml", "yaml",
        -- Documentation / prose
        "markdown", "markdown_inline",
        -- Version control
        "git_config", "gitcommit", "gitignore",
        -- Build tools
        "make", "cmake",
        -- NeoVim internals
        "vim", "vimdoc", "query",
      },

      -- Install parsers synchronously (only applied during headless runs).
      sync_install = false,

      -- Automatically install missing parsers when opening a new buffer.
      auto_install = true,

      -- ── Modules ─────────────────────────────────────────────────────────────
      highlight = {
        enable = true,
        -- Disable for very large files to keep performance snappy.
        disable = function(_, buf)
          local max_filesize = 200 * 1024  -- 200 KB
          local ok, stats = pcall(vim.loop.fs_stat, vim.api.nvim_buf_get_name(buf))
          if ok and stats and stats.size > max_filesize then
            return true
          end
        end,
        -- Let Treesitter handle all highlighting; disable legacy regex highlighting.
        additional_vim_regex_highlighting = false,
      },

      indent = {
        enable = true,   -- Use Treesitter for indentation (`=` operator, auto-indent).
      },

      -- Text-objects: select / move / swap by syntactic units (function, class, etc.)
      textobjects = {
        select = {
          enable    = true,
          lookahead = true,   -- Jump forward to the next match automatically.
          keymaps = {
            ["af"] = "@function.outer",
            ["if"] = "@function.inner",
            ["ac"] = "@class.outer",
            ["ic"] = "@class.inner",
            ["aa"] = "@parameter.outer",
            ["ia"] = "@parameter.inner",
          },
        },
        move = {
          enable              = true,
          set_jumps           = true,   -- Add to the jump list so <C-o>/<C-i> work.
          goto_next_start     = { ["]f"] = "@function.outer", ["]c"] = "@class.outer" },
          goto_previous_start = { ["[f"] = "@function.outer", ["[c"] = "@class.outer" },
        },
      },
    },
    config = function(_, opts)
      require("nvim-treesitter.configs").setup(opts)
    end,
  },
}
