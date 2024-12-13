use screenshots::Screen;
use std::time::Instant;

pub fn capture() {
    let start = Instant::now();
    let screens = Screen::all().unwrap();

    for screen in screens {
        
        let mut image = screen.capture().unwrap();
    
            image = screen.capture_area(588, 30, 350, 700).unwrap();
            image
                .save(format!("capture.png"))
                .unwrap();

      
    }

    
}