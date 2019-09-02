extern crate paste;

#[macro_export]
macro_rules! default_struct {
    ($struct:ident, $param:ident, $type:tt,
     ($($field_name:ident),*$(,)*),
     ($($field_default_expr:expr),*$(,)*))
     => {

        pub struct $struct {
            $param: $type,
        }

        impl Default for $struct {
            fn default() -> Self {
                let $param = $type {
                    $($field_name: $field_default_expr,)*
                };
                Self { $param }
            }
        }
    }
}

#[macro_export]
macro_rules! create_struct {
    ($struct:ident, $param:ident, $type:tt,
     ($($field_name:ident),*$(,)*),
     ($($field_type:ty),*$(,)*),
     ($($field_default_expr:expr),*$(,)*),
     ($($field_expr:expr),*$(,)*)
     ) => {

        default_struct!($struct, $param, $type,
                       ($($field_name,)*),
                       ($($field_default_expr,)*));

        set_and_get_params!($struct, $param,
                   ($($field_name,)*),
                   ($($field_type,)*),
                   ($($field_expr,)*));
    }
}

#[macro_export]
macro_rules! set_and_get_params {
    ($struct:ident, $param:ident,
    ($($field_name:ident),*$(,)*),
    ($($field_type:ty),*$(,)*),
    ($($field_expr:expr),*$(,)*))
    => {
            impl $struct {
                paste::item! {
                    $(
                        pub fn $field_name(&self) -> $field_type  {
                            self.$param.$field_name as $field_type
                        }

                        pub fn [<set_ $field_name>](&mut self, $field_name: $field_type) {
                            self.$param.$field_name = paste::expr! { ($field_expr) }
                        }
                    )*
                }
            }
       }
}
