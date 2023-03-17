fn main() {

    let mut moved = String::from("shake it");

    moved = move_me(moved, 4);

    moved + " baby";

}

fn move_me(the_thing: String, oohs: u8) -> String {
    let result = if oohs > 0 {
        move_me(the_thing, oohs -1) + " ooh"
    } else {
        the_thing
    };
    result
}