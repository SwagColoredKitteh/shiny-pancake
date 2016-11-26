# Shiny Pancake [![Crates.io](https://img.shields.io/crates/v/shiny-pancake.svg)](https://crates.io/crates/shiny-pancake)

What is this?
-------------

It's a small rendering tool that takes commands from stdin.

Show me gifs!
-------------

![Preview](https://swagcoloredkitteh.github.io/shiny-pancake/preview.gif)

![Preview](https://swagcoloredkitteh.github.io/shiny-pancake/preview2.gif)

How do I install it?
--------------------

You can use `cargo install shiny-pancake`.

How do I use it?
----------------

You pipe the commands to the shiny-pancake executable. You can use the --size WIDTHxHEIGHT
and --title TITLE command line parameters to change the window size and title.

What's the protocol like?
-------------------------

It listens for a few commands on its standard input, these commands are:

    #RESET                     Reset the draw state.
    #COLOR <R> <G> <B> <A>     Set the current drawing color to (R, G, B) and the alpha to A.
                               These are all unsigned integers with values from 0 to 255.
    #RECT <X> <Y> <W> <H>      Draw a rectangle at (X, Y) with size (W, H).
    #CIRCLE <X> <Y> <RADIUS>   Draw a circle at (X, Y) with radius RADIUS.
    #ELLIPSE <X> <Y> <W> <H>   Draw an ellipse at (X, Y) with size (W, H).
    #DELAY <MS>                Pause for MS milliseconds, this is useful for recording replays.

The commands are all case-insensitive.
Every line that is not prefixed with any of the commands is ignored.

What license is this code under?
--------------------------------

GPLv3.
