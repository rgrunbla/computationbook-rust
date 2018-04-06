mod cyclic_tag_rule;
mod cyclic_tag_rulebook;
mod cyclic_tag_system;
mod tag_to_cyclic;

use universality_is_everywhere::tag_systems::tag_rule::{TagRule};
use universality_is_everywhere::tag_systems::tag_rulebook::{TagRulebook};
use universality_is_everywhere::tag_systems::tag_system::{TagSystem};

use self::cyclic_tag_rule::{CyclicTagRule};
use self::cyclic_tag_rulebook::{CyclicTagRulebook};
use self::cyclic_tag_system::{CyclicTagSystem};
use self::tag_to_cyclic::{TagToCyclic};

pub fn main() {
    let rulebook = CyclicTagRulebook::new(&[CyclicTagRule::new("1"), CyclicTagRule::new("0010"), CyclicTagRule::new("10")].to_vec());

    let mut system = CyclicTagSystem::new("11", rulebook);

    for _ in 0..16 {
        println!("{}", system.current_string);
        system.step();
    }
    println!("{}", system.current_string);

    for _ in 0..20 {
        println!("{}", system.current_string);
        system.step();
    }
    println!("{}", system.current_string);

    let tagrulebook = TagRulebook::new(2, [TagRule::new('a', "ccdd"), TagRule::new('b', "dd")].to_vec());
    let tagsystem = TagSystem::new("aabbbb", tagrulebook);
    println!("{:?}", tagsystem.alphabet());

    let encoder = tagsystem.encoder();
    println!("{}", encoder.encode_character('c'));
    println!("{}", encoder.encode_string("cab"));

    let rule = tagsystem.rulebook().rules()[0].clone();
    println!("{:?}", rule.to_cyclic(&encoder));

    println!("{:?}", tagsystem.rulebook().cyclic_rules(&encoder));
    println!("{:?}", tagsystem.rulebook().cyclic_padding_rules(&encoder));

    let mut cyclic_system = tagsystem.to_cyclic(&encoder);

    cyclic_system.run()
}