

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimi_termini() {
        let raz1 = Razionale::new_razionale(5, 10);
        assert_eq!(raz1.num, 1);
        assert_eq!(raz1.denum, 2);
    }

    #[test]
    fn test_somma() {
        let raz1 = Razionale::new_razionale(5, 10);
        let raz2 = Razionale::new_razionale(3, 4);
        let raz_somma = raz1.somma(&raz2);
        assert_eq!(raz_somma.num, 11);
        assert_eq!(raz_somma.denum, 4);
    }

    #[test]
    fn test_prodotto() {
        let raz1 = Razionale::new_razionale(5, 10);
        let raz2 = Razionale::new_razionale(3, 4);
        let raz_prodotto = raz1.prodotto(&raz2);
        assert_eq!(raz_prodotto.num, 3);
        assert_eq!(raz_prodotto.denum, 8);
    }

    #[test]
    fn test_int_to_raz() {
        let raz_intero = Razionale::int_to_raz(7);
        assert_eq!(raz_intero.num, 7);
        assert_eq!(raz_intero.denum, 1);
    }

    #[test]
    fn test_mcd() {
        let mcd_test = Razionale::MCD(48, 180);
        assert_eq!(mcd_test, 12);
    }
}
