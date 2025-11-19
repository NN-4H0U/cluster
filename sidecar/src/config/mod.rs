pub mod server;
pub mod player;
pub mod csv_saver;

pub use server::ServerConfig;
pub use player::PlayerConfig;
pub use csv_saver::CsvSaverConfig;

#[macro_export]
macro_rules! create_config {
    ($ident:ident, $namespace:literal, {$($field:ident: $value:ty),+$(,)?}) => {
        #[derive(Debug, Default)]
        pub struct $ident {
            $(
                pub $field: Option<$value>,
            )*
        }

        impl $ident {
            pub fn into_args(self) -> Vec<String> {
                let mut args = vec![];
                $(
                    if let Some(value) = self.$field {
                        args.push(format!("{}::{} {value}", $namespace, stringify!($field)));
                    }
                )*
                args
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub player: PlayerConfig,
    pub csv_saver: CsvSaverConfig,
}

impl Config {
    pub fn into_args(self) -> Vec<String> {
        let mut args = vec![];
        args.append(&mut self.server.into_args());
        args.append(&mut self.player.into_args());
        args.append(&mut self.csv_saver.into_args());
        args
    }
}