fn main(){
        let x = 2;

        //x = 3; //^^^^^ cannot assign twice to immutable variable

        let mut y = 2;
        y = 3;

        println!("Mutable variable: {}",y);
}
