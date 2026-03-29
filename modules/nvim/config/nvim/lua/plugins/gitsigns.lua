-- Git signs — shows git diff decorations in the sign column and provides
-- hunk-level staging, resetting, and navigation.

return {
  {
    "lewis6991/gitsigns.nvim",
    event = { "BufReadPost", "BufNewFile" },
    opts  = {
      signs = {
        add          = { text = "▎" },
        change       = { text = "▎" },
        delete       = { text = "" },
        topdelete    = { text = "" },
        changedelete = { text = "▎" },
        untracked    = { text = "▎" },
      },

      -- Show inline git blame for the current line after a short delay.
      current_line_blame = false,   -- off by default; toggle with <leader>gb
      current_line_blame_opts = {
        virt_text         = true,
        virt_text_pos     = "eol",
        delay             = 1000,
        ignore_whitespace = false,
      },

      -- Highlight the word under the cursor differently from other changes.
      word_diff = false,

      -- Attach to every buffer, including untracked files in a git repo.
      attach_to_untracked = true,

      on_attach = function(buf)
        local gs  = package.loaded.gitsigns
        local map = function(mode, keys, func, desc)
          vim.keymap.set(mode, keys, func, { buffer = buf, desc = "Git: " .. desc })
        end

        -- ── Navigation ──────────────────────────────────────────────────────
        map("n", "]h", function()
          if vim.wo.diff then return "]c" end
          vim.schedule(gs.next_hunk)
          return "<Ignore>"
        end, "Next hunk")

        map("n", "[h", function()
          if vim.wo.diff then return "[c" end
          vim.schedule(gs.prev_hunk)
          return "<Ignore>"
        end, "Previous hunk")

        -- ── Actions ─────────────────────────────────────────────────────────
        map("n", "<leader>hs", gs.stage_hunk,           "Stage hunk")
        map("n", "<leader>hr", gs.reset_hunk,           "Reset hunk")
        map("n", "<leader>hS", gs.stage_buffer,         "Stage buffer")
        map("n", "<leader>hu", gs.undo_stage_hunk,      "Undo stage hunk")
        map("n", "<leader>hR", gs.reset_buffer,         "Reset buffer")
        map("n", "<leader>hp", gs.preview_hunk,         "Preview hunk")
        map("n", "<leader>hd", gs.diffthis,             "Diff this")
        map("n", "<leader>hD", function() gs.diffthis("~") end, "Diff this (HEAD)")

        -- Stage / reset a range of lines in visual mode.
        map("v", "<leader>hs", function()
          gs.stage_hunk({ vim.fn.line("."), vim.fn.line("v") })
        end, "Stage selected hunk")
        map("v", "<leader>hr", function()
          gs.reset_hunk({ vim.fn.line("."), vim.fn.line("v") })
        end, "Reset selected hunk")

        -- Toggle inline blame.
        map("n", "<leader>gb", gs.toggle_current_line_blame, "Toggle line blame")

        -- ── Text object ─────────────────────────────────────────────────────
        -- `ih` selects the inner hunk (use with d/y/c).
        map({ "o", "x" }, "ih", ":<C-U>Gitsigns select_hunk<cr>", "Select hunk")
      end,
    },
  },
}
