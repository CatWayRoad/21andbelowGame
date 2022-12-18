use rand::{prelude::*, distributions::Standard};

#[allow(unused)]
#[derive(Clone, PartialEq, Debug)]
pub enum CardType{
    Clubs1, Clubs2, Clubs3, Clubs4, Clubs5, Clubs6, Clubs7, Clubs8, Clubs9, Clubs10, Clubs11, Clubs12, Clubs13,
    Dim1, Dim2, Dim3, Dim4, Dim5, Dim6, Dim7, Dim8, Dim9, Dim10, Dim11, Dim12, Dim13,
    Heart1, Heart2, Heart3, Heart4, Heart5, Heart6, Heart7, Heart8, Heart9, Heart10, Heart11, Heart12, Heart13, 
    Spade1, Spade2, Spade3, Spade4, Spade5, Spade6, Spade7, Spade8, Spade9, Spade10, Spade11, Spade12, Spade13
}

impl CardType { 

    pub fn get_card (card_num: u8) -> CardType {
        use CardType::*;

        match card_num{
            0 => Clubs1,
            1 => Clubs2,
            2 => Clubs3,
            3 => Clubs4,
            4 => Clubs5,
            5 => Clubs6,
            6 => Clubs7,
            7 => Clubs8,
            8 => Clubs9,
            9 => Clubs10,
            10 => Clubs11,
            11 => Clubs12,
            12 => Clubs13,
            13 => Dim1,
            14 => Dim2,
            15 => Dim3,
            16 => Dim4,
            17 => Dim5,
            18 => Dim6,
            19 => Dim7,
            20 => Dim8,
            21 => Dim9,
            22 => Dim10,
            23 => Dim11,
            24 => Dim12,
            25 => Dim13,
            26 => Heart1,
            27 => Heart2,
            28 => Heart3,
            29 => Heart4,
            30 => Heart5,
            31 => Heart6,
            32 => Heart7,
            33 => Heart8,
            34 => Heart9,
            35 => Heart10,
            36 => Heart11,
            37 => Heart12,
            38 => Heart13,
            39 => Spade1,
            40 => Spade2,
            41 => Spade3,
            42 => Spade4,
            43 => Spade5,
            44 => Spade6,
            45 => Spade7,
            46 => Spade8,
            47 => Spade9,
            48 => Spade10,
            49 => Spade11,
            50 => Spade12,
            51 => Spade13,
            _ => panic!("Card exceed 51 (52 on base 1)")
        }
        
    }
    
    pub fn get_deck () -> Vec<CardType>  {

        let mut  numbers: Vec<u8> = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51];
        let mut deck: Vec<CardType> = Vec::new();
        let  mut rand_num;

        while numbers.is_empty() == false { //When we did not fill all

            rand_num = rand::thread_rng().gen_range(0..numbers.len());

            //Push the given card (from the stock) to the deck
            deck.push(CardType::get_card(numbers.remove(rand_num)))
        }
        //We fill all
        //Return the deck
        deck
    }
}

pub struct Cards{
    stack: Vec<CardType>

}