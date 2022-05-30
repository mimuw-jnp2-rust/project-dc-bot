# Discord Game Bot

## Autorzy
- Katarzyna Mielnik (gr 4, @mielnikk na githubie)
- Julia Podrażka (gr 4, @julia-podrazka na githubie)

## Opis
Bot do Discorda umożliwiający granie w Wordle.

## Funkcjonalność
Gra w Wordle na czacie. Każdy użytkownik może rozpocząć instancję gry dla siebie, przy czym może mieć maksymalnie jedną aktywną rozgrywkę w obrębie jednego kanału. 

## Użycie
Token do bota osadzonego na swoim serwerze należy umieścić w pliku `config.ron`.

Następnie odpalić bota za pomocą 
```
cargo run
```

## Plany na drugą część
- dodawanie zestawu customowych emoji na każdym serwerze, na którym się pojawi bot
- wybór długości słowa, które należy zgadnąć
- opcja "poddania się" poprzez dodanie reakcji białej flagi - bot wyświetlałby wtedy słowo oraz jego definicję zdobytą za pomocą API jakiegoś słownika
- ???

## Biblioteki
W głównej mierze Serenity, pojawiły się również Tokio i Serde.
