mod framebuffer;
mod math;
mod camera;
mod geometry;
mod materials;
mod lighting;
mod scene;
mod textures;

use framebuffer::{Framebuffer, Color};
use camera::orbit_camera::OrbitCamera;
use scene::Scene;
use math::vector3::Vector3;
use materials::{material::Material, texture::Texture};
use lighting::phong::Light;
use std::time::Instant;
use raylib::prelude::*;
use raylib::prelude::Color as RColor;

fn main() {
    println!("Diorama interactivo");
    run_interactive_diorama();
}

fn run_interactive_diorama() {
    let window_width = 1200;
    let window_height = 800;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Diorama Interactivo")
        .build();

    rl.set_target_fps(30);

    let fb_width = 400u32;
    let fb_height = 300u32;

    let mut camera = OrbitCamera::new(
        Vector3::new(0.0, 1.0, 0.0),
        10.0,
        std::f32::consts::PI / 3.0,
        fb_width as f32 / fb_height as f32,
    );

    let mut scene = create_diorama_scene();
    let mut framebuffer = Framebuffer::new(fb_width as usize, fb_height as usize);

    let mut last_render = Instant::now();
    let mut needs_render = true; 
    let mut rendering_in_progress = false;
    let mut render_row: usize = 0;
    let rows_per_frame: usize = 12;

    let mut scene_yaw: f32 = 0.0;
    let mut scene_pitch: f32 = 0.0;

    while !rl.window_should_close() {
        let rot_speed = 0.05;
        let mut changed = false;
        if rl.is_key_down(KeyboardKey::KEY_W) { scene_pitch += rot_speed; changed = true; }
        if rl.is_key_down(KeyboardKey::KEY_S) { scene_pitch -= rot_speed; changed = true; }
        if rl.is_key_down(KeyboardKey::KEY_A) { scene_yaw -= rot_speed; changed = true; }
        if rl.is_key_down(KeyboardKey::KEY_D) { scene_yaw += rot_speed; changed = true; }
        if rl.is_key_pressed(KeyboardKey::KEY_Q) { camera.zoom(-0.5); changed = true; }
        if rl.is_key_pressed(KeyboardKey::KEY_E) { camera.zoom(0.5); changed = true; }
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            camera.set_radius(10.0);
            scene_yaw = 0.0;
            scene_pitch = 0.0;
            changed = true;
        }

        scene.set_rotation(scene_yaw, scene_pitch);

        if changed { needs_render = true; }

        if needs_render {
            render_row = 0;
            rendering_in_progress = true;
            needs_render = false;
        }

        if rendering_in_progress {
            if render_row == 0 {
                let cx = framebuffer.width / 2;
                let cy = framebuffer.height / 2;
                let ray = camera.generate_ray(cx as u32, cy as u32, framebuffer.width as u32, framebuffer.height as u32);
                println!("DEBUG: center ray origin={:?} dir={:?}", ray.origin, ray.direction);
                if let Some((dist, uv)) = scene.debug_intersect(&ray) {
                    println!("DEBUG: center ray intersects at distance={} uv={:?}", dist, uv);
                } else {
                    println!("DEBUG: center ray did NOT intersect any object");
                }
                let col = scene.trace_ray(&ray, 3);
                println!("DEBUG: trace_ray color for center = {:?}", col);
                println!("diag: scene has {} cubes and {} lights", scene.cubes.len(), scene.lights.len());
            }

            let height = framebuffer.height;
            if render_row < height {
                let rows_left = height - render_row;
                let count = rows_left.min(rows_per_frame);
                render_scene_partial(&scene, &camera, &mut framebuffer, render_row, count, scene_yaw, scene_pitch);
                render_row += count;
            }

            if render_row >= framebuffer.height {
                rendering_in_progress = false;
                last_render = Instant::now();
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(RColor::DARKBLUE);

        draw_framebuffer_scaled(&framebuffer, &mut d, window_width, window_height);

        d.draw_text("WASD: rotar escena  Q/E: zoom  R: reset", 10, window_height - 50, 20, RColor::WHITE);

        d.draw_text("Modo: Cámara fija (el diorama rota). Zoom = cámara.", 10, window_height - 30, 18, RColor::LIGHTGRAY);

        if rendering_in_progress {
            let percent = (render_row as f32 / framebuffer.height as f32) * 100.0;
            let status = format!("Renderizando: {}/{} filas ({:.0}%)", render_row, framebuffer.height, percent);
            d.draw_text(&status, window_width - 380, window_height - 30, 18, RColor::WHITE);
        } else {
            d.draw_text("Render completado", window_width - 180, window_height - 30, 18, RColor::GREEN);
        }
    }
}

fn render_scene_partial(scene: &Scene, camera: &OrbitCamera, framebuffer: &mut Framebuffer, start_row: usize, num_rows: usize, _scene_yaw: f32, _scene_pitch: f32) {
    let width = framebuffer.width;
    let height = framebuffer.height;

    let end_row = (start_row + num_rows).min(height);
    for y in start_row..end_row {
        for x in 0..width {
            let ray = camera.generate_ray(x as u32, y as u32, width as u32, height as u32);
            let color = scene.trace_ray(&ray, 3);
            framebuffer.set_pixel(x, y, color);
        }
    }
}



fn draw_framebuffer_scaled(fb: &Framebuffer, d: &mut RaylibDrawHandle, win_w: i32, win_h: i32) {
    let scale_x = win_w / fb.width as i32;
    let scale_y = win_h / fb.height as i32;
    let mut scale = scale_x.min(scale_y);
    if scale < 1 { scale = 1; }

    let draw_w = fb.width as i32 * scale;
    let draw_h = fb.height as i32 * scale;
    let offset_x = (win_w - draw_w) / 2;
    let offset_y = (win_h - draw_h) / 2;

    for y in 0..fb.height {
        for x in 0..fb.width {
            let c = fb.get_pixel(x, y);
            let r = (c.x.clamp(0.0, 1.0) * 255.0) as u8;
            let g = (c.y.clamp(0.0, 1.0) * 255.0) as u8;
            let b = (c.z.clamp(0.0, 1.0) * 255.0) as u8;
            let col = RColor::new(r, g, b, 255);

            let sx = offset_x + (x as i32 * scale);
            let sy = offset_y + (y as i32 * scale);
            d.draw_rectangle(sx, sy, scale, scale, col);
        }
    }
}


fn create_diorama_scene() -> Scene {
    let mut scene = Scene::new();
    
    scene.set_skybox(textures::skybox::Skybox::new(
        Color::new(0.4, 0.6, 1.0), 
        Color::new(0.3, 0.5, 0.9), 
    ));
    
    fn find_images_in_dir(dir: &str) -> Vec<String> {
        use std::path::Path;

        fn is_image_file(p: &Path) -> bool {
            if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                match ext.to_lowercase().as_str() {
                    "png" | "jpg" | "jpeg" | "webp" | "bmp" | "tga" | "gif" => return true,
                    _ => {}
                }
            }
            false
        }

        fn score_name(name: &str) -> i32 {
            let name = name.to_lowercase();
            if name.contains("albedo") || name.contains("basecolor") || name.contains("diffuse") { return 100; }
            if name.contains("color") { return 80; }
            if name.contains("diff") { return 60; }
            if name.contains("water") { return 40; }
            10
        }

        fn visit(path: &Path, out: &mut Vec<(i32, String)>) {
            if let Ok(entries) = std::fs::read_dir(path) {
                for e in entries.flatten() {
                    let p = e.path();
                    if p.is_dir() {
                        visit(&p, out);
                    } else if is_image_file(&p) {
                        if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                            let s = score_name(name);
                            out.push((s, p.to_string_lossy().to_string()));
                        }
                    }
                }
            }
        }

        let p = Path::new(dir);
        let mut out: Vec<(i32, String)> = Vec::new();
        if p.exists() && p.is_dir() {
            visit(p, &mut out);
        }

        out.sort_by(|a, b| b.0.cmp(&a.0));
        let paths: Vec<String> = out.into_iter().map(|(_, p)| p).collect();
        if !paths.is_empty() {
            println!("find_images_in_dir: candidates for '{}' -> {:?}", dir, paths);
        } else {
            println!("find_images_in_dir: no image candidates in '{}'", dir);
        }
        paths
    }

    fn try_load_dir(dir: &str, fallback: Color, tiling: (f32,f32)) -> Texture {
        let candidates = find_images_in_dir(dir);
        for img_path in candidates {
            match Texture::from_image(&img_path) {
                Ok(t) => {
                    println!("try_load_dir: loaded '{}'", img_path);
                    return t.with_tiling(tiling.0, tiling.1)
                }
                Err(e) => {
                    println!("try_load_dir: failed to load image '{}' : {}", img_path, e);
                }
            }
        }

        println!("try_load_dir: using fallback color for '{}'", dir);
        Texture::solid_color(fallback).with_tiling(tiling.0, tiling.1)
    }

    fn load_emissive_average(dir: &str) -> Option<math::vector3::Vector3> {
        use std::path::Path;

        let p = Path::new(dir);
        if !p.exists() || !p.is_dir() { return None; }

        let mut best_path: Option<String> = None;
        for entry in std::fs::read_dir(p).unwrap().flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    let n = name.to_lowercase();
                    if n.contains("emiss") || n.contains("emission") || n.contains("emit") || n.contains("glow") {
                        best_path = Some(path.to_string_lossy().to_string());
                        break;
                    }
                }
            }
        }

        if let Some(p) = best_path {
            if let Ok(img) = image::open(&p) {
                let img = img.to_rgba8();
                let (w, h) = img.dimensions();
                let mut acc = (0.0f64, 0.0f64, 0.0f64);
                let mut count = 0u64;
                for y in 0..h {
                    for x in 0..w {
                        let px = img.get_pixel(x, y);
                        acc.0 += px[0] as f64;
                        acc.1 += px[1] as f64;
                        acc.2 += px[2] as f64;
                        count += 1;
                    }
                }
                if count > 0 {
                    let r = (acc.0 / count as f64) as f32 / 255.0;
                    let g = (acc.1 / count as f64) as f32 / 255.0;
                    let b = (acc.2 / count as f64) as f32 / 255.0;
                    println!("load_emissive_average: using emissive map '{}' with avg color ({:.3},{:.3},{:.3})", p, r, g, b);
                    return Some(math::vector3::Vector3::new(r, g, b));
                }
            } else {
                println!("load_emissive_average: failed to open emissive image {}", p);
            }
        }

        None
    }

    let rock_texture = try_load_dir("assets/stone/Rock12", Color::new(0.4, 0.4, 0.4), (2.0,2.0));
    
    let island_stone = Material::new(
        Color::new(0.4, 0.4, 0.4),
        0.1, 8.0, 0.0, 0.0, 1.0
    ).with_texture(rock_texture);
    
    let moss_texture = try_load_dir("assets/stone_plants/Moss1", Color::new(0.3, 0.4, 0.3), (3.0,3.0));
        
    let mossy_stone = Material::new(
        Color::new(0.3, 0.4, 0.3),
        0.15, 12.0, 0.05, 0.0, 1.0
    ).with_texture(moss_texture);
    
    let grass_texture = try_load_dir("assets/stone_plants/moss2", Color::new(0.2, 0.5, 0.2), (4.0,4.0));
    
    let grass_dirt = Material::new(
        Color::new(0.2, 0.4, 0.2),
        0.05, 4.0, 0.0, 0.0, 1.0
    ).with_texture(grass_texture);
    
    let water_texture = try_load_dir("assets/water", Color::new(0.1, 0.3, 0.6), (1.0,1.0));
    let water_material = Material::new(
        Color::new(0.1, 0.3, 0.6),
        0.8, 64.0, 0.6, 0.8, 1.33
    ).with_texture(water_texture);
    
    let lava_texture = try_load_dir("assets/lava", Color::new(1.0, 0.3, 0.1), (1.0,1.0));
        
    let lava_material = Material::new_emissive(
        Color::new(1.0, 0.3, 0.1),
        0.2, 16.0, 0.1, 0.0, 1.0,
        Color::new(1.0, 0.4, 0.1),
        1.2
    ).with_texture(lava_texture);
    
    let wood_texture = try_load_dir("assets/wood/TreeEnd3", Color::new(0.3, 0.2, 0.1), (2.0,2.0));
    
    let dark_wood = Material::new(
        Color::new(0.3, 0.2, 0.1),
        0.2, 8.0, 0.0, 0.0, 1.0
    ).with_texture(wood_texture);
    
    let portal_texture = try_load_dir("assets/portal", Color::new(0.5, 0.1, 0.8), (1.0,1.0));
    let portal_emissive_color = load_emissive_average("assets/portal").unwrap_or(Color::new(0.6, 0.2, 1.0));
    let portal_material = Material::new_emissive(
        Color::new(0.5, 0.1, 0.8),
        0.9, 128.0, 0.3, 0.7, 1.4,
        portal_emissive_color,
        1.5
    ).with_texture(portal_texture);
    
    let island_y = 0.0;
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-3.0, island_y-2.0, -3.0),
        Vector3::new(3.0, island_y+0.5, 3.0),
        island_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-4.0, island_y-1.5, -2.0),
        Vector3::new(-3.0, island_y, 2.0),
        island_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(3.0, island_y-1.5, -2.0),
        Vector3::new(4.0, island_y, 2.0),
        island_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-2.0, island_y-1.8, -4.0),
        Vector3::new(2.0, island_y-0.5, -3.0),
        island_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-2.0, island_y-1.8, 3.0),
        Vector3::new(2.0, island_y-0.5, 4.0),
        island_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-3.5, island_y+0.5, -3.5),
        Vector3::new(3.5, island_y+0.6, 3.5),
        grass_dirt.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-2.5, island_y+0.5, -1.0),
        Vector3::new(2.5, island_y+3.0, 0.0),
        mossy_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-2.5, island_y, -1.0),
        Vector3::new(-2.0, island_y+3.0, 0.0),
        mossy_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(2.0, island_y, -1.0),
        Vector3::new(2.5, island_y+3.0, 0.0),
        mossy_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-2.5, island_y+0.5, -1.0),
        Vector3::new(-2.0, island_y+2.5, 2.0),
        mossy_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(2.0, island_y+0.5, -1.0),
        Vector3::new(2.5, island_y+2.5, 2.0),
        mossy_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-1.0, island_y+2.5, 0.5),
        Vector3::new(-0.8, island_y+3.0, 0.7),
        island_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(0.8, island_y+2.5, 0.5),
        Vector3::new(1.0, island_y+3.0, 0.7),
        island_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-1.5, island_y+0.2, 0.5),
        Vector3::new(1.5, island_y+0.4, 2.5),
        water_material.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-0.3, island_y+0.4, 1.2),
        Vector3::new(0.3, island_y+1.6, 1.8),
        portal_material,
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-0.3, island_y+0.4, 2.5),
        Vector3::new(0.3, island_y+2.8, 2.9),
        dark_wood.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-1.0, island_y+1.8, 2.6),
        Vector3::new(-0.3, island_y+2.1, 2.8),
        dark_wood.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(0.3, island_y+1.8, 2.6),
        Vector3::new(1.0, island_y+2.1, 2.8),
        dark_wood.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-0.2, island_y+2.3, 2.2),
        Vector3::new(0.2, island_y+3.0, 2.4),
        dark_wood.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-0.6, island_y+2.0, 2.3),
        Vector3::new(-0.4, island_y+2.3, 2.5),
        dark_wood.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(0.4, island_y+2.0, 2.3),
        Vector3::new(0.6, island_y+2.3, 2.5),
        dark_wood.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-0.4, island_y+0.2, 2.3),
        Vector3::new(-0.2, island_y+0.4, 2.5),
        dark_wood.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(0.2, island_y+0.2, 2.3),
        Vector3::new(0.4, island_y+0.4, 2.5),
        dark_wood,
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-1.2, island_y-4.0, 3.0),
        Vector3::new(-0.8, island_y+0.5, 3.3),
        lava_material.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(0.8, island_y-4.0, 3.0),
        Vector3::new(1.2, island_y+0.5, 3.3),
        lava_material.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-1.2, island_y-6.0, 3.0),
        Vector3::new(-0.8, island_y-4.0, 3.3),
        lava_material.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(0.8, island_y-6.0, 3.0),
        Vector3::new(1.2, island_y-4.0, 3.3),
        lava_material.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-0.8, island_y+2.0, 0.8),
        Vector3::new(-0.6, island_y+2.8, 1.0),
        lava_material.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(0.6, island_y+2.0, 0.8),
        Vector3::new(0.8, island_y+2.8, 1.0),
        lava_material.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-0.6, island_y+0.4, 1.8),
        Vector3::new(-0.4, island_y+2.0, 2.0),
        lava_material.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(0.4, island_y+0.4, 1.8),
        Vector3::new(0.6, island_y+2.0, 2.0),
        lava_material.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-0.8, island_y+0.1, 0.2),
        Vector3::new(-0.5, island_y+0.3, 0.5),
        island_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(0.5, island_y+0.1, 0.3),
        Vector3::new(0.8, island_y+0.3, 0.6),
        island_stone.clone(),
    ));
    
    scene.add_cube(geometry::cube::Cube::new(
        Vector3::new(-0.2, island_y+0.1, 0.8),
        Vector3::new(0.2, island_y+0.3, 1.1),
        island_stone,
    ));
    
    scene.add_light(Light::new(
        Vector3::new(8.0, 10.0, 5.0),
        Color::new(1.0, 0.95, 0.8),
        1.2,
    ));
    
    scene.add_light(Light::new(
        Vector3::new(-1.0, island_y+1.0, 3.15),
        Color::new(1.0, 0.4, 0.1),
        1.0,
    ));
    
    scene.add_light(Light::new(
        Vector3::new(1.0, island_y+1.0, 3.15),
        Color::new(1.0, 0.4, 0.1),
        1.0,
    ));
    
    scene.add_light(Light::new(
        Vector3::new(-0.7, island_y+2.4, 0.9),
        Color::new(1.0, 0.3, 0.1),
        0.8,
    ));
    
    scene.add_light(Light::new(
        Vector3::new(0.7, island_y+2.4, 0.9),
        Color::new(1.0, 0.3, 0.1),
        0.8,
    ));
    
    scene.add_light(Light::new(
        Vector3::new(-0.5, island_y+1.2, 1.9),
        Color::new(1.0, 0.4, 0.1),
        0.6,
    ));
    
    scene.add_light(Light::new(
        Vector3::new(0.5, island_y+1.2, 1.9),
        Color::new(1.0, 0.4, 0.1),
        0.6,
    ));
    
    scene.add_light(Light::new(
        Vector3::new(0.0, island_y+1.0, 1.5),
        Color::new(0.6, 0.2, 1.0),
        1.0,
    ));
    
    scene.add_light(Light::new(
        Vector3::new(0.0, island_y+0.8, 1.3),
        Color::new(0.4, 0.1, 0.8),
        0.4,
    ));
    
    scene
}


