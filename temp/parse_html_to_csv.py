#!/usr/bin/env python3
"""
HTML Parser to generate proper CSV from prediction and betting tables.

This script parses:
1. predictions_table.html - contains model predictions and win percentages
2. betting_table.html - contains actual betting lines from sportsbooks

And generates a corrected nfl_predictions.csv with proper data separation.
"""

import re
import csv
from bs4 import BeautifulSoup
from datetime import datetime
import sys

def parse_predictions_table(html_file):
    """Parse the predictions table HTML to extract prediction data."""
    with open(html_file, 'r', encoding='utf-8') as f:
        soup = BeautifulSoup(f.read(), 'html.parser')
    
    games = []
    
    # Find all game rows (they have IDs like "row_2025_03_ATL_CAR") in document order
    # This preserves the original ordering from the HTML table
    game_rows = soup.find_all('tr', id=re.compile(r'row_\d{4}_\d{2}_\w+_\w+'))
    
    print(f"Found {len(game_rows)} game rows in predictions table")
    
    for row in game_rows:
        try:
            # Extract game ID parts
            row_id = row.get('id')
            parts = row_id.split('_')
            year, week, away_team, home_team = parts[1], parts[2], parts[3], parts[4]
            
            # Extract date and time
            date_cell = row.find('font', string=re.compile(r'\d+/\d+'))
            time_cell = row.find('font', string=re.compile(r'\d+:\d+'))
            
            game_date = date_cell.get_text().strip() if date_cell else "9/21"
            game_time = time_cell.get_text().strip() if time_cell else "1:00"
            
            # Extract prediction confidence values (+/- metrics)
            away_confidence_elem = row.find('p', id=f'away_{year}_{week}_{away_team}_{home_team}')
            home_confidence_elem = row.find('p', id=f'home_{year}_{week}_{away_team}_{home_team}')
            
            away_confidence = 0.0
            home_confidence = 0.0
            
            if away_confidence_elem:
                away_text = away_confidence_elem.get_text().strip()
                # Extract numeric value from text like "+14.1" or "-3.1"
                try:
                    if '+' in away_text or '-' in away_text:
                        # Remove any non-numeric characters except +, -, and .
                        clean_text = ''.join(c for c in away_text if c.isdigit() or c in '+-.')
                        away_confidence = float(clean_text)
                except ValueError:
                    away_confidence = 0.0
                    
            if home_confidence_elem:
                home_text = home_confidence_elem.get_text().strip()
                # Extract numeric value from text like "+14.1" or "-19.9"
                try:
                    if '+' in home_text or '-' in home_text:
                        # Remove any non-numeric characters except +, -, and .
                        clean_text = ''.join(c for c in home_text if c.isdigit() or c in '+-.')
                        home_confidence = float(clean_text)
                except ValueError:
                    home_confidence = 0.0
            
            # Extract market win percentage from the input element
            market_percentage = 50  # Default
            market_input = row.find('input', {'name': f'{year}_{week}_{away_team}_{home_team}'})
            if market_input and market_input.get('market'):
                market_percentage = int(market_input.get('market'))
            
            # Convert market probabilities to predicted scores
            # market_percentage = home team win probability
            home_win_prob = market_percentage / 100.0
            away_win_prob = 1.0 - home_win_prob
            
            # Use a more sophisticated approach based on win probabilities
            # Convert win probability to expected point differential
            if home_win_prob > 0.5:
                # Home team favored
                point_diff = (home_win_prob - 0.5) * 28  # Scale to reasonable point spread
                predicted_home_score = 24 + (point_diff / 2)
                predicted_away_score = 24 - (point_diff / 2)
            else:
                # Away team favored  
                point_diff = (away_win_prob - 0.5) * 28
                predicted_away_score = 24 + (point_diff / 2)
                predicted_home_score = 24 - (point_diff / 2)
            
            # Ensure scores are reasonable (14-35 range)
            predicted_away_score = max(14, min(35, predicted_away_score))
            predicted_home_score = max(14, min(35, predicted_home_score))
            
            # Convert market percentage to confidence (0-1 scale)
            confidence = abs(market_percentage - 50) / 50.0
            
            games.append({
                'week': int(week),
                'date': f"2025-09-21",  # Standardized date format
                'time': game_time,
                'away_team': away_team,
                'home_team': home_team,
                'predicted_away_score': round(predicted_away_score, 1),
                'predicted_home_score': round(predicted_home_score, 1),
                'confidence': round(confidence, 2),
                'away_confidence': away_confidence,
                'home_confidence': home_confidence,
                'market_percentage': market_percentage
            })
            
        except Exception as e:
            print(f"Error parsing row {row_id}: {e}")
            continue
    
    return games

