struct Quiz1{
    question1: String,
    answer1: Answer1
}
struct Quiz2{
    question2: String,
    answer2: Answer2
}
struct Quiz3{
    question3: String,
    answer3: Answer3
}
enum Answer1{
    Blue,
    Green,
    Violet
}
enum Answer2{
    One,
    Two,
    Fifteen
}
enum Answer3{
    Apple,
    Pencil,
    Flowers
}

fn main() {
    let are_you_ok1 = Quiz1{
        question1: String::from("What color is the sky?"),
        answer1: Answer1::Blue
    };
    let are_you_ok2 = Quiz2 {
        question2: String::from("How many suns do we have?"),
        answer2: Answer2::One
    };
    let are_you_ok3 = Quiz3 {
        question3: String::from("What would you like to eat?"),
        answer3: Answer3::Pencil
    };


    let mut num = 0;
    match are_you_ok1.answer1 {
        Answer1::Blue => {
            // 
        }
        _ => {
            num += 1;
        }
    }
    match are_you_ok2.answer2 {
        Answer2::One => {
            // 
        }
        _ => {
            num += 1;
        }
    }
    match are_you_ok3.answer3 {
        Answer3::Apple => {
            // 
        }
        _ => {
            num += 1;
        }
    }
    if num == 0{
        println!("You are completely OK 👍");
    }
     else if num == 1{
        println!("I hope you just made a mistake. If not, then Google it👀");
    }
    else if num == 2{
        println!("You better get some rest. Get some sleep. Drink some water🌊");
    }
    else if num == 3{
        println!("You are 3 years old or you need professional help🆘");
    }
}