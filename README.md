# top down shooter 3D

Prototype rougelike extraction shooter esque game, kind of an upgrade of the prior 2d one. The idea is an extraction shooter but its procedually generated. You get resources and stuff exploring the levels and go to the next and every few levels there will be a bossfight until the end.

## Plans for procedual generation

I want to make a not tilemap based world with corredors and rooms which is mostly linear with a start and end. 

## How it was made

The game is made in godot with the godot-rust library to provide the rust bindings. I plan on using mainly rust with simple things being in gdscript. I decided to use rust because I think the strong typing system is useful for more complicated games and also because rust has more features than gdscript (im sick of haing to make a new file for EVERY class). I think it will also benefit prodecual generation, having access to more libraries and also a faster language than gdscript.
