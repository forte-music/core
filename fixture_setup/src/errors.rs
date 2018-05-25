error_chain! {
    foreign_links {
        DotEnv(::dotenv::DotenvError);
        Diesel(::diesel::result::Error);
        Io(::std::io::Error);
        TomlDe(::toml::de::Error);
        R2d2(::r2d2::Error);
        VarError(::std::env::VarError);
    }
}
