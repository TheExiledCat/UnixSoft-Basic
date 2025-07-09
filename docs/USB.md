---
hide:
  - toc
---

# UnixSoft-BASIC (USB) Reference

## General basics

USB is a superset of AppleSoft BASIC. This means that most syntax specifics will be similar or the same to the AppleSoft variant, with small adjustments or additions here and there.

Certain things are the same:

```basic
LET X = 5
```

While others are slightly different:

```basic
LET S$ = "My age is " + 23 //valid string, non string operands get stringified
```

However, most syntax hasnt changed. So when lost, you can also get away with using the original [AppleSoft BASIC](https://www.calormen.com/jsbasic/reference.html) references.

USB Keywords, identifiers and functions are case-insensitive. this means `print` is the same as `PRINT` and `LET X = 5; PRINT x` are valid USB

### Grammar

The core of the USB syntax is split into 2 types of symbols:

- Expressions
- Statements

These expand into the following:

- Expressions
  - arithmic expressions or identifiers (`x + y, 10 - 20 ...`)
  - function calls, constants or variables(`ABS(x), y, 10, "Hello" ...`)
  - conditionals (`x < 10, b == FALSE, ...`)
  - More...
- Statements
  - Assignments (`LET X = 10`)
  - Keywords (`PRINT, FOR, IF ...`)
  - Function calls
  - Definitions (`DEF MyLambda x, y = x \* y)
  - More...

### Differences from AppleSoft-BASIC

USB differs from applesoft basic in some fundamental areas, however, most AppleSoft Syntax is still mostly supported for legacy reasons.
A lot of redundant functionality has been either added on top of. Or an alternative has been added.

#### Changes:

- Optional linenumbers:
  - ```basic
      10 PRINT X // numbered lines
      LET A = 10 // Line inferred by position (11)
      LET B = 20 // Same here (12). In a fully unnumbered file the line numbers start at 1
    ```
    - Note this does come with a drawback for the compilers resolver:
      - ```basic
        10 PRINT X //(10)
        LET A = 10 //(11) Inferred
        11 PRINT Y //(11) Compiler error: duplicate statement
        ```
- Scopes:

  - ```basic
    BEGIN
    LET X = 5
    PRINT X
    FIN

    PRINT X //undefined token error
    ```

  - ```basic
    IF X < 5 BEGIN
    // statements....
    FIN
    ```

- `;` instead of `:` for statement seperating
  - ```basic
    LET X = 10; PRINT X
    ```
  - Note: A single statement cant be spread over multiple lines, Commands are defined as a single line till `\n` or a `;` seperated list of statements

## Statements

1. ### Variable Control

   - #### CLEAR

     - Syntax: `CLEAR`
     - `Clear all global static variables`

   - #### CSCOPE

     - Syntax: `CLEAR`
     - `Same as above, but only clears variables in the current scope`

   - #### DIM

     - Syntax: `DIM <name> (columns [,rows]) [AS <DATATYPE>]`
     - `Define array with specified width and height (or single dimension if left empty)` .
     - Infers datatype when assigned if not annotated.

   - #### DEF

     - Syntax: `DEF <name> [param1, param2 , ...] = _expression`
     - `Define a lambda function. This lambda can only return a single datatype.`

   - #### LET

     - Syntax: `LET {<name> | <name> = <expression> | <name> AS <DATATYPE> | <name> = <expression> AS <DATATYPE>}`
     - `Declare and optionally assign a new variable.`. Infers datatype if not annotated.

   - **NOTE**: LET, DEF and DIM:

     - Assignments using `=`can only be used on variables already created using their respective keyword, you can NOT assign as declaration. E.g. `X = 5`without using `LET X` beforehand. This will throw a undefined variable error on compilation

2. ### Flow

   - #### GOTO

     - Syntax: `GOTO { linenumber | alias linenumber }`
     - Jump to the specified line or line in a file imported using _alias_

   - #### GOSUB

     - Same as `GOTO` but push onto the function stack and jump to sub routine. Expects a `RET` or `END` somewhere in the Sub routine
     - `RET` can not return a value when using `GOSUB`. However global variables can be edited by the subroutine. For proper functions see [`FN`](#fn)

   - #### MENU

     - Syntax: `MENU <expression> {<statement1> [<statement2> ...] | {GOTO | GOSUB } linenumber1 [ , linenumber2, ...}`
     - A modern replacement of the `ON`command from applesoft.
     - When used with an expression, calls one of the statements given where _expression's_ value is the index of the statement to use (1 based)
     - When used with either a `GOTO`or a `GOSUB` statement, instead pass a comma seperated list of linenumbers (can be aliased) to jump to.

   - #### FOR

     - Syntax: `FOR <temp-var> = <expression>, <condition>, <step-action> = <statement> { [statement1 [, statement2, ...] NEXT] | <scope>}`
     - starts a C-style for loop.

     - after the paramaters for the for loop are given can either call any statements until a `NEXT` call or given a Scope which will automatically call `NEXT` at the end.

   - #### FOR

     - Syntax: `<temp-var> = <number-expression> TO <number target> [STEP stepsize] {statement1 [, statement2, ...] NEXT | <scope>}`
     - Original Applesoft for loop. can be used with both floats and integers.

     - Also supports scopes or next like the C style loop

   - #### NEXT

     - Syntax `NEXT [count]`
     - Forces a loop to continue from its start.

     - when used with for loops, _count_ can be used to specify how many times to call the step size or the C style end of loop statement.

     - will error if used outside of a loop context.

   - #### IF

     - Syntax: `IF <expression> {THEN <statement> | GOTO | GOSUB | <scope>}`
     - if expression is true perform the following:

       - THEN: perform a single statement

       - GOTO or GOSUB: jump to a line using the logic in GOTO or GOSUB

       - IF can also be given a scope using `BEGIN`and `FIN` to call when the if statement is true

   - #### END

     - Terminate the program early and cleanly

     - Every script has an invisible `END` at the end of it. Hence this is mostly used for early returns

   - #### STOP
     - Syntax: `STOP <errorcode>`
     - Terminate the program early with the given error code

3. ### Error Handling

   - #### ONERR

     - Syntax: `ONERR <statement>`
     - Sets the current error handler to a given statement. If a built in function throws an error or the `THROW` keyword is used, the error handler is called and the error code is passed into the `ERR` global variable

   - #### THROW
     - Syntax: `THROW <code>`
     - Trigger the `ONERR` handler and store _code_ into `ERR`
