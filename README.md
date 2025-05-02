# Paddle Pong

A simple, modular Pong clone written in Rust using the [ggez](https://ggez.rs/) game framework.  
The ball is rendered as a rotating circle, it bounces off all walls and your paddle, and you earn points for each successful hit. You start with 3 lives, and the game ends when you run out.

---

## Features

- **GUI window** 800 × 600 px  
- **Paddle** you control with Left/Right arrow keys  
- **Ball**  
  - Diameter: 30 px  
  - Rotates continuously  
  - Bounces off left, right, and top walls  
  - Bounces off your paddle  
- **Score counter** increments on each paddle bounce  
- **Lives counter** starts at 3, decrements on each miss  
- **Game over** when lives drop to zero  

---

## Prerequisites

1. **Rust toolchain** (via [rustup](https://rustup.rs/))  
2. **ggez** (added as a dependency in `Cargo.toml`)  
3. A modern OS (Windows, macOS, Linux) and compatible GPU drivers  
