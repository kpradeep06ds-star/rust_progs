pub fn count(num: u32) -> (u32, Vec<u32>){
    let mut count: u32 = 1;
    let mut newnum : u32 = num;
    
    let mut v:Vec<u32> = Vec::new();

    if newnum < 10{
        v.push(newnum);
        return (count, v);
    } else {
        
        while newnum >= 10{
            let modval = newnum % 10;
            v.push(modval);
            count += 1;
            newnum /= 10 ;
        }
        v.push(newnum);
    }
    (count, v)
}

pub fn is_armstrong_number(num: u32) -> bool {
    let tup = count(num);
    let mut sums = 0;

    for i in tup.1{
        sums += i.pow(tup.0);
    }
    sums == num

}
