include!("grammar.rs");
include!("lr.rs");
include!("first.rs");
include!("follow.rs");
include!("action.rs");
include!("goto.rs");
fn get_parser() -> LR1_Parser<TestCase, Token, TranslatorStack> {
    LR1_Parser {
        grammar: __grammar__(),
        LR1_automata: __lr__(),
        follow_set: __follow__(),
        first_set: __first__(),
        conflicts: false,
        goto: __goto__(),
        action: __action__(),
        item_closure_map: IndexMap::new(),
        closure_map: IndexMap::new(),
    }
}
