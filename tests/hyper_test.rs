#[macro_export]
macro_rules! hyper_test {
    ($fn_name:ident, $stub:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<hyper_ $fn_name>]() {
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await$( .$meth($($arg),*) )+;)+
            }
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<hyper_result_ $fn_name>]() {
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $stub:literal, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<hyper_ $fn_name>]() {
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<hyper_result_ $fn_name>]() {
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await$( .$meth($($arg),*) )+;)+
            }
        }
    };
}