# Synopsis
FoE Tactics is a multiplayer, round-based strategy game.

# Game Mechanics
Every player controls four characters. Prior to the game a player can choose stats of his characters and their respective loadout and skillsets

## Core Loop
- Move character
- Attack opponent

### Advanced mechanics
All actions cost action points. A character receives action points per turn based on Agility.
A character can:
- Move (base cost of movement is 1AP per Tile, more if the terrain is less acessible)
- Attack (cost is dependent on which attack is performed)
- Open inventory
- Use Item
- Cast Spell
- Fly (if enabled, a character can fly, 1AP can lift up to 2 Height units)
- spend XP

### Action Points
Action points are burned to perform actions. A character starts with floor(5 + (Agility / 2)) and receives the same number of Action Points as they have agility at the start of their players turn. Action Points can be stored up to twice the endurance.

### Movement
The map is a hex grid. Pathfinding will be implemented via A*. Tiles have accessibility and movement cost. On each tile only one character can stand.
Moving over a tile will cost it's movement cost in AP. A character can only spend action points for movement up to his endurance in tiles.

### Races
There are three pony races to choose from:
- Earth pony
- Pegasi
- Unicorn

Earth ponies receive a boon to strength, Pegasi can fly and Unicorns can do Magic

#### Skill Trees
The game will reward XP for certain conditions (like killing an enemy), that can be spent on enhancing abilities or buying new skills.
Unicorns have access to the Magic Skill Tree, Pegasi have access to the Flight Skill Tree.

## End of the game
Player with last pony standing wins

## Game Setup
After successfully connecting, a random turn order is determined and each player places one character on each turn until all characters are placed.
After all characters are placed, the turn order is reversed and each player controls all his characters