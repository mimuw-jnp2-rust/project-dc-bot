# Discord Game Bot

## Autors
- Katarzyna Mielnik @mielnikk
- Julia Podrażka @julia-podrazka

## Description
Discord bot for playing Wordle.
Each user can start a game to play alone or in a group.
If you want to give up a game, click on a white flag in reactions under your last guess - then the game will be finished, and you will see a correct word with its definition.
Each game has a timer - you only have 5 minutes to guess the word.

## Installing a bot
First, put your bot's token in the correct struct in `config.rs`.

Then run command:
```
cargo run
```

## Libraries
Our program uses primarily Serenity, as well as Tokio and Serde.
