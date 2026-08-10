use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Hash)]
pub struct HeightGenerator {
    seed: u64,
    x_offset: i32,
    z_offset: i32,
}

impl HeightGenerator {
    const OCTAVES: u32 = 3;
    const AMPLITUDE: f32 = 25.0;
    const ROUGHNESS: f32 = 0.05;

    pub fn new(seed: u64, grid_x: i32, grid_z: i32, vertices_per_side: usize) -> Self {
        Self {
            seed,
            x_offset: grid_x * (vertices_per_side - 1) as i32,
            z_offset: grid_z * (vertices_per_side - 1) as i32,
        }
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn generate(&self, x: i32, z: i32) -> f32 {
        let mut total = 0.0;
        let d = 2.0_f32.powf((Self::OCTAVES - 1) as f32);
        for i in 0..Self::OCTAVES {
            let freq = 2.0_f32.powf(i as f32) / d;
            let amp = Self::ROUGHNESS.powf(i as f32) * Self::AMPLITUDE;
            total += self.get_interpolated_noise(
                (x + self.x_offset) as f32 * freq,
                (z + self.z_offset) as f32 * freq,
            ) * amp;
        }
        total
    }

    fn get_interpolated_noise(&self, x: f32, z: f32) -> f32 {
        let int_x = x as i32;
        let int_z = z as i32;
        let frac_x = x - int_x as f32;
        let frac_z = z - int_z as f32;

        let n1 = self.get_smooth_noise(int_x, int_z);
        let n2 = self.get_smooth_noise(int_x + 1, int_z);
        let n3 = self.get_smooth_noise(int_x, int_z + 1);
        let n4 = self.get_smooth_noise(int_x + 1, int_z + 1);

        let i1 = self.interpolate(n1, n2, frac_x);
        let i2 = self.interpolate(n3, n4, frac_x);

        self.interpolate(i1, i2, frac_z)
    }

    fn get_smooth_noise(&self, x: i32, z: i32) -> f32 {
        let corners = (self.get_noise(x - 1, z - 1)
            + self.get_noise(x + 1, z - 1)
            + self.get_noise(x - 1, z + 1)
            + self.get_noise(x + 1, z + 1))
            / 16.0;
        let sides = (self.get_noise(x - 1, z)
            + self.get_noise(x + 1, z)
            + self.get_noise(x, z - 1)
            + self.get_noise(x, z + 1))
            / 8.0;
        let center = self.get_noise(x, z) / 4.0;
        corners + sides + center
    }

    fn get_noise(&self, x: i32, z: i32) -> f32 {
        let mut h = (x as i64)
            .wrapping_mul(49632)
            .wrapping_add((z as i64).wrapping_mul(325176))
            .wrapping_add(self.seed as i64) as u64;

        // nteger hash (splitmix64-style)
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51afd7ed558ccd);
        h ^= h >> 33;
        h = h.wrapping_mul(0xc4ceb9fe1a85ec53);
        h ^= h >> 33;

        ((h as u32) as f32 / u32::MAX as f32) * 2.0 - 1.0
    }

    // fn get_noise(&self, x: i32, z: i32) -> f32 {
    //     use rand::prelude::*;
    //     let seed = (x as i64)
    //         .wrapping_mul(49632)
    //         .wrapping_add((z as i64).wrapping_mul(325176))
    //         .wrapping_add(self.seed as i64);

    //     let mut rng = StdRng::seed_from_u64(seed as u64);
    //     rng.r#gen::<f32>() * 2.0 - 1.0
    // }

    fn interpolate(&self, a: f32, b: f32, blend: f32) -> f32 {
        let theta = blend * std::f32::consts::PI;
        let f = (1.0 - theta.cos()) * 0.5;
        a * (1.0 - f) + b * f
    }
}
