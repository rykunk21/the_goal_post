#!/bin/bash

# NFL Prediction Dashboard API Test Script
# Run this after starting the Docker containers with: ENV=dev docker-compose up

BASE_URL="http://localhost:8000/api"

echo "🏈 Testing NFL Prediction Dashboard API"
echo "========================================"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to test API endpoint
test_endpoint() {
    local method=$1
    local endpoint=$2
    local data=$3
    local description=$4
    
    echo -e "\n${BLUE}Testing: $description${NC}"
    echo "Command: curl -X $method $BASE_URL$endpoint"
    
    if [ -n "$data" ]; then
        echo "Data: $data"
        response=$(curl -s -X $method \
            -H "Content-Type: application/json" \
            -d "$data" \
            "$BASE_URL$endpoint")
    else
        response=$(curl -s -X $method "$BASE_URL$endpoint")
    fi
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Success${NC}"
        echo "Response: $response"
    else
        echo -e "${RED}✗ Failed${NC}"
    fi
}

# Test 1: Create Teams
echo -e "\n${BLUE}=== TESTING TEAMS ===${NC}"

# Create Kansas City Chiefs
chiefs_data='{
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

test_endpoint "POST" "/teams" "$chiefs_data" "Create Kansas City Chiefs"

# Create Buffalo Bills
bills_data='{
    "id": "buffalo-bills",
    "name": "Buffalo Bills", 
    "abbreviation": "BUF",
    "conference": "AFC",
    "division": "East",
    "stats": {
        "offensive_rating": 92.1,
        "defensive_rating": 91.5,
        "points_per_game": 26.8,
        "points_allowed_per_game": 18.2,
        "yards_per_game": 368.9,
        "yards_allowed_per_game": 298.7,
        "turnover_differential": 12,
        "recent_form": [],
        "injury_report": [],
        "season": 2024,
        "games_played": 10,
        "wins": 7,
        "losses": 3,
        "ties": 0,
        "last_updated": "2024-11-15T10:00:00Z"
    },
    "created_at": "2024-11-15T10:00:00Z",
    "updated_at": "2024-11-15T10:00:00Z"
}'

test_endpoint "POST" "/teams" "$bills_data" "Create Buffalo Bills"

# Test 2: Get All Teams
test_endpoint "GET" "/teams" "" "Get all teams"

# Test 3: Get Specific Team
test_endpoint "GET" "/teams/kansas-city-chiefs" "" "Get Kansas City Chiefs"

# Test 4: Create a Game
echo -e "\n${BLUE}=== TESTING GAMES ===${NC}"

game_data='{
    "id": "kc-vs-buf-week12-2024",
    "home_team": {
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
    },
    "away_team": {
        "id": "buffalo-bills",
        "name": "Buffalo Bills",
        "abbreviation": "BUF", 
        "conference": "AFC",
        "division": "East",
        "stats": {
            "offensive_rating": 92.1,
            "defensive_rating": 91.5,
            "points_per_game": 26.8,
            "points_allowed_per_game": 18.2,
            "yards_per_game": 368.9,
            "yards_allowed_per_game": 298.7,
            "turnover_differential": 12,
            "recent_form": [],
            "injury_report": [],
            "season": 2024,
            "games_played": 10,
            "wins": 7,
            "losses": 3,
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

test_endpoint "POST" "/games" "$game_data" "Create KC vs BUF game"

# Test 5: Get All Games
test_endpoint "GET" "/games" "" "Get all games"

# Test 6: Get Games by Week/Season
test_endpoint "GET" "/games/week/12/season/2024" "" "Get Week 12, 2024 games"

# Test 7: Create Betting Line
echo -e "\n${BLUE}=== TESTING BETTING LINES ===${NC}"

betting_line_data='{
    "id": "draftkings-kc-buf-week12",
    "game_id": "kc-vs-buf-week12-2024",
    "provider": "DraftKings",
    "spread": -2.5,
    "total": 47.5,
    "moneyline_home": -130,
    "moneyline_away": +110,
    "timestamp": "2024-11-15T10:00:00Z",
    "is_active": true
}'

test_endpoint "POST" "/betting-lines" "$betting_line_data" "Create betting line"

# Test 8: Get Betting Lines for Game
test_endpoint "GET" "/betting-lines/game/kc-vs-buf-week12-2024" "" "Get betting lines for KC vs BUF"

echo -e "\n${GREEN}🎉 API Testing Complete!${NC}"
echo "Check the responses above to verify everything is working correctly."