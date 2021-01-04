# Trigonometric Tiling

Playing around with triangles and hexagons, I found an infinite set of trigonometric identities, that fit neatly on a triangle tile.

I didn't find this trigonometric tiling online, so I decided to share it with everyone.

## Definition

For all triangles used in the tiling on this page:

![Triangle Definition](define-triangle.png)

### a<sup>2</sup> + b<sup>2</sup> = c<sup>2</sup>

Please note, the points have nothing to do with the triangle lengths or angles. The triangles just provide a neat way of showing all the points.

In all of the tiled triangles below, the top left point squared plus top right point squared equal the bottom point squared.

## Trigonometric Tile

![Trigonometric Tile](trigonometric-tiling.png)

Please read the multiline

![cossin](cossin.png)

as **cos θ * sin θ**.

## Example of identities from tiling

### sin<sup>2</sup>θ + cos<sup>2</sup>θ = 1
### 1 + tan<sup>2</sup>θ = sec<sup>2</sup>θ
### cot<sup>2</sup>θ + 1 = csc<sup>2</sup>θ
### csc<sup>2</sup>θ + sec<sup>2</sup>θ = csc<sup>2</sup>θ sec<sup>2</sup>θ
### cos<sup>2</sup>θ sin<sup>2</sup>θ + sin<sup>4</sup>θ = sin<sup>2</sup>θ

This trigonometric identity diagram can be extended infinitely in all directions (sin, cos, tan, csc, sec, cot).

## Simplified Trigonometric Tile

The tile can be rewritten to use just Sine and Cosine:

![Simplified Trigonometric Tile](trigonometric-tiling-simple.png)

The exponent of each point can be converted from cartesian coordinates for any point:

**cos<sup>c</sup>θ * sin<sup>s</sup>θ**

    c = (y - x) / 2

    s = (y + x) / 2

## Proof

The pattern we see in the tiling:

(f(x)<sup>a + 1</sup> * g(x)<sup>b</sup>)<sup>2</sup> + (f(x)<sup>a</sup> * g(x)<sup>b + 1</sup>)<sup>2</sup> = (f(x)<sup>a</sup> * f(x)<sup>b</sup>)<sup>2</sup>

is only true if the following is valid:

f(x)<sup>2</sup> + g(x)<sup>2</sup> = 1

Hence, the tiling can also be doing for hyberbolic functions too.

## Hyperbolic Tile

![Hyperbolic Tile](hyperbolic-tiling.png)

## More Properties

Here are a few more useful properties I found..

For all triangles:

![Reverse Triangle](reverse-triangle.png)

### a<sup>-2</sup> + b<sup>-2</sup> = c<sup>-2</sup>

--

For all hexagons:

![Hexagon](hexagon.png)

### a<sup>2</sup> = bB = cC = dD

--

For all triangles:

![Big Triangle](bit-triangle.png)

### a + b = c

## Acknowledgements

I got the idea to build this tiling after watching the video [Super Hexagon](https://www.youtube.com/watch?v=T7D1W1oD8wo).

I don't know who the original author of the "Super Hexagon" is, but it looks like it's used in some schools as a learning tool to help students with trigonometry.
