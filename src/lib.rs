pub use secrecy;
pub use serde;

/// Generate a numeric id type
///
/// The `id!` macro generates a numeric id type that wraps a `u64`. The type implements common
/// traits and conversations that make it easy to use.
///
/// # Example
///
/// ```rust
/// use idtype::id;
///
/// id!(UserId);
///
/// let id: UserId = 42.into();
/// println!("User {} registered", id);
/// ```
#[macro_export]
macro_rules! id {
    (
        $(#[$meta:meta])*
        $id:ident
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, $crate::serde::Deserialize, $crate::serde::Serialize)]
        pub struct $id(u64);

        impl $id {
            /// Initializes a new id.
            pub fn new(id: u64) -> Self {
                Self(id)
            }

            /// Returns the inner value of the id.
            pub fn get(&self) -> u64 {
                self.0
            }
        }

        impl std::fmt::Display for $id {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<u64> for $id {
            fn from(id: u64) -> $id {
                $id(id)
            }
        }
    };
}

/// Generate a string type
///
/// The `name!` macro generates a string type. The type implements common traits and conversations
/// that make it easy to use.
///
/// # Example
///
/// ```rust
/// use idtype::name;
///
/// name!(Username);
///
/// let username: Username = "jdno".into();
/// println!("User {} registered", username);
/// ```
#[macro_export]
macro_rules! name {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, $crate::serde::Deserialize, $crate::serde::Serialize)]
        pub struct $name(String);

        impl $name {
            /// Initializes a new name.
            pub fn new(name: &str) -> Self {
                Self(name.into())
            }

            /// Returns the inner value of the name.
            pub fn get(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<&str> for $name {
            fn from(string: &str) -> $name {
                $name(string.into())
            }
        }

        impl From<String> for $name {
            fn from(string: String) -> $name {
                $name(string)
            }
        }
    };
}

/// Generate a secret type
///
/// The `secret!` macro generates a type for secrets such as passwords or tokens. The type uses the
/// [`secrecy`](https://crates.io/crates/secrecy) crate internally to prevent accidentally leaking
/// the inner value in debug or log statements.
///
/// # Example
///
/// ```rust
/// use idtype::secret;
///
/// secret!(ApiToken);
///
/// let token: ApiToken = "super-secret-api-token".into();
/// let header = format!("Authorization: Bearer {}", token.expose());
/// ```
#[macro_export]
macro_rules! secret {
    (
        $(#[$meta:meta])*
        $secret:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug, $crate::serde::Deserialize)]
        pub struct $secret($crate::secrecy::SecretString);

        impl $secret {
            /// Initializes a new secret.
            pub fn new(secret: &str) -> Self {
                Self($crate::secrecy::SecretString::new(String::from(secret)))
            }

            /// Returns the inner value of the secret.
            pub fn expose(&self) -> &str {
                use $crate::secrecy::ExposeSecret;
                self.0.expose_secret()
            }
        }

        impl std::fmt::Display for $secret {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[REDACTED]")
            }
        }

        impl From<&str> for $secret {
            fn from(secret: &str) -> $secret {
                $secret($crate::secrecy::SecretString::new(String::from(secret)))
            }
        }

        impl From<String> for $secret {
            fn from(secret: String) -> $secret {
                $secret($crate::secrecy::SecretString::new(secret))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    mod id {
        use super::*;

        id!(
            /// Identifier for tests
            TestId
        );

        #[test]
        fn id() {
            let id = TestId::new(42);

            assert_eq!(42, id.get());
        }

        #[test]
        fn trait_display() {
            let id = TestId::new(42);

            assert_eq!("42", id.to_string());
        }

        #[test]
        fn trait_from_u64() {
            let _id: TestId = 42.into();
        }

        #[test]
        fn trait_send() {
            fn assert_send<T: Send>() {}
            assert_send::<TestId>();
        }

        #[test]
        fn trait_sync() {
            fn assert_sync<T: Sync>() {}
            assert_sync::<TestId>();
        }

        #[test]
        fn trait_unpin() {
            fn assert_unpin<T: Unpin>() {}
            assert_unpin::<TestId>();
        }
    }

    mod name {
        use super::*;

        name!(
            /// Name for tests
            TestName
        );

        #[test]
        fn name() {
            let name = TestName::new("test");

            assert_eq!("test", name.get());
        }

        #[test]
        fn trait_display() {
            let name = TestName::new("test");

            assert_eq!("test", name.to_string());
        }

        #[test]
        fn trait_from_str() {
            let _name: TestName = "test".into();
        }

        #[test]
        fn trait_from_string() {
            let _name: TestName = String::from("test").into();
        }

        #[test]
        fn trait_send() {
            fn assert_send<T: Send>() {}
            assert_send::<TestName>();
        }

        #[test]
        fn trait_sync() {
            fn assert_sync<T: Sync>() {}
            assert_sync::<TestName>();
        }

        #[test]
        fn trait_unpin() {
            fn assert_unpin<T: Unpin>() {}
            assert_unpin::<TestName>();
        }
    }

    #[cfg(feature = "secret")]
    mod secret {
        use super::*;

        secret!(
            /// Secret for tests
            TestSecret
        );

        #[test]
        fn secret() {
            let secret = TestSecret::new("test");

            assert_eq!("test", secret.expose());
        }

        #[test]
        fn trait_display() {
            let secret = TestSecret::new("test");

            assert_eq!("[REDACTED]", secret.to_string());
        }

        #[test]
        fn trait_from_str() {
            let _secret: TestSecret = "test".into();
        }

        #[test]
        fn trait_from_string() {
            let _secret: TestSecret = "test".into();
        }

        #[test]
        fn trait_send() {
            fn assert_send<T: Send>() {}
            assert_send::<TestSecret>();
        }

        #[test]
        fn trait_sync() {
            fn assert_sync<T: Sync>() {}
            assert_sync::<TestSecret>();
        }

        #[test]
        fn trait_unpin() {
            fn assert_unpin<T: Unpin>() {}
            assert_unpin::<TestSecret>();
        }
    }
}
