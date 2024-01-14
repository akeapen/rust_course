fn main() {
    let proceed = true;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 190;
    // if height < 180 {
    //     println!("Tall");
    // } else if height > 170 {
    //     println!("Average");
    // } else {
    //     println!("Short");
    // }

    let age = 15;
    if age < 12 {
        // println!("Child");
        if height < 180 {
            println!("Tall height Child");
        } else if height > 170 {
            println!("Average height Child");
        } else {
            println!("Short height Child");
        }
    } else if age > 18 {
        // println!("Adult");
        if height < 180 {
            println!("Tall height Adult");
        } else if height > 170 {
            println!("Average height Adult");
        } else {
            println!("Short height Adult");
        }
    } else {
        // println!("Teenager");
        if height < 180 {
            println!("Tall height Teenager");
        } else if height > 170 {
            println!("Average height Teenager");
        } else {
            println!("Short height Teenager");
        }
    }

}
