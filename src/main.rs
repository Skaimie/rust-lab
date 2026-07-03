fn main() {
    let konosuba = anime{
        name :"konosuba".to_string(),
        seasons : 3,
        genres : ["comedy"]
    };
}
struct anime{
    name :String,
    seasons : i8,
    genres : [&str;1]
}
impl anime{
    fn print_description(&self){
        print!("{} is an anime with {} seasons and these genres {:}", self.name,self.seasons,self.genres);
    }
}