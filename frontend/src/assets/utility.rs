use web_sys::{js_sys::Math, window};

const IMAGES321PX: &[&str] = &[
        "https://www.awxcdn.com/adc-assets/images/hero/1/375x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/2/375x450.jpg",
        "https://www.awxcdn.com/adc-assets/images/hero/3/375x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/4/375x450.jpg",  
        "https://www.awxcdn.com/adc-assets/images/hero/6/375x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/6/375x450.jpg"
    ];
const IMAGES376PX: &[&str] = &[
        "https://www.awxcdn.com/adc-assets/images/hero/1/425x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/2/425x450.jpg",
        "https://www.awxcdn.com/adc-assets/images/hero/3/425x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/4/425x450.jpg",  
        "https://www.awxcdn.com/adc-assets/images/hero/6/425x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/6/425x450.jpg"
        
    ];
const IMAGES426PX: &[&str] = &[
        "https://www.awxcdn.com/adc-assets/images/hero/1/768x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/2/768x450.jpg",
        "https://www.awxcdn.com/adc-assets/images/hero/3/768x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/4/768x450.jpg",  
        "https://www.awxcdn.com/adc-assets/images/hero/6/768x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/6/768x450.jpg"
    ];
const IMAGES769PX: &[&str] = &[
        "https://www.awxcdn.com/adc-assets/images/hero/1/1024x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/2/1024x450.jpg",
        "https://www.awxcdn.com/adc-assets/images/hero/3/1024x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/4/1024x450.jpg",  
        "https://www.awxcdn.com/adc-assets/images/hero/6/1024x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/6/1024x450.jpg"
    ];
const IMAGES1025PX: &[&str] = &[
        "https://www.awxcdn.com/adc-assets/images/hero/1/1440x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/2/1440x450.jpg",
        "https://www.awxcdn.com/adc-assets/images/hero/3/1440x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/4/1440x450.jpg",  
        "https://www.awxcdn.com/adc-assets/images/hero/6/1440x450.jpg", 
        "https://www.awxcdn.com/adc-assets/images/hero/6/1440x450.jpg"
    ];
fn get_window_width() -> f64{
    let window = window().expect("Could not get window");
    let width = window.inner_width().unwrap().as_f64().unwrap_or(1024.0);
    width
}
pub fn get_image_related_to_width() -> &'static [&'static str]{

    let width = get_window_width();
    let chosen_image = if width < 321.0 {
        IMAGES321PX
    } else if width < 321.0 {
        IMAGES376PX
    }
    else if width < 376.0 {
        IMAGES376PX
    }
    else if width < 426.0 {
        IMAGES426PX
    }
    else if width < 769.0 {
        IMAGES769PX
    } else{
        IMAGES1025PX
    };
    chosen_image
}
pub fn get_random_int(number: f64) -> usize{
    Math::floor(Math::random() * number) as usize
}