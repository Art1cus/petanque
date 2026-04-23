# petanque backend

Fullstack rust web app for a petanque tournament.

# Build and push everything to git hub

`sudo docker buildx bake --file compose_deploy.yaml --push`
`docker buildx bake --file compose_deploy.yaml --push`

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

# Deploy on server

```
docker compose --file compose_deploy.yaml pull
docker compose --file compose_deploy.yaml up -d
```
