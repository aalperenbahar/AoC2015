use std::fs;

fn main() {
 let contents= fs::read_to_string("input.txt").unwrap();//.expect("Problem");
//    let contents  =String::from("^>v<");
    let (mut x,mut y)=(0,0);
    let (mut rx,mut ry)=(0,0);

    let mut stack:Vec<(i32,i32)>=Vec::new();
    stack.push((x,y));
    for d in contents.chars().step_by(2){
 //print!("{x}");
        match d{
            '<'=> x=x-1,
            '>'=> x=x+1,
            '^'=> y=y+1,
            'v'=> y=y-1,
            _=> continue,
        }
        if !stack.contains(&(x,y)){
            stack.push((x,y));
        }}
        let mut t=contents.chars();
        t.next();
        for d in t.step_by(2){
            //print!("{x}");
            match d{
                '<'=> rx=rx-1,
                '>'=> rx=rx+1,
                '^'=> ry=ry+1,
                'v'=> ry=ry-1,
                _=> continue,
            }
            if !stack.contains(&(rx,ry)){
                stack.push((rx,ry));
            }
        }

    println!("{}",stack.len());
}
