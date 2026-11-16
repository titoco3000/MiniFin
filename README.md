# MiniFin

Lightweight desktop app for small-business expense tracking. It records expenses with supplier, company, sector, payment
type and invoice number, and lets you search, filter and sort them. Data is stored locally.

Highly experimental!

## Features

- Expense entry form with supplier, company, sector, cash box, date and invoice (NF) inputs
- CSV import
- Filtering and sorting of expenses
- Supplier editor
- Built-in log console (custom Rust logging macros forwarded to the UI)

## Stack

- **Backend:** Rust + [Tauri](https://tauri.app) (domain types in `src-tauri/src/tipos/`, persistence in `storage.rs`)
- **Frontend:** SvelteKit, TypeScript

## Development

```bash
npm install
npm run tauri dev
npm run tauri build
```

## Roadmap

- Better search
- Delete directly from the table
- First-run setup for new users
- Choose database location
