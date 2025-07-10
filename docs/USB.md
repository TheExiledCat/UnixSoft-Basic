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

-   Expressions
-   Statements

These expand into the following:

-   Expressions
    -   arithmic expressions or identifiers (`x + y, 10 - 20 ...`)
    -   function calls, constants or variables(`ABS(x), y, 10, "Hello" ...`)
    -   conditionals (`x < 10, b == FALSE, ...`)
    -   More...
-   Statements
    -   Assignments (`LET X = 10`)
    -   Keywords (`PRINT, FOR, IF ...`)
    -   Function calls
    -   Definitions (`DEF MyLambda x, y = x \* y)
    -   More...

### Differences from AppleSoft-BASIC

USB differs from applesoft basic in some fundamental areas, however, most AppleSoft Syntax is still mostly supported for legacy reasons.
A lot of redundant functionality has been either added on top of. Or an alternative has been added.

#### Changes:

-   Optional linenumbers

    ```basic
        10 PRINT X // numbered lines
        LET A = 10 // Line inferred by position (11)
        LET B = 20 // Same here (12). In a fully unnumbered file the line numbers start at 1
    ```

-   Note this does come with a drawback for the compilers resolver:

    ```basic
        10 PRINT X //(10)
        LET A = 10 //(11) Inferred
        11 PRINT Y //(11) Compiler error: duplicate statement
    ```

-   Scopes:

    ```basic
    BEGIN
    LET X = 5
    PRINT X
    FIN

    PRINT X //undefined token error
    ```

    ```basic
    IF X < 5 BEGIN
    // statements....
    FIN
    ```

-   `;` instead of `:` for statement seperating

    ```basic
    LET X = 10; PRINT X
    ```

-   Note: A single statement cant be spread over multiple lines, Commands are defined as a single line till `\n` or a `;` seperated list of statements

## Statements

1.  ### Variable Control

    -   #### CLEAR

        -   Syntax: `CLEAR`
        -   Clear all global static variables
        -   Note that 'Clearing' in USB means resetting the variables to their default values (e.g. 0 for numbers, "" for strings)
        -   Example:

            ```basic
            LET X = 10
            LET Y = TRUE

            CLEAR

            PRINT X
            PRINT Y
            ```

            Output

            ```console
            0
            FALSE
            ```

    -   #### CSCOPE

        -   Syntax: `CLEAR`
        -   `Same as above, but only clears variables in the current scope`
        -   Example:

            ```basic
            LET X = 10

            BEGIN
            LET Y = 20
            CSCOPE
            PRINT X
            PRINT Y
            ```

            Output:

            ```console
            10
            0
            ```

    -   #### DIM

        -   Syntax: `DIM <name> (columns [,rows]) [AS <DATATYPE>]`
        -   Define array with specified width and height (or single dimension if left empty) .
        -   Infers datatype when assigned if not annotated.
        -   Example:

            ```basic
            DIM arr(10) AS INTEGER // define an  integer array, containing only 10 0's
            arr[0] = 100
            PRINT arr[0]

            //Inferred type
            DIM arr1(10)
            arr1[0] = 100
            PRINT TYPE (arr1[0]) // prints INTEGER

            //Indexing multiple dimensions:
            DIM arr2(10, 10) AS INTEGER
            arr2[0][0] = 100 //First array stores the rows, second indexer selects a column from that row
            ```

    -   #### DEF

        -   Syntax: `DEF <name> [param1, param2 , ...] = <expression>`
        -   Define a lambda function. This lambda can only return a single datatype.
        -   Example:
            ```basic
            DEF square x = x*x
            PRINT square(5) // prints 25
            ```

    -   #### LET

        -   Syntax: `LET {<name> | <name> = <expression> | <name> AS <DATATYPE> | <name> = <expression> AS <DATATYPE>}`
        -   Declare and optionally assign a new variable.
            Infers datatype if not annotated.
        -   Example:

            ```basic
            LET X = 10 // Declare and assign, Infers datatype

            LET Y AS FLOAT = 10 // Declare and assign with datatype
            //or
            LET Y = 10 AS FLOAT // Declare and assign with datatype

            LET Z AS FLOAT //Default value of datatype is assigned, e.g. 0.0
            ```

    -   **NOTE**: LET, DEF and DIM:

        -   Assignments using `=`can only be used on variables already created using their respective keyword, you can NOT assign as declaration. E.g. `X = 5`without using `LET X` beforehand. This will throw a undefined variable error on compilation

    -   #### TYPE
        -   Syntax: `TYPE <expression>`
        -   Returns the type of the expression as a string, mostly used for debugging purposes
        -   This is a compile time operation as it only works on expressions with a known return type at compile time. It just stores the type annotation as a constant string.
        -   Example
            ```basic
            LET X = 5
            LET Y - TYPE (X) // Gets replaced with INTEGER at compile time
            PRINT Y //INTEGER
            ```
    -   #### ENUM
        -   Syntax: `ENUM <name> = <key1 [value1]> [, ...]`
        -   Define a C style enum where every key is mapped to a constant integer value
        -   The values for each key can be inferred (starting at 0) but must be annotated if the order is not sequential
        -   Example:
            ```basic
            ENUM direction = UP, DOWN, LEFT, RIGHT // 0, 1, 2, 3
            ENUM job = FASTFOOD 2, TEACHER, DOCTOR // 2, 3, 4
            ENUM class = ROGUE 2, WARRIOR 5 , MAGE // 2, 5, 6
            ENUM color = CYAN 2, GREEN 0, BLUE // Error, value must be given if not sequential
            ```
        -   Note: Enums in USB are nothing but named integers. that means that you can use them in any place where an integer is expected. Enums are not a type, just syntactic sugar. whenever a value is annotated with `AS <MYENUM>` it will just become an integer with the value of the enum. Default values of enum typed variables are the first keys value in the enum. Casting between integers and enum types is implicit

2.  ### Flow

    -   #### GOTO

        -   Syntax: `GOTO { linenumber | alias, linenumber }`
        -   Jump to the specified line or line in a file imported using _alias_
        -   Example (Prints the current date and time to the console permanently):

            ```basic
            10 PRINT DATE
            GOTO 10

            // used with an alias
            IMPORT "my_lib.usb" AS MYLIB
            GOTO MYLIB, 10

            //my_lib.usb
            10 PRINT "Hello from my_lib.usb"
            ```

    -   #### GOSUB

        -   Same as `GOTO` but push onto the function stack and jump to sub routine. Expects a `RET` or `END` somewhere in the Sub routine
        -   `RET` can not return a value when using `GOSUB`. However global variables can be edited by the subroutine. For proper functions see [`FN`](#fn)
        -   Example:

            ```basic
            10 GOSUB 30
            20 PRINT "BYE"

            30 LET X$ = INPUT "Fill in your name\n> "
            40 PRINT "Hello " + X$
            50 RET
            ```

            Output:

            ```console
            Fill in your name
            > John
            Hello John
            BYE
            ```

    -   #### RET
        -   Syntax: `RET [expression]`
        -   Return from a subroutine or function
        -   Pops from the call stack and returns to the line Popped
        -   When used with functions can optionally return a value
    -   #### MENU

        -   Syntax: `MENU <expression> {<statement1> [,<statement2> ,...] | {GOTO | GOSUB } linenumber1 [ , linenumber2, ...}`
        -   A modern replacement of the `ON`command from applesoft.
        -   When used with an expression, calls one of the statements given where _expression's_ value is the index of the statement to use (1 based)
        -   When used with either a `GOTO`or a `GOSUB` statement, instead pass a comma seperated list of linenumbers (can be aliased) to jump to.
        -   Example:
            ```basic
            LET X = INT(INPUT "Fill in a number from 1 - 3\n> ")
            MENU X PRINT "Hello", GOTO 10, GOSUB 20
            or
            MENU X GOTO 10, 20, 30
            ```

    -   #### FOR

        -   Syntax: `FOR <temp-var> = <expression>, <condition>, <step-action> = <statement> { [statement1 [, statement2, ...] NEXT] | <scope>}`
        -   starts a C-style for loop.

        -   after the paramaters for the for loop are given can either call any statements until a `NEXT` call or given a Scope which will automatically call `NEXT` at the end.
        -   Example:
            ```basic
            FOR I = 0, i < 10, I += 1
            PRINT I
            NEXT
            ```

    -   #### FOR

        -   Syntax: `<temp-var> = <number-expression> TO <number target> [STEP stepsize] {statement1 [, statement2, ...] NEXT | <scope>}`
        -   Original Applesoft for loop. can be used with both floats and integers.

        -   Also supports scopes or next like the C style loop
        -   Example:

            ```basic
            FOR I = 0 TO 10 STEP 1
            PRINT I
            NEXT

            ```

    -   #### NEXT

        -   Syntax `NEXT [count]`
        -   Forces a loop to continue from its start with its step incremented _count_ times. If no count is given, it will increment by 1 or the loops STEP value.

        -   when used with for loops, _count_ can be used to specify how many times to call the step size or the C style end of loop statement. If a negative value is given a runtime error will occur

        -   will error if used outside of a loop context.
        -   Example:

            ```basic
            FOR I = 0 TO 10 STEP 1
            PRINT I
            IF I > 5 THEN NEXT 3 // increment I by 3 * STEP
            NEXT
            ```

    -   #### IF

        -   Syntax: `IF <expression> {THEN <statement> | GOTO | GOSUB | <scope>}`
        -   if expression is true perform the following:

            -   THEN: perform a single statement

            -   GOTO or GOSUB: jump to a line using the logic in GOTO or GOSUB

            -   IF can also be given a scope using `BEGIN`and `FIN` to call when the if statement is true
            -   Example:
                ```basic
                LET X = 10
                IF X >= 5 THEN PRINT X
                IF X >= 5 GOTO 100
                IF X >= 5 GOSUB 100
                IF X >= 5 BEGIN
                PRINT X
                PRINT X * 2
                FIN
                ```

    -   #### END

        -   Terminate the program early and cleanly with a 0 statuscode

        -   Every script has an invisible `END` at the end of it. Hence this is mostly used for early returns
        -   Example:
            ```basic
            LET X = 10
            IF X >= 5 THEN END
            PRINT X
            ```

    -   #### STOP
        -   Syntax: `STOP <errorcode>`
        -   Terminate the program early with the given error code
        -   Example:
            ```basic
            LET X = INT("10q") // format error, ERR gets value mapped to that
            IF ERR > 0 STOP ERR // Terminate program with error code
            PRINT X
            ```

3.  ### Error Handling

    Error handling in USB is a bit different from other programming languages. It uses a special variable called `ERR` to store the error code . The `ERR` variable can be used to check if an error occurred and what the error code is. The `ERR` variable is automatically set to the error code when an error occurs.

    Instead of a recursive backtracking approach to errors where nested scopes are unwound, errors in USB are global and linear. This means that handling errors in USB is a bit simpler and limited than other languages.

    Heres how to write error handlers and create your own errors:

    -   #### ONERR

        -   Syntax: `ONERR <statement>`
        -   Sets the current error handler to a given statement. If a built in function throws an error or the `THROW` keyword is used, the error handler is called and the error code is passed into the `ERR` global variable
        -   Example (Retry mechanism for input):

            ```basic
            5 ONERR PRINT ERR, GOTO 10
            10 LET X = INT(INPUT "Fill in a number\n> ") // if format error, ERR gets value mapped to that
            20 PRINT X
            ```

        -   Note: ONERR is nothing but a label for a statement, whenever THROW is used, GOSUB ONERR is called. If ONERR is a single statement the RET at the end is inferred, otherwise a RET is needed.

    -   #### THROW
        -   Syntax: `THROW <code>`
        -   Trigger the `ONERR` handler and store _code_ into `ERR`
        -   _code_ must be a positive integer greater than 0
        -   Example:
            ```basic
            ONERR PRINT "Something went wrong. Errorcode: {}", ERR
            LET X = 5
            IF X < 10 THEN THROW 100
            // after ONERR is called, program continues here
            ```

4.  ### User I/O

    -   #### PRINT

        -   Syntax: `PRINT { <format> [, <expression> ...] | <expression> }`
        -   Prints the given expression(s) to the console. If a format is given, it is used to
            format the output, replacing any `{}`s with the expression(s) given from left to right.
        -   Example:

            ```basic
            LET Name$ = "Kevin"
            PRINT "Hello {}!", Name$
            ```

            Output:

            ```console
            Hello Kevin!
            ```

    -   #### PENUM

        -   Syntax: `PENUM <ENUM>, <expression>`
        -   Prints the given expression as if it were part of an enumeration.
        -   Will throw an error if used on a non number value. floats are rounded down before checked.
        -   Example:
            ```basic
            ENUM direction = NORTH, EAST, SOUTH, WEST
            LET X = NORTH
            PRINT X // 0
            PENUM direction, X // NORTH
            ```
        -   Note: `PENUM` generates an array of strings at compile time storing every stringified version of the enum value. This array is then used at runtime to print the correct string. If you used no PENUM calls in your code or on a specific enum, this array is not generated for that code or enum. You can use this fact to reduce binary size

    -   #### INPUT

        -   Syntax: `INPUT [prompt]`
        -   Reads a line from `STDIN`
        -   Can be given an optional prompt to show the user
        -   Hangs till input is given
        -   Used as a function, returns the line read from `STDIN`
        -   Example:

            ```basic
            LET X$ = INPUT "Enter a number"
            PRINT X$
            ```

            Output:

            ```console
            Enter a number
            > 5
            5
            ```

    -   #### GET

        -   Syntax: `GET [prompt]`
        -   Reads a single key from `STDIN`
        -   Hangs till input is given
        -   Optionally prints the prompt first
        -   Returns the pressed key as a single character string the moment it is pressed.
        -   Example:
            ```basic
            LET X$ = GET "Press any key to continue (q to quit)"
            IF X$ == "q" THEN END
            // ... other code
            ```

    -   #### HOME

        -   Syntax: `HOME`
        -   Clears the screen

    -   #### HTAB and VTAB

        -   Syntax: `HTAB <expression>` or `VTAB <expression>`
        -   Sets the horizontal or vertical position of the cursor on the `STDOUT` buffer.
        -   top left is (0,0)
        -   Values greater than the screen size are clamped to the size of the screen
        -   To see the size of the screen buffer, see `SCREEN` command

    -   #### FLASH
        -   Syntax: `FLASH <expression>`
        -   Sets the cursor flash to the boolean evaluation of the expression

5.  ### GRAPHICS
    -
