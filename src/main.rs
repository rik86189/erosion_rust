/*
based on this youtube video done by Sebastian Lague:
https://www.youtube.com/watch?v=eaXk97ujbPQ&t=251s

*/


use std::{vec};

use image::{open, ImageBuffer, Luma};
use rand::{thread_rng, Rng};

const WIDTH: usize = 512;
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]

struct Particle {
    posX: i16,
    posY: i16,
    isDead: bool,
    lifetime: i16,
    caryCapacity: f32,
    carriedSendiment: f32,
    stuckcounter: i16,
    maxLifetime: i16,
}

impl Particle {
    fn new(x: i16, y: i16, max_life_time: i16) -> Particle {
        Particle {
            posX: x,
            posY: y,
            isDead: false,
            lifetime: max_life_time,
            caryCapacity: 0.0,
            carriedSendiment: 0.0,
            stuckcounter: 0,
            maxLifetime: max_life_time,
        }
    }
    #[allow(non_snake_case)]

    fn moveParticleRefactor(&mut self, heightmap: &Vec<f32>) -> [f32; 2] {
        let mut erodeOutput: f32 = 0.0;
        let mut dispositeOutput: f32 = 0.0;

        if self.posX - 1 > 2
            && self.posY - 1 > 2
            && self.posX - 1 < 510
            && self.posY - 1 < 510
            && self.isDead == false
        {
            // find new position

            let move_vector = self.findDirrection(heightmap);

            //typecast index of current position in array format

            let pos_xusize = self.posX as usize;
            let pos_yusize = self.posY as usize;
            let pos_cord = pos_xusize + (pos_yusize * WIDTH) as usize;

            // typecast inded of new position

            let added_cords_x = self.posX as i32 + move_vector[0] as i32;
            let added_cords_y = self.posY as i32 + move_vector[1] as i32;
            let new_pos_index = added_cords_x as usize + (added_cords_y * WIDTH as i32) as usize;

            //calculate the height diffrence and speed
            let height_diffrence = &heightmap[pos_cord] - &heightmap[new_pos_index];
            let speed: f32 = (1.2 * 9.81 * height_diffrence as f32).sqrt();

            if speed <= 0.0 {
                self.isDead = true;
                dispositeOutput += self.carriedSendiment;
            }

            //calculate carry capactiy

            self.caryCapacity = (speed / (1.2 * 9.81 * 256 as f32).sqrt()
                * (self.lifetime as f32 / self.maxLifetime as f32))
                * 30.0;

            //Add erroded away material to buffer

            erodeOutput += ((height_diffrence as f32) * 0.2).min(height_diffrence);
            self.carriedSendiment += ((height_diffrence as f32) * 0.2).min(height_diffrence);

            //disposite

            if self.carriedSendiment >= self.caryCapacity {
                let diffrence = (self.caryCapacity - self.carriedSendiment).abs();

                dispositeOutput += &diffrence;
                self.carriedSendiment -= &diffrence;
            }

            //update position

            if move_vector[0] == 0 && move_vector[0] == 0 {
                self.stuckcounter += 1;

                if self.stuckcounter > 10 {
                    self.isDead;
                    let posXu = self.posX as usize;
                    let posYu = self.posY as usize;
                    let posCord = posXu + (posYu * WIDTH) as usize;
                    dispositeOutput += self.carriedSendiment;
                } else {
                    let mut rng = thread_rng();
                    let rand_x = rng.gen_range(0..2);
                    let rand_y = rng.gen_range(0..2);

                    self.posX += rand_x;
                    self.posY += rand_y;
                }
            }

            self.posX += move_vector[0];
            self.posY += move_vector[1];
        } else {
            //ensure particle  dies.
            self.isDead = true;
            let posXu = self.posX as usize;
            let posYu = self.posY as usize;
            let posCord = posXu + (posYu * WIDTH) as usize;
            dispositeOutput += self.carriedSendiment;
        }

        //subtract lifetime
        if self.lifetime <= 1 {
            //disposite material
            self.isDead = true;
            let posXu = self.posX as usize;
            let posYu = self.posY as usize;
            let posCord = posXu + (posYu * WIDTH) as usize;
            dispositeOutput += self.carriedSendiment;
        } else {
            self.lifetime -= 1;
        }

        [erodeOutput, dispositeOutput]
    }

