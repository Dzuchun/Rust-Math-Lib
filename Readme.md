# IMMEDIATE

- Replace regular tests with documentation tests!!!!

# General Math Lib

## Overview

This is my Rust learning project.<br>
Here I try to implement math structures and operations as abstractly as possible, while preserving their useabillity.<br>
I'm very new to Rust (like, 10 days old), so I'd appreciate any feedback on my code quality.<br>
This packge isnot meant to be published on [crates.io](crates.io), since <b>I'm very bad at Rust now. Please, be very sceptical about using this code anywhere.</b>

## Released modules

No releases yet. Hope to pack&polish at least one module soon.

## Indev modules

- Mean. Contains couple of functions to allow several "mean values" (arithmetic, geometric, harmonic and general arith-like) calculation.
- Numbers. Currently contains only [Pochhammer Symbol](https://mathworld.wolfram.com/PochhammerSymbol.html) and [Binomial Coefficient](https://mathworld.wolfram.com/BinomialCoefficient.html) for u128-bounded arguments and values. Obviously, I plan on removing mentioned restrictions and implement these objects for any complex arguments in the future. I plan on adding couple more well-known numbers such as [Bernoulli Number](https://mathworld.wolfram.com/BernoulliNumber.html) and others I might need while implementing various special functions.
- Progression. Has two structs: ArithIterator and GeometricIterator corresponding to linear and exponential sequence.
- Integration. Contains simple function - Euler (naive) integration. More algorytms planned on, like polynomial and Runge-Kutta integration (possibly macro-optimized).
- Function Macros is a separate crate (since procedural macros enforce that). It contains macro definitions that allow recursive or absolute series-defined function definition. Have a plan to create recursive Fibonacci-like definition macro too.
- Macro Functions. These contain definitions of a couple series-defined functions such as sin(x) and cos(x). Definitions are created by macros mentioned above. Plan on implementing much more functions over there.
- Matrix. Contains definition of a matrix trait, that allows for row/column extraction, and determinant evaluation (if applicable). Would especially appreciate any guidance with this module, as sometimes I'm completely lost in generic types. I'd like to implement matrix in such a way that would allow for [Cross Product](https://mathworld.wolfram.com/CrossProduct.html) definition with it. Also I dream of [Curl](https://mathworld.wolfram.com/Curl.html) definition with it. Basic matric operation were not implemented yet.
- Memoized. Defines fingle function, that would convern supplied function into memoized metrics-aware-estimator thing. This means, that (obviously) call to the same argument will just return already computed value, but also call to argument near several already computed onces (right now it's 7) would result in 7th-order polinomial estination to be computed, saved and returned. The plan is to make estimation order and number of arguments configurable.
- Traits is a sort of lib module. It contains traits I might use all over the place.
- General Functions is sort of endgame module. The idea is to define really hard-to-compute-integral-and-series-containing-functions there. But for now it's just $Ei(x)$, $E_1(x)$ and $li(x)$.

## Planned modules

- Table Generator. Idea is to tabulate function of interest for visualization in some sort of program. Might actually inlude that program too.
- Vectors. Not way i'd like without these. I plan on making them kinda <b>very abstract</b> so that they could describe things like differential operators and functional spaces too.
- Tensors. No, not graphics ones, just regular multi-dimentional matrices. High-order physics theory is <b>PACKED</b> with these.
- Differential operators. There are a lot of them, and even more numeric formlulae for each of them. Would like to implement at least a couple.
- Equation Solver. There are a couple of numeric equation-solving methods for linear and non-linear problems.
- Diff. Equation Solver. No experiment is done without a differential equation to solve.
