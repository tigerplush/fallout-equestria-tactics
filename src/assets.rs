use bevy::reflect::TypeUuid;
use serde::Deserialize;

#[derive(Deserialize, TypeUuid)]
#[uuid = "ab9db2b3-65f7-490e-b22f-c17db17b8f52"]
pub struct Names {
    pub names: Vec<String>,
}
