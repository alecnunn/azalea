pub mod arguments;
pub mod builder;
pub mod command_dispatcher;
pub mod context;
pub mod exceptions;
pub mod message;
pub mod modifier;
pub mod parse_results;
pub mod string_reader;
pub mod tree;

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use crate::{
        arguments::integer_argument_type::{get_integer, integer},
        builder::{literal_argument_builder::literal, required_argument_builder::argument},
        command_dispatcher::CommandDispatcher,
        context::CommandContext,
    };

    struct CommandSourceStack {
        player: String,
    }

    #[test]
    fn it_works() {
        let mut dispatcher = CommandDispatcher::new();

        let source = Rc::new(CommandSourceStack {
            player: "player".to_string(),
        });

        dispatcher.register(
            literal("foo")
                .then(argument("bar", integer()).executes(
                    |c: &CommandContext<CommandSourceStack>| {
                        println!(
                            "Bar is {:?} and player is {}",
                            get_integer(c, "bar"),
                            c.source.player
                        );
                        2
                    },
                ))
                .executes(|_| {
                    println!("Called foo with no arguments");
                    1
                }),
        );

        let parse = dispatcher.parse("foo 123".into(), source.clone());
        assert_eq!(CommandDispatcher::<_>::execute_parsed(parse).unwrap(), 2);
        assert_eq!(dispatcher.execute("foo".into(), source).unwrap(), 1);
    }
}
