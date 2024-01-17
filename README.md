# Vertigo
Y is this so weird

## Description
This project is a compiler for my esoteric language vertigo. This language is not meant to be usable it's just a fun project to see if I can parse and compile a language like this. What is the language you may be asking? Well let me show you how to write hello world (Currently a work in progress)

```
"p
hr
ei
ln
lt
o;

w 
o
r
l
d
!
" 
;
```

Yes it's that painful.

The program is read top to bottom left to right.
Comments are formatted vertically too, via the semicolon which also ends the statement. Fun :D

As well, anything returned from a column is pushed to the stack just to make things better.
This makes addition somewhat bearable

```
25+;
;;;7
```
