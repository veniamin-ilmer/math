# Balanced Ternary Algebra

I have been unhappy with the existing [Three-valued logic](https://en.wikipedia.org/wiki/Three-valued_logic) which is usually true, unknown, and false. I have been trying to derive a form of logic which would better represent balanced ternary numbers, with positive, zero, and negative.

The purpose behind this is to find a [balanced ternary](https://en.wikipedia.org/wiki/Balanced_ternary) analog for a [boolean algebra](https://en.wikipedia.org/wiki/Boolean_algebra). This can be used to develop logic gates which handle positive and negative electric flows for a balanced ternary computer. It can take advantage of 3 having the most efficient [radix economy](https://en.wikipedia.org/wiki/Radix_economy).

Whenever I bring up this kind of balanced ternary gates, people are quick to interpret it from a logic perspective, and run into philosophical problems. I think that actually has been holding 3 value logic back from being useful. That's why I am starting from a algebraic perspective, to make it useful first, and only afterward see how the result can be logically interpreted.

Let's see if we can use boolean algebra as an analog for balanced ternary algebra.

## Addition

In electric circuits, addition between bits is split into two parts - The sum, and the carry.

The sum of two bits can be calculated by using the XOR gate.

The carry of two bits can be calculated by using the AND gate.

Here is the truth table of balanced ternary addition:

| Sum | - | 0 | + |
|:-:|:-:|:-:|:-:|
| **-** | + | - | 0 |
| **0** | - | 0 | + |
| **+** | 0 | + | - |

Let's try to define sum as XOR. It is both commutative and associative.

| Carry | - | 0 | + |
|:-:|:-:|:-:|:-:|
| **-** | - | 0 | 0 |
| **0** | 0 | 0 | 0 |
| **+** | 0 | 0 | + |

Let's try to define the carry as AND. It is both commutative and associative.

Now, to make the a bit more complicated, if we were to implement a full adder, we can try to use the formula used boolean algebra for the Carry, however it requires an OR:

(A AND B) OR (Old Carry AND (A XOR B))

Experimenting a bit, I found with these balanced ternary gates, XOR can replace OR. Here is the balanced ternary algebra for a full adder:

* Sum = A XOR B XOR Old Carry
* New Carry = (A AND B) XOR (Old Carry AND (A XOR B))

Using this formula with our newly defined ternary algebra, results in this table:

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
| + | - | + |   | 0 | + |
| + | 0 | - |   | 0 | 0 |
| + | 0 | 0 |   | 0 | + |
| + | 0 | + |   | + | - |
| + | + | - |   | 0 | + |
| + | + | 0 |   | + | - |
| + | + | + |   | + | 0 |

This formula exactly matches ternary addition between 3 numbers. Awesome.

But how to handle negation?

## Subtraction

Something of interest in boolean algebra, is that NOT is not the same thing as negation. To perform negation, computers usually have to perform [Two's complement](https://en.wikipedia.org/wiki/Two%27s_complement), where you first do NOT, and then add one.

This algebra might be simpler with balanced ternary, because the negative of any balanced ternary number simply involves flipping around the signs. For example the negative of +-0- is -+0+. The concept of being negative, is built into balanced ternary, which may make its use simpler within the gates. If we define NOT as follows, then it will equal negation:

| Input | Output |
|:-----:|:------:|
| **-** | + |
| **0** | 0 |
| **+** | - |

The sum was defined as A XOR B. So in subtraction to find a difference, we would negate B, hence do A XOR NOT(B):

| Difference | - | 0 | + |
|:-:|:-:|:-:|:-:|
| **-** | 0 | - | + |
| **0** | + | 0 | - |
| **+** | - | + | 0 |

(A is vertical rows, B is horizontal columns)

The carry was defined as A AND B. So in subtraction to find a difference, we would negate B, hence do A AND NOT(B):

| Borrow | - | 0 | + |
|:-:|:-:|:-:|:-:|
| **-** | 0 | 0 | - |
| **0** | 0 | 0 | 0 |
| **+** | + | 0 | 0 |

Awesome! Now let's take the previous formula for a full adder:

* Sum = A XOR B XOR Old Carry
* New Carry = (A AND B) XOR (Old Carry AND (A XOR B))

And convert it to a full subtractor:

* Difference = A XOR NOT(B) XOR NOT(Old Borrow)
* New Borrow = (A AND NOT(B)) XOR (NOT(Old Borrow) AND (A XOR NOT(B)))

This reveals the full subtractor table:

| A | B | Old Borrow | = | New Borrow | Difference |
|:-:|:-:|:-:|:-:|:-:|:-:|
| - | - | - |   | 0 | + |
| - | - | 0 |   | 0 | 0 |
| - | - | + |   | 0 | - |
| - | 0 | - |   | 0 | 0 |
| - | 0 | 0 |   | 0 | - |
| - | 0 | + |   | - | + |
| - | + | - |   | 0 | - |
| - | + | 0 |   | - | + |
| - | + | + |   | 0 | - |
| 0 | - | - |   | + | - |
| 0 | - | 0 |   | 0 | + |
| 0 | - | + |   | 0 | 0 |
| 0 | 0 | - |   | 0 | + |
| 0 | 0 | 0 |   | 0 | 0 |
| 0 | 0 | + |   | 0 | - |
| 0 | + | - |   | 0 | 0 |
| 0 | + | 0 |   | 0 | - |
| 0 | + | + |   | - | + |
| + | - | - |   | + | 0 |
| + | - | 0 |   | + | - |
| + | - | + |   | 0 | + |
| + | 0 | - |   | + | - |
| + | 0 | 0 |   | 0 | + |
| + | 0 | + |   | 0 | 0 |
| + | + | - |   | 0 | + |
| + | + | 0 |   | 0 | 0 |
| + | + | + |   | 0 | - |

This formula exactly matches ternary subtraction between 3 numbers! Yey!

Two's compliment is not necessary. With this balanced ternary algebra, NOT is negation.

## Discussion

Now that we have derived each gate, let's dive into each one:

### NOT

| X | NOT(X) |
|:-----:|:------:|
| **-** | + |
| **0** | 0 |
| **+** | - |

The NOT operation flips negativity to positivity and vice versa, with neutrality remaining unchanged. This is straightforward in terms of inversion but interesting in that zero remains an immutable neutral state. From the perspective of direct electric flow, this makes sense because direct current can only flow forward, or backward or not at all. It is not possible for electricity to flow backward and forward at the same time.

Electrically, building this gate can be extremely simple - Just reverse the writes input. Positive becomes negative, negative becomes positive. Neutral stays the same.

### AND

![AND Venn diagram](and.svg)

| AND | - | 0 | + |
|:-:|:-:|:-:|:-:|
| **-** | - | 0 | 0 |
| **0** | 0 | 0 | 0 |
| **+** | 0 | 0 | + |

* Negative AND Negative = Negative: This suggests that two negative statements together reinforce the negativity.
* Positive AND Positive = Positive: Similarly, two positive statements together reinforce positivity.
* Any other variant AND'd = Zero: This implies that any uncertainty (introduced by a mix of values) results in a neutral or undefined outcome.

### XOR

![XOR Venn diagram](xor.svg)

| XOR | - | 0 | + |
|:-:|:-:|:-:|:-:|
| **-** | + | - | 0 |
| **0** | - | 0 | + |
| **+** | 0 | + | - |

* Negative XOR Negative = Positive: This could be interpreted as two negatives leading to a positive outcome when their exclusivity is considered, perhaps suggesting opposition or inversion
* Zero with anything results in the other value or remains Zero, which maintains the notion of neutrality or the lack of change unless opposed.
* Positive XOR Positive = Negative: This is particularly intriguing, suggesting that two positives, when exclusively considered, result in a negative. This could imply a concept of excess or conflict between positive forces.

### Secondary operations

Exploring the truth tables for this algebra, below are findings. & is for AND, ^ is for XOR, ! is for NOT.

* A & 0 = 0
* A ^ 0 = A
* A & A = A
* A & !A = 0
* A ^ A = !A
* A ^ !A = 0

* !(A & B) = !A & !B
* !(A ^ B) = !A ^ !B
* A & B & (A ^ B) = 0
* A & B & !(A ^ B) = 0
* A & B = A & !(A ^ B) = B & !(A ^ B)
* A & !B = A & (!A ^ B) = !B & (!A ^ B)
* A & (A ^ B) = A & (A ^ !B)
* A & (A ^ B) = (A ^ !B) & (A ^ B)
* A & (A ^ B) = A & (A ^ B ^ (A & B))

* A & (A ^ B) = (A ^ B) & (A ^ !B)
* A ^ (A & B) = !(A & B) ^ (A & (A ^ (A & B)))
* A ^ (!A & B) = (A & B) ^ (A & (A ^ B))

It would be nice to find a distribution for A ^ (B & C) and A & (B ^ C)

## Conclusion

I think establishing this balanced ternary algebra is a good first step towards building a balanced ternary computer.

If you are interested, I built a simple library using these gates here - [gates.rs](https://github.com/veniamin-ilmer/math/blob/master/balanced-ternary-algebra/gates.rs)

It would be nice to try and build these gates with transistors.
