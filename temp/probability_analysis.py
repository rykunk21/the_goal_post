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
            
            games.append({
                'away_team': row['Away Team'],
                'home_team': row['Home Team'],
                'predicted_away': float(row['Predicted Away Score']),
                'predicted_home': float(row['Predicted Home Score']),
                'market_spread': market_spread,
                'confidence': float(row['Confidence'])
            })
    
    print("PROBABILITY-BASED VALUE ANALYSIS")
    print("=" * 50)
    
    # We need to get the community probabilities from the HTML
    # For now, let's use some example probabilities based on the market data we saw
    community_probs = {
        'ATL_CAR': 0.67,  # market="33" means 33% for CAR, so 67% for ATL
        'GB_CLE': 0.81,   # market="19" means 19% for CLE, so 81% for GB
        'HOU_JAX': 0.47,  # market="53" means 53% for JAX, so 47% for HOU
        'CIN_MIN': 0.40,  # market="60" means 60% for MIN, so 40% for CIN
        'PIT_NE': 0.55,   # market="45" means 45% for NE, so 55% for PIT
        'LA_PHI': 0.37,   # market="63" means 63% for PHI, so 37% for LA
        'NYJ_TB': 0.25,   # market="75" means 75% for TB, so 25% for NYJ
        'IND_TEN': 0.65,  # market="35" means 35% for TEN, so 65% for IND
        'LV_WAS': 0.41,   # market="59" means 59% for WAS, so 41% for LV
        'DEN_LAC': 0.41,  # market="59" means 59% for LAC, so 41% for DEN
        'NO_SEA': 0.22,   # market="78" means 78% for SEA, so 22% for NO
        'DAL_CHI': 0.52,  # market="48" means 48% for CHI, so 52% for DAL
        'ARI_SF': 0.45,   # market="55" means 55% for SF, so 45% for ARI
        'KC_NYG': 0.74,   # market="26" means 26% for NYG, so 74% for KC
        'DET_BAL': 0.32,  # market="68" means 68% for BAL, so 32% for DET
    }
    
    value_opportunities = []
    
    for game in games:
        game_key = f"{game['away_team']}_{game['home_team']}"
        
        # Get community probability for away team winning
        community_away_prob = community_probs.get(game_key, 0.5)
        community_home_prob = 1.0 - community_away_prob
        
        # Calculate implied probability from betting spread
        # Positive spread means home team is underdog
        if game['market_spread'] > 0:
            # Home team is underdog, away team favored
            betting_home_prob = spread_to_probability(-game['market_spread'])
        else:
            # Home team is favored
            betting_home_prob = spread_to_probability(-game['market_spread'])
        
        betting_away_prob = 1.0 - betting_home_prob
        
        # Calculate value opportunities
        away_value = community_away_prob - betting_away_prob
        home_value = community_home_prob - betting_home_prob
        
        # Significant value threshold (5% probability difference)
        if abs(away_value) >= 0.05 or abs(home_value) >= 0.05:
            if abs(away_value) > abs(home_value):
                # Value on away team
                value_team = game['away_team']
                value_amount = away_value
                team_type = "AWAY"
            else:
                # Value on home team
                value_team = game['home_team']
                value_amount = home_value
                team_type = "HOME"
            
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
    
    # Sort by value amount (highest first)
    value_opportunities.sort(key=lambda x: abs(x['value_amount']), reverse=True)
    
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