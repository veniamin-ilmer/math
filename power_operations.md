# Power Operations

This is an alternative math representation for exponentiation.

## Motivation

Off the top of your head:

*Multipication* is to *Division*, as *Exponentiation* is to *what*?

Children learn addition, subtraction, multiplication, and division in grade school, yet exponentiation and logarithms are left over for precalculus students to fully understand.

Multiplication is repeated addition. Exponentiation is repeated multiplication.

Yet, exponentiation is considered much more complicated than multiplication.

I feel the reason for this, is a poor representation for exponentiation and logarithms.

It is inconsistent with all the other math operators, and makes it look quite alien to a grade school student.

## Building a better representation

### Anti-exponentiation

I feel that consistency with the rest of math operators is extremely important.

Hence, let's start with the basic math operators.

* Multiplication involves repeat adding. For example, 7 * 5 or 5 * 7 means you add 7 together 5 times.

* Division is the reverse of multiplication, and it can be done with repeat subtraction. For example, 41 / 7 means you start with 41, and keep subtracting it by 7 until the number becomes less than the divisior. The answer is how many subtractions you did, plus some remainder.

As a reminder, lets divide 40 by 7 with repeated subtraction:

* 41 - 7 = 34
* 34 - 7 = 27
* 27 - 7 = 20
* 20 - 7 = 13
* 13 - 7 = 6

Since 6 is smaller than 7, we stop subtracting.

In total, we needed to subtract 5 times. Hence, 41 / 7 = about 5.

Moving on, we already know repeat multiplication is exponentation.

Following the pattern, we'd want the reverse of exponentiation to be repeated division.

So for example, I have the number 100, and want to keep dividing it by, say 3, until the number becomes less than my divisor.

* 100 / 3 = 33.333...
* 33.333... / 3 = 11.111...
* 11.111... / 3 = 3.703...
* 3.703... / 3 = 1.235...

Since 1.235.. is smaller than 3, we stop divising.

In total, we needed to divide 4 times, so 4 with some remainder is our answer.

In our current math system, how do we represent this calculation?

Well, that would be ln(100) / ln(3)

Why does the reverse of exponentiation require us to use three math operations? Convention.

Even the shorter variant, log<sub>3</sub>(100), is difficult to read.

3 + 4 is much easier to read than add<sub>4</sub>(3).

Let's redefine the reverse of exponentation as:

x <sub>v</sub> y = ln(x) / ln(y)

Hence, in our example 100 <sub>v</sub> 3 = about 4

I will call this operation *apower*, short for anti-power.

### Exponentation

Although having a single operation for the reverse of exponentiation, is an excellent step towards consistency, there is one more important step we should do.

Exponentiation is unusual because the order matters.

a<sup>b</sup> does not equal b<sup>a</sup>

Now that we have *apower*, let's try and analyze what should be the correct order.

Starting with addition and subtraction. What is the solution to this problem?

a + b - b + b - b

Visually, it is easy to see that the `b`s cancel out, and we just have `a` remaining.

Okay, lets apply the same calculation to multiplication and division..

a * b / b * b / b

Visually, it is easy to see that the `b`s cancel out, and we just have `a` remaining.

What about exponentiation?

First, let's see how that looks with standard notation, shall we?

ln(b<sup>(ln(b<sup>a</sup>) / ln(b))</sup>) / ln(b)

or:

log<sub>b</sub>(b<sup>(log<sub>b</sub>(b<sup>a</sup>))</sup>)

Thought that was hard to read? It was even harder to write up.

Okay, now let's convert it to use the new *apower* operator:

b<sup>(b<sup>a</sup> <sub>v</sub> b)</sup> <sub>v</sub> b

Although it looks a little cleaner, it still is inconsistent with the addition and multiplication examples.

Biggest difference: Notice how `a` is in the middle of the equation, rather than the left side.

The reason why is that the reverse of exponentiation, reverses the left side of the exponent, rather than the right side of the exponent.

So, perhaps the exponent order is wrong?

Let me redefine *power* as this:

a <sup>^</sup> b = b<sup>a</sup>

I have reversed the order in which something is placed to a power.

This means: 2 <sup>^</sup> a = a * a

By doing this switch, we finally get a simple answer that is consistent with the addition and multiplication:

