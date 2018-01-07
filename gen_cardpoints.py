
# XXX: really I should be using a Rust macro for this rather than Python
# codegen

# because macro pratice is useful, whereas this abomination is not

def gen_cardpoints():
    return {
        (suit, value): (
            eval(("\"\\N{{PLAYING CARD {} OF {}}}\""
                  .format(value.upper(), suit.upper()+'S'))))
        for suit in ["heart", "diamond", "spade", "club"]
        for value in ["ace", "two", "three", "four", "five", "six", "seven",
                      "eight", "nine", "ten", "jack", "queen", "king"]
    }


def gen_source():
    template = """use std::fmt;

use card::{{Card, Suit, Value}};

impl fmt::Display for Card {{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
        match *self {{
{}
        }}
    }}
}}
"""
    arms = [("            Card {{ suit: Suit::{}, value: Value::{} }} => "
             "write!(f, \"{{}}\", '{}'),"
             .format(suit.title(), value.title(), glyph))
            for (suit, value), glyph in gen_cardpoints().items()]

    source = template.format('\n'.join(arms))
    with open("src/cardpoints.rs", 'w') as f:
        f.write(source)

if __name__ == "__main__":
    gen_source()
