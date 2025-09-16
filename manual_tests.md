# Manual API Testing Commands

Run these curl commands after starting your Docker containers with `ENV=dev docker-compose up`

## Quick Health Check

```bash
# Test if the API is responding
curl -X GET http://localhost:8000/api/teams
```

## Team Operations

### Create a Team
```bash
curl -X POST http://localhost:8000/api/teams \
  -H "Content-Type: application/json" \
  -d '{
    "id": "kansas-city-chiefs",
    "name": "Kansas City Chiefs",
    "abbreviation": "KC",
    "conference": "AFC",
    "division": "West",
    "stats": {
      "offensive_rating": 95.5,
      "defensive_rating": 88.2,
      "points_per_game": 28.5,
      "points_allowed_per_game": 19.8,
      "yards_per_game": 385.2,
      "yards_allowed_per_game": 312.1,
      "turnover_differential": 8,
      "recent_form": [],
      "injury_report": [],
      "season": 2024,
      "games_played": 10,
      "wins": 8,
      "losses": 2,
      "ties": 0,
      "last_updated": "2024-11-15T10:00:00Z"
    },
    "created_at": "2024-11-15T10:00:00Z",
    "updated_at": "2024-11-15T10:00:00Z"
  }'
```

### Get All Teams
```bash
curl -X GET http://localhost:8000/api/teams
```

### Get Specific Team
```bash
curl -X GET http://localhost:8000/api/teams/kansas-city-chiefs
```

## Game Operations

### Create a Simple Game
```bash
curl -X POST http://localhost:8000/api/games \
  -H "Content-Type: application/json" \
  -d '{
    "id": "test-game-1",
    "home_team": {
      "id": "home-team",
      "name": "Home Team",
      "abbreviation": "HT",
      "conference": null,
      "division": null,
      "stats": {
        "offensive_rating": 80.0,
        "defensive_rating": 80.0,
        "points_per_game": 20.0,
        "points_allowed_per_game": 20.0,
        "yards_per_game": 300.0,
        "yards_allowed_per_game": 300.0,
        "turnover_differential": 0,
        "recent_form": [],
        "injury_report": [],
        "season": 2024,
        "games_played": 0,
        "wins": 0,
        "losses": 0,
        "ties": 0,
        "last_updated": "2024-11-15T10:00:00Z"
      },
      "created_at": "2024-11-15T10:00:00Z",
      "updated_at": "2024-11-15T10:00:00Z"
    },
    "away_team": {
      "id": "away-team",
      "name": "Away Team",
      "abbreviation": "AT",
      "conference": null,
      "division": null,
      "stats": {
        "offensive_rating": 80.0,
        "defensive_rating": 80.0,
        "points_per_game": 20.0,
        "points_allowed_per_game": 20.0,
        "yards_per_game": 300.0,
        "yards_allowed_per_game": 300.0,
        "turnover_differential": 0,
        "recent_form": [],
        "injury_report": [],
        "season": 2024,
        "games_played": 0,
        "wins": 0,
        "losses": 0,
        "ties": 0,
        "last_updated": "2024-11-15T10:00:00Z"
      },
      "created_at": "2024-11-15T10:00:00Z",
      "updated_at": "2024-11-15T10:00:00Z"
    },
    "game_time": "2024-11-24T18:00:00Z",
    "week": 12,
    "season": 2024,
    "status": "Scheduled",
    "home_score": null,
    "away_score": null,
    "created_at": "2024-11-15T10:00:00Z",
    "updated_at": "2024-11-15T10:00:00Z"
  }'
```

### Get All Games
```bash
curl -X GET http://localhost:8000/api/games
```

## Betting Line Operations

### Create a Betting Line
```bash
curl -X POST http://localhost:8000/api/betting-lines \
  -H "Content-Type: application/json" \
  -d '{
    "id": "test-line-1",
    "game_id": "test-game-1",
    "provider": "DraftKings",
    "spread": -3.5,
    "total": 45.5,
    "moneyline_home": -150,
    "moneyline_away": +130,
    "timestamp": "2024-11-15T10:00:00Z",
    "is_active": true
  }'
```

### Get Betting Lines for a Game
```bash
curl -X GET http://localhost:8000/api/betting-lines/game/test-game-1
```

## Testing Tips

1. **Start Simple**: Begin with the team creation and retrieval
2. **Check Responses**: Look for proper JSON responses and HTTP status codes
3. **Verify Data**: Use GET endpoints to confirm data was stored correctly
4. **Test Edge Cases**: Try invalid IDs, malformed JSON, etc.

## Expected Responses

### Successful Team Creation
```json
{
  "id": "kansas-city-chiefs",
  "name": "Kansas City Chiefs",
  "abbreviation": "KC",
  // ... rest of team data
}
```

### Successful Team Retrieval
```json
[
  {
    "id": "kansas-city-chiefs",
    "name": "Kansas City Chiefs",
    // ... team data
  }
]
```

## Troubleshooting

If you get errors:

1. **Connection Refused**: Make sure Docker containers are running
2. **404 Not Found**: Check the endpoint URL
3. **400 Bad Request**: Verify JSON format and required fields
4. **500 Internal Server Error**: Check Docker logs with `docker-compose logs backend`