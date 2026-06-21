#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    // they have not provided if I should keep this blank or not
    // but I need to see this that it can't be blank - why?
    // because - I need variables which can keep track of pins down and current frame
    // I have 10 pins - with 2 frames (at max, frames are fancy names of turn)
    // So I need two variables -> one for keeping pins and accumulation 
    // andother to track the frames -> this way I can not allow incorrect number of pins

    rolls: Vec<u16>,
    current_frame: u16,
    turn: u16,
    max_current_frame: u16,
    // no need for mut here, rust doesn't work that way, I can provide mut once I call this 
}

impl BowlingGame {
    // what did I learn 
    // A struct can be empty and still valid;
    pub fn new() -> Self {
        Self{
            rolls : vec![],
            current_frame : 1,
            turn: 1,
            max_current_frame: 1, // only initalised at 10th frame
        }
    }

    

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {

        if pins > 10{
            return Err(Error::NotEnoughPinsLeft) ;
        }
        
        
        // if current frame is 11th -> here are the possibilities
        // at 10th frame 1st chance someone hit 10 (then two more throws are allowed)
        // 10th frame - 2nd throw and 11th frame - 1st throw (max limit)
        // at 10th frame 2nd chance someone hit 10 (1st chance was 0)
        // 11th frame 1st throw and 2nd throw
        // at 10th frame 1st + 2nd chance some hit 10 
        // 11th frame 1st throw and 2nd throw
        // Now there is no 11th frame so above should be wrapped inside 10th frame itself
        if self.rolls.is_empty(){
                if pins != 10{
                    self.rolls.push(pins);
                    self.turn += 1;
                } else {
                    self.rolls.push(pins);
                    self.turn = 1;
                    self.current_frame = 2;
                }
                return Ok(());
            } 
        let templast = self.rolls.last().unwrap();
        if self.current_frame < 10 {
                // valid pushes:
                // if total in current frame must not be greater than 10 or first frame iteself is 10
                //let templast = self.rolls.last().unwrap();
            
            if self.turn == 1 && pins == 10{
                self.turn = 1;
                self.current_frame += 1;
                self.rolls.push(pins);
                //self.rolls.push(0); // adding extra zero to make even cells for each frame
                return Ok(());
            } else if self.turn == 1 && pins < 10{
                self.turn += 1;
                self.rolls.push(pins);
                return Ok(());
            } else if self.turn == 2 && pins + templast > 10{
                return Err(Error::NotEnoughPinsLeft);
            } else if self.turn == 2 && pins + templast <= 10 {
                self.rolls.push(pins);
                self.turn = 1;
                self.current_frame += 1;
                return Ok(());
            } else {
                return Ok(());
            }

        } else {
            // 1st hit is 10 -> give two more attempts then after 2 more attempts game over
            // 1st  hit + 2nd hit is 10 -> then give one more attempt and then game over
            // 1st hit + 2nd hit < 10 , game over
            // if both hits combined > 10 error
            // Throw1 is always allowed
            // Throw2 is always allowed
            // Throw3 is possible iff throw1 is 10 or throw1 + throw2 = 10

            if self.max_current_frame == 1 {
                self.rolls.push(pins);
                self.max_current_frame += 1;
            } else if self.max_current_frame == 2{
                let tempval = self.rolls.last().unwrap();
                if *tempval == 10{
                    self.rolls.push(pins);
                    self.max_current_frame += 1;
                    return Ok(());
                }
                if pins + tempval < 10{
                    self.rolls.push(pins);
                    return Ok(());
                } else if pins + tempval == 10{
                    self.max_current_frame += 1;
                    self.rolls.push(pins);
                    return Ok(());
                } else {
                    return Err(Error::NotEnoughPinsLeft);
                }
            } else if self.max_current_frame == 3{
                let throw_2 = self.rolls.last().unwrap();
                let throw_1 = self.rolls[self.rolls.len()-2];
                if throw_1== 10  && *throw_2 < 10 && pins + throw_2 > 10{
                    return Err(Error::NotEnoughPinsLeft);
                } else {
                    self.rolls.push(pins);
                    return Ok(());
                }
            } 

        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
    let mut total = 0;
    let mut roll_index = 0;
    let mut last_required_index = 0;
    
    // this is the biggest joke in my understanding - 10 iterations but the vector has more values !!!
    // why? - each of the 9 frames have 2 games at max so, 9x2 =  18, but to understand the bonus 
    // we have to have 1 extra game = so 10th game has 3 turns = 18 + 3 = 21
    // this was the understanding which made me confused 

    for _frame in 0..10 {
        let first = *self.rolls.get(roll_index)?;
        // find the values
        if first == 10 {
            // edge case = 10 then just add the next two with 10
            let bonus1 = *self.rolls.get(roll_index + 1)?;
            let bonus2 = *self.rolls.get(roll_index + 2)?;

            total += 10 + bonus1 + bonus2;
            last_required_index = roll_index + 2;
            roll_index += 1;
        } else {
            // not edge case: then just keep adding
            // if keep adding two gives larger than 10, then None should be returned
            // seems like this line is duplicate as I have implemented this in the roll function as welll
            // this I need to think through !!!
            // but I need this as the output must be an Option type -> 
            let second = *self.rolls.get(roll_index + 1)?;

            if first + second > 10 {
                return None;
            }
            // if exactly 10 then add the next value as bonus
            if first + second == 10 {

                let bonus = *self.rolls.get(roll_index + 2)?;

                total += 10 + bonus;
                last_required_index = roll_index + 2;
            } else {
                // if not 10 then simply add them to total
                total += first + second;
                last_required_index = roll_index + 1;
            }

            roll_index += 2;
        }
    }
    // this is my safeguard against the index error 
    if self.rolls.len() == last_required_index + 1 {
        Some(total)
    } else {
        None
    }
}
}
