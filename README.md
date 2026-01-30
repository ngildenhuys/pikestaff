# pikestaff

Taking inspiration from the [Pitchfork ](https://joholl.github.io/pitchfork-website/) project for C++,
this repo describes and provides an example of how to structure a system verilog package for design
and verification using an emulation first methodology. In an ideal world all the components that
need to be built and exposed as packages are built and exposed with the same system...This could be
bazel, buck, nix, etc... Doesn't really matter, so for now this repo won't "package" the rtl and dv
code for "release". But I will be using build tools, like cargo, to build the software side, and build
the dv executables and libraries.

## DV Methodology
Each testbench is an executable that can be built and run, taking some input. Testbenches are just
programs that you run in hops of them crashing on finding some bug.

The cornerstones of DV are assertions and checkers (which are just more complex assertions).
These should be written inline with rtl code, the caveat is that all DV modules should contain a feature
flag to be able to enable them when building the rtl. You don't want to tape out DV code.
The caveat to this is you don't want all your engineers to write their own `#ifdefs`, this will take some enforcment.
The core of this is we want to treat ifdefs like rust treats feature flags, that is, code can always be
compiled with the defines enabled, there should be no "else" case for ifdefs, they are only additive.

The main issue if you want assertions to be inside the rtl code, you need to pipe down dv signals,
like the always on dv clock, and enable signals for assertions and checkers. These should be done in
a module DV interface that all submodules that need checking take in. Then the ifdef for actually
instantiating those signals can live and be owned by the DV interface. This could also be a struct
instead of an interface, then the parent module can set each signal as they want. The use of an interface
over a struct would be if you want to be explicit about what dv signals in the "struct" are driven
to the dv code and what is output by the dv code.

So it seems you can't get rid of it all...the best bet is probably just to ifdef the module input
on each rtl module, which is a pain, but means that if we ifdef the struct declaration too, the
compiler should catch if the dv type is used outside its feature context.
