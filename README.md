# MarkdownEdit

- A personal markdown editor with the following features:
  - Google OAuth2 authentication
  - Create, read, update, and delete markdown files
  - Real-time preview of markdown files
  - Export markdown files to PDF
  - Dark mode
  - Responsive design

### Development

- Ensure Docker and Docker Compose are installed
- Ensure your environment variables are set in `./backend/.env` and `./frontend/.env`

```sh
docker-compose --env-file ./backend/.env --env-file ./frontend/.env up --build
```

- Frontend: `http://localhost:3000`
  - The frontend will hot reload on code changes
- Backend: `http://localhost:3001
  - The backend will need to be restarted on code changes
  - You can Ctrl+C and then run the `docker-compose` command again

### Architecture

- Frontend: TypeScript, React
- Backend: Rust, Axum, Tokio, SQLx
- Database: PostgreSQL
- Authentication: Google OAuth2
- Hosting: Railway via frontend and backend Docker images
