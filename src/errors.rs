error_chain!{
    foreign_links {
        ConfigError(::config::ConfigError);
        RedmineError(::redmine_api::errors::Error);
    }
}
