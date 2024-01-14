#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn region_popularity(w: WineRegions) -> &'static str {
    match w {
        WineRegions::Bordeaux   => return "Very Popular!",
        WineRegions::Burgundy   => return "Somewhat Popular!",
        WineRegions::Champagne  => return "Lukewarm",
        WineRegions::Tuscany    => return "Mediocre",
        WineRegions::Rioja      => return "Budget Friendly", 
        WineRegions::NapaValley => return "Very Popular!",
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Merlot"),
        region: WineRegions::NapaValley,
    };
    // println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    // println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);

    println!("Wine 3: {} popularity: {:?}", wine3.name, region_popularity(wine3.region));
}
