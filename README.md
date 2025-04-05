# MarkdownEdit

- A personal markdown editor with the following features:
  - Google OAuth2 authentication
  - Create, read, update, and delete markdown files
  - Real-time preview of markdown files
  - Export markdown files to HTML
  - Dark mode
  - Responsive design

### Development

### Server

- These env vars must be set in `.env`
  - DATABASE_URL=postgres://test:test@test/test
  - GOOGLE_CLIENT_ID=\<google_client_id\>
  - GOOGLE_CLIENT_SECRET=\<google_client_secret\>
  - BASE_URL=http://localhost:8080
  - CLIENT_URL=http://localhost:5173
- `cargo run`

### Client

- `npm install`
- `npm run dev` to watch dev
- `npm run style` to watch tailwind styles

### Architecture

- Frontend: TypeScript, React, Tailwind CSS/Material-Tailwind, Zustand
- Backend: Rust, Axum, Tokio, SQLx
- Database: PostgreSQL
- Authentication: Google OAuth2
- Hosting: Self hosted on [homelab](https://github.com/ManeeshWije/homelab)