def parse_betting_table(html_file):
    """Parse the betting table HTML to extract actual betting lines."""
    with open(html_file, 'r', encoding='utf-8') as f:
        soup = BeautifulSoup(f.read(), 'html.parser')
    
    betting_lines = {}
    
    # Convert team names to abbreviations
    team_abbr_map = {
        'Packers': 'GB', 'Browns': 'CLE', 'Texans': 'HOU', 'Jaguars': 'JAX',
        'Bengals': 'CIN', 'Vikings': 'MIN', 'Steelers': 'PIT', 'Patriots': 'NE',
        'Rams': 'LA', 'Eagles': 'PHI', 'Jets': 'NYJ', 'Buccaneers': 'TB',
        'Colts': 'IND', 'Titans': 'TEN', 'Raiders': 'LV', 'Commanders': 'WAS',
        'Broncos': 'DEN', 'Chargers': 'LAC', 'Saints': 'NO', 'Seahawks': 'SEA',
        'Cowboys': 'DAL', 'Bears': 'CHI', 'Cardinals': 'ARI', 'Niners': 'SF',
        'Chiefs': 'KC', 'Giants': 'NYG', 'Lions': 'DET', 'Ravens': 'BAL',
        'Falcons': 'ATL', 'Panthers': 'CAR'
    }
    
    # Find all table rows that contain game data
    all_rows = soup.find_all('tr')
    
    for row in all_rows:
        try:
            # Look for game info div within this row
            game_info_div = row.find('div', class_='best-odds__game-info')
            if not game_info_div:
                continue
                
            # Find team names - look for both desktop and mobile versions
            team_spans = game_info_div.find_all('span')
            team_names = []
            
            for span in team_spans:
                text = span.get_text().strip()
                if text in team_abbr_map:
                    team_names.append(text)
                    
            if len(team_names) < 2:
                continue
                
            away_team = team_names[0]
            home_team = team_names[1]
            away_abbr = team_abbr_map[away_team]
            home_abbr = team_abbr_map[home_team]
            
            # Look for odds containers in this row
            odds_containers = row.find_all('div', class_='best-odds__odds-container')
            
            if odds_containers:
                # Take the first odds container (usually the "Best Odds" column)
                first_container = odds_containers[0]
                
                # Find all the individual odds divs within this container
                odds_divs = first_container.find_all('div', class_='css-rppihz')
                
                if len(odds_divs) >= 2:
                    # First div is away team spread, second div is home team spread
                    # We want the BOTTOM div (home team spread)
                    home_odds_div = odds_divs[1]  # Second div = home team
                    
                    # Find the spread value in the home team div
                    spread_span = home_odds_div.find('span', class_='css-1jlt5rt')
                    
                    if spread_span:
                        spread_text = spread_span.get_text().strip()
                        
                        try:
                            # Parse the spread value
                            if spread_text.startswith('+'):
                                home_spread = float(spread_text[1:])
                            elif spread_text.startswith('-'):
                                home_spread = -float(spread_text[1:])
                            else:
                                home_spread = float(spread_text)
                            
                            game_key = f"{away_abbr}_{home_abbr}"
                            betting_lines[game_key] = {
                                'away_team': away_abbr,
                                'home_team': home_abbr,
                                'market_spread': home_spread,
                                'home_spread': home_spread,
                                'total': 45.0
                            }
                            
                            print(f"Found betting line: {away_abbr} @ {home_abbr}, home spread: {home_spread}")
                            
                        except ValueError:
                            print(f"Could not parse spread value: {spread_text}")
                            continue
                    
        except Exception as e:
            print(f"Error parsing betting row: {e}")
            continue
    
    return betting_lines

def combine_data_and_generate_csv(predictions, betting_lines, output_file):
    """Combine prediction and betting data and generate CSV."""
    
    with open(output_file, 'w', newline='', encoding='utf-8') as csvfile:
        fieldnames = [
            'Week', 'Date', 'Time', 'Away Team', 'Home Team',
            'Predicted Away Score', 'Predicted Home Score', 'Confidence',
            'Market Spread (Home)', 'Total'
        ]
        
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        writer.writeheader()
        
        for i, game in enumerate(predictions):
            # Look for matching betting line
            game_key = f"{game['away_team']}_{game['home_team']}"
            
            betting_data = betting_lines.get(game_key, {})
            
            if not betting_data:
                print(f"WARNING: No betting line found for {game_key}")
            
            # Get the home team spread directly from betting data
            # This is already in the correct format (home team perspective)
            home_spread = betting_data.get('home_spread', 0.0)
            total = betting_data.get('total', 45.0)
            
            # Format the spread properly
            if home_spread > 0:
                spread_display = f"+{home_spread:.1f}"
            elif home_spread < 0:
                spread_display = f"{home_spread:.1f}"
            else:
                spread_display = "0.0"
            
            print(f"Game {i+1}: {game['away_team']} @ {game['home_team']}, spread: {spread_display}")
            
            writer.writerow({
                'Week': game['week'],
                'Date': game['date'],
                'Time': game['time'],
                'Away Team': game['away_team'],
                'Home Team': game['home_team'],
                'Predicted Away Score': game['predicted_away_score'],
                'Predicted Home Score': game['predicted_home_score'],
                'Confidence': game['confidence'],
                'Market Spread (Home)': spread_display,
                'Total': total
            })

def main():
    """Main function to parse HTML files and generate CSV."""
    
    predictions_file = 'predictions_table.html'
    betting_file = 'betting_table.html'
    output_file = 'nfl_predictions.csv'  # Overwrite the existing file
    
    print("Parsing predictions table...")
    predictions = parse_predictions_table(predictions_file)
    print(f"Found {len(predictions)} games in predictions table")
    
    print("Parsing betting table...")
    betting_lines = parse_betting_table(betting_file)
    print(f"Found {len(betting_lines)} games in betting table")
    
    print("Combining data and generating CSV...")
    combine_data_and_generate_csv(predictions, betting_lines, output_file)
    
    print(f"Generated corrected CSV: {output_file}")
    
    # Print sample of the data for verification
    print("\nSample data with corrected betting lines:")
    for i, game in enumerate(predictions[:3]):
        print(f"Game {i+1}: {game['away_team']} @ {game['home_team']}")
        print(f"  Predicted scores: {game['predicted_away_score']} - {game['predicted_home_score']}")
        print(f"  Confidence: {game['confidence']}")
        
        game_key = f"{game['away_team']}_{game['home_team']}"
        if game_key in betting_lines:
            betting = betting_lines[game_key]
            print(f"  Home spread: {betting['home_spread']}")
        else:
            print(f"  No betting line found for {game_key}")
        print()

if __name__ == "__main__":
    main()