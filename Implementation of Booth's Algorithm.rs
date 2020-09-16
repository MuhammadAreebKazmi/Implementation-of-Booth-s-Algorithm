use std::io;
//use std::cmp;
use std::time::{Instant};

fn main() {
    println!("Please input the multiplicand");
    let mut multiplicand = String::new();
    io::stdin().read_line(&mut multiplicand).expect("Failed to read line");
    let multiplicand: u32 = multiplicand.trim().parse().expect("Please type a number");

    //entering sign for multiplicand
    println!("Please input the sign");
    let mut firstsign = String::new();
    io::stdin().read_line(&mut firstsign).expect("Failed to read line");
    let firstsign: char = firstsign.trim().parse().expect("Please type a char");

    //entering multiplier
    println!("Please input the multiplier");
    let mut multiplier = String::new();
    io::stdin().read_line(&mut multiplier).expect("Failed to read the number");
    let multiplier: u32 = multiplier.trim().parse().expect("Please type a number");

    //entering the multiplier sign
    println!("Please input the sign");
    let mut secondsign = String::new();
    io::stdin().read_line(&mut secondsign).expect("Failed to read line");
    let secondsign: char = secondsign.trim().parse().expect("Please type a char");

    //Converting multiplicand denary/decimal into binary
    let mut vector = vec![];
    let mut remainder = multiplicand % 2;
    let mut divided = multiplicand / 2;
    vector.push(remainder);
    //println!("vector is: {:?}", vector);

    while divided != 0 {
        remainder = divided % 2;
        //println!("remainder is: {}", remainder);
        vector.push(remainder);
        divided = divided / 2;
        //println!("divided is: {}", divided);
        //println!("vector is: {:?}", vector);
    }
    let mut m = vec![0];
    let mut index:isize = (vector.len()-1) as isize;

    while index >= 0 {
        m.push(vector[index as usize]);
        index = index - 1;
        //println!("m is: {:?}", m);
    }
    //Converting denary/decimal into binary
    let mut vector = vec![];
    let mut remainder = multiplier % 2;
    let mut divided = multiplier / 2;
    //println!("remainder is: {}", remainder);
    //println!("divided is: {}", divided);
    vector.push(remainder);
    //println!("vector is: {:?}", vector);

    while divided != 0 {
        remainder = divided % 2;
        //println!("remainder is: {}", remainder);
        vector.push(remainder);
        divided = divided / 2;
        //println!("divided is: {}", divided);
        //println!("vector is: {:?}", vector);
    }
    let mut q = vec![0];
    let mut index:isize = (vector.len()-1) as isize;

    while index >= 0 {
        q.push(vector[index as usize]);
        index = index - 1;
        //println!("q is: {:?}", q);
    }
    //making the size of bits equal for m and q
    let msbbit = 0;
    let mut mlength = m.len();
    let mut qlength = q.len();
    if mlength > qlength {
        while qlength != mlength {
            q.push(0);
            //Shifting right
            let mut position:isize = (q.len()-2) as isize;
            while position >= 0 {
                q[(position + 1) as usize] = q[position as usize];
                position = position - 1;
            }
            q[0] = msbbit;
            qlength = q.len();
        }
        //println!("q enlarged is: {:?}", q);
    }
    //making m bits equal to q if not equal
    if qlength > mlength {
        while mlength != qlength {
            m.push(0);
            //shifting right
            let mut position:isize = (m.len()-2) as isize;
            while position >= 0 {
                m[(position + 1) as usize] = m[position as usize];
                position = position - 1;
            }
            m[0] = msbbit;
            mlength = m.len();
        }
        //println!("m enlarged is: {:?}", m);
    }

    //converting into negative if negative sign
    if firstsign == '-' {
        let mut minusm = vec![];
        let mut carry = 0;
        //To take minus of M
        // One's complement
        for i in &m {
            if i == &0 {
                minusm.push(1);
            } else {
                minusm.push(0);
            }
        }
        //println!("minus m inverted is: {:?}", minusm);
        let minmlength = minusm.len()-1;
        // Two's complement
        //let mut binsum = 0;
        let binsum = minusm.last().unwrap() + 1;
        //println!("binsum: {}", binsum);
        if binsum < 2 {
            minusm[minmlength] = binsum;
            //println!("minus m update is: {:?}", minusm);
        } else {
            carry = 1;
            minusm[minmlength] = 0;
            //println!("minus m updated 2nd cond: {:?}", minusm);
        }
        // Two's Complement
        let mut  mlength:isize = (m.len()-2) as isize; // works as an index to add with carry
        while carry == 1 && mlength >= 0 { // OR 0
            let binsum = minusm[mlength as usize] + carry;
            //println!("binsum is: {}", binsum);
            if binsum < 2 {
                minusm[mlength as usize] = binsum;
                carry = 0;
            } else {
                minusm[mlength as usize] = 0;
                carry = 1;
            }
            mlength = mlength - 1;
        }
        m = minusm;
        //println!("minus m is assigned to m: {:?}", m); // code works perfectly fine till here// Implementing Booth's Algorithm
    }

    //converting into negative if negative sign
    if secondsign == '-' {
        let mut minusq = vec![];
        let mut carry = 0;
        //To take minus of M
        // One's complement
        for i in &q {
            if i == &0 {
                minusq.push(1);
            } else {
                minusq.push(0);
            }
        }
        //println!("minus q inverted is: {:?}", minusq);
        let minqlength = minusq.len()-1;
        // Two's complement
        let binsum = minusq.last().unwrap() + 1;
        //println!("binsum: {}", binsum);
        if binsum < 2 {
            minusq[minqlength] = binsum;
            //println!("minus q update is: {:?}", minusq);
        } else {
            carry = 1;
            minusq[minqlength] = 0;
            //println!("minus q updated 2nd cond: {:?}", minusq);
        }

        // Two's Complement
        let mut  qlength:isize = (q.len()-2) as isize; // works as an index to add with carry
        while carry == 1 && qlength >= 0 { // OR 0
            let binsum = minusq[qlength as usize] + carry;
            //println!("binsum is: {}", binsum);
            if binsum < 2 {
                minusq[qlength as usize] = binsum;
                carry = 0;
            } else {
                minusq[qlength as usize] = 0;
                carry = 1;
            }
            qlength = qlength - 1;
        }
        q = minusq;
        //println!("minus q is assigned to q: {:?}", q); // code works perfectly fine till here// Implementing Booth's Algorithm
    }
    let now = Instant::now();
    let mut carry = 0;
    let mut minusm = vec![];
    //To take minus of M
    // One's complement
    for i in &m {
        if i == &0 {
            minusm.push(1);
        } else {
            minusm.push(0);
        }
    }
    //println!("minus m inverted is: {:?}", minusm);
    let minmlength = minusm.len()-1;
    // Two's complement
    //let mut binsum = 0;
    let binsum = minusm.last().unwrap() + 1;
    //println!("binsum: {}", binsum);
    if binsum < 2 {
        minusm[minmlength] = binsum;
        //println!("minus m update is: {:?}", minusm);
    } else {
        carry = 1;
        minusm[minmlength] = 0;
        //println!("minus m updated 2nd cond: {:?}", minusm);
    }

    // Two's Complement
    let mut  mlength:isize = (m.len()-2) as isize; // works as an index to add with carry
    while carry == 1 && mlength >= 0 { // OR 0
        let binsum = minusm[mlength as usize] + carry;
        //println!("binsum is: {}", binsum);
        if binsum < 2 {
            minusm[mlength as usize] = binsum;
            carry = 0;
        } else {
            minusm[mlength as usize] = 0;
            carry = 1;
        }
        mlength = mlength - 1;
    }

    println!("minus m is: {:?}", minusm); // code works perfectly fine till here// Implementing Booth's Algorithm
    let mut counter = m.len();
    let mut acc = vec![0; m.len()];
    let mut carry = 0;
    let mut qmone = 0;
    while counter != 0 {
        // q = [00011]
        if q.last().unwrap() == &1 && qmone == 0 { //if LSB of Q is 1 and Q-1 is0 then
            // then acc = acc - m
            //binary addition
             // ignore this comment please //for j in acc.iter().rev() { // in case if rust gives an error, just till the register bits size
            let mut j: isize  = (acc.len()-1) as isize;
            while j >= 0 {
                let binsum = acc[j as usize] + minusm[j as usize] + carry; //values at the respective index will be added
                if binsum < 2 {
                    acc[j as usize] = binsum;
                    //println!("10 acc update is: {:?}", acc);
                    carry = 0;
                } else if binsum == 2 {
                    carry = 1;
                    acc[j as usize] = 0;
                    //println!("10 elseif acc updated is: {:?}", acc);
                } else {
                    carry = 1;
                    acc[j as usize] = 1;
                    //println!("10 else acc is: {:?}", acc);
                }
                j = j - 1;
                if j == -1 {
                    carry = 0;
                }
                //println!("j is: {}", j);
            }
        } else if q.last().unwrap() == &0 && qmone == 1 { // if LSB of Q is 0 and Q-1 is 1 then...
            // acc = acc + m
            let mut k: isize = (acc.len()-1) as isize;
            while k >= 0 {
                let binsum = acc[k as usize] + m[k as usize] + carry;
                if binsum < 2 {
                    acc[k as usize] = binsum;
                    carry = 0;
                    //println!("01 acc update is: {:?}", acc);
                } else if binsum == 2 {
                    carry = 1;
                    acc[k as usize] = 0;
                    //println!("01 elseif acc is: {:?}", acc);
                } else {
                    carry = 1;
                    acc[k as usize] = 1;
                    //println!("01 else acc is: {:?}", acc);
                }
                k = k - 1;
                if k == -1 {
                    carry = 0;
                }
            }
        }
        //Arithmetic Shift Right
        let msbindex = 0;
        let lsbindex = acc.len()-1;
        qmone = q[lsbindex];
        //println!("q minus one is: {}", qmone);
        let mut pos: isize = (q.len()-2) as isize;
        while pos >= 0 {
            q[(pos+1) as usize] = q[pos as usize];
            //println!("q ASR update is: {:?}", q);
            pos = pos - 1;
        }

        q[msbindex] = acc[lsbindex];
        //println!("q MSB updated is: {:?}", q);
        let acmsb = acc[msbindex];
        //println!("ACMSB copied is: {:?}", acmsb);

        let mut position: isize = (acc.len()-2) as isize;
        while position >= 0 {
            acc[(position + 1) as usize] = acc[position as usize];
            //println!("acc ASR updated: {:?}", acc);
            position = position - 1;
        }
        acc[0] = acmsb;
        //println!("ACC MSR UPDATED IS: {:?}",acc);
        counter = counter - 1;
    }

    println!("acc: {:?}, and q is: {:?}", acc, q);

    //binary into denary/decimal conversion
    let two: usize = 2;
    let tlength = acc.len()+q.len();
    let mut number = tlength - 1;
    let msbvalue:isize = -((acc[0]) as isize*((two as isize).pow(number as u32)));
    let mut denaryvalue:isize = 0 as isize;
    denaryvalue = denaryvalue + msbvalue;


    for i in 1..acc.len() {
        number = number - 1;
        let value = (acc[i] as usize)*(two.pow(number as u32));
        //println!("value is: {}", value);
        denaryvalue = denaryvalue + value as isize;
        //println!("denary value is: {}", denaryvalue);
    }

    for i in 0..q.len() {
        number = number - 1;
        let value = (q[i] as usize)*(two.pow(number as u32));
        //println!("value q is: {}", value);
        denaryvalue = denaryvalue + value as isize;
        //println!("denary value is: {}", denaryvalue);
    }

    //Time calculation
    let nseconds = now.elapsed().subsec_nanos();
    println!("Time elapsed for Booth's algorithm for {} bits: {}",(q.len()), nseconds);
    println!("Bingo! the answer of {}{} into {}{} is: {}", firstsign, multiplicand, secondsign, multiplier, denaryvalue);
}



