use std::{
    fs::{copy, create_dir_all},
    ops::DerefMut,
    path::Path,
};

use walkdir::WalkDir;

#[derive(Debug)]
struct GarenaCache {
    avatars: Vec<String>,
    user_stickers: Vec<GarenaUserSticker>,
    user_images: Vec<GarenaUserImages>,
}

#[derive(Debug)]
struct GarenaUserSticker {
    user: String,
    stickers: Vec<String>,
}

#[derive(Debug)]
struct GarenaUserImages {
    user: String,
    images: Vec<String>,
}

impl GarenaUserImages {

    fn get_user_images_path(&self) -> String {
        format!("{}/images", &self.user)
    }
}

impl GarenaCache {
    fn copy_file(output_folder_path: &Path, file: &Path) {
        if file.extension() == None {
            copy(
                file.to_str().unwrap(),
                format!("{}/{}.jpg", output_folder_path.display(), file.display()),
            )
            .ok();
            // println!("{}/{}.jpg", output_folder_path.display(), file.display());
        } else {
            copy(
                file.to_str().unwrap(),
                format!("{}/{}", output_folder_path.display(), file.display()),
            )
            .ok();
        }
    }
}

impl GarenaCache {
    pub fn load_avatar(&mut self, path: &str) {
        for entry in WalkDir::new(path) {
            let entry = entry.unwrap();
            self.avatars.push(entry.path().display().to_string());
        }
    }

    pub fn load_all_user_stickers(&mut self, path: &str) {
        for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
            let entry = entry.unwrap();
            let sticker = GarenaUserSticker {
                user: entry.path().display().to_string(),
                stickers: Vec::new(),
            };
            self.user_stickers.push(sticker);
        }

        for user_sticker in self.user_stickers.deref_mut() {
            for entry in WalkDir::new(&user_sticker.user) {
                let entry = entry.unwrap();
                user_sticker
                    .stickers
                    .push(entry.path().display().to_string());
            }

            println!(
                "user: {}, stickerLen: {}",
                user_sticker.user,
                user_sticker.stickers.len()
            );
        }
    }

    pub fn load_all_user_images(&mut self, path: &str) {
        for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
            let entry = entry.unwrap();
            let images = GarenaUserImages {
                user: entry.path().display().to_string(),
                images: Vec::new(),
            };
            self.user_images.push(images);
        }

        for user_image in self.user_images.deref_mut() {
            let images_path = user_image.get_user_images_path();

            for entry in WalkDir::new(&images_path) {
                if !Path::new(&images_path).exists() {
                    break;
                }
                let entry = entry.unwrap();
                user_image.images.push(entry.path().display().to_string());
            }

            println!(
                "user: {}, imagesLen: {}",
                user_image.user,
                user_image.images.len()
            );
        }
    }

    pub fn generate_output(&self, path: &str) {
        let output_path = Path::new(path);
        let avatar_folder_path = &output_path.join(self.avatars[0].as_str());
        create_dir_all(avatar_folder_path).unwrap();

        for avatar in &self.avatars {
            GarenaCache::copy_file(output_path, Path::new(&avatar));
        }

        for user_sticker in &self.user_stickers {
            let sticker_folder_path = &output_path.join(user_sticker.user.as_str());
            create_dir_all(sticker_folder_path).unwrap();

            for sticker in &user_sticker.stickers {
                GarenaCache::copy_file(output_path, Path::new(&sticker));
            }
        }

        for user_image in &self.user_images {
            let image_folder_path = &output_path.join(user_image.get_user_images_path());
            create_dir_all(image_folder_path).unwrap();

            for image in &user_image.images {
                GarenaCache::copy_file(output_path, Path::new(&image));
            }
        }
    }
}

fn main() {
    let avatars: Vec<String> = Vec::new();
    let stickers: Vec<GarenaUserSticker> = Vec::new();
    let images: Vec<GarenaUserImages> = Vec::new();
    let mut garena_cache = GarenaCache {
        avatars,
        user_stickers: stickers,
        user_images: images,
    };

    garena_cache.load_avatar("data/gxx_cache/avatar");
    garena_cache.load_all_user_stickers("data/gxx_cache/sticker");
    garena_cache.load_all_user_images("data/user");
    garena_cache.generate_output("out");

    // TODO: write reader for sqlite3, chat db.

    // println!("{:#?}", garena_cache);
}
