# Meme Coin Platform

## Overview
Decentralized platform for creating AI-powered meme tokens on Solana.

## Prerequisites
- Node.js 18+
- Docker
- Kubernetes Cluster
- Solana Wallet

## Local Development
1. Clone the repository
2. Install dependencies: `npm install`
3. Copy `.env.example` to `.env.local`
4. Run development server: `npm run dev`

## Deployment

### Docker
```bash
docker-compose up --build
```

### Kubernetes
```bash
kubectl apply -f kubernetes/deployment.yml
```

## Environment Variables
- `NEXT_PUBLIC_SOLANA_NETWORK`: Solana network (mainnet/devnet)
- `DATABASE_URL`: PostgreSQL connection string
- `NEXTAUTH_SECRET`: Authentication secret

## Security
- Multi-layer security configuration
- Rate limiting implemented
- Comprehensive CSP headers

## Monitoring
- Datadog integration
- Sentry error tracking

## CI/CD
Automated pipeline via GitHub Actions:
- Lint and test
- Build Docker image
- Deploy to Kubernetes

## Contributing
1. Fork the repository
2. Create feature branch
3. Commit changes
4. Push and create PR

## License
MIT License
