use common::*;

use crate::MediaRegistry;
use common::media::MediaData;
use common::rocket_contrib::json::Json;
use common::{Rocket, State};
use std::sync::Mutex;

pub(crate) trait MountApi: Sized {
    fn mount_api(self) -> Self;
}

impl MountApi for Rocket {
    fn mount_api(self) -> Self {
        // this function assumes the media registry is already managed
        self.mount("/", routes![get_registry, get_mediadata])
    }
}

/// returns a json list of items in the registry
#[get("/registry")]
fn get_registry(registry: State<Mutex<MediaRegistry>>) -> Json<Vec<String>> {
    Json(
        registry
            .lock()
            .expect("")
            .content
            .keys()
            .map(|k| k.clone())
            .collect(),
    )
}

#[get("/registry/<media>")]
fn get_mediadata(media: String, registry: State<Mutex<MediaRegistry>>) -> Option<Json<MediaData>> {
    let reg = registry.lock().unwrap();

    if let Some(d) = reg.content.get(&media) {
        Some(Json(d.clone()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::MediaRegistry;
    use common::media::{MediaData, MediaType};
    use common::rocket::local::Client;
    use common::rocket::*;
    use common::*;
    use rstest::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[fixture]
    pub fn empty() -> Rocket {
        rocket::ignite().manage(Mutex::new(MediaRegistry {
            content: HashMap::new(),
        }))
    }

    #[fixture]
    pub fn one() -> Rocket {
        let data = MediaData {
            file: "mock".to_string(),
            thumbnail: None,
            mediatype: MediaType::GIF,
            pixelated: false,
            alt: "it's a gif!".to_string(),
            width: 256,
            height: 256
        };

        let mut content = HashMap::new();
        content.insert("mock".to_string(), data);

        rocket::ignite().manage(Mutex::new(MediaRegistry {
            content: HashMap::new(),
        }));
    }

    #[rstest]
    fn registry_empty() {
        let rocket = empty();
        let client = Client::new(rocket).expect("valid rocket instance");
        let req = client.get("/registry");
        let res = req.dispatch();
    }
}
