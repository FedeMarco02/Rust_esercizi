

#[derive(PartialEq,Debug)]use std::ops::Mul;
use std::ops::Add;
use std::ops::Mul;


pub struct Razionale {
    num:i32,
    denum:i32,
}

impl Razionale {
    
    fn minimi_termini(num:i32,denum:i32)->Razionale{
        let max_com_div:i32=Razionale::MCD(num,denum);
        let num_ridotto:i32=num/max_com_div;
        let denum_ridotto:i32=denum/max_com_div;
        Razionale{
            num:num_ridotto,
            denum:denum_ridotto,
        }
    }
        
    

    

    
    
    
    
    fn new_razionale(num:i32,denum:i32)->Self{
        if denum ==0{
            panic!("il denominatore deve essere diverso da 0");
        }
        let razionale = Razionale::minimi_termini(num,denum);
        Razionale{
            num:razionale.num,
            denum:razionale.denum,
        }
        
    }
  

    fn MCD (a:i32,b:i32)->i32{
        let mut a=a.abs();
        let mut b=b.abs();
        if b==0 {
            return a;
        }else{  
            while b!=0 {
                let temp=b;
                b=a%b;
                a=temp;
            }
        }
        return a;
    }

    fn somma(&self,altro_raz:&Razionale)->Razionale{
        let mcd_denum=self.denum*altro_raz.denum;
        let addendo1:i32=(mcd_denum/self.denum)*self.num;
        let addendo2:i32=(mcd_denum/altro_raz.denum)*altro_raz.num;
        let risultato:i32=addendo1+addendo2;
        let razionale = Razionale::minimi_termini(risultato, mcd_denum);
       Razionale{
        num:razionale.num,
        denum:razionale.denum,
       }
    }

    
    fn prodotto (&self, altro_raz:&Razionale)->Razionale{
        let prodotto_num:i32=self.num*altro_raz.num;
        let prodotto_denum:i32=self.denum*altro_raz.denum;
        let razionale = Razionale::minimi_termini(prodotto_num, prodotto_denum);
        
        Razionale {
            num:razionale.num,
            denum:razionale.denum,
        }
    }
    

    fn int_to_raz(n:i32)->Self{
        Razionale{
            num:n,
            denum:1,
        }
    }
}


impl Add for Razionale {
    type Output=Self;
    fn add(self,other:Self)->Self::Output{
        let mcd_denum=self.denum*other.denum;
        let addendo1:i32=(mcd_denum/self.denum)*self.num;
        let addendo2:i32=(mcd_denum/other.denum)*other.num;
        Razionale{
            num:addendo1+addendo2,
            denum:mcd_denum,
        }
    }
}
        
        
        
    


impl Add<i32> for Razionale{
    type Output=Self;
    fn add(self,other:i32)->Self::Output{
        let risultato:i32=self.num+(self.denum*other);
        Razionale {
            num:risultato,
            denum:self.denum,
        }
    }
}

impl Mul for Razionale{
    type Output=Self;
    fn mul(self,other:Self)->Self::Output{
        let prodotto_num:i32=self.num*other.num;
        let prodotto_denum:i32=self.denum*other.denum;
        let razionale = Razionale::minimi_termini(prodotto_num, prodotto_denum);
        Razionale{
            num:razionale.num,
            denum:razionale.denum,
        }
    }
}

impl Mul<i32> for Razionale{
    type Output=Self;
    fn mul (self,other:i32)->Self::Output{
        let prodotto_num:i32=self.num*other;
        let razionale=Razionale::minimi_termini(prodotto_num,self.denum);
        Razionale{
            num:razionale.num,
            denum:razionale.denum,
        }
    }
}


fn main() {
    
    let raz1 = Razionale::new_razionale(5, 10);
    assert_eq!(raz1.num, 1);  
    assert_eq!(raz1.denum, 2);

    
    let raz2 = Razionale::new_razionale(3, 4);
    assert_eq!(raz2.num, 3);
    assert_eq!(raz2.denum, 4);

    
    let raz_somma = raz1.somma(&raz2);
    assert_eq!(raz_somma.num, 11);  
    assert_eq!(raz_somma.denum, 4); 

    
    let raz_prodotto = raz1.prodotto(&raz2);
    assert_eq!(raz_prodotto.num, 3);  
    assert_eq!(raz_prodotto.denum, 8);

    
    let raz_intero = Razionale::int_to_raz(7);
    assert_eq!(raz_intero.num, 7);  
    assert_eq!(raz_intero.denum, 1);

    
    let mcd_test = Razionale::MCD(48, 180);
    assert_eq!(mcd_test, 12); 

    let raz3 = Razionale::new_razionale(1, 2);
    let raz4 = Razionale::new_razionale(2, 3);
    let raz_somma_trait = raz3 + raz4; 
    assert_eq!(raz_somma_trait.num, 7);  
    assert_eq!(raz_somma_trait.denum, 6);

    
    let raz5 = Razionale::new_razionale(1, 2);
    let intero = 2;
    let raz_somma_intero = raz5 + intero; 
    assert_eq!(raz_somma_intero.num, 5); 
    assert_eq!(raz_somma_intero.denum, 2);

    
    let raz6 = Razionale::new_razionale(2, 3);
    let raz7 = Razionale::new_razionale(4, 5);
    let raz_prodotto_trait = raz6 * raz7; 
    assert_eq!(raz_prodotto_trait.num, 8);  
    assert_eq!(raz_prodotto_trait.denum, 15);

    
    let raz8 = Razionale::new_razionale(3, 4);
    let intero2 = 3;
    let raz_prodotto_intero = raz8 * intero2; 
    assert_eq!(raz_prodotto_intero.num, 9);  
    assert_eq!(raz_prodotto_intero.denum, 4);

    
}       

    


