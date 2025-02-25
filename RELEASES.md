# Release notes

## August 19, 2022 circom 2.0.7

#### Extensions
- Log operation receives as parameters a list of expressions and string. (Documentation is [here](https://github.com/iden3/circom/blob/master/mkdocs/docs/circom-language/code-quality/debugging-operations.md).
- New heuristics that improves the constraint simplification process is added by default. The old version of the simplification process can be still used using the option "--use_old_simplification_heuristics".
- Initialization of every array position to 0 in both C and WASM witness generator.
- New check of size vector when vector assignment:
           + If a vector is assigned in a smaller vector, then an error is produced. 
           + If a vector is assigned in a larger vector, then a warning is produced. (The remaining positions will have their previous values, or 0     
             otherwise. 
- Improvement of the trace error message.

## July 23, 2022 circom 2.0.6

#### Extensions
- Adding three new prime numbers in circom and a flag to choose which one to use. 
- Adding custom templates to circom. (Documentation is [here](https://github.com/iden3/circom/edit/master/mkdocs/docs/circom-language/custom-templates-snarkjs.md)). 
- Adding published binaries for mac, linux and windows.


## June 23, 2022 circom 2.0.5

#### Extensions
- Removing non-determinism in linear constraints that cannot be removed.
- Making deterministic the code generation. 
- Adding signal one in the wires counting for optimization option '-O0'.


#### Fixed Bugs
- Bug in conditional creation of components inside loops

## April 24, 2022 circom 2.0.4

#### Extensions
-	Improvement of unused signals removal in constraint optimization.
-	macos (x86_64) support for cpp backend added: use template to generate makefile (Not available for M1).
-	wabt dependency swapped to wast dependency instead.
-	Improvement of the known/unknown analysis.
-	Single signal declaration with initialization added for both <== and <--.
-	Input signal size check added in wasm/JS and C++ generated code.
-	Recommendation of using C++ when the memory needed is beyond WebAssembly limit added.
-	Making deterministic the R1CS file: constraints will be always written in the R1CS file in the same order. 

#### Fixed Bugs
-	Bug in C++ error trace management for asserts in functions.
-	Bug in multiple line(s) comments.
-	Bug in unassigned inputs of arrays of size 0.
-	Bug in the use of constant division operation on constraints.
-	Bug in evaluation of expressions involving subcomponents signals.

## Dec 23, 2021 circom 2.0.3

#### Extensions
-	Improvement in the check all array dimensions in its definition have known size.
-	A new verbose flag is added: If –verbose is set the compiler shows log messages during constraint generation.

#### Fixed Bugs
-	Bug in functions that return calls to other functions.

## Dec 10, 2021 circom 2.0.2

#### Extensions
-	A check that all inputs are set is added in the wasm/JS and C++ generated code.
-	Improvement of the “merge_code” implementation in code generators. 

## Nov 9, 2021 circom 2.0.1

#### Extensions
-	Error trace printed in the C++ code when an assert fails.
-	Compiler versions in circom programs handling added (improving the pragma instruction).
-	Arrays of components with uninstantiated positions are handled.
-	Comments in the wat code only generated when –wat flag is used.

#### Fixed bug

-	Bug in the line number generation for wasm error message.
-	Bug: R1CS map is an array of 64bit elements (instead of 32bit).
-	Bug on the initial memory size (too large) defined in the generated wasm code and error message handling in wasm generation added.
-	Bug with use of circuit file names with dots.
