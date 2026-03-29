-- Lualine — statusline that picks up the active colorscheme automatically.

return {
  {
    "nvim-lualine/lualine.nvim",
    event        = "VimEnter",   -- Load early so the bar appears on startup.
    dependencies = {
      -- Devicons for filetype icons in the statusline.
      { "nvim-tree/nvim-web-devicons", lazy = true },
    },
    opts = {
      options = {
        -- "auto" inherits colours from the active colorscheme — works with all
        -- three theme families (Catppuccin, Rosé Pine, Kanagawa).
        theme             = "auto",
        component_separators = { left = "", right = "" },
        section_separators   = { left = "", right = "" },
        globalstatus         = true,   -- Single statusline across all windows.
        disabled_filetypes   = {
          statusline = { "dashboard", "alpha", "starter" },
        },
      },

      -- ── Sections ──────────────────────────────────────────────────────────
      sections = {
        lualine_a = { "mode" },
        lualine_b = {
          "branch",
          {
            "diff",
            symbols = { added = " ", modified = " ", removed = " " },
          },
          {
            "diagnostics",
            symbols = { error = " ", warn = " ", info = " ", hint = "󰠠 " },
          },
        },
        lualine_c = {
          {
            "filename",
            path = 1,    -- Show relative path (0 = filename only, 2 = absolute).
            symbols = {
              modified = "  ",
              readonly = "",
              unnamed  = "",
            },
          },
        },
        lualine_x = {
          -- Show the active LSP client(s).
          {
            function()
              local clients = vim.lsp.get_active_clients({ bufnr = 0 })
              if #clients == 0 then return "" end
              local names = {}
              for _, c in ipairs(clients) do
                table.insert(names, c.name)
              end
              return " " .. table.concat(names, ", ")
            end,
            color = { fg = "#a6e3a1" },
          },
          "encoding",
          "fileformat",
          "filetype",
        },
        lualine_y = { "progress" },
        lualine_z = { "location" },
      },

      -- ── Inactive window sections ──────────────────────────────────────────
      inactive_sections = {
        lualine_a = {},
        lualine_b = {},
        lualine_c = { { "filename", path = 1 } },
        lualine_x = { "location" },
        lualine_y = {},
        lualine_z = {},
      },

      -- ── Extensions ────────────────────────────────────────────────────────
      -- Pre-built integrations that adjust the statusline in special buffers.
      extensions = { "quickfix", "man", "lazy" },
    },
  },
}
