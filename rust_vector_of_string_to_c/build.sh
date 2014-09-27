#!/bin/sh

rustc rust.rs
gcc -std=c99 -Wall -g  main.c -o test -L. -lrust
