-- Completion — nvim-cmp with LuaSnip as the snippet engine.
-- Sources: LSP, current buffer, file paths, and LuaSnip snippets.

return {
  -- ── Snippet engine ────────────────────────────────────────────────────────
  {
    "L3MON4D3/LuaSnip",
    version      = "v2.*",
    build        = "make install_jsregexp",   -- Optional: enables regex in snippets.
    dependencies = {
      -- A large community snippet library (optional but very useful).
      "rafamadriz/friendly-snippets",
    },
    config = function()
      require("luasnip.loaders.from_vscode").lazy_load()
    end,
  },

  -- ── Completion engine ─────────────────────────────────────────────────────
  {
    "hrsh7th/nvim-cmp",
    event = { "InsertEnter", "CmdlineEnter" },
    dependencies = {
      -- Completion sources
      "hrsh7th/cmp-nvim-lsp",      -- LSP completions
      "hrsh7th/cmp-buffer",        -- Words from the current buffer
      "hrsh7th/cmp-path",          -- File system paths
      "saadparwaiz1/cmp_luasnip",  -- LuaSnip snippet completions
      "hrsh7th/cmp-cmdline",       -- Command-line completions
      -- Snippet engine (declared above, but must be listed here too)
      "L3MON4D3/LuaSnip",
    },
    config = function()
      local cmp     = require("cmp")
      local luasnip = require("luasnip")

      -- Helper: check if the cursor is at a word boundary (not at pure whitespace).
      local has_words_before = function()
        unpack = unpack or table.unpack
        local line, col = unpack(vim.api.nvim_win_get_cursor(0))
        return col ~= 0
          and vim.api.nvim_buf_get_lines(0, line - 1, line, true)[1]:sub(col, col):match("%s") == nil
      end

      cmp.setup({
        snippet = {
          expand = function(args)
            luasnip.lsp_expand(args.body)
          end,
        },

        -- ── Completion window ──────────────────────────────────────────────
        window = {
          completion    = cmp.config.window.bordered(),
          documentation = cmp.config.window.bordered(),
        },

        -- ── Key mappings ───────────────────────────────────────────────────
        mapping = cmp.mapping.preset.insert({
          -- Scroll documentation
          ["<C-b>"] = cmp.mapping.scroll_docs(-4),
          ["<C-f>"] = cmp.mapping.scroll_docs(4),

          -- Manual trigger / cancel
          ["<C-Space>"] = cmp.mapping.complete(),
          ["<C-e>"]     = cmp.mapping.abort(),

          -- Confirm the selected item. `select = false` means Enter only confirms
          -- an explicitly selected item; it won't silently pick the first entry.
          ["<CR>"] = cmp.mapping.confirm({ select = false }),

          -- Tab: move forward through items or expand / jump snippets.
          ["<Tab>"] = cmp.mapping(function(fallback)
            if cmp.visible() then
              cmp.select_next_item()
            elseif luasnip.expand_or_jumpable() then
              luasnip.expand_or_jump()
            elseif has_words_before() then
              cmp.complete()
            else
              fallback()
            end
          end, { "i", "s" }),

          -- Shift-Tab: move backward through items or jump snippets in reverse.
          ["<S-Tab>"] = cmp.mapping(function(fallback)
            if cmp.visible() then
              cmp.select_prev_item()
            elseif luasnip.jumpable(-1) then
              luasnip.jump(-1)
            else
              fallback()
            end
          end, { "i", "s" }),
        }),

        -- ── Sources (ordered by priority) ─────────────────────────────────
        sources = cmp.config.sources({
          { name = "nvim_lsp", priority = 1000 },
          { name = "luasnip",  priority = 750 },
          { name = "buffer",   priority = 500, keyword_length = 3 },
          { name = "path",     priority = 250 },
        }),

        -- ── Formatting ────────────────────────────────────────────────────
        formatting = {
          format = function(entry, item)
            -- Show a short label for each source next to the completion item.
            local source_labels = {
              nvim_lsp = "[LSP]",
              luasnip  = "[Snip]",
              buffer   = "[Buf]",
              path     = "[Path]",
            }
            item.menu = source_labels[entry.source.name] or ""
            return item
          end,
        },

        -- ── Experimental ──────────────────────────────────────────────────
        experimental = {
          ghost_text = true,   -- Show inline preview of the top completion candidate.
        },
      })

      -- ── Command-line completions ───────────────────────────────────────
      -- '/' search — complete from buffer words.
      cmp.setup.cmdline({ "/", "?" }, {
        mapping = cmp.mapping.preset.cmdline(),
        sources = { { name = "buffer" } },
      })

      -- ':' commands — complete from path and Ex commands.
      cmp.setup.cmdline(":", {
        mapping = cmp.mapping.preset.cmdline(),
        sources = cmp.config.sources(
          { { name = "path" } },
          { { name = "cmdline" } }
        ),
      })
    end,
  },
}
