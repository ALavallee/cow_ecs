extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, FnArg, Type, PathArguments, DeriveInput};

#[proc_macro_attribute]
pub fn cow_task(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input_fn = parse_macro_input!(item as ItemFn);

    // make sure there is no return type
    match input_fn.sig.output {
        // Unit type `()` implies no return type, which is acceptable
        syn::ReturnType::Default => (),
        // Any other return type will trigger an error
        _ => return syn::Error::new_spanned(&input_fn.sig.output, "cow_task functions must not have a return type")
            .to_compile_error()
            .into(),
    }

    if check_fn_args_for_generics(&input_fn) == false {
        return syn::Error::new_spanned(&input_fn.sig.output, "cow_task must only take templated arguments of type Comp<T> or Res<T>")
            .to_compile_error()
            .into();
    }

    let mut templates = vec![];

    let mut args_call = vec![];

    let mut tasks_type = vec![];


    for input_arg in input_fn.sig.inputs.iter() {
        if let FnArg::Typed(pat_type) = input_arg {
            // Check if the type is a path
            if let Type::Path(type_path) = &*pat_type.ty {
                let actual_path = type_path.path.segments.iter()
                    .map(|seg| seg.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");

                if let Some(last_segment) = type_path.path.segments.last() {
                    if let syn::PathArguments::AngleBracketed(angle_bracketed_param) = &last_segment.arguments {
                        if let Some(syn::GenericArgument::Type(generic_type)) = angle_bracketed_param.args.first() {
                            // Convert the generic type to a string and push it to template_types
                            if actual_path == "Comps" {
                                templates.push(generic_type);
                                tasks_type.push(quote!(cow_ecs::schedule::task_type::TaskType::Comp(std::any::TypeId::of::<#generic_type>())));
                                args_call.push(quote!(Comps::new(&&comps.query::<#generic_type>().unwrap().storage().read().unwrap())));
                            } else if actual_path == "CompsMut" {
                                templates.push(generic_type);
                                tasks_type.push(quote!(cow_ecs::schedule::task_type::TaskType::CompMut(std::any::TypeId::of::<#generic_type>())));
                                args_call.push(quote!(CompsMut::new(& mut comps.query::<#generic_type>().unwrap().storage().write().unwrap())));
                            } else if actual_path == "Res" {
                                tasks_type.push(quote!(cow_ecs::schedule::task_type::TaskType::Res(std::any::TypeId::of::<#generic_type>())));
                                args_call.push(quote!(Res::new(&res.query::<#generic_type>().unwrap().resource().read().unwrap())));
                            } else if actual_path == "ResMut" {
                                tasks_type.push(quote!(cow_ecs::schedule::task_type::TaskType::ResMut(std::any::TypeId::of::<#generic_type>())));
                                args_call.push(quote!(ResMut::new(&mut res.query::<#generic_type>().unwrap().resource().write().unwrap())));
                            } else {
                                return syn::Error::new_spanned(&input_fn.sig.output, "cow_task expect arguments to be &Comps<T>,&Res<T> or &Entities, not ".to_owned() + &actual_path)
                                    .to_compile_error()
                                    .into();
                            }
                        } else {
                            return syn::Error::new_spanned(&input_fn.sig.output, "cow_task expect arguments to be &Comps<T>,&Res<T> or &Entities, not ".to_owned() + &actual_path)
                                .to_compile_error()
                                .into();
                        }
                    } else if actual_path == "Commands" {
                        tasks_type.push(quote!(cow_ecs::schedule::task_type::TaskType::Commands()));
                        args_call.push(quote!(cow_ecs::comps::Commands::new(commands)));
                    } else {
                        return syn::Error::new_spanned(&input_fn.sig.output, "cow_task expect arguments to be &Comps<T>, &Res<T> or &Entities, not ")
                            .to_compile_error()
                            .into();
                    }
                } else {
                    return syn::Error::new_spanned(&input_fn.sig.output, "cow_task expect arguments to be &Comps<T> or &Res<T>, not ")
                        .to_compile_error()
                        .into();
                }
            } else {
                return syn::Error::new_spanned(&input_fn.sig.output, "cow_task expect arguments to be &Comps<T> or &Res<T>")
                    .to_compile_error()
                    .into();
            }
        }
    }

    // Extract the function name
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();

    // Generate the implementation
    let expanded = quote::quote! {

        #[allow(non_camel_case_types)]
        struct #fn_name;

        impl cow_ecs::Task for #fn_name {
            fn name(&self) -> String {
                #fn_name_str.to_string()
            }

            fn register(&self, world: &mut cow_ecs::world::World){
                #(world.components_mut().register::<#templates>();)*
            }

            fn arguments(&self) -> Vec<cow_ecs::schedule::task_type::TaskType> {
                vec![#(#tasks_type),*]
            }

            fn run(&self, comps: &cow_ecs::component::comp_manager::CompManager,
                commands : &mut cow_ecs::commands::EntityCommands,
                res : &cow_ecs::resource::res_manager::ResManager) {
                #input_fn

                use cow_ecs::component::comp_storage::CompStorage;
                use cow_ecs::comps::Comps;
                use cow_ecs::comps::CompsMut;
                use cow_ecs::comps::Res;
                use cow_ecs::comps::ResMut;
                use cow_ecs::comps::Commands;

                #fn_name(#(#args_call),*);
            }

        }
    };

    // Hand the generated implementation back to the compiler
    TokenStream::from(expanded)
}

fn check_fn_args_for_generics(func: &ItemFn) -> bool {
    for input in func.sig.inputs.iter() {
        match input {
            syn::FnArg::Typed(pat_type) => {
                match *pat_type.ty {
                    Type::Path(ref type_path) => {
                        for segment in &type_path.path.segments {
                            match &segment.arguments {
                                PathArguments::AngleBracketed(angle_bracketed) => {
                                    if angle_bracketed.args.is_empty() {
                                        return false;
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    return true;
}

#[proc_macro_derive(Component)]
pub fn cow_component_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used for the implementation
    let name = &input.ident;

    // Generate the implementation
    let expanded = quote! {
        impl cow_ecs::component::component::ComponentAny for #name {

             fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }

        }
        impl cow_ecs::component::component::Component for #name {}
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[proc_macro_derive(Resource)]
pub fn cow_resource_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used for the implementation
    let name = &input.ident;

    // Generate the implementation
    let expanded = quote! {
        impl cow_ecs::resource::resource::Resource for #name { }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}