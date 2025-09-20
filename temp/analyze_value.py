#!/usr/bin/env python3
"""
Analyze value opportunities in NFL predictions vs market spreads.
"""

import csv

def analyze_value_opportunities():
    """Analyze if there's a bias toward favorites in value opportunities."""
    
    games = []
    
    # Read the CSV data
    with open('nfl_predictions.csv', 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            games.append({
                'away_team': row['Away Team'],
                'home_team': row['Home Team'],
                'predicted_away': float(row['Predicted Away Score']),
                'predicted_home': float(row['Predicted Home Score']),
                'market_spread': float(row['Market Spread (Home)'].replace('+', '')),
                'confidence': float(row['Confidence'])
            })
    
    print("VALUE OPPORTUNITY ANALYSIS")
    print("=" * 50)
    
    value_opportunities = []
    
    for game in games:
        # Calculate predicted spread (from home team perspective)
        predicted_spread = game['predicted_home'] - game['predicted_away']
        market_spread = game['market_spread']
        
        # Calculate the difference (positive = model likes home team more than market)
        spread_diff = predicted_spread - market_spread
        
        # Determine who the market favorite is
        if market_spread < 0:
            market_favorite = game['home_team']
            market_underdog = game['away_team']
        elif market_spread > 0:
            market_favorite = game['away_team'] 
            market_underdog = game['home_team']
        else:
            market_favorite = "EVEN"
            market_underdog = "EVEN"
        
        # Determine value opportunity
        if abs(spread_diff) >= 2.0:  # Significant difference threshold
            if spread_diff > 2.0:
                # Model likes home team more than market
                value_team = game['home_team']
                value_type = "HOME" if market_spread > 0 else "FAVORITE" if market_spread < 0 else "HOME"
            else:
                # Model likes away team more than market  
                value_team = game['away_team']
                value_type = "AWAY" if market_spread < 0 else "FAVORITE" if market_spread > 0 else "AWAY"
            
            # Determine if value is on favorite or underdog
            if market_favorite == "EVEN":
                fav_or_dog = "EVEN"
            elif value_team == market_favorite:
                fav_or_dog = "FAVORITE"
            else:
                fav_or_dog = "UNDERDOG"
            
            value_opportunities.append({
                'game': f"{game['away_team']} @ {game['home_team']}",
                'predicted_spread': predicted_spread,
                'market_spread': market_spread,
                'spread_diff': spread_diff,
                'value_team': value_team,
                'market_favorite': market_favorite,
                'fav_or_dog': fav_or_dog,
                'confidence': game['confidence']
            })
    
    # Print detailed analysis
    print(f"Found {len(value_opportunities)} value opportunities (>=2 point difference):\n")
    
    favorite_values = 0
    underdog_values = 0
    
    for i, opp in enumerate(value_opportunities, 1):
        print(f"{i}. {opp['game']}")
        print(f"   Predicted spread: {opp['predicted_spread']:+.1f}")
        print(f"   Market spread: {opp['market_spread']:+.1f}")
        print(f"   Difference: {opp['spread_diff']:+.1f}")
        print(f"   Value team: {opp['value_team']}")
        print(f"   Market favorite: {opp['market_favorite']}")
        print(f"   Value on: {opp['fav_or_dog']}")
        print(f"   Confidence: {opp['confidence']:.2f}")
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
            print("The model appears to consistently favor betting on favorites.")
        elif dog_pct > 70:
            print("\n⚠️  STRONG UNDERDOG BIAS DETECTED") 
            print("The model appears to consistently favor betting on underdogs.")
        else:
            print("\n✅ BALANCED VALUE DETECTION")
            print("The model shows relatively balanced value opportunities.")

if __name__ == "__main__":
    analyze_value_opportunities()