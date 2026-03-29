-- Telescope — fuzzy finder for files, grep, buffers, and more.

return {
  {
    "nvim-telescope/telescope.nvim",
    branch       = "0.1.x",
    cmd          = "Telescope",
    dependencies = {
      "nvim-lua/plenary.nvim",
      -- Native C fzf sorter — significantly faster than the Lua fallback.
      {
        "nvim-telescope/telescope-fzf-native.nvim",
        build = "make",
        -- Only load if make is available on the system.
        cond = function()
          return vim.fn.executable("make") == 1
        end,
      },
    },
    keys = {
      -- ── File pickers ──────────────────────────────────────────────────────
      { "<leader>ff", "<cmd>Telescope find_files<cr>",              desc = "Find files" },
      { "<leader>fg", "<cmd>Telescope live_grep<cr>",               desc = "Live grep" },
      { "<leader>fw", "<cmd>Telescope grep_string<cr>",             desc = "Grep word under cursor" },
      { "<leader>fr", "<cmd>Telescope oldfiles<cr>",                desc = "Recent files" },

      -- ── Buffer / editor pickers ───────────────────────────────────────────
      { "<leader>fb", "<cmd>Telescope buffers<cr>",                 desc = "Buffers" },
      { "<leader>fs", "<cmd>Telescope current_buffer_fuzzy_find<cr>", desc = "Search in buffer" },

      -- ── Meta pickers ──────────────────────────────────────────────────────
      { "<leader>fc", "<cmd>Telescope commands<cr>",                desc = "Commands" },
      { "<leader>fk", "<cmd>Telescope keymaps<cr>",                 desc = "Keymaps" },
      { "<leader>fh", "<cmd>Telescope help_tags<cr>",               desc = "Help tags" },
      { "<leader>fm", "<cmd>Telescope marks<cr>",                   desc = "Marks" },
      { "<leader>fd", "<cmd>Telescope diagnostics<cr>",             desc = "Diagnostics" },
    },
    opts = function()
      local actions = require("telescope.actions")
      return {
        defaults = {
          -- Sort by modification time by default so recent files surface first.
          file_sorter      = require("telescope.sorters").get_fuzzy_file,
          generic_sorter   = require("telescope.sorters").get_generic_fuzzy_sorter,
          path_display     = { "truncate" },
          sorting_strategy = "ascending",
          layout_config    = {
            horizontal = { prompt_position = "top", preview_width = 0.55 },
            width  = 0.87,
            height = 0.80,
          },
          -- Prompt appearance.
          prompt_prefix   = "   ",
          selection_caret = "  ",
          entry_prefix    = "  ",
          -- Keybindings inside the Telescope window.
          mappings = {
            i = {
              ["<C-k>"]  = actions.move_selection_previous,
              ["<C-j>"]  = actions.move_selection_next,
              ["<C-q>"]  = actions.send_selected_to_qflist + actions.open_qflist,
              ["<Esc>"]  = actions.close,
            },
          },
        },
        extensions = {
          fzf = {
            fuzzy                   = true,
            override_generic_sorter = true,
            override_file_sorter    = true,
            case_mode               = "smart_case",
          },
        },
      }
    end,
    config = function(_, opts)
      local telescope = require("telescope")
      telescope.setup(opts)
      -- Load the fzf-native extension if it compiled successfully.
      pcall(telescope.load_extension, "fzf")
    end,
  },
}
