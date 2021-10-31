use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Token,
};

/// Attempts to unwrap the given value expression
///
/// # Example
/// ```
/// fn some_function(value: Option<u32>) -> u32 {
///     return unwrap_or_return!(value, 0);
/// }
///
/// // will print "100"
/// println!("{}", some_function(Some(100)));
///
/// // will print "0"
/// println!("{}", some_function(None));
/// ```
///
/// # Closures
/// If a closure declaration is given as the return expression, the closure will be called upon return if the given value expression cannot be unwrapped.
/// ```
/// unwrap_or_return!(None, |x| println("{}", x), 12)
/// // will print out "12"
/// ```
#[proc_macro]
pub fn unwrap_or_return(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as Unwrapper);

    let source_expression = input.source_expression;
    let closure_arguments = match input.optional_closure_arguments {
        None => quote! {},
        Some(args) => {
            let iter = args.iter();

            quote! {
                #(#iter),*
            }
        }
    };
    let return_expression = match *input.return_expression {
        Expr::Closure(closure) => {
            quote! {
                (#closure)(#closure_arguments)
            }
        }
        expr => quote! {
            #expr
        },
    };

    proc_macro::TokenStream::from(quote! {
        match #source_expression.to_option() {
            Some(value) => value,
            None => {
                return #return_expression;
            }
        }
    })
}

struct Unwrapper {
    source_expression: Box<Expr>,
    return_expression: Box<Expr>,
    optional_closure_arguments: Option<Punctuated<Expr, Token![,]>>,
}

impl Parse for Unwrapper {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let source_expression: Box<Expr> = input.parse()?;
        let _: Token![,] = input.parse()?;
        let return_expression: Box<Expr> = input.parse()?;
        let delimiter: syn::Result<Token![,]> = input.parse();
        let optional_closure_arguments = {
            if delimiter.is_ok() {
                match *return_expression {
                    Expr::Closure(_) => {
                        Some(Punctuated::<Expr, Token![,]>::parse_terminated(input)?)
                    }
                    _ => None,
                }
            } else {
                None
            }
        };

        Ok(Unwrapper {
            source_expression,
            return_expression,
            optional_closure_arguments,
        })
    }
}
