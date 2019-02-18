use synstructure::decl_derive;
use quote::quote;

fn bincode_max_size_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let variant_max = s.variants().iter().fold(quote!(0),
        |acc, variant| {
            // compute size of each variant by summing the sizes of its bindings
            let variant_size = variant.bindings().iter().map(|binding| {
                let ty = &binding.ast().ty;
                quote!(<#ty as bincode_maxsize::BincodeMaxSize>::bincode_max_size())
            }).fold(quote!(0), |acc, x| quote!(#acc + #x));

            quote!(max(#acc, #variant_size))
        });

    let desc_size = match &s.variants()[..] {
        // a single variant with no prefix is 'struct' so we don't need a discriminant
        [v] if v.prefix.is_none() => { quote!(0) }
        _ => { quote!(4) }
    };
    let body = quote!(#desc_size + #variant_max);

    s.bound_impl(quote!(bincode_maxsize::BincodeMaxSize), quote!(
        fn bincode_max_size() -> usize {
            use std::cmp::max;
            #body
        }
    ))
}
decl_derive!([BincodeMaxSize] => bincode_max_size_derive);

#[cfg(test)]
mod tests {
    use synstructure;
    use syn;

    #[test]
    fn it_works() {
        let source = syn::parse_str(
            //"enum Foo<T> { M(i32), O(Bar, T, Arc<T>) }",
            "enum Foo<T> { M(Option<i32>) }",

        )
            .unwrap();
        let source = synstructure::Structure::new(&source);

        let expanded = crate::bincode_max_size_derive(source).to_string();
        println!("{}", expanded);
    }
}
