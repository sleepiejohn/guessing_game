extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng; // trait available on context

fn main() {

   let fireworks = r#"
                   *    *
   *         '       *       .  *   '     .           * *
                                                               '
       *                *'          *          *        '
   .           *               |               /
               '.         |    |      '       |   '     *
                 \*        \   \             /
       '          \     '* |    |  *        |*                *  *
            *      `.       \   |     *     /    *      '
  .                  \      |   \          /               *
     *'  *     '      \      \   '.       |
        -._            `                  /         *
  ' '      ``._   *                           '          .      '
   *           *\*          * .   .      *
*  '        *    `-._                       .         _..:='        *
             .  '      *       *    *   .       _.:--'
          *           .     .     *         .-'         *
   .               '             . '   *           *         .
  *       ___.-=--..-._     *                '               '
                                  *       *
                *        _.'  .'       `.        '  *             *
     *              *_.-'   .'            `.               *
                   .'   You Win!            `._             *  '
   '       '                        .       .  `.     .
       .                      *                  `
               *        '             '                          .
     .                          *        .           *  *
             *        .                                    '
"#;

    let secret = rand::thread_rng().gen_range(1, 100);

    loop {
        let mut guess = String::new(); // create a mutable String, sequence of char with dynamic size

        println!("Enter a number >");

        io::stdin() // use std input
        .read_line(&mut guess) // make the access mutable so the value can be written to it
        .expect("Number could not be captured");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("No, no, no, we need a number");
                continue;
            }
        };

        println!("You entered: {}", guess);

        match guess.cmp(&secret) {
            // cmp takes the reference of what to compare
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("{}", fireworks);
                break; // break out of the loop 
            }
        }
    }
}
