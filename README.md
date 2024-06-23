# Tetris

Modern Tetris TUI written in Rust

![Preview](preview.png)

## Installation

The project can be installed from source with the following:

```bash
git clone https://github.com/abusch8/Tetris
cd Tetris
make clean install
```

The repo is also available as a package on the AUR.  It can be installed from source using an AUR helper such as Yay:

```bash
yay -S tetris-tui-git
```

## Configuration

The configuration file is located at `~/.config/tetris.ini`.

The default control scheme is as follows:

|Command            |Key            |
|-------------------|---------------|
|Rotate Right       |`[↑]` / `[W]`  |
|Move Left          |`[←]` / `[A]`  |
|Soft-Drop          |`[↓]` / `[S]`  |
|Move Right         |`[→]` / `[D]`  |
|Hard-Drop          |`[SPACE]`      |
|Rotate Left        |`[Z]`          |
|Hold               |`[C]`          |
|Quit               |`[ESC]` / `[Q]`|

## TODO

- Leaderboard
- T-Spin scoring
- Fix soft drop scoring accuracy
- Line clear animation
- Scoring feedback
- Prevent infinity
