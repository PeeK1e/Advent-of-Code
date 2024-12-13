struct Machine {
    x1: i128,
    y1: i128,
    x2: i128,
    y2: i128,
    xtarget: i128,
    ytarget: i128,
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t1(input: &str) -> Result<i128, String> {
    let p = parse(input,0); 

    let count = p.iter().map(|m| {
        if let Some((a,b)) = wirsing(m) {
            3 * a + b
        } else {
            0
        }
    }).sum();

    Ok(count) 
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<i128, String> {
    let p = parse(input,10000000000000); 

    let count = p.iter().map(|m| {
        if let Some((a,b)) = wirsing(m) {
            3 * a + b
        } else {
            0
        }
    }).sum();

    Ok(count) 
}

fn parse(input: &str, modifyer: i128) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|claw|{
            let split = claw
                .split(&['+', ',', '\n', '=']).collect::<Vec<_>>();
            Machine{
                x1: split[1].parse::<i128>().unwrap(),
                y1: split[3].parse::<i128>().unwrap(),
                x2: split[5].parse::<i128>().unwrap(),
                y2: split[7].parse::<i128>().unwrap(),
                xtarget: modifyer + split[9].parse::<i128>().unwrap(),
                ytarget: modifyer + split[11].parse::<i128>().unwrap(),
            }
        })
        .collect::<Vec<Machine>>()
}

fn wirsing(m: &Machine) -> Option<(i128,i128)> {
    let det = m.x1 * m.y2 - m.y1 * m.x2;
    if det == 0 {
        return None;
    }

    let det_a = m.xtarget * m.y2 - m.x2 * m.ytarget;
    let det_b = m.x1 * m.ytarget - m.xtarget * m.y1;

    if det_a % det != 0 || det_b % det != 0 {
       return None;
    }
    let a = det_a / det;
    let b = det_b / det;

    if a < 0 || b < 0 {
        return None;
    }
    return Some((a,b));
}
