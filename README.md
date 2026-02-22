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
## Parsing

| Component    | Syntax                           | Description                            | Example        |
| :---         | :---                             | :---                                   | :---           |
| die          | {int}'d'{int} / 'd'{int} / {int} | A single die.                          | 1d20, d12, d8  |
| advantage    | 'adv'                            | Roll 2 dice and take the highest value | 1d20adv        |
| disadvantage | 'dis'                            | Roll 2 dice and take the lower value   | 1d20dis        |
| modifier     | +{int} / -{int}                  | Modify the final value                 | 1d12+3, 1d12-5 |


## Examples
```bash
#====> SIMPLE ROLLS <=====#
❯ ./roll 2d20adv
2d20: [17, 13] = 17

❯ ./roll 2d20dis    # for adv/dis rolls, amount may be not specified, 1 or 2
2d20: [20, 6] = 6

❯ ./roll 3d8
3d8: [3, 8, 1] = 12

❯ ./roll 4d6+5
4d6: [5, 4, 6, 4] + 5 = 24

❯ ./roll 4d6-3
4d6: [6, 2, 3, 5] - 3 = 13

❯ ./roll 4d6-3+5-1  # you can use multiple modifiers
4d6: [2, 6, 5, 3] + 1 = 17

#====> ARGUMENTS ROLLS <=====#
# you can use quick arguments for adv/dis rolling
❯ ./roll -a
2d20: [4, 14] = 14

❯ ./roll -d
2d20: [5, 13] = 5

#====> SET OF ROLLS <=====#
❯ ./roll 12 d20adv+3 3d4-2
1d12: [1] = 1
2d20: [20, 20] + 3 = 23
3d4: [2, 2, 3] - 2 = 5

Total: 29 # sum of every roll
```
