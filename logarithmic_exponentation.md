# Logarithmic Exponentation

Addition and multiplication are quite useful because of their commutative properties. Exponentiation is harder to work with because of its lacking commutativity.

Inspired by the logarithmic product rule:

ln(a) + ln(b) = ln(a * b)

Let's work with a new operation:

ln(a) * ln(b) = ln(a @ b)

"@" just happened to be a convenient symbol available on the keyboard. It can be replaced with another symbol.

Here are some example equivalents:

a @ b = e<sup>ln(a) * ln(b)</sup> = a<sup>ln(b)</sup> = b<sup>ln(a)</sup>

This operation has several useful properties. I will go each and compare with multiplication.

#### Communtative property

Multiplication | New
---------------|--------------
a * b = b * a | a @ b = b @ a

#### Associative property

Multiplication | New
---------------|--------------
(a * b) * c = a * (b * c) | (a @ b) @ c = a @ (b @ c)

#### Distributive property

Multiplication | New
---------------|--------------
a * (b + c) = a * b + a * c | a @ (b * c) = a @ b * a @ c
(a + b) * (c + d) = a * c + a * b + b * c + b * d | (a * b) @ (c * d) = a @ c * a @ b * b @ c * b @ d

#### Property of 0 and identities

|Multiplication | New          |
|---------------|--------------|
|a * 0 = 0         | a @ 0 = 0 |
|a * 1 = a         | a @ 1 = 1 |
|                  | a @ e = a |
|a * 2 = a + a     | a @ e<sup>2</sup> = a * a |
|a * 3 = a + a + a | a @ e<sup>3</sup> = a * a * a |
|a * -1 = -a | a @ e<sup>-1</sup> = 1 / a |

Notice how @ is not a replacement for exponentation. However it can be used to complement exponentation.

#### Property of Powers

These are particularly interesting because they deviate from multiplication:

Muliplication | New          
--------------|--------------
(a * b)<sup>c</sup> = a<sup>c</sup> * b<sup>c</sup> | (a @ b)<sup>c</sup> = a<sup>c</sup> @ b = a @ b<sup>c</sup>

#### Derivative

