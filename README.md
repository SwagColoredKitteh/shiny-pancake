# Shiny Pancake [![Crates.io](https://img.shields.io/crates/v/shiny-pancake.svg)](https://crates.io/crates/shiny-pancake) [![Build Status](https://travis-ci.org/SwagColoredKitteh/shiny-pancake.svg?branch=master)](https://travis-ci.org/SwagColoredKitteh/shiny-pancake)

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
If you wish to use the SDL2 backend, you can use `--no-default-features --features sdl2-backend`.

How do I use it?
----------------

You pipe the commands to the shiny-pancake executable. You can use the --size WIDTHxHEIGHT
and --title TITLE command line parameters to change the window size and title.

What's the protocol like?
-------------------------

It listens for a few commands on its standard input, these commands are:

    #STROKE_COLOR <R> <G> <B> <A> Set the stroke color. Values from 0 to 255.
    #FILL_COLOR <R> <G> <B> <A>   Set the fill color. Values from 0 to 255.
    #NOSTROKE                     Set the stroke color to (0, 0, 0, 0).
    #NOFILL                       Set the fill color to (0, 0, 0, 0).
    #STROKE_WIDTH <W>             Set the stroke width.
    #RECT <X> <Y> <W> <H>         Draw a rectangle at (X, Y) with size (W, H).
    #CIRCLE <X> <Y> <RADIUS>      Draw a circle at (X, Y) with radius RADIUS.
    #ELLIPSE <X> <Y> <W> <H>      Draw an ellipse at (X, Y) with size (W, H).
    #LINE <X1> <Y1> <X2> <Y2>     Draw a line from (X1, Y1) to (X2, Y2).
    #ARROW <X1> <Y1> <X2> <Y2>    Draw an arrow from (X1, Y1) to (X2, Y2).
    #FRAME_START                  Start a new frame.

The commands are all case-insensitive.
Every line that is not prefixed with any of the commands is ignored.

What license is this code under?
--------------------------------

GPLv3.
