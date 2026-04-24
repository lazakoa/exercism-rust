/*
 Ten green bottles hanging on the wall,
Ten green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be nine green bottles hanging on the wall.

Nine green bottles hanging on the wall,
Nine green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be eight green bottles hanging on the wall.

Eight green bottles hanging on the wall,
Eight green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be seven green bottles hanging on the wall.

Seven green bottles hanging on the wall,
Seven green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be six green bottles hanging on the wall.

Six green bottles hanging on the wall,
Six green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be five green bottles hanging on the wall.

Five green bottles hanging on the wall,
Five green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be four green bottles hanging on the wall.

Four green bottles hanging on the wall,
Four green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be three green bottles hanging on the wall.

Three green bottles hanging on the wall,
Three green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be two green bottles hanging on the wall.

Two green bottles hanging on the wall,
Two green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be one green bottle hanging on the wall.

One green bottle hanging on the wall,
One green bottle hanging on the wall,
And if one green bottle should accidentally fall,
There'll be no green bottles hanging on the wall.
*/

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();
    let mut index = start_bottles;
    let end = start_bottles - take_down;

    loop {
        if index == end {
            break;
        }
        for _ in 0..2 {
            if index == 1 {
                song.push_str(&format!(
                    "{} green bottle hanging on the wall,\n",
                    convert_bottles(index)
                ));
            } else {
                song.push_str(&format!(
                    "{} green bottles hanging on the wall,\n",
                    convert_bottles(index)
                ));
            }
        }

        song.push_str("And if one green bottle should accidentally fall,\n");

        if index - 1 == 1 {
            song.push_str(&format!(
                "There'll be {} green bottle hanging on the wall.\n",
                convert_bottles(1).to_lowercase()
            ));
        } else {
            song.push_str(&format!(
                "There'll be {} green bottles hanging on the wall.\n",
                convert_bottles(index - 1).to_lowercase()
            ));
        }
        if index != end {
            song.push('\n');
        }
        index -= 1;
    }
    song
}

fn convert_bottles(bottles: u32) -> String {
    match bottles {
        10 => String::from("Ten"),
        9 => String::from("Nine"),
        8 => String::from("Eight"),
        7 => String::from("Seven"),
        6 => String::from("Six"),
        5 => String::from("Five"),
        4 => String::from("Four"),
        3 => String::from("Three"),
        2 => String::from("Two"),
        1 => String::from("One"),
        0 => String::from("No"),
        _ => String::from(""),
    }
}
