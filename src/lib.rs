// https://rustwasm.github.io/wasm-bindgen/examples/dom.html
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

/* tworzenie bitowego obrazu */
#[wasm_bindgen]
pub fn mandel(
    ctx: &CanvasRenderingContext2d, 
    width: u32, height: u32,
    cx: f64, cy: f64) -> Result<(), JsValue> {

    let i_start = -1.92;
    let i_end   = 1.92;
    let j_start = -1.92;
    let j_end   = 1.92;

    let i_delta = (i_end-i_start)/width as f64;
    let j_delta = (j_end-j_start)/height as f64;

    /* obliczenia */    
    let iterations = 255;

    let mut pix_julia = Vec::with_capacity( usize::try_from(width * height).unwrap() );

    let mut j = j_start;
    for _wj in 0..height {

        let mut i = i_start;   
        for _wi in 0..width {
        
            let mut c: u8 = 0;

            let mut x = cx;
            let mut y = cy;
    
            while ( (x*x + y*y) < 4.0) && (c < iterations)
            {
                let _tx = x;
                x = x * x - y * y + i;
                y = 2.0 * _tx * y + j;
                c += 1;
            }

            pix_julia.push( 255-c );
            pix_julia.push( 255-c );
            pix_julia.push( 255-(c/2) );
            pix_julia.push( 255 );
            
            i  += i_delta;
        }
    
        j  += j_delta;
    }    

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&pix_julia), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}