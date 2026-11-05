# Rabita — Commerce Operating Platform

## Overview

Rabita is an Egypt-first commerce operating platform that enables independent merchants to sell online without assembling their own commerce stack.

### Key Features

- **Merchant Dashboard**: Catalog, branches, inventory, orders, analytics, employees
- **Consumer Storefront**: Browse, search, cart, checkout, account, order tracking
- **Multi-Branch**: Per-branch stock and shipping rules
- **Customizable Pages**: Puck editor for landing pages and storefronts
- **Community**: Job board, employee hiring, support tickets, feedback
- **Administration**: Merchant onboarding, abuse controls, content moderation
- **Built for Egypt**: Arabic-first UI, local payment methods (COD), WhatsApp OTP, Cairo timezone

## Technology Stack

- **Backend**: Rust, Axum, SQLx, PostgreSQL 16, Valkey 8
- **Frontend**: Leptos SSR, React/Puck editor
- **Infrastructure**: AWS (Aurora, RDS Proxy, S3, GuardDuty, SES, EventBridge, WAF)
- **Media**: Private S3 with GuardDuty Malware Protection for S3
- **Container**: Docker, Docker Compose

## Getting Started

### Prerequisites

- Rust 1.75+
- PostgreSQL 16
- Valkey 8
- Docker & Docker Compose (optional)

### Development Setup

```bash
# Clone repository
git clone https://github.com/your-org/rabita
cd rabita

# Copy environment template
cp .env.example .env

# Start services (Docker)
docker-compose up -d

# Run migrations
cargo sqlx migrate run

# Start backend
cargo run --package rabita-backend

# Start frontend (in another terminal)
cd frontend
cargo leptos serve
```

### Running Tests

```bash
# Backend unit tests
cargo test --package rabita-backend

# Integration tests
cargo test --package rabita-backend --test '*'

# All tests
cargo test
```

## Project Structure

.
├── backend/ # Rust backend
│ ├── src/
│ │ ├── main.rs # Axum server entry
│ │ ├── routes/ # API endpoints
│ │ ├── auth/ # Authentication
│ │ ├── stores/ # Store management
│ │ ├── catalog/ # Products, variants, categories
│ │ ├── orders/ # Order management
│ │ ├── refunds/ # Refund processing
│ │ ├── support/ # Support tickets
│ │ └── ... (30+ modules)
│ ├── migrations/ # SQL migrations
│ └── tests/ # Integration tests
├── frontend/ # Leptos frontend
│ ├── src/
│ │ ├── main.rs # SSR server
│ │ ├── pages/ # Consumer pages
│ │ ├── merchant/ # Merchant dashboard
│ │ ├── admin/ # Admin pages
│ │ ├── components/ # Shared components
│ │ ├── server/ # Server functions
│ │ └── editor/ # Puck editor integration
│ └── style/ # CSS
├── .github/workflows/ # CI/CD pipelines
├── docker-compose.yml # Local development stack
├── Dockerfile # Container image
├── Cargo.toml # Workspace
└── README.md # This file
