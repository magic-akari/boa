use crate::parser::tests::check_script_parser;
use boa_ast::{
    declaration::{LexicalDeclaration, Variable},
    expression::literal::Literal,
    function::{AsyncGenerator, FormalParameterList, FunctionBody},
    statement::Return,
    Declaration, Statement, StatementListItem,
};
use boa_interner::Interner;

///checks async generator expression parsing
#[test]
fn check_async_generator_expr() {
    let interner = &mut Interner::default();
    let add = interner.get_or_intern("add");
    check_script_parser(
        "const add = async function*(){
            return 1;
        };
        ",
        vec![Declaration::Lexical(LexicalDeclaration::Const(
            vec![Variable::from_identifier(
                add.into(),
                Some(
                    AsyncGenerator::new(
                        Some(add.into()),
                        FormalParameterList::default(),
                        FunctionBody::new(
                            vec![StatementListItem::Statement(Statement::Return(
                                Return::new(Some(Literal::from(1).into())),
                            ))]
                            .into(),
                        ),
                        false,
                    )
                    .into(),
                ),
            )]
            .try_into()
            .unwrap(),
        ))
        .into()],
        interner,
    );
}

#[test]
fn check_nested_async_generator_expr() {
    let interner = &mut Interner::default();
    let a = interner.get_or_intern("a");
    let b = interner.get_or_intern("b");
    check_script_parser(
        "const a = async function*() {
            const b = async function*() {
                return 1;
            };
        };
        ",
        vec![Declaration::Lexical(LexicalDeclaration::Const(
            vec![Variable::from_identifier(
                a.into(),
                Some(
                    AsyncGenerator::new(
                        Some(a.into()),
                        FormalParameterList::default(),
                        FunctionBody::new(
                            vec![Declaration::Lexical(LexicalDeclaration::Const(
                                vec![Variable::from_identifier(
                                    b.into(),
                                    Some(
                                        AsyncGenerator::new(
                                            Some(b.into()),
                                            FormalParameterList::default(),
                                            FunctionBody::new(
                                                vec![StatementListItem::Statement(
                                                    Statement::Return(Return::new(Some(
                                                        Literal::from(1).into(),
                                                    ))),
                                                )]
                                                .into(),
                                            ),
                                            false,
                                        )
                                        .into(),
                                    ),
                                )]
                                .try_into()
                                .unwrap(),
                            ))
                            .into()]
                            .into(),
                        ),
                        false,
                    )
                    .into(),
                ),
            )]
            .try_into()
            .unwrap(),
        ))
        .into()],
        interner,
    );
}
