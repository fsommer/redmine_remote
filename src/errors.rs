error_chain!{
    foreign_links {
        ConfigError(::config::ConfigError);
        CsvError(::csv::Error);
        IoError(::std::io::Error);
        RedmineError(::redmine_api::errors::Error);
        ParseError(::std::num::ParseIntError);
    }
}
