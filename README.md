# petanque backend

Fullstack rust web app for a petanque tournament.

# Build and push everything to git hub

`docker buildx bake --push`

# Development

## run frontend

```
cd frontend
trunk serve
```

## run backend

in the main dir remove frontend from compose.yaml

```
docker compose build 
docker compose up -d
```