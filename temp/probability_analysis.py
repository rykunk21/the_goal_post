#!/usr/bin/env python3
"""
Probability-based value analysis comparing community predictions to betting line probabilities.
"""

import csv
import math

def spread_to_probability(spread):
    """
    Convert point spread to implied win probability using a logistic model.
    This is more accurate than using moneyline odds for spread bets.
    """
    if spread == 0:
        return 0.5
    
    # Standard NFL model: each point is worth about 3.3% win probability
    # Using logistic function: P = 1 / (1 + e^(-spread/3.3))
    return 1 / (1 + math.exp(-spread / 3.3))

def moneyline_to_probability(moneyline):
    """
    Convert American moneyline odds to implied probability.
    """
    if moneyline > 0:
        # Positive moneyline (underdog)
        return 100 / (moneyline + 100)
    else:
        # Negative moneyline (favorite)
        return abs(moneyline) / (abs(moneyline) + 100)

def analyze_probability_value():
    """Analyze value opportunities by comparing community vs market probabilities."""
    
    games = []
    
    # Read the CSV data
    with open('nfl_predictions.csv', 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            # Parse market spread
            market_spread_str = row['Market Spread (Home)'].replace('+', '')
            market_spread = float(market_spread_str)
            
            # Parse home moneyline odds
            home_moneyline = int(row.get('Home Moneyline', -110))
            
            # Parse community probability
            community_prob_str = row.get('Community Probability', '50.0%')
            community_percentage = float(community_prob_str.replace('%', ''))
            
            games.append({
                'away_team': row['Away Team'],
                'home_team': row['Home Team'],
                'predicted_away': float(row['Predicted Away Score']),
                'predicted_home': float(row['Predicted Home Score']),
                'market_spread': market_spread,
                'home_moneyline': home_moneyline,
                'confidence': float(row['Confidence']),
                'community_percentage': community_percentage
            })
    
    print("PROBABILITY-BASED VALUE ANALYSIS")
    print("=" * 50)
    
    # Community probabilities are now read from the CSV data
    
    value_opportunities = []
    
    for game in games:
        # The community percentage from the HTML represents the home team win probability
        community_home_prob = game['community_percentage'] / 100.0
        community_away_prob = 1.0 - community_home_prob
        
        # Calculate implied WIN probability from point spread (CORRECT approach)
        # This converts the spread to "probability of winning the game"
        # which matches what the community percentage represents
        market_spread = game['market_spread']
        if market_spread > 0:
            # Home team is underdog
            betting_home_prob = spread_to_probability(-market_spread)
        else:
            # Home team is favorite  
            betting_home_prob = spread_to_probability(-market_spread)
        
        betting_away_prob = 1.0 - betting_home_prob
        
        # Calculate value opportunities
        away_value = community_away_prob - betting_away_prob
        home_value = community_home_prob - betting_home_prob
        
        # Significant POSITIVE value threshold (5% probability difference)
        # Only show opportunities where community > betting (positive value)
        if away_value >= 0.05 or home_value >= 0.05:
            if away_value >= 0.05:
                # Positive value on away team
                value_team = game['away_team']
                value_amount = away_value
                team_type = "AWAY"
            elif home_value >= 0.05:
                # Positive value on home team
                value_team = game['home_team']
                value_amount = home_value
                team_type = "HOME"
            else:
                # This shouldn't happen given our filter above, but just in case
                continue
            
            # Determine if this team is favorite or underdog based on spread
            if game['market_spread'] > 0:
                # Home team underdog, away team favorite
                if team_type == "AWAY":
                    fav_or_dog = "FAVORITE"
                else:
                    fav_or_dog = "UNDERDOG"
            elif game['market_spread'] < 0:
                # Home team favorite, away team underdog
                if team_type == "HOME":
                    fav_or_dog = "FAVORITE"
                else:
                    fav_or_dog = "UNDERDOG"
            else:
                fav_or_dog = "EVEN"
            
            # Format the betting recommendation
            if team_type == "AWAY":
                # Away team value - they get the opposite of home spread
                away_spread = -game['market_spread']
                if away_spread > 0:
                    bet_line = f"{value_team} +{away_spread:.1f}"
                elif away_spread < 0:
                    bet_line = f"{value_team} {away_spread:.1f}"
                else:
                    bet_line = f"{value_team} EVEN"
            else:
                # Home team value - they get the home spread
                home_spread = game['market_spread']
                if home_spread > 0:
                    bet_line = f"{value_team} +{home_spread:.1f}"
                elif home_spread < 0:
                    bet_line = f"{value_team} {home_spread:.1f}"
                else:
                    bet_line = f"{value_team} EVEN"
            
            value_opportunities.append({
                'game': f"{game['away_team']} @ {game['home_team']}",
                'bet_line': bet_line,
                'value_team': value_team,
                'value_amount': value_amount,
                'community_prob': community_away_prob if team_type == "AWAY" else community_home_prob,
                'betting_prob': betting_away_prob if team_type == "AWAY" else betting_home_prob,
                'fav_or_dog': fav_or_dog,
                'spread': game['market_spread']
            })
    
    # Sort by value amount (highest positive value first)
    value_opportunities.sort(key=lambda x: x['value_amount'], reverse=True)
    
    print(f"Found {len(value_opportunities)} value opportunities (>=5% probability difference):\n")
    
    favorite_values = 0
    underdog_values = 0
    
    for i, opp in enumerate(value_opportunities, 1):
        print(f"{i}. {opp['game']}")
        print(f"   Value opportunity: {opp['bet_line']}")
        print(f"   Community probability: {opp['community_prob']:.1%}")
        print(f"   Betting implied probability: {opp['betting_prob']:.1%}")
        print(f"   Value: {opp['value_amount']:+.1%}")
        print()
        
        if opp['fav_or_dog'] == "FAVORITE":
            favorite_values += 1
        elif opp['fav_or_dog'] == "UNDERDOG":
            underdog_values += 1
    
    # Summary statistics
    print("SUMMARY:")
    print(f"Total value opportunities: {len(value_opportunities)}")
    print(f"Value on favorites: {favorite_values}")
    print(f"Value on underdogs: {underdog_values}")
    
    if len(value_opportunities) > 0:
        fav_pct = (favorite_values / len(value_opportunities)) * 100
        dog_pct = (underdog_values / len(value_opportunities)) * 100
        print(f"Favorite bias: {fav_pct:.1f}%")
        print(f"Underdog bias: {dog_pct:.1f}%")
        
        if fav_pct > 70:
            print("\n⚠️  STRONG FAVORITE BIAS DETECTED")
        elif dog_pct > 70:
            print("\n⚠️  STRONG UNDERDOG BIAS DETECTED") 
        else:
            print("\n✅ BALANCED VALUE DETECTION")

if __name__ == "__main__":
    analyze_probability_value()