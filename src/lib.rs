use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(SeedState)]
pub fn seed_state_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_seed_state(&ast)
}

fn impl_seed_state(ast: &syn::DeriveInput) -> TokenStream {
    let variants = if let syn::Data::Enum(ref data_enum) = ast.data {
        &data_enum.variants
    } else {
        panic!("#[derive(SeedState)] is only valid for enums");
    };

    let variants_count = variants.len();
    let name = &ast.ident;

    // Generate match arms: `0 => Enum::Variant0, 1 => Enum::Variant1, ...`
    let match_arms = variants.iter().enumerate().map(|(index, variant)| {
        let variant_name = &variant.ident;
        quote! {
            #index => #name::#variant_name,
        }
    });

    let gen = quote! {
        impl SeedState for #name {
            fn seed(seed: usize) {
                let variant = match seed % #variants_count {
                    #(#match_arms)*
                    _ => unreachable!("All cases should be handled"),
                };

                println!("I have a seed: {}", seed);
                println!("Selected variant: {:?}", variant);
                println!("Number of variants in the enum: {}", #variants_count);
            }
        }
    };
    gen.into()
}
