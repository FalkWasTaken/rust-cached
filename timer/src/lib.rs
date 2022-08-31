//use proc_macro::TokenStream;
//use quote::quote;
//
//#[proc_macro]
//pub fn time(input: TokenStream) -> TokenStream {
//    let mut res: TokenStream = quote! {
//        let now = std::time::Instant::now();
//    }.into();
//    res.extend(input);
//    res.extend::<TokenStream>(quote! {
//        println!("Time elapsed: {}ms", now.elapsed().as_millis());
//    }.into());
//
//    println!("{res}");
//
//    res
//}

#[macro_export]
macro_rules! time_s {
    ($block:block) => {
        let now = std::time::Instant::now();
        let res = $block;
        println!("Elapsed time:\t{}s", now.elapsed().as_secs());
        res
    };

    ($block:block, $callback:expr) => {
        let now = std::time::Instant::now();
        let res = $block;
        let elapsed = now.elapsed().as_secs();
        $callback(elapsed);
        res
    };
}

#[macro_export]
macro_rules! time_ms {
    ($block:block) => {
        let now = std::time::Instant::now();
        let res = $block;
        println!("Elapsed time:\t{}ms", now.elapsed().as_millis());
        res
    };

    ($block:block, $callback:expr) => {
        let now = std::time::Instant::now();
        let res = $block;
        let elapsed = now.elapsed().as_millis();
        $callback(elapsed);
        res
    };
}