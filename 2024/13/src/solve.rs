
#[allow(unused_variables,unused_mut)]
pub fn solve_t1(input: &str) -> Result<i64, String> {
    let mut count = 0;

    let list = make_list(input);

    for ele in list {
        let tuple = claw_price(ele.0, ele.1);
        if tuple != (0,0) {
            count += tuple.0 * 3 + tuple.1;
        }
    }

    Ok(count as i64) 
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut count = 0;

    let list = make_list(input);

    // this will run forever
    for ((tx,ax,ay),(ty, bx,by)) in list {
        println!("x:{},y:{}", tx+10000000000000, ty+10000000000000);
        let tuple = claw_price((tx+10000000000000,ax,ay),(ty+10000000000000, bx,by));
        if tuple != (0,0) {
            count += tuple.0 * 3 + tuple.1;
        }
    }

    Ok(count) 
}

fn make_list(input: &str) -> Vec<((i64,i64,i64),(i64,i64,i64))> {
    input
        .split("\n\n")
        .map(|block|{
            let split = block.split("\n").collect::<Vec<&str>>();
            // line 0
            let line = split[0];
            let line = line.replace("Button A: X+", "");
            let line = line.replace(" Y+", "");
            let line = line.split(",").collect::<Vec<&str>>();
            let ax = line[0].parse::<i64>().unwrap();
            let ay = line[1].parse::<i64>().unwrap();
            // line 1
            let line = split[1];
            let line = line.replace("Button B: X+", "");
            let line = line.replace(" Y+", "");
            let line = line.split(",").collect::<Vec<&str>>();
            let bx = line[0].parse::<i64>().unwrap();
            let by = line[1].parse::<i64>().unwrap();
            // line 2
            let line = split[2];
            let line = line.replace("Prize: X=", "");
            let line = line.replace(" Y=", "");
            let line = line.split(",").collect::<Vec<&str>>();
            let a = line[0].parse::<i64>().unwrap();
            let b = line[1].parse::<i64>().unwrap();

            ((a,ax,ay),(b,bx,by))      
        })
        .collect::<Vec<((i64,i64,i64),(i64,i64,i64))>>()
}

// a tuple is build like this (target, inc_x, inc_y)
fn claw_price(eq1: (i64, i64, i64), eq2: (i64, i64, i64)) -> (i64, i64) {
    let (tx,ty) = (eq1.0,eq2.0);
    let (ax,ay) = (eq1.1,eq1.2);
    let (bx,by) = (eq2.1,eq2.2);

    let mut count_a = 0;
    while ay*count_a < ty && ax*count_a < tx {
        let mut count_b = 0;
        while (ay*count_a + by * count_b) <= ty && (ax*count_a + bx * count_b) <= tx {
            let eqa = ax*count_a + bx * count_b;
            let eqb = ay*count_a + by * count_b;
            if eqa == tx && eqb == ty {
                return (count_a,count_b);
            }
            count_b += 1;
        }
        count_a += 1;
    }


    return (0,0);
}
