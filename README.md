# paddle_game_rust_macroqaud
building a paddle game in rust (not c++ because i am afraid of it) using macroquad game library
# you can play my game by pressing this link on a web browser
- https://d7mnch.github.io/paddles_game_rust_macroqaud/

# Screenshots

- for the people who don't use screenshots, if you love god pls use them in your projects
![Alt text](/screenshots/ping_pong.png?raw=true "ping pong game")

#  running the Game
- Need to get rust first
``` bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
- `git clone` this repo
- `cargo run` at the root directory (is where the file `Cargo.toml`on it !)

> [!warning] No such file or directory
> You need to be on the directory that have `cargo.toml` on it 
## Control the paddles

### this for left paddle
| Keys | action                |
| ---- | --------------------- |
| `s`  | move left paddle down |
| `w`  | move lett paddle up   |

### this for the right paddle
| Keys   | action                 |
| ------ | ---------------------- |
| `Up`   | move right paddle down |
| `Down` | move right paddle up   |

## settings
| Keys   | action                  |
| ------ | ----------------------  |
| `Space`| suspend (stops the game)|


## What i learn
- you can make multiple  related structs on one file, if those structs are small
- it's better to have Gui library for ui's stuff, it makes manipulating parameters at runtime (save a lot of time)
- Get rid off magic numbers
- use enums everytime you are facing a state (control, gamestate)
- use structs as much as you can when you see related data
- don't mix functionality into one big function, separate them using functions
- Tratis need context (like update needs game state in order to update the entity based on that state)

## TODO


