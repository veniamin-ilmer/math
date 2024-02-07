# Balanced Ternary Logic

I have been unhappy with the existing [Three-valued logic](https://en.wikipedia.org/wiki/Three-valued_logic) which mostly involve a true state, a false state, and something in between like unknown.

I have been trying to derive a form of logic which would better represent balanced ternary numbers, with positive, zero, and negative.

The purpose behind this has been to find a balanced ternary analog for a [binary adder](https://en.wikipedia.org/wiki/Adder_(electronics))

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

Now, to make the a bit more complicated, if we were to implement a full adder, we can copy the binary logic gates:

* Sum = A XOR B XOR Old Carry
* New Carry = (A AND B) OR (Old Carry OR (A XOR B)

To do this with balanced ternary, although we can use our AND and XOR gates, we would need to define what is OR.

After a bit of experimentation, I have defined OR as follows:

| OR | - | 0 | + |
|:-:|:-:|:-:|:-:|
| **-** | - | - | 0 |
| **0** | - | 0 | + |
| **+** | 0 | + | + |

Testing out the formulas from above:

Sum = A XOR B XOR Old Carry
New Carry = (A AND B) OR (Old Carry OR (A XOR B)

Results in this table:

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

Luckily this formula exactly matches ternary addition between 3 numbers. Awesome.
