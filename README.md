# Dice
A small CLI for rolling Dungeons & Dragons style dice.

Supports:
- Standard dice: `d4`, `d6`, `d8`, `d10`, `d12`, `d20`, `d100`.
- Multiple dice: `8d6`, `2d8`, etc.
- Advantage and Disadvantage rolls.
- Heterogeneous roll sets (`3d6 2d8 1d10`)

---

## Index

- [Features](#Dice)
- [Installation](#Installation)
- [Usage](#usage)
  - [Basic Rolls](#basic-rolls)
  - [Multiple Rolls](#multiple-rolls)
  - [Advantage / Disadvantage](#advantage-/-disadvantage-rolls)

---

# Installation
```bash
git clone www.github.com/schiz0idcat/roll.git
cd roll
cargo build --release
```

# Usage
## Basic rolls
```bash
roll 1d6
roll 3d12
roll d4     # amount is optional
roll 20     # if the amount is not specified, the 'd' is optional
```

## Multiple rolls
```bash
roll 1d6 3d12 1d4 1d20
roll 1d6 3d12 d4 20     # you can mix the syntax
```

## Advantage / Disadvantage rolls
```bash
roll -a 2d20
roll --advantage d20

roll -d 20
roll --disadvantage      # if no die specified, use a d20 by default 
```
