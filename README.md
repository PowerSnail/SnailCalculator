# SnailCalculator
Studying rust

## Calculator Grammar

### Prefix

expression = 

integer = digit { digit }
number = integer
    | integer "." 
    | integer "." digit { digit }
    | "." digit { digit }
digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" .
