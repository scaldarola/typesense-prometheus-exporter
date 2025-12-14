use clap::Parser;

/// Expose Typesense metrics and stats in Prometheus format
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Typesense Host url(s).
    ///
    /// Can be provided multiple times (e.g. `--typesense-host a --typesense-host b`)
    /// or as a comma-separated list via env var `TYPESENSE_HOST` (e.g. `a,b`).
    ///
    /// Each entry may optionally include a port (e.g. `tshost:8108` or `[::1]:8108`).
    #[arg(long, env = "TYPESENSE_HOST", value_delimiter = ',', required = true)]
    pub(crate) typesense_host: Vec<String>,

    /// Typesense protocol
    #[arg(long, env, default_value_t= String::from("http"))]
    pub(crate) typesense_protocol: String,

    /// Typesense API key
    #[arg(long, env)]
    pub(crate) typesense_api_key: String,

    /// Typesense port number
    #[arg(long, env, default_value_t = 8108)]
    pub(crate) typesense_port: u16,

    /// Bind address for internal server
    #[arg(long, env, default_value_t = String::from("0.0.0.0"))]
    pub(crate) exporter_bind_address: String,

    /// Bind port for internal server
    #[arg(long, env, default_value_t = 8888)]
    pub(crate) exporter_bind_port: u16,
}

#[derive(Debug, Clone)]
pub(crate) struct TypesenseTarget {
    pub(crate) host: String,
    pub(crate) port: u16,
}

impl CliArgs {
    pub(crate) fn typesense_targets(&self) -> Vec<TypesenseTarget> {
        self.typesense_host
            .iter()
            .map(|entry| parse_host_and_port(entry, self.typesense_port))
            .collect()
    }
}

fn parse_host_and_port(input: &str, default_port: u16) -> TypesenseTarget {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return TypesenseTarget {
            host: String::new(),
            port: default_port,
        };
    }

    // IPv6 in brackets: [::1]:8108
    if let Some(rest) = trimmed.strip_prefix('[') {
        if let Some(end_bracket) = rest.find(']') {
            let host_part = &rest[..end_bracket];
            let after = &rest[end_bracket + 1..];
            if let Some(port_str) = after.strip_prefix(':') {
                if let Ok(port) = port_str.parse::<u16>() {
                    return TypesenseTarget {
                        host: format!("[{}]", host_part),
                        port,
                    };
                }
            }
            return TypesenseTarget {
                host: format!("[{}]", host_part),
                port: default_port,
            };
        }
    }

    // Hostname:port (take the last ':' segment as port if it's numeric)
    if let Some((host_part, port_part)) = trimmed.rsplit_once(':') {
        if !host_part.is_empty() {
            if let Ok(port) = port_part.parse::<u16>() {
                return TypesenseTarget {
                    host: host_part.to_string(),
                    port,
                };
            }
        }
    }

    TypesenseTarget {
        host: trimmed.to_string(),
        port: default_port,
    }
}
