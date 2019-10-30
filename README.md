# snek

## Module organization

### Main

Entry point of the game.

- Parses commands/option flags.
- Creates and invokes a `Driver`.

### Driver

Run the animation/update/event loop.

Owns: a `Terminal`, a `Game`.

At each animation tick:

- Updates its `Game`, accounting for user input from its `Terminal`.
- Has its `Terminal` render a `UI` based on its `Game`.

### Terminal

Wraps interface between game and user.

- Detects and reports user input.
- Renders a `UI`.

### UI

Wraps a `Game`, turning it into a `Terminal`-renderable format.

### Game

Maintains and updates game state.

Owns: a `Snake`, `Food`.

- Tracks `Snake`, `Food` in bounded 2D space.
- Updates in response to animation ticks, input events.
- Detects and reports game events, such as collisions.

#### Snake

Represents a snake object.

#### Food

Represents a food object.

## Questions/enhancements

- Snake can wrap around edges?
- Food can have multiple colors for multiple values?
- Multiple foods?
- Multiple snakes?
- Customize food/snake sprites with flags
- Networked play
