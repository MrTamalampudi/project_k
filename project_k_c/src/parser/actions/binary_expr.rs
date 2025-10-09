use manodae::error::ParseError;

<<<<<<< HEAD
use crate::parser::errors::MISMATCHED_TYPES;
use crate::parser::errorss::ActionError;
use crate::parser::translator_stack::TLVec;
=======
use crate::location::SpanTrait;
use crate::parser::errors::MISMATCHED_TYPES;
use crate::parser::errorss::ActionError;
>>>>>>> origin/master
use crate::{
    ast::{
        expression::{BinOpKind, ExpKind, Expr},
        testcase::TestCase,
    },
    class::BinaryExpressionAction,
<<<<<<< HEAD
    parser::translator_stack::TranslatorStack,
    token::Token,
};

pub struct BinaryExpression;

impl BinaryExpression {
    fn common(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
        op: BinOpKind,
    ) {
        let token = _token_stack.pop().unwrap(); //pop 'op' token

        let expr2 = match _tl_stack.pop_expr() {
            Ok(expr) => expr,
            Err((error, span)) => {
                _errors.push_error(&token, &span, error);
                return;
            }
        };

        let expr1 = match _tl_stack.pop_expr() {
            Ok(expr) => expr,
            Err((error, span)) => {
                _errors.push_error(&token, &span, error);
                return;
            }
        };

        let span = expr1.span.to(&expr2.span);

        //typechecking
        if expr1.primitive != expr2.primitive {
            _errors.push_error(&token, &span, MISMATCHED_TYPES.to_string());
            return;
        }

        let primitive = expr1.primitive.clone();
        let kind = ExpKind::Binary(op, Box::new(expr1), Box::new(expr2));
        let expr = Expr {
            kind,
            span,
            primitive,
        };
        _tl_stack.push(TranslatorStack::Expression(expr));
    }
}

impl BinaryExpressionAction for BinaryExpression {
=======
    parser::{errors::EXPECT_EXPR, translator_stack::TranslatorStack},
    token::Token,
};

pub struct BE;

impl BinaryExpressionAction for BE {
>>>>>>> origin/master
    //expr + expr
    fn ADD(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Add);
=======
        let plus_token = _token_stack.pop().unwrap(); //pop 'plus' token

        let expr2_tl = _tl_stack.pop().unwrap();
        let expr2 = if let TranslatorStack::Expression(expr) = expr2_tl {
            expr
        } else {
            _errors.push_error(&plus_token, &expr2_tl.get_span(), EXPECT_EXPR.to_string());
            return;
        };

        let expr1_tl = _tl_stack.pop().unwrap();
        let expr1 = if let TranslatorStack::Expression(expr) = expr1_tl {
            expr
        } else {
            _errors.push_error(&plus_token, &expr1_tl.get_span(), EXPECT_EXPR.to_string());
            return;
        };

        let span = expr1.span.to(&expr2.span);
        if expr1.primitive != expr2.primitive {
            _errors.push_error(&plus_token, &span, MISMATCHED_TYPES.to_string());
            return;
        }
        let primitive = expr1.primitive.clone();
        let kind = ExpKind::Binary(BinOpKind::Add, Box::new(expr1), Box::new(expr2));
        let expr = Expr {
            kind,
            span,
            primitive,
        };
        _tl_stack.push(TranslatorStack::Expression(expr));
>>>>>>> origin/master
    }
    fn SUB(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Sub);
    }
    // expr expr
    // for this production there is no proper token to report errors
    // so we're getting last token from the stack and adding again to the stack
    // just to meet the requirements
    // for any production there be should atleast one token for source file path
=======
    }
>>>>>>> origin/master
    fn SPL_SUB(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        let token = _token_stack.last().unwrap().clone();
        _token_stack.push(token);
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Sub);
=======
>>>>>>> origin/master
    }
    fn MUL(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Mul);
=======
>>>>>>> origin/master
    }
    fn DIV(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Div);
=======
>>>>>>> origin/master
    }
    fn REM(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Rem);
=======
>>>>>>> origin/master
    }
    fn AND(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::And);
=======
>>>>>>> origin/master
    }
    fn OR(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Or);
=======
>>>>>>> origin/master
    }
    fn EQ(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Eq);
=======
>>>>>>> origin/master
    }
    fn NE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Ne);
=======
>>>>>>> origin/master
    }
    fn LT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Lt);
=======
>>>>>>> origin/master
    }
    fn LE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Le);
=======
>>>>>>> origin/master
    }
    fn GE(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Ge);
=======
>>>>>>> origin/master
    }
    fn GT(
        _testcase: &mut TestCase,
        _token_stack: &mut Vec<Token>,
        _tl_stack: &mut Vec<TranslatorStack>,
        _errors: &mut Vec<ParseError<Token>>,
    ) {
<<<<<<< HEAD
        BinaryExpression::common(_testcase, _token_stack, _tl_stack, _errors, BinOpKind::Gt);
=======
>>>>>>> origin/master
    }
}
