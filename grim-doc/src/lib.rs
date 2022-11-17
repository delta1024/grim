/*! Grim - A toy programing language.

   Grim is intended to be a toy programing language written mainly for practice.

   A prototype of the general syntax can be seen below.
   ```skip
   typedef enum State {
       ONE=13,
       TWO,
       THREE,
       FOUR,
   };

   typedef struct Point {
       x -: int,
       y -: int,
   };

   def add-one(val -: int) int {
       val + 1
   }

   def main() {
       bind first  -: int        = 18;
       bind second -: int        = 13;
       bind third  -: String     = "hello";
       bind array  -: [char; 15] = ['a'..'o'];

       // prints 14.
       print(add-one(second));
   }
   ```


# Backus-Naur form
```skip
NUMBER     -> ( 0..9 )* ;
CHAR       -> "'" * "'" ;
STRING     -> "\"" ( CHAR )* "\"" ;
IDENTIFIER -> ( "a".."z" | "A".."Z" | NUMBER | "_" )* ;

program    -> ( ( variable | typedef ) ";" )?* "main () {" ( expression ";" )* "}" ;

function   -> "def" IDENTIFIER "(" ( IDETIFIER typeDef )?* ")" type "{" ( expression ";" )* "}" ;

expression -> literal
            | unary
            | binary
            | grouping
            | array
            | struct
            | enum
            | typedef
            | variable
            | call ;

call       -> IDENTIFIER "(" ( IDENTIFIER | expression ) "," )?* ")" ;

variable   -> "bind" IDENTIFIER typeDef "=" expression ";" ;

typedef    -> "typedef" ( "struct" | "enum" ) IDENTIFIER "{" ( IDENTIFIER typeId ( "," )? )* "};" ;

typeId     -> "-:" ( "int" | "char" | "String" |  "nil" | type
                   | "[" typeId ";" NUMBER "]"
                   | ( "struct" | "enum" ) IDINTIFIER ) ;

enum       -> "enum" IDENTIFIER "{" ( IDENTIFIER ("=" NUMBER)? ",")* "}" ;

struct     -> "struct" IDINTIFIER "{" ( IDENTIFIER typeId )* "}" ;

array      -> "[" ( CHAR | NUMBER ) ( ".." ( CHAR | NUMBER ) )? "]" ;

grouping   -> "(" expression ")" ;

binary     -> expression operator expression ;

operator   -> "==" | "!=" | "<" | "<=" | ">" | ">="
            | "+"  | "-"  | "*" | "/" ;

unary      -> ( "-" | "!" ) expression ;

literal    -> NUMBER | STRING | "true" | "false" |"nil" ;
```
*/
