use image::{Rgb, Rgba};
use rand::{Rng, RngExt};

const imgx: u32 = 51;
const imgy: u32 = 51;
pub fn man(m: i32, imgbuf: &image::ImageBuffer<Rgba<u8>, Vec<u8>>, yy2: i32, xx2: i32) -> i32 {
    let mut i = 0;
    let mut pixel22: &Rgba<u8> = &Rgba([1 as u8, 1 as u8, 1, 1]);

    for xxy in -m..=m {
        for yyx in -m..=m {
            if imgbuf.get_pixel((yyx + xx2 as i32) as u32, (xxy + yy2 as i32) as u32) == pixel22 {
                i += 1;
            }
        }
    }

    i
}
pub fn fractal() {
    let _rx: i32 =1; for a in 0..551  {
                                                                
                                                                        let mut uuu_y: Vec<i32> = vec![] ;
                                                                        for a in 0..18 {
                                                                            uuu_y.push(rand::rng().random_range(-_rx..=_rx));
                                                                        }
                                                                            let mut imgbuf: image::ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::new(imgx, imgy);

                                                                            for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
      

    *pixel = image::Rgba([1 as u8, 1 as u8 , 1 , 1]);
}


                                                                            imgbuf.put_pixel(
                                                                                imgx / 2,
                                                                                imgy / 2,
                                                                                Rgba([
                                                                                    1 as u8,
                                                                                    111 as u8, 1,255
                                                                                ]),
                                                                            );
                                                                            let mut ii = 0;

                                                                            let mut i2y =
                                                                                vec![imgy / 2];

                                                                            let mut i2x =
                                                                                vec![imgx / 2];

                                                                            'a: while ii
                                                                                < 151 as u64
                                                                            {
                                                                                let mut iii = 0;
                                                                                for (yy2,xx2) in i2y.clone().iter().zip(i2x.clone())  {  
            
            let iiiiii =man(1,&imgbuf,yy2.clone() as i32,xx2 as i32);
            i2y.remove(0);                       

           i2x.remove(0);          
            if (xx2 as i32) as u32 >= imgy-2 || (xx2 as i32) as u32 <=2 {break  'a;;}; if (yy2.clone() as i32) as u32 >=  imgy-2 || (yy2.clone() as i32) as u32 <=2 {break  'a;}; 
             if  150  == ii  {imgbuf.save(format!("e/{}ractal.png",a)).unwrap();};
            if  (1*2+1)* (1*2+1)-1  == iiiiii  {for a in (0..17).step_by(2)  {i2y.push((yy2.clone() as i32+uuu_y[0+a]) as u32);i2x.push((xx2 as i32+uuu_y[1+a]) as u32);}; imgbuf.put_pixel((xx2 as i32) as u32,( yy2.clone() as i32 ) as u32, Rgba([1 as u8, 111 as u8 , 1,255 ]));
           
    //println!("Hello, world!{}",ii);


ii += 1;}}
                                                                     ii += 1;       }
                                                                            
                                                                        }
                                                               
                                                                    

                         }
