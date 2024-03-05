# MarkdownEdit

- A personal markdown editor with the following features:
  - Google OAuth2 authentication
  - Create, read, update, and delete markdown files
  - Real-time preview of markdown files
  - Export markdown files to PDF
  - Dark mode
  - Responsive design

### Development

```sh
docker-compose --env-file ./backend/.env --env-file ./frontend/.env up -d
```

- Frontend: `http://localhost:3000`
- Backend: `http://localhost:3001`

### Architecture

- Frontend: TypeScript, React
- Backend: Rust, Axum, Tokio, SQLx
- Database: PostgreSQL
- Authentication: Google OAuth2
- Hosting: Railway via frontend and backend Docker images
