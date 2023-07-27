struct Company
{
    name: String,
    profit: i32,
    employers: [Employers; 15],
    parts: [Parts;3]
}
struct Employers
{
    //name: String,
    //part: i32,
    id: i32
}
struct Parts
{
    //name: String,
    id: i32
}
fn main() {
    impl Company
    {   
        fn numberE(&self) -> i32
        {
            let mut num = 0;
            for x in 0..15{
                if self.employers[x].id != 0{
                    num+=1;
                }
            }
            num
        }
        fn numberP(&self) -> i32
        {
            let mut num = 0;
            for x in 0..3{
                if self.parts[x].id != 0{
                    num+=1;
                }
            }
            num
        }
    }
    let em: [Employers; 15] = [Employers { id: 0 },Employers { id: 0 },Employers { id: 0 },Employers { id: 4 },Employers { id: 5 },Employers { id: 0 },Employers { id: 0 },Employers { id: 0 },Employers { id: 0 },Employers { id: 0 },Employers { id: 0 },Employers { id: 0 },Employers { id: 0 },Employers { id: 0 },Employers { id: 0 }];
    let pr: [Parts; 3] = [Parts { id: 0 },Parts { id: 2 },Parts { id: 0 }];
    let mut comp: Company = Company{
        name:String::from("Test"),
        profit: 15000,
        employers: em,
        parts: pr
    };
    println!("Number of employers {}", comp.numberE());
    println!("Number of parts {}", comp.numberP());
}






