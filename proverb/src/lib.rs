/*
["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"]`
For want of a nail the shoe was lost.
For want of a shoe the horse was lost.
For want of a horse the rider was lost.
For want of a rider the message was lost.
For want of a message the battle was lost.
For want of a battle the kingdom was lost.
And all for the want of a nail.
*/

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut proverb = String::new();

    for i in 1..list.len() {
        proverb.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            list[i - 1],
            list[i]
        ));
    }

    proverb.push_str(&format!("And all for the want of a {}.", list[0]));

    proverb
}
