#[allow(unused_variables,unused_mut)]
pub fn solve_t1(input: &str) -> Result<i64, String> {
    let mut count = 0;
    let limit = 100;

    let mut dial: i64 = 50; 
    for ele in input.split("\n") {
        let n = decode_line(ele);

        let new_dial = if (dial+n)%limit < 0 {
            ((dial+n)%limit)+limit
        } else {
            (dial+n)%limit
        };

        if new_dial == 0 {
            count += 1;
        }
        dial = new_dial
    }

    Ok(count) 
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut count = 0;
    let limit = 100;

    let mut dial: i64 = 50; 
    for ele in input.split("\n") {
        let n = decode_line(ele);

        // get overflow
        let mut add = ((n+dial)/limit).abs();
        
        // check if we moved back over zero
        if dial != 0 && dial+n < 0 && dial+n != 0{
            add += 1;
        }
        
        // shift the dial around
        dial += n;
        dial %= limit;
        if dial < 0 {
            dial += limit;
        }
        
        // only count += if the add value is also zero, since we have the overflow in add when we get exactly zero by overflowing
        if dial == 0 && add == 0 {
            count+=1;
        }
        count+=add;
    }

    Ok(count) 
}

fn decode_line(line: &str) -> i64 {
    if line.len() == 0 {
        return 0;
    }
    let (dir, n) = line.split_at(1);

    let mut n = match n.parse::<i64>() {
        Ok(x) => x,
        Err(x) => {
            println!("error {}", x);
            return 0;
        }
    };

    if dir == "L" {
        n *= -1;
    }

    return n;
}