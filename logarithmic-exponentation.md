# Logarithmic Exponentation

Addition and multiplication are quite useful because of their commutative properties. Exponentiation is harder to work with because of its lacking commutativity.

Inspired by the logarithmic product rule:

ln(a) + ln(b) = ln(a * b)

Let's work with a new operation:

**ln(a) * ln(b) = ln(a @ b)**

"@" just happened to be a convenient symbol available on the keyboard.

Here are some example equivalents:

**a @ b = e<sup>ln(a) * ln(b)</sup> = a<sup>ln(b)</sup> = b<sup>ln(a)</sup>**

This operation has several useful properties. I will go through each and compare with addition and multiplication.

#### Communtative property

New Op | a @ b = b @ a
-------|--------------
Multiplication | a * b = b * a
Addition | a + b = b + a


#### Associative property

New Op | (a @ b) @ c = a @ (b @ c)
-------|--------------
Multiplication | (a * b) * c = a * (b * c)
Addition | (a + b) + c = a + (b + c)

#### Distributive property

New Op | a @ (b * c) = a @ b * a @ c
-------|--------------
Multiplication | a * (b + c) = a * b + a * c

New Op | (a * b) @ (c * d) = a @ c * a @ b * b @ c * b @ d
-------|--------------
Multiplication | (a + b) * (c + d) = a * c + a * b + b * c + b * d

#### Property of 0 and identities

New Op | a @ 0 = 0 | a @ 1 = 1 | a @ e = a
-------|-----------|-----------|----------
Multiplication | a * 0 = 0 | | a * 1 = a
Addition | | | a + 1 = a

New Op | a @ e<sup>2</sup> = a * a | a @ e<sup>3</sup> = a * a * a | a @ e<sup>-1</sup> = 1 / a
-------|-----------|-----------|----------
Multiplication | a * 2 = a + a | a * 3 = a + a + a | a * -1 = -a

Notice how @ is not a replacement for exponentation. However it can be used to complement exponentation.

#### Property of Powers

These are particularly interesting because they deviate from multiplication:

New Op | (a @ b)<sup>c</sup> = a<sup>c</sup> @ b = a @ b<sup>c</sup>
-------|--------------
Muliplication | (a * b)<sup>c</sup> = a<sup>c</sup> * b<sup>c</sup>

#### Derivative

<math display="block">
  <mrow>
    <mfrac>
      <mrow><mi>d</mi><mo>(</mo>
        <mi>f</mi><mo>(</mo>
          <mi>x</mi>
        <mo>)</mo>
        <mo>@</mo>
        <mi>g</mi><mo>(</mo>
          <mi>x</mi>
        <mo>)</mo>
        <mo>)</mo></mrow>
      <mrow><mi>d</mi><mi>x</mi></mrow>
    </mfrac>
  </mrow>
</math>
