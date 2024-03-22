# 🃏 Card Roulette (Rust)

A Rust implementation of the thrilling card game where players draw cards, hoping to avoid the deadly one.

Looking for the Python version? You can find it [here](https://github.com/Disciple0fMarx/card_roulette).

---

## Description

Card Roulette is a text-based game where players draw cards in rounds.
Each round, players draw cards randomly, and if a player draws the deadly card, they lose the round.
The game introduces a new deadly card with each iteration *(Iteration n's deadly card is card n)*.
If the deadly card is the only one remaining, all players survive.

## Requirements

- Rust 1.56.0 or later (Rust 2021 edition)


## Installation

```bash
git clone https://github.com/Disciple0fMarx/card_roulette_rust
```

## Usage

```bash
cd card_roulette_rust
cargo run
```

### Playing a game

_Default settings: 3 players, 3 rounds_

```bash
cargo run play --players 4 --rounds 2
```

### Simulating games

_Default settings: 5 games, 3 players, 3 rounds_

```bash
cargo run simulate --games 10 --players 4 --rounds 2
```

Results will be saved in the `results` folder like so:

```bash
results/
├── 3_players/
│   ├── 3_rounds/
│   │   ├── 020549a8-1daf-43c2-a0d9-b0a49e31d860.csv
│   │   └── ...
│   └── 4_rounds/
│       ├── a568cd44-9c4d-4ff2-a604-f100307040e4.csv
│       └── ...
├── 4_players/
│   └── 2_rounds/
│       ├── fe54d3d1-ff4e-41f0-a579-e22d208163ea.csv
│       └── ...
├── ...
└── total_results.json
```

- `<id>.csv` contains the scores of all players in each game in the simulation with the given UID.
- `total_results.json` stores the cumulative scores of each simulation.