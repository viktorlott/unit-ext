use std::ops::Add;
use unit_ext::*;

fn main() {
    Some(10)
        .filter(|n| n > &5)
        .map(|n| n * 2)
        .or_else(|| println!("Value too small").ret_none());

    Some(10)
        .map(|v| println!("Some({v})").ret(v + 10))
        .or_else(|| println!("Default value").ret_default());

    Some([1, 2, 3, 4])
        .map(|mut arr| arr.clone_from_slice(&[4, 3, 2, 1]).ret(arr));

    "10".parse::<u8>().map_or_else(
        |e| eprintln!("{e}").ret_none(),
        |v| println!("Got here").ret_some(v.add(10)),
    );

    match "15".parse::<u8>() {
        Ok(v) => v.add(15).into(),
        Err(e) => eprintln!("{e}").ret_none(),
    };

    // One can also use do something like this:
    "15".parse::<u8>()
        .inspect_err(|e| eprintln!("{e}"))
        .ok()
        .map(|v| v + 15);
}
