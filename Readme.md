# General Math Lib
This is my own math library, written in Rust.

Here I implement various math operations I might want to use in my Rust projects. Any feedback is greatly appreciated.

## Indev stuff

- **Mean**: Contains couple of functions to allow several "mean value" calculations (arithmetic, geometric, harmonic and general arith-like).
- **Numbers**: Currently contains only [Pochhammer Symbol] and [Binomial Coefficient] for u128-bounded arguments and values. Obviously, I plan on removing mentioned restrictions and implement these objects for any complex arguments in the future. I plan on adding couple more well-known numbers such as [Bernoulli Number] and others I might need while implementing various special functions.
- **Progression**: Has two `struct`s: `ArithIterator` and `GeometricIterator` corresponding to linear and exponential sequence.
- **Integration**: Contains simple euler-integrating function (attempts to half integration step until desired error is achieved). There is also a function for general step-integration, as well as several Runge-Kutta step-integrating functions. These are defined using macro, you can use to define any Runge-Kutta step function you wish.
- **Function Macros** contains procedural macro to generate function definitions via Tailor series. Usage examples can be found in tests as well as **Macro Functions** module.
- **Memoized** Defines function wrappers, that would wrap supplied function into memoized metrics-aware-estimator thing. Meaning, that each computed value would be stored and reused (if asked for same inputs again), and if functions if called for argument close to lots of values computed in past, polynomial approximation would be used to get the result. Obvious use case for that would be functions that take really long time to compute (idk, something like Gamma-function series).
- **Traits** defines traits I use in my code. I'll try to keep this list as small as I can.

## Planned modules

- **Table Generator**: Idea is to tabulate function of interest for visualization in some sort of program. Might actually include that program too.
- **Differential operators**: There are a lot of them, and even more numeric formulae for each of them. Would like to implement at least a couple.
- **Equation Solver**: There are a couple of numeric equation-solving methods for linear and non-linear problems.
- **Diff. Equation Solver**: No experiment is done without a differential equation to solve.

[Pochhammer Symbol]: https://mathworld.wolfram.com/PochhammerSymbol.html
[Binomial Coefficient]: https://mathworld.wolfram.com/BinomialCoefficient.html
[Bernoulli Number]: https://mathworld.wolfram.com/BernoulliNumber.html