a <sup>^</sup> b <sub>v</sub> b <sup>^</sup> b <sub>v</sub> b = a

## Properties of *power* and *apower*

After years of use, it's difficult to mentally just redefine exponentiation, so let me go over all of the properties of *power* and *apower* with you:

### Order of operations

1 <sup>^</sup> 2 <sup>^</sup> 3 <sub>v</sub> 4 <sup>^</sup> 5 is read as (((1 <sup>^</sup> 2) <sup>^</sup> 3) <sub>v</sub> 4) <sup>^</sup> 5

(You just go left to right. Treat <sup>^</sup> and <sub>v</sub> the same.

1 + 2 * 3 <sup>^</sup> 5 * 8 <sub>v</sub> 9 / 4 - 10 is read as 1 + (2 * (3 <sup>^</sup> 5) * (8 <sub>v</sub> 9) / 4) - 10

Order of operations:

1. *power* and *apower*
2. *multiplication* and *division*
3. *addition* and *subtraction*

### *Power* Identities

* 2 <sup>^</sup> 5 = 25
* 5 <sup>^</sup> 2 = 32

---

* 3 <sup>^</sup> a = a * a * a
* 2 <sup>^</sup> a = a * a
* 1 <sup>^</sup> a = a
* 0 <sup>^</sup> a = 1
* (-1) <sup>^</sup> a = 1 / a
* (-2) <sup>^</sup> a = 1 / (a * a) = 1 / 2 <sup>^</sup> a

---

* 2 <sup>^</sup> (a + b) = (a + b) * (a + b)
* 3 <sup>^</sup> (a + b) = (a + b) * (a + b) * (a + b)
* 4 <sup>^</sup> (a + b) = (a + b) * (a + b) * (a + b) * (a + b)

---

* a <sup>^</sup> (b * c) = a <sup>^</sup> b * a <sup>^</sup> c
* a <sup>^</sup> (b / c) = a <sup>^</sup> b / a <sup>^</sup> c

---

* (a + b) <sup>^</sup> 2 = a <sup>^</sup> 2 * b <sup>^</sup> 2
* (a - b) <sup>^</sup> 2 = a <sup>^</sup> 2 / b <sup>^</sup> 2

---

* a <sup>^</sup> (b <sup>^</sup> c) = (a * b) <sup>^</sup> c

### *APower* Identities

* 10000 <sub>v</sub> 10 = 4
* 64 <sub>v</sub> 2 = 6

---

* a <sub>v</sub> c + b <sub>v</sub> c = (a * b) <sub>v</sub> c
* a <sub>v</sub> c - b <sub>v</sub> c = (a / b) <sub>v</sub> c

---

* a * b <sub>v</sub> c = a <sup>^</sup> b <sub>v</sub> c
* (1 / a) * b <sub>v</sub> c = b <sub>v</sub> a <sup>^</sup> c

---

* 1 / a <sub>v</sub> b = b <sub>v</sub> a
* a <sub>v</sub> c / b <sub>v</sub> c = a <sub>v</sub> b

### Calculus identities

* (pi * i) <sup>^</sup> e = -1
* (2 * pi * i) <sup>^</sup> e = 1

---

* Derivative (x <sup>^</sup> c) / dx = c <sub>v</sub> e * x <sup>^</sup> c
* Derivative (c <sup>^</sup> x) / dx = c * (c - 1) <sup>^</sup> x
* Derivative (x <sub>v</sub> c) / dx = e <sub>v</sub> c / x
* Derivative (c <sub>v</sub> x) / dx = -c <sub>v</sub> x / x <sup>^</sup> x <sub>v</sub> e

---

* Integral (x <sup>^</sup> c) dx = e <sub>v</sub> c * x <sup>^</sup> c + constant
* Integral (c <sup>^</sup> x) dx = (c + 1) ^ x / (c + 1) + constant
* Integral (x <sub>v</sub> c) dx = x * (x <sub>v</sub> c - e <sub>v</sub> c)
* Integral (c <sub>v</sub> x) dx = c <sub>v</sub> e * li(x)

## Algebra with *power* and *apower*

If x <sup>^</sup> y = z then

* x = z <sub>v</sub> y
* y = (1 / x) <sup>^</sup> z

If x <sub>v</sub> y = z then

* x = z <sup>^</sup> y
* y = (1 / z) <sup>^</sup> x
