# spire-rs

A Rust cleanroom implementation of Slay the Spire's gameplay engine.

Canonical repo URL: https://code.kat5.dev/katie/spire-rs

## What
I like Slay the Spire (even though I am bad at it). I frequently think about how hard it would be to develop an AI that can play the game. I decided to finally do it myself and find out, and learn Rust along the way.

Currently, this means a baby game engine which implements the most basic game mechanics, a simple "AI" which can bumble its way through easy encounters, and a simulator that can predict a deck's performance when card rewards are selected.

For way more info than you probably care about, see my [dev logs](https://kat5.dev/blog/2026/spire-rs-1) about the project.

## Roadmap

- [x] Silent starting deck
- [x] Basic and Elite encounters
- [x] Powers
- [x] Monte Carlo card reward picker
- [ ] Relics
- [ ] Events
- [ ] Randomized maps
- [ ] Deterministic seeds
- [ ] The rest of the owl
- [ ] Neural network gameplay

## Generative AI Policy
Every single line of code and text in this project was written by hand. No code produced by generative models is included.