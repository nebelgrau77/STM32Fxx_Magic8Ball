# Magic 8-ball 

## (Work in Progress)

A magic 8-ball toy, inspired by one of my favorite movies, "Interstate 60" :) 

Currently for the STM32F030 board, should be easily portable to other STM32Fxx MCUs.

At the moment it simply displays a randomly chosen answer out of 20 possible answers every second.

TO DO:

* have the generation of a new answer activated by the user: button or tilt switch (ideally an accelerometer)
* when the generation is activated, make the answer appear, then disapperar after some time (default state: prompt screen)

# PROBLEM: cannot get a globally accessible RNG

https://gist.github.com/nebelgrau77/33f080d7302b8ff0cef4db21468487d3