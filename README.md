# YARGL
Yet Another Rust GUI Library

NOTE: This library is still Work-In-Progress and probably not yet very useful for most use cases.

YARGL is a library for building native Rust GUI applications on the basis of HTML and CSS.
Internally it uses sdl

Usage: See [tests/staticwindow.rs](tests/staticwindow.rs)

Note: Using the library requires cmake to be installed on the system

Currently suppports:

- [x] Creation of fixed-size windows
- [x] Parsing static html documents
- [x] Parsing css rules from html documents
- [x] Rendering static html documents onto a window*
- [x] Receiving input events and the html element that they apply to
- [x] TrueType and OpenType fonts

Not supported:

- [ ] Everything else

<p>
* Only the following css properties are supported:
</p>

- width, height, font-size: single numeric value with unit (px,pt,em,%) (fit-content if not set)
- display: "inline", "inline-block", "block"
- color, background-color: "#" + rgb value in hex format (i.e. #ff0000 for red)
- font-family: comma-separated string

Properties are not inherited from parent elements, therefore it is recommended to use 
a reset stylesheet (like [css/reset.css](css/reset.css)).

Working on:
- Documents cant be mutated
- Text is drawn with outlines, not filled
- Performance
