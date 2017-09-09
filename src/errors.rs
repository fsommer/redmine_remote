error_chain!{
    foreign_links {
        ConfigError(::config::ConfigError);
    }
}