fn render_scene(scene: &Scene, camera: &OrbitCamera, framebuffer: &mut Framebuffer) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let cx = width / 2;
        let cy = height / 2;
        let ray = camera.generate_ray(cx as u32, cy as u32, width as u32, height as u32);
        println!("DEBUG: center ray origin={:?} dir={:?}", ray.origin, ray.direction);
        if let Some((dist, uv)) = scene.debug_intersect(&ray) {
            println!("DEBUG: center ray intersects at distance={} uv={:?}", dist, uv);
        } else {
            println!("DEBUG: center ray did NOT intersect any object");
        }
        let col = scene.trace_ray(&ray, 3);
        println!("DEBUG: trace_ray color for center = {:?}", col);
    });

    use std::sync::Once;
    static DIAG: Once = Once::new();
    DIAG.call_once(|| {
        let cx = (width/2) as u32;
        let cy = (height/2) as u32;
        let ray = camera.generate_ray(cx, cy, width as u32, height as u32);
        println!("diag: center ray direction = {:?}", ray.direction);
        if !ray.direction.x.is_finite() || !ray.direction.y.is_finite() || !ray.direction.z.is_finite() {
            println!("diag: center ray has non-finite components; aborting further debug checks");
        } else {
            let color = scene.trace_ray(&ray, 3);
            println!("diag: trace_ray(center) -> ({:.3},{:.3},{:.3})", color.x, color.y, color.z);
        }
        println!("diag: scene has {} cubes and {} lights", scene.cubes.len(), scene.lights.len());
    });
    
    for y in 0..height {
        for x in 0..width {
            let ray = camera.generate_ray(x as u32, y as u32, width as u32, height as u32);
            let color = scene.trace_ray(&ray, 3); // profundidad máxima de recursión
            framebuffer.set_pixel(x, y, color);
        }
    }
}