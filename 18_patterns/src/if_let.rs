fn run(){
    let favourite_colour: Option<&str> =None;
    let is_tuesday = false;
    let age:Result<u8, _> = "34".parse();

    if let Some(colour) = favourite_colour {
        println!("Using your favourite colour, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(ok) = age {
        // introduces a new shadowed age variable that contains the value inside the Ok variant
        if age > 30 {
            println!("Using purple as the background colour");
        }else {
            pritnln!("Using orange as the background colour");
        }
    }else {
        println!("Using blue as the background colour");
    }
}