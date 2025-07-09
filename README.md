# UnixSoft BASIC (USB)

UnixSoft basic is a modern rendition of the AppleSoft BASIC language, written and compiled for linux x64.

## Features

- Scoped blocks:

  - ```basic
    10 BEGIN //Start scope
    20 LET X = 5
    30 PRINT X
    40 FIN // END scope
    50 PRINT X //Out of scope variable error
    ```

- Optional Line numbers:

  - ```basic
    10 LET X = 5
    PRINT X // linenumber inferred to be 11
    PRINT X * 2 // linenumber inferred to be 12
    11 PRINT X // duplicate line number error
    ```

- Static Typing (for compilation purposes):

  - ```basic
    LET X AS INTEGER
    X = 5
    X = "Hello World" // Type mismatch

    LET Y = 10 //Type inferred
    LET Z = 10 AS FLOAT // Type annotated with value
    ```

- Comments (REM also still works):

  - ```basic
    // This line is skipped
    LET X = 5 // This left part is performed but this comment is skipped.
    ```

- Easy FILE IO (cursor tracking text io using file descriptors):

  - ```basic
    LET F = OPEN "./file.txt", "rw" // get file descriptor
    LET C = READ F 1 // Read single 1 char from cursor onward
    LET L = READLINE F // Get next line on cursor
    LET A = READALL F // GET entire rest of file from cursor


    WRITE F "Hello" // Overwrite the file with Hello
    APPEND F "Hello" // Append to file
    //ETC...
    ```

- Easy terminal graphics:

  - ```basic
    FCOLOR RED // set foreground color to red
    BCOLOR #FFF // set background color to white, full rgb supported with compiler flag --color=24-bit
    ICOLOR //invert background and foreground color
    RCOLOR //reset color to default
    ```

- and more :)

## Documentation and guides

For the full spec see [USB](https://theexiledcat.github.io/UnixSoft-Basic/USB.html)
