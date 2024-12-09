- Get player cards
- Get present pool
- Define Hashmap of remaining_pool:player_hand

## For every possible remaining pool
- Calculate player hand
- Fill in hashmap

## For every possible remaining cards (7-present_pool combination)
- Calculate best playable hand
### For every remaining pool that could have led to this combination
- Compare hand to the player hand in the hashmap
- Add to total results
