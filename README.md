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
# Local development

sudo docker compose --file compose_backend.yaml down
sudo docker volume rm petanque_db-data
sudo docker compose --file compose_backend.yaml up d
sudo docker compose --file compose_backend.yaml build

## query to fill in all the games with data
-- Insert random scores for all games in rounds 1-15
INSERT INTO game_results (game_id, team_id, score)
SELECT 
    g.game_id,
    g.team_1_id,
    FLOOR(RANDOM() * 14)::INT -- Random score between 0-13
FROM games g
WHERE g.round_id <= 15
UNION ALL
SELECT 
    g.game_id,
    g.team_2_id,
    FLOOR(RANDOM() * 14)::INT -- Random score between 0-13
FROM games g
WHERE g.round_id <= 15;

-- Mark all these games as played
UPDATE games 
SET played = TRUE 
WHERE round_id <= 15;