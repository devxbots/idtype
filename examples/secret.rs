use idtype::secret;
use serde::Deserialize;

secret!(ApiToken);

#[derive(Deserialize)]
struct Settings {
    token: ApiToken,
}

fn main() -> Result<(), serde_json::Error> {
    let settings_json = r#"
        {
            "token": "super-secret-token"
        }
    "#;

    let settings: Settings = serde_json::from_str(settings_json)?;

    assert_eq!("super-secret-token", settings.token.expose());

    Ok(())
}
