use std::path::{Path, PathBuf};
use std::sync::Arc;

use config::{Config, ConfigLoader, ProviderConfigRef};
use mock_provider::MockProviderFactory;
use oci_provider::OciProviderFactory;

use crate::registry::ProviderFactoryRegistry;
use crate::{Application, InstanceRegistry};

pub struct Bootstrapper;

impl Bootstrapper {
    pub async fn bootstrap() -> Application {
        let config_dir = Self::resolve_config_dir();

        let config = ConfigLoader::load(&config_dir).unwrap_or_else(|err| {
            tracing::error!(dir = %config_dir.display(), error = %err, "failed to load config");
            std::process::exit(1);
        });

        let factories = Self::build_factory_registry();
        let registry = Self::build_instance_registry(&config, &config_dir, &factories);

        Application::new(registry, config.server)
    }

    fn resolve_config_dir() -> PathBuf {
        let mut args = std::env::args().skip(1);

        while let Some(arg) = args.next() {
            if arg == "--config-dir" {
                if let Some(path) = args.next() {
                    return PathBuf::from(path);
                }
            }
        }

        std::env::var("ACTIVATOR_CONFIG_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./config"))
    }

    fn build_factory_registry() -> ProviderFactoryRegistry {
        let mut factories = ProviderFactoryRegistry::new();
        factories.register(Arc::new(MockProviderFactory));
        factories.register(Arc::new(OciProviderFactory));
        factories
    }

    fn build_instance_registry(
        config: &Config,
        config_dir: &Path,
        factories: &ProviderFactoryRegistry,
    ) -> InstanceRegistry {
        let mut registry = InstanceRegistry::new();

        for instance in &config.instances {
            let factory = factories.get(&instance.provider).unwrap_or_else(|err| {
                tracing::error!(instance = %instance.id, error = %err, "unknown provider");
                std::process::exit(1);
            });

            let resolved_path = match &instance.provider_config {
                Some(ProviderConfigRef::File { path }) => Some(config_dir.join(path)),
                None => None,
            };

            let provider = factory.create(resolved_path.as_deref()).unwrap_or_else(|err| {
                tracing::error!(instance = %instance.id, error = %err, "failed to create provider");
                std::process::exit(1);
            });

            registry
                .register(
                    instance.id.clone(),
                    provider,
                    instance.allowed_origins.clone(),
                )
                .unwrap_or_else(|err| {
                    tracing::error!(error = %err, "failed to register instance");
                    std::process::exit(1);
                });
        }

        registry
    }
}
