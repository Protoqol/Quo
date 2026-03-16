use leptos::prelude::*;
use quo::quo;

#[component]
pub fn TestDump() -> impl IntoView {
    #[cfg(debug_assertions)]
    {
        fn test_str() {
            let cool_variable = "Test variable";
            quo!(cool_variable);
        }

        fn test_struct() {
            #[allow(unused)]
            #[derive(Debug)]
            struct Complex {
                string: String,
                integer: i16,
                uinteger: u16,
                array: [String; 3],
            }

            let cool_variable = Complex {
                string: "string".to_string(),
                integer: -16,
                uinteger: 23,
                array: ["This".to_string(), "is".to_string(), "array".to_string()],
            };

            quo!(cool_variable);
        }

        fn test_expression() {
            quo!(42 * 42);
        }

        view! {
            <div class="w-full flex flex-col justify-center items-center gap-y-2 my-2">
                <pre>Debug functions</pre>
                <div on:click=move |_| test_str() class="w-3/4 cursor-pointer bg-pink-700 hover:bg-pink-800 py-1 px-2 rounded">
                    "Send &str to quo"
                </div>
                <div on:click=move |_| test_struct() class="w-3/4 cursor-pointer bg-pink-700 hover:bg-pink-800 py-1 px-2 rounded">
                    "Struct to quo"
                </div>
                <div on:click=move |_| test_expression() class="w-3/4 cursor-pointer bg-pink-700 hover:bg-pink-800 py-1 px-2 rounded">
                    "quo!(42 * 42)"
                </div>
            </div>
        }
    }
}
