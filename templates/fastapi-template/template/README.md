# {{project_name}}

A production-ready FastAPI application created with OpenRye.

## Features

- ⚡ **FastAPI** with async/await support
- 🔐 **JWT Authentication** for secure API access  
- 📦 **SQLAlchemy ORM** with Alembic migrations
- 🧪 **Pytest** for comprehensive testing
- 🐳 **Docker** and Docker Compose support
- 📝 **Pre-commit hooks** for code quality
- 🎯 **Type hints** with mypy validation
- 🚀 **GitHub Actions** CI/CD pipeline

## Quick Start

### Installation

```bash
# Install dependencies
openrye sync

# Copy environment variables
cp .env.example .env
```

### Development

```bash
# Run development server with hot reload
openrye run dev

# Run tests with coverage
openrye run test

# Format and lint code
openrye run format
openrye run lint
```

### Docker

```bash
# Build and run with Docker Compose
docker-compose up --build

# Run in background
docker-compose up -d
```

## API Documentation

Once running, visit:
- **Swagger UI**: http://localhost:8000/api/v1/docs
- **ReDoc**: http://localhost:8000/api/v1/redoc
- **OpenAPI JSON**: http://localhost:8000/api/v1/openapi.json

## Project Structure

```
{{project_name}}/
├── app/
│   ├── api/           # API endpoints and routing
│   ├── core/          # Core functionality (config, security, database)
│   ├── models/        # SQLAlchemy ORM models
│   ├── schemas/       # Pydantic validation schemas
│   ├── services/      # Business logic layer
│   └── main.py        # FastAPI application entry point
├── tests/             # Test suite
│   ├── api/          # API endpoint tests
│   └── unit/         # Unit tests
├── alembic/          # Database migrations
├── .github/          # GitHub Actions workflows
├── docker-compose.yml # Docker orchestration
└── Dockerfile        # Container configuration
```

## Database Migrations

```bash
# Create a new migration
openrye run makemigrations "Add user table"

# Apply migrations
openrye run migrate

# Rollback one migration
alembic downgrade -1
```

## Testing

```bash
# Run all tests
openrye run test

# Run with coverage report
pytest --cov=app --cov-report=html

# Run specific test file
pytest tests/test_auth.py -v
```

## Deployment

### Environment Variables

Configure these in production:

- `DATABASE_URL`: PostgreSQL connection string
- `SECRET_KEY`: JWT signing key (generate with `openssl rand -hex 32`)
- `REDIS_URL`: Redis connection string (optional)
- `BACKEND_CORS_ORIGINS`: Allowed CORS origins

### Production Checklist

- [ ] Set strong `SECRET_KEY`
- [ ] Configure production database
- [ ] Enable HTTPS
- [ ] Set up monitoring
- [ ] Configure rate limiting
- [ ] Enable request logging
- [ ] Set up backup strategy

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

## License

MIT