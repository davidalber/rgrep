This is the guessing game exercise from [Chapter 2 of the Rust Book](https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html) with some additional features that were requested by my son.

With no arguments, the program plays a guessing game with a number between 1 and 100.
```
$ cargo run
I'm thinking of a number between 1 and 100. Guess the number!
Please input your guess.
```

With one argument, you control the lower bound of the range.
```
$ cargo run 60
I'm thinking of a number between 60 and 100. Guess the number!
Please input your guess.
```

Two arguments control the lower and upper bounds of the range.
```
$ cargo run 10 20
I'm thinking of a number between 10 and 20. Guess the number!
Please input your guess.
```
