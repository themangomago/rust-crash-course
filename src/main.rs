mod arrays;
mod conditionals;
mod enums;
mod functions;
mod generics;
mod loops;
mod pointer;
mod print;
mod strings;
mod structs;
mod tupels;
mod types;
mod vargs;
mod vars;
mod vectors;

fn main() {
    print::run();
    vars::run();
    types::run();
    strings::run();
    tupels::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer::run();
    structs::run();
    enums::run();
    vargs::run();
    generics::run();
}
