use std::{path, u8, vec};

use image::{open, DynamicImage, ImageBuffer, Luma, Rgb, RgbImage, RgbaImage};
use rand::{thread_rng, Rng};

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
    fn new(x: i16, y: i16, maxLifetime: i16) -> Particle {
        Particle {
            posX: x,
            posY: y,
            isDead: false,
            lifetime: maxLifetime,
            caryCapacity: 0.0,
            carriedSendiment: 0.0,
            stuckcounter: 0,
            maxLifetime: maxLifetime,
        }
    }
    #[allow(non_snake_case)]
    fn moveParticle(
        &mut self,
        heightmap: &Vec<f32>,
        buffer_erode: &mut Vec<f32>,
        buffer_disposite: &mut Vec<f32>,
    ) {
        if self.posX - 1 > 2
            && self.posY - 1 > 2
            && self.posX - 1 < 510
            && self.posY - 1 < 510
            && self.isDead == false
        {
            // find new position

            let moveVector = self.findDirrection(heightmap);

            //typecast index of current position in array format

            let posXu = self.posX as usize;
            let posYu = self.posY as usize;
            let posCord = posXu + (posYu * 512) as usize;

            // typecast inded of new position

            let addedCordsX = self.posX as i32 + moveVector[0] as i32;
            let addedCordsY = self.posY as i32 + moveVector[1] as i32;
            let newPosIndex = addedCordsX as usize + (addedCordsY * 512) as usize;

            //calculate the height diffrence and speed
            let heightDiffrence = &heightmap[posCord] - &heightmap[newPosIndex];
            let speed: f32 = (1.2 * 9.81 * heightDiffrence as f32).sqrt();

            if speed <= 0.0 {
                self.isDead = true;
                buffer_disposite[posCord] += self.carriedSendiment;
            }

            //calculate carry capactiy

            self.caryCapacity = (speed / (1.2 * 9.81 * 256 as f32).sqrt()
                * (self.lifetime as f32 / self.maxLifetime as f32))
                * 4.0;

            //Add erroded away material to buffer

            buffer_erode[posCord] += ((heightDiffrence as f32) * 0.2).min(heightDiffrence);
            self.carriedSendiment += ((heightDiffrence as f32) * 0.2).min(heightDiffrence);

            //disposite

            if self.carriedSendiment >= self.caryCapacity {
                let diffrence = (self.caryCapacity - self.carriedSendiment).abs();

                buffer_disposite[posCord] += &diffrence;
                self.carriedSendiment -= &diffrence;
            }

            //update position

            if moveVector[0] == 0 && moveVector[0] == 0 {
                self.stuckcounter += 1;

                if self.stuckcounter > 10 {
                    self.isDead;
                    let posXu = self.posX as usize;
                    let posYu = self.posY as usize;
                    let posCord = posXu + (posYu * 512) as usize;
                    buffer_disposite[posCord] += self.carriedSendiment;
                } else {
                    let mut rng = thread_rng();
                    let rand_x = rng.gen_range(0..1);
                    let rand_y = rng.gen_range(0..1);

                    self.posX += rand_x;
                    self.posY += rand_y;
                }
            }

            self.posX += moveVector[0];
            self.posY += moveVector[1];
        } else {
            //ensure particle  dies.
            self.isDead = true;
            let posXu = self.posX as usize;
            let posYu = self.posY as usize;
            let posCord = posXu + (posYu * 512) as usize;
            buffer_disposite[posCord] += self.carriedSendiment;
        }

        //subtract lifetime
        if self.lifetime <= 1 {
            //disposite material
            self.isDead = true;
            let posXu = self.posX as usize;
            let posYu = self.posY as usize;
            let posCord = posXu + (posYu * 512) as usize;
            buffer_disposite[posCord] += self.carriedSendiment;
        } else {
            self.lifetime -= 1;
        }
    }

    #[allow(non_snake_case)]
    fn findDirrection(self, array: &Vec<f32>) -> [i16; 4] {
        let posXu = self.posX as usize;
        let posYu = self.posY as usize;

        let mut LowestPoint = 255.0;

        let mut MoveVector = [0, 0];

        for y in posYu - 1..posYu + 2 {
            for x in posXu - 1..posXu + 2 {
                if &array[x + (512 * y)] <= &LowestPoint {
                    LowestPoint = array[x + (512 * y)] as f32;

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

    let mut Output:Vec<f32> = vec![];

    for (x, i) in image_array.iter().enumerate() {
        let value = image_array[x] as f32;

        buffer_heightmap.push(value);
    }

    //Create buffer
    // itterator
    for _l in 0..300 {

        let ammount_particles = 3000; //100000
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
            let Particle_List_lenght = Particle_List.len();

            for x in 0..Particle_List_lenght {
                if Particle_List[x].isDead == false {
                    Particle_List[x].moveParticle(
                        &buffer_heightmap,
                        &mut buffer_erode,
                        &mut buffer_disposite,
                    );
                } else {
                    continue;
                }
            }
        }

        //  let sumDispote: f32 = buffer_disposite.iter().sum();
        // let sumErode:f32 = buffer_erode.iter().sum();

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

    let mut imageSave = ImageBuffer::new(512, 512);

    for (x, y, pixel) in imageSave.enumerate_pixels_mut() {
        let indexX = x as usize;
        let indexY = y as usize;
        let value = buffer_heightmap[indexX + (indexY * 512)];

        *pixel = image::Rgb([value as f32, value as f32, value as f32]);
    }

    let format = ".exr";
    let mut fileName = "outputImagesFull/testing".to_owned();
    fileName.push_str("end");
    fileName.push_str(format);

    imageSave.save(fileName).unwrap();
}

// fn blur_buffer(array_to_blur:Vec<f32>)->Vec<f32>{

//     let mut blur_buffer = ImageBuffer::new(512,512);

//         for (x, y, pixel) in blur_buffer.enumerate_pixels_mut() {
//             let index_x = x as usize;
//             let index_y = y as usize;
//             let value = array_to_blur[index_x + (index_y * 512)];

//             *pixel = image::Rgb([value as f32, value as  f32, value as  f32]);

//         }

//         blur_buffer =  image::imageops::blur(&blur_buffer, 1.0);

//    // blur_buffer.to_vec()

// }

fn blur_buffer(arr: Vec<f32>) -> Vec<f32> {
    image::imageops::blur(
        &ImageBuffer::<Luma<f32>, Vec<f32>>::from_vec(512, 512, arr).unwrap(),
        0.75,
    )
    .to_vec()
}
