-- Key mappings
-- All custom bindings live here to keep init.lua and plugin files clean.

-- Set Space as the leader key (must happen before lazy.nvim loads plugins).
vim.g.mapleader      = " "
vim.g.maplocalleader = " "

local map = vim.keymap.set

-- ─── File operations ─────────────────────────────────────────────────────────
map("n", "<leader>w", "<cmd>w<cr>",  { desc = "Save file" })
map("n", "<leader>q", "<cmd>q<cr>",  { desc = "Quit window" })
map("n", "<leader>Q", "<cmd>qa<cr>", { desc = "Quit all windows" })

-- ─── Better escape ────────────────────────────────────────────────────────────
-- Clear search highlighting with Escape in normal mode.
map("n", "<Esc>", "<cmd>nohlsearch<cr>", { desc = "Clear search highlight" })

-- ─── Window navigation ────────────────────────────────────────────────────────
map("n", "<C-h>", "<C-w>h", { desc = "Move to left window" })
map("n", "<C-j>", "<C-w>j", { desc = "Move to lower window" })
map("n", "<C-k>", "<C-w>k", { desc = "Move to upper window" })
map("n", "<C-l>", "<C-w>l", { desc = "Move to right window" })

-- ─── Buffer navigation ────────────────────────────────────────────────────────
map("n", "<S-h>", "<cmd>bprevious<cr>", { desc = "Previous buffer" })
map("n", "<S-l>", "<cmd>bnext<cr>",     { desc = "Next buffer" })
map("n", "<leader>bd", "<cmd>bdelete<cr>", { desc = "Delete buffer" })

-- ─── File explorer ────────────────────────────────────────────────────────────
-- netrw toggle (replaced by your file-tree plugin if you add one).
map("n", "<leader>e", "<cmd>Explore<cr>", { desc = "Toggle file explorer" })

-- ─── Window splits ────────────────────────────────────────────────────────────
map("n", "<leader>sv", "<cmd>vsplit<cr>", { desc = "Split vertically" })
map("n", "<leader>sh", "<cmd>split<cr>",  { desc = "Split horizontally" })
map("n", "<leader>se", "<C-w>=",          { desc = "Equalise split sizes" })
map("n", "<leader>sx", "<cmd>close<cr>",  { desc = "Close current split" })

-- ─── Move lines (visual mode) ─────────────────────────────────────────────────
-- Drag selected lines up or down while re-indenting them correctly.
map("v", "J", ":m '>+1<cr>gv=gv", { desc = "Move selection down" })
map("v", "K", ":m '<-2<cr>gv=gv", { desc = "Move selection up" })

-- ─── Indentation (keep visual selection) ─────────────────────────────────────
map("v", "<", "<gv", { desc = "Indent left (keep selection)" })
map("v", ">", ">gv", { desc = "Indent right (keep selection)" })

-- ─── Paste without losing clipboard ──────────────────────────────────────────
-- In visual mode, pasting over text won't clobber the unnamed register.
map("v", "p", '"_dP', { desc = "Paste without overwriting clipboard" })

-- ─── Scroll & centre ──────────────────────────────────────────────────────────
-- Centre the cursor after half-page jumps for spatial awareness.
map("n", "<C-d>", "<C-d>zz", { desc = "Scroll down (centred)" })
map("n", "<C-u>", "<C-u>zz", { desc = "Scroll up (centred)" })

-- ─── Search & centre ──────────────────────────────────────────────────────────
-- Keep the found match in the middle of the screen.
map("n", "n", "nzzzv", { desc = "Next match (centred)" })
map("n", "N", "Nzzzv", { desc = "Previous match (centred)" })

-- ─── Diagnostics ──────────────────────────────────────────────────────────────
map("n", "[d", vim.diagnostic.goto_prev,  { desc = "Previous diagnostic" })
map("n", "]d", vim.diagnostic.goto_next,  { desc = "Next diagnostic" })
map("n", "<leader>d", vim.diagnostic.open_float, { desc = "Show diagnostic float" })
