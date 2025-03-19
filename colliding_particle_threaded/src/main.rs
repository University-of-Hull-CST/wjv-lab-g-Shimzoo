use rand::random;
use std::time::{Duration, Instant};
use scoped_threadpool::Pool;

const PARTICLES_TOTAL: usize = 100;
const BOUNDARY_LIMIT: f32 = 10.0;
const COLLISION_DISTANCE: f32 = 0.1; // Distance threshold for collision //1

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    x: f32,
    y: f32,
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        Particle { x, y }
    }
    pub fn collide(&self, other: &Particle) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance_squared = dx * dx + dy * dy;
        distance_squared < COLLISION_DISTANCE * COLLISION_DISTANCE
    }
}

impl ParticleSystem {
    pub fn new() -> Self {
        let mut particles = Vec::new();
        for _ in 0..PARTICLES_TOTAL {
            let x = random::<f32>() * 10.0;
            let y = random::<f32>() * 10.0;
            particles.push(Particle::new(x, y));
        }
        ParticleSystem { particles }
    }
    //new 3
    pub fn detect_collisions_threaded(&self) {
        const NUM_COLLISION_THREADS: usize = 1; // Start with one thread
        let total_particles = self.particles.len();
        let chunk_size = (total_particles + NUM_COLLISION_THREADS - 1) / NUM_COLLISION_THREADS;
        
        let mut pool = scoped_threadpool::Pool::new(NUM_COLLISION_THREADS as u32);
        
        pool.scoped(|scope| {
            for chunk in self.particles.chunks(chunk_size) {
                scope.execute(move || {
                    thread_collide_main(chunk);
                });
            }
        });
    }
    //new 4
    pub fn thread_collide_main(particles: &[Particle]) {
        let mut collision_count = 0; // Local counter for collisions
    
        for i in 0..particles.len() {
            for j in (i + 1)..particles.len() {
                if particles[i].collide(&particles[j]) {
                    collision_count += 1;
                }
            }
        }
    
        println!("Collisions detected in thread: {}", collision_count);
    }
    
    

    // متد حرکت ذرات به صورت غیر رشته‌ای (برای مقایسه)
    pub fn move_particles(&mut self) {
        for particle in self.particles.iter_mut() {
            let dx = random::<f32>() - 0.5;
            let dy = random::<f32>() - 0.5;
            particle.x += dx;
            particle.y += dy;

            if particle.x < 0.0 { particle.x = 0.0; }
            if particle.x > 10.0 { particle.x = 10.0; }
            if particle.y < 0.0 { particle.y = 0.0; }
            if particle.y > 10.0 { particle.y = 10.0; }
        }
    }

    // متد حرکت ذرات به صورت رشته‌ای
    pub fn move_particles_threaded(&mut self) {
        const NUM_THREADS: usize = 4;
        // ابتدا طول آرایه ذرات را در یک متغیر ذخیره می‌کنیم.
        let total_particles = self.particles.len();
        // محاسبه اندازه هر بخش (chunk)
        let chunk_size = (total_particles + NUM_THREADS - 1) / NUM_THREADS;
        
        let mut pool = scoped_threadpool::Pool::new(NUM_THREADS as u32);
        
        pool.scoped(|scope| {
            // تقسیم آرایه ذرات به بخش‌های کوچکتر به کمک chunks_mut
            for chunk in self.particles.chunks_mut(chunk_size) {
                scope.execute(move || {
                    thread_main(chunk, 10.0);
                });
            }
        });
    }
// new 5 altered
    pub fn run_for_10_seconds(&mut self) {
        let start = Instant::now();
        let duration = Duration::new(10, 0);
    
        while Instant::now().duration_since(start) < duration {
            self.move_particles_threaded();
            self.detect_collisions_threaded(); // Run collision detection in parallel
        }
    }
    
}
pub fn thread_main(list: &mut [Particle], enclosure_size: f32) {
    for particle in list.iter_mut() {
        let dx = random::<f32>() - 0.5;
        let dy = random::<f32>() - 0.5;
        particle.x += dx;
        particle.y += dy;

        if particle.x < 0.0 { particle.x = 0.0; }
        if particle.x > enclosure_size { particle.x = enclosure_size; }
        if particle.y < 0.0 { particle.y = 0.0; }
        if particle.y > enclosure_size { particle.y = enclosure_size; }
    }
}

fn main() {
    let mut ps = ParticleSystem::new();
    println!("Built {} particles. Here are the first three:", ps.particles.len());
    for particle in ps.particles.iter().take(3) {
        println!("({:.2}, {:.2})", particle.x, particle.y);
    }

    println!("Starting 10-second multi-threaded simulation...");
    ps.run_for_10_seconds();
    println!("Simulation finished.");

    println!("Final positions for the first three particles:");
    for particle in ps.particles.iter().take(3) {
        println!("({:.2}, {:.2})", particle.x, particle.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_initialization() {
        let system = ParticleSystem::new();
        assert_eq!(system.particles.len(), PARTICLES_TOTAL);
        for p in &system.particles {
            assert!(p.x >= 0.0 && p.x <= BOUNDARY_LIMIT);
            assert!(p.y >= 0.0 && p.y <= BOUNDARY_LIMIT);
        }
    }

    #[test]
    fn test_particle_movement() {
        let mut system = ParticleSystem::new();
        let initial_positions: Vec<(f32, f32)> =
            system.particles.iter().map(|p| (p.x, p.y)).collect();

        system.move_particles();

        let new_positions: Vec<(f32, f32)> =
            system.particles.iter().map(|p| (p.x, p.y)).collect();

        // انتظار می‌رود که موقعیت‌ها تغییر کرده باشند
        assert_ne!(initial_positions, new_positions, "ذرات باید حرکت کنند");
    }
}