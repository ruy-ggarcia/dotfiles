-- Autocommands
-- Small, focused automatic behaviours triggered by editor events.

local autocmd = vim.api.nvim_create_autocmd
local augroup = vim.api.nvim_create_augroup

-- ─── Highlight on yank ────────────────────────────────────────────────────────
-- Briefly flash the yanked region so you can see exactly what was copied.
local yank_group = augroup("HighlightOnYank", { clear = true })
autocmd("TextYankPost", {
  group    = yank_group,
  desc     = "Highlight yanked text for 200ms",
  callback = function()
    vim.highlight.on_yank({ higroup = "IncSearch", timeout = 200 })
  end,
})

-- ─── Remove trailing whitespace on save ──────────────────────────────────────
local whitespace_group = augroup("TrimWhitespace", { clear = true })
autocmd("BufWritePre", {
  group    = whitespace_group,
  desc     = "Strip trailing whitespace before saving",
  pattern  = "*",
  callback = function()
    -- Save and restore cursor position so the view doesn't jump.
    local pos = vim.api.nvim_win_get_cursor(0)
    vim.cmd([[%s/\s\+$//e]])
    vim.api.nvim_win_set_cursor(0, pos)
  end,
})

-- ─── Return to last edit position ────────────────────────────────────────────
-- When re-opening a file, jump back to where you left off.
local last_pos_group = augroup("LastEditPosition", { clear = true })
autocmd("BufReadPost", {
  group    = last_pos_group,
  desc     = "Jump to last known cursor position",
  callback = function()
    local mark = vim.api.nvim_buf_get_mark(0, '"')
    local line_count = vim.api.nvim_buf_line_count(0)
    -- Only restore if the mark is still within the file.
    if mark[1] > 0 and mark[1] <= line_count then
      vim.api.nvim_win_set_cursor(0, mark)
    end
  end,
})

-- ─── Auto-resize splits on terminal resize ───────────────────────────────────
local resize_group = augroup("AutoResize", { clear = true })
autocmd("VimResized", {
  group    = resize_group,
  desc     = "Equalize split sizes on terminal resize",
  callback = function()
    vim.cmd("tabdo wincmd =")
  end,
})

-- ─── Close certain buffers with just 'q' ─────────────────────────────────────
-- Utility windows (help, quickfix, man pages) should close without :q!.
local close_group = augroup("QuickClose", { clear = true })
autocmd("FileType", {
  group    = close_group,
  desc     = "Close utility buffers with q",
  pattern  = { "help", "qf", "man", "lspinfo", "checkhealth" },
  callback = function(event)
    vim.bo[event.buf].buflisted = false
    vim.keymap.set("n", "q", "<cmd>close<cr>", { buffer = event.buf, silent = true })
  end,
})

-- ─── Set wrap & spell in text files ──────────────────────────────────────────
local text_group = augroup("TextFiles", { clear = true })
autocmd("FileType", {
  group    = text_group,
  desc     = "Enable wrap and spell in prose files",
  pattern  = { "markdown", "text", "gitcommit" },
  callback = function()
    vim.opt_local.wrap  = true
    vim.opt_local.spell = true
  end,
})
