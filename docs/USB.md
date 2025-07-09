[TOC]

# UnixSoft-BASIC (USB) Reference

## General basics

USB is a superset of AppleSoft BASIC. This means that most syntax specifics will be similar or the same to the AppleSoft variant, with small adjustments here and there.



Certain things are the same:

```basic
LET X = 5
```

While others are slightly different:

```basic
LET S$ = "My age is ¨+23 //valid string, non string operands get stringified 
```

However, most syntax hasnt changed. So when lost, you can also get away with using the original AppleSoft BASIC references.



USB Keywords, identifiers and functions are case-insensitive. this means `print` = `PRINT` and `LET X = 5\n PRINT x` are valid USB

## Statements

1. ### Variable Control
   
   * #### CLEAR
     
     * `Clear all global static variables`
   
   * #### CSCOPE
     
     * `Same as above, but only clears variables in the current scope`
   
   * #### DIM _name_ (columns [,rows]) [AS \<DATATYPE\>]
     
     * `Define array with specified width and height (or single dimension if left empty)` . Infers datatype when assigned if not annotated.
   
   * #### DEF _name_  [param1, param2 , ...] = _expression_
     
     * `Define a lambda function. This lambda can only return a single datatype.`
   
   * #### LET {_name_ |  _name_ = _expression_ | _name_ AS <DataType> | _name_ = _expression_ AS <DataType>}
     
     * `Declare and optionally assign a new variable.`. Infers datatype if not annotated. 
   
   * **NOTE**: LET, DEF and DIM:
     
     * Assignments using `=`can only be used on variables already created using their respective keyword, you can NOT assign as declaration. E.g. `X = 5`without using `LET X` beforehand. This will throw a undefined variable error on compilation

2. ## Flow
   
   * #### GOTO { linenumber | alias linenumber }
     
     * Jump to the specified line or line in a file imported using _alias_
   
   * #### GOSUB {linenumber | alias linenumber}
     
     * Same as `GOTO` but push onto the function stack and jump to sub routine. Expects a `RET` or `END` somewhere in the Sub routine 
     
     * `RET` can not return a value when using `GOSUB`. However global variables can be edited by the subroutine. For proper functions see [#FN] 
   
   * #### MENU _expression_ {_statement1_ [_statement2_ ...]__ | {GOTO | GOSUB } linenumber1 [ , linenumber2, ...}
     
     * A modern replacement of the `ON`command from applesoft. 
     
     * When used with an expression, calls one of the statements given where _expression's_  value is the index of the statement to use (1 based)
     
     * When used with either a `GOTO`or a `GOSUB` statement, instead pass a comma seperated list of linenumbers (can be aliased) to jump to.
   
   * #### FOR _temp-var_ =  _expression_,  _condition_, _step-action_ = _statement_ {[statement1 [, statement2, ...]  NEXT] | _scope_}
     
     * starts a C-style for loop.
     
     * after the paramaters for the for loop are given  can either call any statements until a `NEXT` call or given a Scope which will automatically call `NEXT` at the end.
   
   * #### FOR _temp-var_ = _number-expression_ TO _number target_ [STEP stepsize]  {statement1 [, statement2, ...] NEXT | *scope*}
     
     * Original Applesoft for loop. can be used with both floats and integers.
     
     * Also supports scopes or next like the C style loop
   
   * ### NEXT [count]
     
     * Forces a loop to continue from its start.
     
     * when used with for loops, _count_ can be used to specify how many times to call the step size or the C style end of loop statement.
     
     * will error if used outside of a loop context.
   
   * #### IF _expression_ {THEN _statement_ | GOTO | GOSUB | _scope_}
     
     * if expression is true perform the following: 
       
       * THEN: perform a single statement
       
       * GOTO  or GOSUB: jump to a line using the logic in GOTO or GOSUB
       
       * IF can also be given a scope using `BEGIN`and `FIN` to call when the if statement is true
   
   * #### END
     
     * Terminate the program early and cleanly 
     
     * Every script has an invisible `END` at the end of it. Hence this is mostly used for early returns
   
   * #### STOP _errorcode_
     
     * Terminate the program early with the given error code



3. ## Error Handling
   
   * #### ONERR _statement_
     
     * Sets the current error handler to a given statement. If a built in function throws an error or the `THROW` keyword is used, the error handler is called and the error code is passed into the `ERR` global variable
   
   * #### THROW _code_
     
     * Trigger the `ONERR` handler and store _code_ into `ERR` 
