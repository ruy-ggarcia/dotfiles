-- LSP configuration
-- mason.nvim   — installs and manages LSP server binaries
-- mason-lspconfig — bridges mason with nvim-lspconfig
-- nvim-lspconfig   — configures each LSP server for NeoVim

return {
  -- ── mason.nvim ────────────────────────────────────────────────────────────
  {
    "williamboman/mason.nvim",
    cmd  = "Mason",
    keys = { { "<leader>cm", "<cmd>Mason<cr>", desc = "Open Mason" } },
    build = ":MasonUpdate",
    opts = {
      ui = {
        border = "rounded",
        icons  = {
          package_installed   = "✓",
          package_pending     = "➜",
          package_uninstalled = "✗",
        },
      },
    },
  },

  -- ── mason-lspconfig ───────────────────────────────────────────────────────
  {
    "williamboman/mason-lspconfig.nvim",
    dependencies = { "williamboman/mason.nvim" },
    opts = {
      -- Servers installed automatically on first launch.
      ensure_installed = {
        "lua_ls",          -- Lua (NeoVim config)
        "rust_analyzer",   -- Rust
        "ts_ls",           -- TypeScript / JavaScript
        "pyright",         -- Python
        "jsonls",          -- JSON
        "yamlls",          -- YAML
        "bashls",          -- Bash
        "marksman",        -- Markdown
      },
      automatic_installation = true,
    },
  },

  -- ── nvim-lspconfig ────────────────────────────────────────────────────────
  {
    "neovim/nvim-lspconfig",
    event        = { "BufReadPost", "BufNewFile" },
    dependencies = {
      "williamboman/mason.nvim",
      "williamboman/mason-lspconfig.nvim",
      -- Show a spinner while LSP is indexing.
      { "j-hui/fidget.nvim", opts = {} },
      -- Extra Lua types for the NeoVim API (improves lua_ls completions).
      { "folke/neodev.nvim", opts = {} },
    },
    config = function()
      -- ── On-attach keymaps ─────────────────────────────────────────────────
      -- Runs once per buffer when an LSP server attaches.
      local on_attach = function(_, buf)
        local map = function(keys, func, desc)
          vim.keymap.set("n", keys, func, { buffer = buf, desc = "LSP: " .. desc })
        end

        -- Navigation
        map("gd",         vim.lsp.buf.definition,      "Go to definition")
        map("gD",         vim.lsp.buf.declaration,     "Go to declaration")
        map("gr",         require("telescope.builtin").lsp_references, "Go to references")
        map("gi",         vim.lsp.buf.implementation,  "Go to implementation")
        map("gt",         vim.lsp.buf.type_definition, "Go to type definition")

        -- Documentation & signature
        map("K",          vim.lsp.buf.hover,           "Hover documentation")
        map("<C-s>",      vim.lsp.buf.signature_help,  "Signature help")

        -- Code actions
        map("<leader>ca", vim.lsp.buf.code_action,     "Code action")
        map("<leader>rn", vim.lsp.buf.rename,          "Rename symbol")

        -- Workspace
        map("<leader>wa", vim.lsp.buf.add_workspace_folder,    "Add workspace folder")
        map("<leader>wr", vim.lsp.buf.remove_workspace_folder, "Remove workspace folder")
        map("<leader>wl", function()
          print(vim.inspect(vim.lsp.buf.list_workspace_folders()))
        end, "List workspace folders")

        -- Formatting (synchronous so it runs reliably on BufWritePre if wired)
        map("<leader>lf", function()
          vim.lsp.buf.format({ async = true })
        end, "Format buffer")
      end

      -- ── Capabilities (advertise nvim-cmp completions to the server) ───────
      local capabilities = vim.lsp.protocol.make_client_capabilities()
      local ok_cmp, cmp_lsp = pcall(require, "cmp_nvim_lsp")
      if ok_cmp then
        capabilities = cmp_lsp.default_capabilities(capabilities)
      end

      -- ── Server-specific settings ──────────────────────────────────────────
      local servers = {
        lua_ls = {
          settings = {
            Lua = {
              runtime    = { version = "LuaJIT" },
              workspace  = { checkThirdParty = false },
              telemetry  = { enable = false },
              diagnostics = { globals = { "vim" } },
              completion = { callSnippet = "Replace" },
            },
          },
        },
        rust_analyzer = {
          settings = {
            ["rust-analyzer"] = {
              checkOnSave = { command = "clippy" },
            },
          },
        },
        pyright = {
          settings = {
            python = {
              analysis = { typeCheckingMode = "basic" },
            },
          },
        },
      }

      -- ── Setup all servers via mason-lspconfig ─────────────────────────────
      require("mason-lspconfig").setup_handlers({
        -- Default handler — used for every server not listed below.
        function(server_name)
          require("lspconfig")[server_name].setup(
            vim.tbl_deep_extend("force", {
              on_attach    = on_attach,
              capabilities = capabilities,
            }, servers[server_name] or {})
          )
        end,
      })

      -- ── Diagnostic appearance ─────────────────────────────────────────────
      vim.diagnostic.config({
        virtual_text = {
          prefix = "●",
          source = "if_many",
        },
        float = {
          source = "always",
          border = "rounded",
        },
        signs      = true,
        underline  = true,
        update_in_insert = false,
        severity_sort    = true,
      })

      -- Better sign column icons.
      local signs = { Error = " ", Warn = " ", Hint = "󰠠 ", Info = " " }
      for type, icon in pairs(signs) do
        local hl = "DiagnosticSign" .. type
        vim.fn.sign_define(hl, { text = icon, texthl = hl, numhl = hl })
      end
    end,
  },
}
