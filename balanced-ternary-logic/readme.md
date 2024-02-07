# Balanced Ternary Logic

I have been unhappy with the existing [Three-valued logic](https://en.wikipedia.org/wiki/Three-valued_logic) which mostly involve a true state, a false state, and something in between like unknown.

I have been trying to derive a form of logic which would better represent balanced ternary numbers, with positive, zero, and negative. The purpose behind this has been to find a balanced ternary analog for a [binary adder](https://en.wikipedia.org/wiki/Adder_(electronics)) This can be used to develop logic gates which handle positive and negative electric flows for a balanced ternary computer. It can take advantage of 3 having the most efficient [radix economy](https://en.wikipedia.org/wiki/Radix_economy).

Let's see if we can use binary logic gates as an analog for balanced ternary gates.

## Addition

In electric circuits, addition between bits is split into two parts - The sum, and the carry.

The sum of two bits can be calculated by using the XOR gate.

The carry of two bits can be calculated by using the AND gate.

Here is the truth table of balanced ternary addition:

| Sum | - | 0 | + |
|:-:|:-:|:-:|:-:|
| **-** | + | - | 0 |
| **0** | - | 0 | + |
| **+** | - | + | - |

Let's try to define sum as XOR.

| Carry | - | 0 | + |
|:-:|:-:|:-:|:-:|
| **-** | + | 0 | 0 |
| **0** | 0 | 0 | 0 |
| **+** | 0 | 0 | + |

Let's try to define the carry as AND.

Now, to make the a bit more complicated, if we were to implement a full adder, we can copy the binary logic gates, however they require an OR.

Using intuition and a bit of experimentation, I have defined OR as follows:

| OR | - | 0 | + |
|:-:|:-:|:-:|:-:|
| **-** | - | - | 0 |
| **0** | - | 0 | + |
| **+** | 0 | + | + |

Here is the binary logic for a full adder:

* Sum = A XOR B XOR Old Carry
* New Carry = (A AND B) OR (Old Carry OR (A XOR B)

Using the same formulas with our newly defined ternary logic, results in this table:

| A | B | Old Carry | = | New Carry | Sum |
|:-:|:-:|:-:|:-:|:-:|:-:|
| - | - | - |   | - | 0 |
| - | - | 0 |   | - | + |
| - | - | + |   | 0 | - |
| - | 0 | - |   | - | + |
| - | 0 | 0 |   | 0 | - |
| - | 0 | + |   | 0 | 0 |
| - | + | - |   | 0 | - |
| - | + | 0 |   | 0 | 0 |
| - | + | + |   | 0 | + |
| 0 | - | - |   | - | + |
| 0 | - | 0 |   | 0 | - |
| 0 | - | + |   | 0 | 0 |
| 0 | 0 | - |   | 0 | - |
| 0 | 0 | 0 |   | 0 | 0 |
| 0 | 0 | + |   | 0 | + |
| 0 | + | - |   | 0 | 0 |
| 0 | + | 0 |   | 0 | + |
| 0 | + | + |   | + | - |
| + | - | - |   | 0 | - |
| + | - | 0 |   | 0 | 0 |
| + | - | + |   | 0 | - |
| + | 0 | - |   | 0 | 0 |
| + | 0 | 0 |   | 0 | + |
| + | 0 | + |   | + | - |
| + | + | - |   | 0 | + |
| + | + | 0 |   | + | - |
| + | + | + |   | + | 0 |

This formula exactly matches ternary addition between 3 numbers. Awesome.

But how to handle negation?

## Subtraction

Something of interest in binary logic, is that NOT is not the same thing as negation. To perform negation, computers usually have to perform [Two's complement](https://en.wikipedia.org/wiki/Two%27s_complement), where you first do NOT, and then add one.

This logic might be simpler with balanced ternary, because the negative of any balanced ternary number simply involves flipping around the signs. For example the negative of +0-- is -0++.
