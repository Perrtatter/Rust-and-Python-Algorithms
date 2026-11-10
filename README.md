# Rust and Python algorithm speed test

I compare **8** algorithms to do benchmark at speed comparaison beetween Python and Rust

| Algorithm | <img src="https://thumb.wikimedia.org/wikipedia/commons/thumb/0/0f/Original_Ferris.svg/960px-Original_Ferris.svg.png?utm_source=commons.wikimedia.org&utm_campaign=index&utm_content=thumbnail" width=40> | <img src="https://thumb.wikimedia.org/wikipedia/commons/thumb/c/c3/Python-logo-notext.svg/3840px-Python-logo-notext.svg.png?utm_source=commons.wikimedia.org&utm_campaign=index&utm_content=thumbnail" width=30> | Difference |
|-----------|------|--------|--------|
| Bubble sort | `0.005s` | 0.02s | ~ x4 | 
| Inseration sort | `0.004s` | 0.04s | ~ x10 |
| Selection sort | `0.003s` | 0.06s | ~ x20 |
| Dichotomic algorithm | `0.004s` | 0.048s | ~ x10 |
| Comb sort | `0.004s` |  0.04s | ~ x10 |
| Coktail sort | `0.005s` | 0.048s | ~ x10 |
| Hanoi algorithm |      |        |        |
| Odd-even sort | `0.007s` |  0.078s | ~ x10 |

<br>

> **Warning**: this time are calculated on Apple M5 Macbook Pro (10 CPU, 10 GPU) (pretty good computer), the result might change with your configuration, and it was calculate **1 time**

<br>

## Why Rust is faster ?
Python is a **interpreted** langage, tho Rust is a **compile** langage :

 - **Interpreted** is slow cause a interpreter will run every single line in live ( and the execution time is dynamique ).
 - **Compiled** is fast cause your PC will instantly run the code in binaire ( and the execution time is fixed ).

 <br>

 - But Python is most used and easy than Rust (so high level)
 - And in Rust your can choose RAM lenght of your variable (so low level)

<br>

## Source
The code source was create from Wikipedia pseudo code.

<br>

## Tests
The `test` folder, is some file to test Rust fonction and systax ( cause i'm learning Rust ... )
