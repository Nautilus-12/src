use logos::Logos;

#[allow(non_camel_case_types)]
#[allow(unused_imports)]

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // TODO: Define variants and their token/regex
    //, logos::skip -> um sie zu skippen

    // TODO: alle nicht in C1 erlaubten Zeichen (etwa "&") sollen zur Rückgabe der Error Variante führen
    //       CONST_FLOAT	{FLOAT} ( [eE] ([-+])? {INTEGER} )? | {INTEGER} [eE] ([-+])? {INTEGER}
    //       CONST_STRING	"\"" [^\n\"]* "\""
    //       ID	({LETTER})+ ({DIGIT} | {LETTER})*
    //       Pseudotoken zur Konstruktion von den obrigen Token:
    //       DIGIT	[0-9] - DONE
    //       INTEGER	{DIGIT}+ - DONE
    //       FLOAT	{INTEGER} "." {INTEGER} | "." {INTEGER}
    //       LETTER	[a-zA-Z] - DONE
    //       Kommentare skippen - DONE
    //       Whitespaces skippen - DONE

    #[regex("//.*\n", logos::skip)]
    CPlusPlusKommentar,

    #[regex(r"/\*[^*]*\*/", logos::skip)]
    CKommentar,

    #[regex("\"[^\n\"]* \"")]
    CONST_STRING,

    #[regex("(LETTER)+[(DIGIT)(LETTER)]*")]
    ID,

    #[regex("INTEGER"."INTEGER | "." INTEGER")]
    FLOAT,

    #[regex("(FLOAT)([eE][+-]?(Integer))? | (Integer)[eE][-+]?(Integer)")]
    CONST_FLOAT,


    #[regex("[a-zA-z]")]
    LETTER,

    //compiled aber funktioniert alles nicht
    #[regex("[0-9]", priority=2)]
    DIGIT,

    #[regex("DIGIT+", priority=1)]
    INTEGER,

    //gab nen Fehler aus weil INTEGER UND CONST_INT gleiche Definition, 
    //müsst ihr die priority vllt nochmal anpassen
    #[regex("[1-9](INTEGER)")]
    CONST_INT,

    #[regex("true | false")]
    CONST_BOOLEAN,

    #[token("bool")]
    KW_BOOLEAN,

    #[token("do")]
    KW_DO,

    #[token("else")]
    KW_ELSE,

    #[token("float")]
    KW_FLOAT,

    #[token("for")]
    KW_FOR,

    #[token("if")]
    KW_IF,

    #[token("int")]
    KW_INT,

    #[token("printf")]
    KW_PRINTF,

    #[token("return")]
    KW_RETURN,

    #[token("void")]
    KW_VOID,

    #[token("while")]
    KW_WHILE,

    #[token("+")]
    PLUS,

    #[token("-")]
    MINUS,

    #[token("*")]
    ASTERISK,

    #[token("/")]
    SLASH,

    #[token("=")]
    ASSIGN,

    #[token("==")]
    EQ,

    #[token("!=")]
    NEQ,

    #[token("<")]
    LSS,

    #[token(">")]
    GRT,

    #[token(">=")]
    LEQ,

    #[token("<=")]
    GEQ,

    #[token("&&")]
    AND,

    #[token("||")]
    OR,

    #[token(",")]
    COMMA,

    #[token(";")]
    SEMICOLON,

    #[token("(")]
    LPAREN,

    #[token(")")]
    RPAREN,

    #[token("{")]
    LBRACE,

    #[token("}")]
    RBRACE,

    









    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]

    //Skips Whitespaces
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

fn main(){

}

