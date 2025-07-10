---
hide:
    - toc
---

# USB Guide

This chapter of the documentation shows you common design patterns and how to use the USB language.

## Creating a selection menu

Getting structured user input is one of the most fundamental parts of terminal based applications. The USB language provides a simple way to create a selection menu.

```basic
LET choice = POPTIONS "Schedule a meeting", "Edit a meeting", "Cancel a meeting", "Quit", "Please select an option from the list above"
```

This will prompt the user with the following list:

```console
1. Schedule a meeting
2. Edit a meeting
3. Cancel a meeting
4. Quit
Please select an option from the list above
>
```

this will internally store the user's choice in the `choice` variable as an integer.

If the choice value is not an integer or outside of the range of options, THROW will be called with a `Format` or an `OutOfRange` error.

Then to match the chosen option:

```basic
MENU choice PRINT "You scheduled a new meeting", PRINT "You edited a meeting", PRINT "You cancelled a meeting", END
```

This will perform the corresponding statements.

Output:

```console
1. Schedule a meeting
2. Edit a meeting
3. Cancel a meeting
4. Quit
Please select an option from the list above
> 1

You scheduled a new meeting
```
