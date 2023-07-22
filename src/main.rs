fn list(x:usize) -> [usize; 9] {
    let mut result = [0; 9];
    let mut ind = 0;
    for el in result{
        result[ind] = x*(ind+1);
        ind+=1;
    }
    result
}
fn main() {
    let mut x = 1;
    let mut arr = [3;9];
    for el in arr{
    arr = list(x);
    for n in 1..=9{
        println!("{x} * {n} = {}", arr[n-1]);
    }
    println!("");
    x+=1;
    }
}






// fn main() {
//     let mut x = 1;
//     let mut arr = [3;9];
//     arr = list(x);
//     println!("{x} * 1 = {}", arr[0]);
//     println!("{x} * 2 = {}", arr[1]);
//     println!("{x} * 3 = {}", arr[2]);
//     println!("{x} * 4 = {}", arr[3]);
//     println!("{x} * 5 = {}", arr[4]);
//     println!("{x} * 6 = {}", arr[5]);
//     println!("{x} * 7 = {}", arr[6]);
//     println!("{x} * 8 = {}", arr[7]);
//     println!("{x} * 9 = {}", arr[8]);
//     println!("");

//     x = 2;
//     arr = list(x);
//     println!("{x} * 1 = {}", arr[0]);
//     println!("{x} * 2 = {}", arr[1]);
//     println!("{x} * 3 = {}", arr[2]);
//     println!("{x} * 4 = {}", arr[3]);
//     println!("{x} * 5 = {}", arr[4]);
//     println!("{x} * 6 = {}", arr[5]);
//     println!("{x} * 7 = {}", arr[6]);
//     println!("{x} * 8 = {}", arr[7]);
//     println!("{x} * 9 = {}", arr[8]);
//     println!("");

//     x = 3;
//     arr = list(x);
//     println!("{x} * 1 = {}", arr[0]);
//     println!("{x} * 2 = {}", arr[1]);
//     println!("{x} * 3 = {}", arr[2]);
//     println!("{x} * 4 = {}", arr[3]);
//     println!("{x} * 5 = {}", arr[4]);
//     println!("{x} * 6 = {}", arr[5]);
//     println!("{x} * 7 = {}", arr[6]);
//     println!("{x} * 8 = {}", arr[7]);
//     println!("{x} * 9 = {}", arr[8]);
//     println!("");

//     x = 4;
//     arr = list(x);
//     println!("{x} * 1 = {}", arr[0]);
//     println!("{x} * 2 = {}", arr[1]);
//     println!("{x} * 3 = {}", arr[2]);
//     println!("{x} * 4 = {}", arr[3]);
//     println!("{x} * 5 = {}", arr[4]);
//     println!("{x} * 6 = {}", arr[5]);
//     println!("{x} * 7 = {}", arr[6]);
//     println!("{x} * 8 = {}", arr[7]);
//     println!("{x} * 9 = {}", arr[8]);
//     println!("");

//     x = 5;
//     arr = list(x);
//     println!("{x} * 1 = {}", arr[0]);
//     println!("{x} * 2 = {}", arr[1]);
//     println!("{x} * 3 = {}", arr[2]);
//     println!("{x} * 4 = {}", arr[3]);
//     println!("{x} * 5 = {}", arr[4]);
//     println!("{x} * 6 = {}", arr[5]);
//     println!("{x} * 7 = {}", arr[6]);
//     println!("{x} * 8 = {}", arr[7]);
//     println!("{x} * 9 = {}", arr[8]);
//     println!("");

//     x = 6;
//     arr = list(x);
//     println!("{x} * 1 = {}", arr[0]);
//     println!("{x} * 2 = {}", arr[1]);
//     println!("{x} * 3 = {}", arr[2]);
//     println!("{x} * 4 = {}", arr[3]);
//     println!("{x} * 5 = {}", arr[4]);
//     println!("{x} * 6 = {}", arr[5]);
//     println!("{x} * 7 = {}", arr[6]);
//     println!("{x} * 8 = {}", arr[7]);
//     println!("{x} * 9 = {}", arr[8]);
//     println!("");

//     x = 7;
//     arr = list(x);
//     println!("{x} * 1 = {}", arr[0]);
//     println!("{x} * 2 = {}", arr[1]);
//     println!("{x} * 3 = {}", arr[2]);
//     println!("{x} * 4 = {}", arr[3]);
//     println!("{x} * 5 = {}", arr[4]);
//     println!("{x} * 6 = {}", arr[5]);
//     println!("{x} * 7 = {}", arr[6]);
//     println!("{x} * 8 = {}", arr[7]);
//     println!("{x} * 9 = {}", arr[8]);
//     println!("");

//     x = 8;
//     arr = list(x);
//     println!("{x} * 1 = {}", arr[0]);
//     println!("{x} * 2 = {}", arr[1]);
//     println!("{x} * 3 = {}", arr[2]);
//     println!("{x} * 4 = {}", arr[3]);
//     println!("{x} * 5 = {}", arr[4]);
//     println!("{x} * 6 = {}", arr[5]);
//     println!("{x} * 7 = {}", arr[6]);
//     println!("{x} * 8 = {}", arr[7]);
//     println!("{x} * 9 = {}", arr[8]);
//     println!("");

//     x = 9;
//     arr = list(x);
//     println!("{x} * 1 = {}", arr[0]);
//     println!("{x} * 2 = {}", arr[1]);
//     println!("{x} * 3 = {}", arr[2]);
//     println!("{x} * 4 = {}", arr[3]);
//     println!("{x} * 5 = {}", arr[4]);
//     println!("{x} * 6 = {}", arr[5]);
//     println!("{x} * 7 = {}", arr[6]);
//     println!("{x} * 8 = {}", arr[7]);
//     println!("{x} * 9 = {}", arr[8]);
//     println!("");
// }
