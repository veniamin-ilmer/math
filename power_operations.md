# Power Operations

This is an alternative math representation for exponentation.

## Motivation

Off the top of your head:

*Multipication* is to *Division*, as *Exponention* is to *what*?

Children learn addition, subtraction, multiplication, and division in grade school, yet exponentation and logarithms are left over for precalculus students to fully understand.

Multiplication is repeated addition. Exponentation is repeated multiplication.

Yet, exponentation is considered much more complicated than multiplication.

I feel the reason for this, is a poor representation for exponentation and logarithms.

It is inconsistent with all the other math operators, and makes it look quite alien to a grade school student.

## Building a better representation

### Anti-exponentation

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

Following the pattern, we'd want the reverse of exponentation to be repeated division.

So for example, I have the number 100, and want to keep dividing it by, say 3, until the number becomes less than my divisor.

* 100 / 3 = 33.333...
* 33.333... / 3 = 11.111...
* 11.111... / 3 = 3.703...
* 3.703... / 3 = 1.235...

Since 1.235.. is smaller than 3, we stop divising.

In total, we needed to divide 4 times, so 4 with some remainder is our answer.

In our current math system, how do we represent this calculation?

Well, that would be ln(100) / ln(3)

Why does the reverse of exponentation require us to use three math operations? Convention.

Even the shorter variant, log<sub>3</sub>(100), is difficult to read.

3 + 4 is much easier to read than say add<sub>4</sub>(3).

Let's redefine the reverse of exponentation as:

x <sub>v</sub> y = ln(x) / ln(y)

Hence, in our example 100 <sub>v</sub> 3 = about 4

I will call this operation *apower*, short for anti-power.

### Exponentation

Although having a single operation for the reverse of exponentation, is an excellent step towards consistency, there is one more important step we should do.

Exponentation is unusual because the order of operations matter.

a<sup>b</sup> does not equal b<sup>a</sup>

Now that we have *apower*, let's try and analyze what should be the correct order.

Starting with addition and subtraction. What is the solution to this problem?

a + b - b + b - b + b - b

Visually, it is easy to see that the `b`s cancel out, and we just have `a` remaining.

Okay, lets apply the same calculation to multiplication and division..

a * b / b * b / b * b / b

Visually, it is easy to see that the `b`s cancel out, and we just have `a` remaining.

What about exponentation?

First, let's see how that looks with standard notation, shall we?

a ^ b <sub>v</sub> b <sub>^</sub> b <sub>v</sub> b ^ b <sub>v</sub> b

b<sup>(b<sup>(b<sup>a</sup> <sub>v</sub> b)</sup> <sub>v</sub> b)</sup> <sub>v</sub> b

ln(b<sup>(ln(b<sup>(ln(b<sup>a</sup>) / ln(b))</sup>) / ln(b))</sup>) / ln(b)
