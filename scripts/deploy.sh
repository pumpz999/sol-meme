#!/bin/bash

# Deployment Script for Meme Coin Platform

set -e  # Exit immediately if a command exits with a non-zero status.

# Load environment variables
source .env.production

# Pre-deployment checks
echo "Running pre-deployment checks..."
npm run lint
npm run test

# Build Docker image
echo "Building Docker image..."
docker build -t meme-coin-platform:latest .

# Push to container registry
echo "Pushing image to registry..."
docker tag meme-coin-platform:latest dockerhub-username/meme-coin-platform:$GITHUB_SHA
docker push dockerhub-username/meme-coin-platform:$GITHUB_SHA

# Kubernetes deployment
echo "Deploying to Kubernetes..."
kubectl apply -f kubernetes/deployment.yml
kubectl set image deployment/meme-coin-platform meme-coin-platform=dockerhub-username/meme-coin-platform:$GITHUB_SHA

# Run database migrations
echo "Running database migrations..."
npx prisma migrate deploy

# Warm up cache and perform health checks
echo "Performing post-deployment health checks..."
curl -f http://localhost:3000/health || exit 1

echo "Deployment completed successfully!"
