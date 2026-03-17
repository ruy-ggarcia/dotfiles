-- Editor options
-- These settings apply globally to every buffer and window.

local opt = vim.opt

-- ─── Line numbers ─────────────────────────────────────────────────────────────
opt.number         = true   -- Show absolute line number on the current line
opt.relativenumber = true   -- Relative numbers on all other lines (great for jump motions)

-- ─── Indentation ─────────────────────────────────────────────────────────────
opt.tabstop     = 2    -- A <Tab> character looks like 2 spaces
opt.shiftwidth  = 2    -- >> / << indents by 2 spaces
opt.expandtab   = true -- Insert spaces, never actual tab characters
opt.smartindent = true -- Auto-indent when starting a new line

-- ─── Appearance ──────────────────────────────────────────────────────────────
opt.termguicolors = true         -- Enable 24-bit RGB color in the terminal
opt.signcolumn    = "yes"        -- Always show the sign column (avoids layout shifts)
opt.cursorline    = true         -- Highlight the line the cursor is on
opt.scrolloff     = 8            -- Keep 8 lines of context above/below the cursor
opt.sidescrolloff = 8            -- Keep 8 columns of context left/right of the cursor
opt.wrap          = false        -- Don't wrap long lines
opt.colorcolumn   = "120"        -- Visual ruler at column 120

-- ─── Clipboard ───────────────────────────────────────────────────────────────
opt.clipboard = "unnamedplus"   -- Use the system clipboard for all yank/paste

-- ─── Undo ────────────────────────────────────────────────────────────────────
opt.undofile = true   -- Persist undo history across sessions

-- ─── Search ──────────────────────────────────────────────────────────────────
opt.ignorecase = true   -- Case-insensitive search by default
opt.smartcase  = true   -- Switch to case-sensitive when the query has uppercase
opt.hlsearch   = true   -- Highlight all search matches
opt.incsearch  = true   -- Show partial matches as you type

-- ─── Splits ──────────────────────────────────────────────────────────────────
opt.splitbelow = true   -- :split opens below the current window
opt.splitright = true   -- :vsplit opens to the right

-- ─── Performance & UX ────────────────────────────────────────────────────────
opt.updatetime  = 250   -- Faster CursorHold events (used by LSP hover, gitsigns)
opt.timeoutlen  = 300   -- Shorter key sequence timeout (helps with which-key feel)
opt.mouse       = "a"   -- Enable mouse in all modes
opt.showmode    = false -- Don't echo "-- INSERT --" (lualine handles this)
opt.pumheight   = 10    -- Max items visible in the completion pop-up
opt.conceallevel = 3    -- Hide markup in Markdown/org (show rendered text)

-- ─── Files ───────────────────────────────────────────────────────────────────
opt.fileencoding = "utf-8"   -- Always write files as UTF-8
opt.swapfile     = false     -- Don't create swap files (undofile is enough)
opt.backup       = false     -- Don't create backup files
