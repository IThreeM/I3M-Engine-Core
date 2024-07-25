#![allow(clippy::manual_unwrap_or_default)]

mod component;
mod reflect;
mod uuid;
mod visit;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Implements `Visit` trait
///
/// User has to import `Visit`, `Visitor` and `VisitResult` to use this macro.
///
/// # Expansion
///
/// For example,
///
/// ```
/// use i3m_core::visitor::{Visit, VisitResult, Visitor};
/// #[derive(Visit)]
/// struct Foo<T> {
///     example_one: String,
///     example_two: T,
/// }
/// # fn main() {}
/// ```
///
/// would expand to something like:
///
/// ```
/// # use i3m_core::visitor::{Visit, VisitResult, Visitor};
/// # struct Foo<T> { example_one: String, example_two: T,}
/// impl<T> Visit for Foo<T> where T: Visit {
///     fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
///         let mut region = visitor.enter_region(name)?;
///         self.example_one.visit("ExampleOne", &mut region)?;
///         self.example_two.visit("ExampleTwo", &mut region)?;
///         Ok(())
///     }
/// }
/// # fn main() {}
/// ```
///
/// Field names are converted to strings using
/// [to_case(Case::UpperCamel)](https://docs.rs/convert_case/0.6.0/convert_case/enum.Case.html#variant.Pascal).
///
/// ```
/// # use i3m_core::visitor::{Visit, VisitResult, Visitor};
/// #[derive(Visit)]
/// struct Pair (usize, usize);
/// # fn main() {}
/// ```
///
/// would expand to something like:
///
/// ```
/// # use i3m_core::visitor::{Visit, VisitResult, Visitor};
/// # struct Pair (usize, usize);
/// impl Visit for Pair {
///     fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
///         let mut region = visitor.enter_region(name)?;
///         self.0.visit("0", &mut region)?;
///         self.1.visit("1", &mut region)?;
///         Ok(())
///     }
/// }
/// # fn main() {}
/// ```
///
/// ```
/// # use i3m_core::visitor::{Visit, VisitResult, Visitor};
/// #[derive(Visit)]
/// enum EnumExample { A, B(usize) }
/// # fn main() {}
/// ```
///
/// would expand to something like:
///
/// ```
/// # use i3m_core::visitor::{Visit, VisitResult, Visitor};
/// # enum EnumExample { A, B(usize) }
/// impl Visit for EnumExample {
///     fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
///         let mut region = visitor.enter_region(name)?;
///         let mut id = id(self);
///         id.visit("Id", &mut region)?;
///         if region.is_reading() {
///             *self = from_id(id)?;
///         }
///         match self {
///             EnumExample::A => (),
///             EnumExample::B(x) => { x.visit("0", &mut region)?; }
///         }
///         return Ok(());
///         fn id(me: &EnumExample) -> u32 {
///             match me {
///                 EnumExample::A => 0,
///                 EnumExample::B(_) => 1,
///             }
///         }
///         fn from_id(id: u32) -> std::result::Result<EnumExample,String> {
///             match id {
///                 0 => Ok(EnumExample::A),
///                 1 => Ok(EnumExample::B(Default::default())),
///                 _ => Err(format!("Unknown ID for type `EnumExample`: `{}`", id)),
///             }
///         }
///     }
/// }
/// # fn main() {}
/// ```
#[proc_macro_derive(Visit, attributes(visit))]
pub fn visit(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::from(visit::impl_visit(ast))
}

/// Implements `Reflect` trait
#[proc_macro_derive(Reflect, attributes(reflect))]
pub fn reflect(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut ty_args = reflect::args::TypeArgs::from_derive_input(&ast).unwrap();
    ty_args.validate();

    let reflect_impl = reflect::impl_reflect(&ty_args);
    let prop_key_impl = reflect::impl_prop_constants(&ty_args);

    TokenStream::from(quote::quote! {
        #reflect_impl
        #prop_key_impl
    })
}

/// Implements `Reflect` by analyzing derive input, without adding property constants
///
/// This is used to implement the `Reflect` trait for external types.
#[proc_macro]
pub fn impl_reflect(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut ty_args = reflect::args::TypeArgs::from_derive_input(&ast).unwrap();
    ty_args.validate();

    let reflect_impl = reflect::impl_reflect(&ty_args);

    TokenStream::from(reflect_impl)
}

#[proc_macro]
pub fn impl_visit(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::from(visit::impl_visit(ast))
}

/// Implements `TypeUuidProvider` trait
///
/// User has to import `TypeUuidProvider` trait to use this macro.
#[proc_macro_derive(TypeUuidProvider, attributes(type_uuid))]
pub fn type_uuid(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::from(uuid::impl_type_uuid_provider(ast))
}

/// Implements `ComponentProvider` trait
///
/// User has to import `ComponentProvider` trait to use this macro.
#[proc_macro_derive(ComponentProvider, attributes(component))]
pub fn component(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::from(component::impl_type_uuid_provider(ast))
}
