// Module 6:
// proverb expander
//
// expand_proverb
// Given a list of inputs, generate the relevant proverb.
// INPUT  || nouns : []
//
// For want of a horseshoe nail, a kingdom was lost, or so the saying goes.
// Given a list of inputs, generate the relevant proverb. For example, given the list
// ["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"], you will output the full text
// of this proverbial rhyme:
//
// For want of a nail the shoe was lost.
// For want of a shoe the horse was lost.
// For want of a horse the rider was lost.
// For want of a rider the message was lost.
// For want of a message the battle was lost.
// For want of a battle the kingdom was lost.
// And all for the want of a nail.
//
// Note that the list of inputs may vary; your solution should be able to handle lists of arbitrary
// length and content. No line of the output text should be a static, unchanging string; all should
// vary according to the input given.

pub fn expand_proverb<'a>(inp: &[&'a str]) {
    for n in 0..(inp.len() - 1) {
        println!("For want of a {} the {} was lost.", inp[n],inp[n+1] );
    }

    println!("And all for the want of a {}.", inp[0]);

}
