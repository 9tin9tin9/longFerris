const FERRIS_HEAD: &str =     "    x^^^^^^x";
const FERRIS_BODY: &str =     "  <          >";
const FERRIS_BOTTOM: &str = r#"  <          > /
_<`   0  0    >_
\\`-\,== ---'`//
 \   '==      /"#;
const FERRIS_HAND: &str = "{_}";
const FERRIS_HAND_POS : usize = 15;

fn main() {
    let length = std::env::args()
        .nth(1)
        .expect("No length given")
        .parse::<usize>()
        .expect("Not an unsigned integer");

    let mut pic = vec![FERRIS_HEAD.to_string()];
    for _ in 0..length {
        pic.push(FERRIS_BODY.to_string());
    }
    pic.push(FERRIS_BOTTOM.to_string());

    // add hand
    let linenum = pic.len()-2;
    let space = FERRIS_HAND_POS - pic[linenum].len();
    for _ in 0..space {
        pic[linenum].push(' ');
    }
    pic[linenum].push_str(FERRIS_HAND);

    // print
    for line in pic {
        println!("{}", line);
    }
}
