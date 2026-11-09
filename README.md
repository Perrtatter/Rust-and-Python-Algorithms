# Rust and Python algorithm speed test

I compare **8** algorithms to do benchmark at speed comparaison beetween Python and Rust

| Algorithm | <img src="https://thumb.wikimedia.org/wikipedia/commons/thumb/0/0f/Original_Ferris.svg/960px-Original_Ferris.svg.png?utm_source=commons.wikimedia.org&utm_campaign=index&utm_content=thumbnail" width=40> | <img src="https://thumb.wikimedia.org/wikipedia/commons/thumb/c/c3/Python-logo-notext.svg/3840px-Python-logo-notext.svg.png?utm_source=commons.wikimedia.org&utm_campaign=index&utm_content=thumbnail" width=30> | Difference |
|-----------|------|--------|--------|
| Bubble sort |      |        |        | 
| Inseration sort |      |        |        |
| Selection sort | `0.003` | 0.06 | ~ x20 |
| Dichotomic algorithm | `0.004` | 0.048 | ~ x10 |
| Hair comb sort |      |        |        |
| Coktail sort |      |        |        |
| Hanoi algorithm |      |        |        |
| Odd-even sort |      |        |        |

<br>

> **Warning**: this time are calculated on Apple M5 Macbook Pro (10 CPU, 10 GPU) (pretty good computer), the result might change with your configuration, and it was calculate **1 time**

<br>

## Why Rust is faster ?
Python is a **interpreted** langage, tho Rust is a **compile** langage :

 - **Interpreted** is slow cause a interpreter will run every single line in live.
 - **Compiled** is fast cause your PC will instantly run the code in binaire.

 <br>

 - But Python is most used and easy than Rust (so high level)
 - And in Rust your can choose RAM lenght of your variable (so low level)