    #[allow(non_snake_case)]
    fn findDirrection(self, array: &Vec<f32>) -> [i16; 4] {
        let posXu = self.posX as usize;
        let posYu = self.posY as usize;

        let mut LowestPoint = 255.0;

        let mut MoveVector = [0, 0];

        for y in posYu - 1..posYu + 2 {
            for x in posXu - 1..posXu + 2 {
                if &array[x + (WIDTH * y)] <= &LowestPoint {
                    LowestPoint = array[x + (WIDTH * y)] as f32;

                    MoveVector[0] = x as i16;
                    MoveVector[1] = y as i16;
                }
            }
        }

        [
            MoveVector[0] - self.posX,
            MoveVector[1] - self.posY,
            MoveVector[0],
            MoveVector[1],
        ]
    }
}

fn main() {
    // globalVariables
    let max_particle_lifetime = 30;

    //load Image
    let path_to_image = "test.png";

    let img = open(path_to_image);
    let image_array = &img.unwrap().into_luma8().to_vec();
    let mut buffer_heightmap = vec![];

    for (x, i) in image_array.iter().enumerate() {
        let value = image_array[x] as f32;

        buffer_heightmap.push(value);
    }

    //Create buffer
    // itterator
    for _l in 0..3000 {
        let ammount_particles = 10000; //100000
        let mut Particle_List: Vec<Particle> = vec![];
        let mut buffer_erode: Vec<f32> = vec![];
        let mut buffer_disposite: Vec<f32> = vec![];

        for _i in 0..image_array.len() {
            buffer_erode.push(0.0);
        }

        for _i in 0..image_array.len() {
            buffer_disposite.push(0.0);
        }

        //create Particles
        for _x in 0..ammount_particles {
            let mut rng = thread_rng();
            let rand_x = rng.gen_range(1..511);
            let rand_y = rng.gen_range(1..511);
            let new_particle = Particle::new(rand_x, rand_y, max_particle_lifetime);

            Particle_List.push(new_particle);
        }

        //move particles
        for _z in 0..max_particle_lifetime {
            let particle_list_lenght = Particle_List.len();

            for x in 0..particle_list_lenght {
                if Particle_List[x].isDead == false {
                    // Particle_List[x].moveParticle(
                    //     &buffer_heightmap,
                    //     &mut buffer_erode,
                    //     &mut buffer_disposite,
                    // );

                    let output = Particle_List[x].moveParticleRefactor(&buffer_heightmap);
                    let index_pos =
                        (Particle_List[x].posX as usize) + (Particle_List[x].posY as usize * WIDTH);

                    if index_pos < buffer_erode.len() {
                        buffer_erode[index_pos] += output[0];
                        buffer_disposite[index_pos] += output[1];
                    }

                    //println!("{:?}",output);
                } else {
                    continue;
                }
            }
        }


        buffer_erode = blur_buffer(buffer_erode);
        buffer_disposite = blur_buffer(buffer_disposite);

        Particle_List.clear();

        for (x, i) in image_array.iter().enumerate() {
            buffer_heightmap[x] -= buffer_erode[x];
            buffer_heightmap[x] += buffer_disposite[x];
        }

        buffer_erode.clear();
        buffer_disposite.clear();

        // println!("sum disposite array:{} \n sum erode array {}",sumDispote,sumErode);

        println!("{}", _l);
    }

    let mut image_save = ImageBuffer::new(WIDTH as u32, WIDTH as u32);

    for (x, y, pixel) in image_save.enumerate_pixels_mut() {
        let index_x = x as usize;
        let index_y = y as usize;
        let value = buffer_heightmap[index_x + (index_y * WIDTH)];

        *pixel = image::Rgb([value as f32, value as f32, value as f32]);
    }

    let format = ".exr";
    let mut fileName = "outputImagesFull/testing".to_owned();
    fileName.push_str("end");
    fileName.push_str(format);

    image_save.save(fileName).unwrap();
}

fn blur_buffer(arr: Vec<f32>) -> Vec<f32> {
    image::imageops::blur(
        &ImageBuffer::<Luma<f32>, Vec<f32>>::from_vec(WIDTH as u32, WIDTH as u32, arr).unwrap(),
        2.0,
    )
    .to_vec()
}
