---
-- Code style
---

vim.o.textwidth = 80
vim.o.tabstop = 4
vim.o.shiftwidth = 4

---
-- VimLaTeX
---

vim.g.tex_flavor = 'latex'
vim.g.vimtex_view_method = 'zathura'
vim.g.vimtex_format_enabled = 1

---
-- GUI configuration
---

vim.o.mouse = 'a'

vim.o.number = true
vim.o.relativenumber = true
vim.o.signcolumn = 'number'

vim.o.conceallevel = 1

-- Use a global status bar instead of showing one for each window.
vim.g.laststatus = 3

-- Make concealed text blend in with normal text.
vim.cmd 'highlight Conceal ctermbg=NONE'

vim.cmd 'highlight LineNr gui=NONE cterm=NONE ctermfg=242 ctermbg=233'
vim.cmd 'highlight StatusLine gui=NONE cterm=NONE ctermfg=244 ctermbg=235'
vim.cmd 'highlight StatusLineNC gui=NONE cterm=NONE ctermfg=243 ctermbg=234'
vim.cmd 'highlight VertSplit gui=NONE cterm=NONE ctermfg=236 ctermbg=233'

---
-- LSP
---

local on_attach = function(client, bufnr)
	local opts = { noremap = true, silent = true }
	vim.api.nvim_buf_set_keymap(bufnr, 'n', 'gD', '<Cmd>lua vim.lsp.buf.declaration()<CR>', opts)
	vim.api.nvim_buf_set_keymap(bufnr, 'n', 'gd', '<Cmd>lua vim.lsp.buf.definition()<CR>', opts)
	vim.api.nvim_buf_set_keymap(bufnr, 'n', 'gi', '<Cmd>lua vim.lsp.buf.implementation()<CR>', opts)
	vim.api.nvim_buf_set_keymap(bufnr, 'n', '<Space>r', '<Cmd>lua vim.lsp.buf.rename()<CR>', opts)
	vim.api.nvim_buf_set_keymap(bufnr, 'n', '<Space>ca', '<Cmd>lua vim.lsp.buf.code_action()<CR>', opts)
end

local capabilities = require('cmp_nvim_lsp').update_capabilities(vim.lsp.protocol.make_client_capabilities())

local lspconfig = require 'lspconfig'
local servers = { 'rust_analyzer', 'clangd' }
for _, server in pairs(servers) do
	lspconfig[server].setup {
		on_attach = on_attach,
		capabilities = capabilities,
		flags = {
			debounce_text_changes = 150,
		},
	}
end

vim.lsp.handlers['textDocument/publishDiagnostics'] = vim.lsp.with(
	vim.lsp.diagnostic.on_publish_diagnostics, {
		virtual_text = true,
		signs = true,
		update_in_insert = true,
	}
)

---
-- Auto completion
---

vim.o.completeopt = 'menu,menuone,noselect'

local cmp = require 'cmp'
local cmp_ultisnips_mappings = require 'cmp_nvim_ultisnips.mappings'
cmp.setup {
	snippet = {
		expand = function(args)
			vim.fn["UltiSnips#Anon"](args.body)
		end,
	},
	mapping = {
		['<CR>'] = cmp.mapping.confirm {
			behavior = cmp.ConfirmBehavior.Replace,
			select = true,
		},
        ['<Tab>'] = cmp.mapping(
			function(fallback)
				cmp_ultisnips_mappings.expand_or_jump_forwards(fallback)
			end,
			{ 'i', 's' }
		),
		['<S-Tab>'] = cmp.mapping(
			function(fallback)
				cmp_ultisnips_mappings.jump_backwards(fallback)
			end,
			{ 'i', 's' }
		),
	},
	sources = {
		{ name = 'nvim_lsp' },
		{ name = 'ultisnips' },
	},
	experimental = {
		ghost_text = true,
	},
}

--
-- Key bindings
--

vim.api.nvim_set_keymap('', '<C-p>', '<Cmd>FZF<CR>', {})
