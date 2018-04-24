fn main() {
    let x: u32 = 100;
    let y: u64 = 200;
    let z: char = '\u{2013}';
    let l: f32 = 100.3245;
    let m: f64 = 100.3245;
    println!("uint32 = {}
uint64= {} 
unicode 2013 long dash = {}
float 32 = {} 
float 64 = {} ",x,y,z, l, m);

   let mytup: (u32,f32,char) = (x,l,z);
   // All std library types automatically are printable with {:?} too:
   println!("tuple = {:?}",mytup);
   println!("tuple element 0 = {:?}",mytup.0);
   println!("tuple element 1 = {:?}",mytup.1);
   let mylist:[u32 ; 5] = [1,2,3,4,5];
   println!("list = {:?}",mylist);
   println!("list = {}",mylist[0]);
}
