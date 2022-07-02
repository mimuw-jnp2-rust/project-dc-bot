# Discord Game Bot

## Autorzy
- Katarzyna Mielnik (gr 4, @mielnikk na githubie)
- Julia Podrażka (gr 4, @julia-podrazka na githubie)

## Opis
Bot do Discorda umożliwiający granie w Wordle.

## Funkcjonalność
Gra w Wordle na czacie. Każdy użytkownik może rozpocząć instancję gry dla siebie, przy czym może mieć maksymalnie jedną aktywną rozgrywkę w obrębie jednego kanału. 

## Użycie
Token do bota osadzonego na swoim serwerze należy umieścić odpowiednim structcie w `config.rs`.

Następnie odpalić bota za pomocą 
```
cargo run
```

## Zmiany w drugiej części
- możliwość grania w grupie
- opcja "poddania się" poprzez dodanie reakcji białej flagi - bot wyświetla wtedy słowo oraz jego definicję zdobytą za pomocą API słownika
- pięciominutowe ograniczenie czasowe na każdą grę

## Biblioteki
W głównej mierze Serenity, pojawiły się również Tokio i Serde.
