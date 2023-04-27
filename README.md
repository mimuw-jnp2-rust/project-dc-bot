# Discord Game Bot

## Autors
- Katarzyna Mielnik @mielnikk
- Julia Podrażka @julia-podrazka

## Description
Discord bot for playing Wordle.

Each user can start a game to play alone or in a group.

Each channel can have multiple solo games at a time but only one multiplayer game.

There are only 5-letter words.

Each game has a timer - you only have 5 minutes to guess the word, and you are given a maximum of 6 guesses.

## Installing a bot
First, put your bot's token in the correct struct in `config.rs`.

Then run command:
```
cargo run
```

## How to play
To start a solo game enter:
```
!start
```

To start a game with your friends enter:
```
!start <number of players>
```
Now, each of your friends can enter:
```
!join
```
to play with you.

To guess, enter:
```
!guess <your guess>
```

After each guess, the color of the letters will change to show how close your guess was to the word.
If the letter is green, it is in the word and in the correct spot.
If the letter is yellow, it is in the word but in the wrong spot.
If the letter is red, it is not in the word in any spot.

If you want to give up a game, click on a white flag in reactions under your last guess or enter
```
!giveup
```
Then the game will be finished, and you will see a correct word with its definition.

To see the rules in the game enter:
```
!help
```

## Libraries
Our program uses primarily Serenity, as well as Tokio and Serde.
