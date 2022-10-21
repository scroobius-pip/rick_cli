use clap::Parser;

// #[derive(Parser)]
struct Args {
    proxy: bool,
}

trait Rickuest {

}


fn main() {
//    RICK AND MORTY API 
// TYPE A QUERY TO REQUEST DATA FROM THE RICK AND MORTY API
// SYNTAX: CHARACTER::Name(rick)::Page(1)::Contains(rick,name)::Length(10, episode)::Index(0)::Sort(ASC, name)::Pick(id, name,...)

    let args = Args::parse();

    if args.proxy {
        println!("Proxy mode enabled");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}