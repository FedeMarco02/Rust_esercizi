#[allow(dead_code)]
use std::cell::RefCell;
pub struct ContoBancario{
    nome_cliente:String,
    saldo:RefCell<f64>,
    limite_inferiore:f64,
    limite_superiore:f64,
    interesse:f64,
    stato:RefCell <Box<dyn StatoConto>>,
}

pub trait StatoConto{
    fn deposita(self:Box<Self>,conto:&mut ContoBancario,importo:f64)->Box<dyn StatoConto>;
    fn preleva(self:Box<Self>,conto:&mut ContoBancario,importo:f64)->Box<dyn StatoConto>;
    fn paga_interessi(self:Box<Self>,conto:&mut ContoBancario,importo:f64)->Box<dyn StatoConto>;
    fn nome(&self) -> &'static str;
}
pub struct Rosso;
pub struct Argento;
pub struct Oro;
impl StatoConto for Rosso{
    fn deposita (self:Box<Self>,conto:&mut ContoBancario,importo:f64)->Box<dyn StatoConto>{
        let mut saldo=conto.saldo.borrow_mut();
        *saldo+=importo;
        let nuovo_saldo=*saldo;
        
        
        
        if nuovo_saldo < conto.limite_inferiore {
           //return Box::new(Rosso);
           conto.stato=Box::new(Rosso);
        return Box::new(Rosso);}
            else if nuovo_saldo > conto.limite_inferiore && nuovo_saldo < conto.limite_superiore{
                 return Box::new(Argento);
            }
            else {
                 return Box::new(Oro);
            }
            
    }
    

    fn preleva (self:Box<Self>,conto:&mut ContoBancario,_importo:f64)->Box<dyn StatoConto>{
        *conto.stato.borrow_mut()=Box::new(Rosso);
        self
    }
    
        
        
           
           
              
        
    

    fn paga_interessi(self:Box<Self>,conto:&mut ContoBancario,_importo:f64)->Box<dyn StatoConto>{
        
        *conto.stato.borrow_mut()=Box::new(Rosso);
        self
    }

    
    fn nome(&self) -> &'static str {
         "Rosso"
    }
}


    

    
            
        
    

    impl StatoConto for Argento{
        fn deposita (self:Box<Self>,conto:&mut ContoBancario,importo:f64)->Box<dyn StatoConto>{
            let mut saldo=conto.saldo.borrow_mut();
            *saldo+=importo;
           let nuovo_saldo=*saldo;
        
        
        if nuovo_saldo < conto.limite_inferiore {
            return Box::new(Rosso);}
            else if nuovo_saldo > conto.limite_inferiore && nuovo_saldo < conto.limite_superiore{
                 return Box::new(Argento);
            }
            else {
                 return Box::new(Oro);
            }
            
    }
            
             
            
        
    
        fn preleva (self:Box<Self>,conto:&mut ContoBancario,importo:f64)->Box<dyn StatoConto>{
            let mut saldo=conto.saldo.borrow_mut(); 
            *saldo-=importo;
           let nuovo_saldo=*saldo;
        
        
        if nuovo_saldo < conto.limite_inferiore {
            return Box::new(Rosso);}
            else if nuovo_saldo > conto.limite_inferiore && nuovo_saldo < conto.limite_superiore{
                 return Box::new(Argento);
            }
            else {
                 return Box::new(Oro);
            }
            
    }
    
        fn paga_interessi(self:Box<Self>,conto:&mut ContoBancario,_importo:f64)->Box<dyn StatoConto>{
            *conto.stato.borrow_mut()=Box::new(Argento);
            self
        }

        fn nome(&self) -> &'static str {
        "Argento"
        }
    }

        
    
            
        
    

    impl StatoConto for Oro{
        fn deposita (self:Box<Self>,conto:&mut ContoBancario,importo:f64)->Box<dyn StatoConto>{
            let mut saldo=conto.saldo.borrow_mut();
            *saldo+=importo;
            self
            
        }
    
        fn preleva (self:Box<Self>,conto:&mut ContoBancario,importo:f64)->Box<dyn StatoConto>{
            let mut saldo=conto.saldo.borrow_mut(); 
            *saldo-=importo;
           let nuovo_saldo=*saldo;
        
        
        if nuovo_saldo < conto.limite_inferiore {
            return Box::new(Rosso);}
            else if nuovo_saldo > conto.limite_inferiore && nuovo_saldo < conto.limite_superiore{
                 return Box::new(Argento);
            }
            else {
                 return Box::new(Oro);
            }
            
    }
        
    
        fn paga_interessi(self:Box<Self>,conto:&mut ContoBancario,importo:f64)->Box<dyn StatoConto>{
            let mut saldo=conto.saldo.borrow_mut();
            *saldo+=importo;
            self
        }

         fn nome(&self) -> &'static str {
        "Oro"
         }
            
            
    }
    #[cfg(test)]
mod tests {
    use super::*;
    
    fn conto_di_test() -> ContoBancario {
        ContoBancario {
            nome_cliente: "Mario Rossi".to_string(),
            saldo: RefCell::new(0.0),
            limite_inferiore: 100.0,
            limite_superiore: 1000.0,
            interesse: 0.05,
            stato: RefCell::new(Box::new(Rosso)),
        }
    }

    #[test]
    fn test_rosso_deposita() {
        let mut conto = conto_di_test();
        *conto.saldo.borrow_mut() = 50.0;
        *conto.stato.borrow_mut() = Box::new(Rosso);

        let nuovo_stato = conto.stato.replace(Box::new(Rosso)).deposita(&mut conto, 60.0);
        //*conto.stato.borrow_mut() = nuovo_stato;

        assert_eq!(conto.stato.borrow().nome(), "Argento");
    }

    #[test]
    fn test_rosso_preleva() {
        let mut conto = conto_di_test();
        *conto.saldo.borrow_mut() = 50.0;
        *conto.stato.borrow_mut() = Box::new(Rosso);

        let nuovo_stato = conto.stato.replace(Box::new(Rosso)).preleva(&mut conto, 10.0);
        *conto.stato.borrow_mut() = nuovo_stato;

        assert_eq!(conto.stato.borrow().nome(), "Rosso");
    }

    #[test]
    fn test_rosso_paga_interessi() {
        let mut conto = conto_di_test();
        *conto.saldo.borrow_mut() = 50.0;
        *conto.stato.borrow_mut() = Box::new(Rosso);

        let nuovo_stato = conto.stato.replace(Box::new(Rosso)).paga_interessi(&mut conto, 5.0);
        *conto.stato.borrow_mut() = nuovo_stato;

        assert_eq!(conto.stato.borrow().nome(), "Rosso");
    }

    #[test]
    fn test_argento_deposita() {
        let mut conto = conto_di_test();
        *conto.saldo.borrow_mut() = 200.0;
        *conto.stato.borrow_mut() = Box::new(Argento);

        let nuovo_stato = conto.stato.replace(Box::new(Argento)).deposita(&mut conto, 900.0);
        *conto.stato.borrow_mut() = nuovo_stato;

        assert_eq!(conto.stato.borrow().nome(), "Oro");
    }

    #[test]
    fn test_argento_preleva() {
        let mut conto = conto_di_test();
        *conto.saldo.borrow_mut() = 500.0;
        *conto.stato.borrow_mut() = Box::new(Argento);

        let nuovo_stato = conto.stato.replace(Box::new(Argento)).preleva(&mut conto, 450.0);
        *conto.stato.borrow_mut() = nuovo_stato;

        assert_eq!(conto.stato.borrow().nome(), "Rosso");
    }

    #[test]
    fn test_argento_paga_interessi() {
        let mut conto = conto_di_test();
        *conto.saldo.borrow_mut() = 500.0;
        *conto.stato.borrow_mut() = Box::new(Argento);

        let nuovo_stato = conto.stato.replace(Box::new(Argento)).paga_interessi(&mut conto, 25.0);
        *conto.stato.borrow_mut() = nuovo_stato;

        assert_eq!(conto.stato.borrow().nome(), "Argento");
    }

    #[test]
    fn test_oro_deposita() {
        let mut conto = conto_di_test();
        *conto.saldo.borrow_mut() = 1500.0;
        *conto.stato.borrow_mut() = Box::new(Oro);

        let nuovo_stato = conto.stato.replace(Box::new(Oro)).deposita(&mut conto, 100.0);
        *conto.stato.borrow_mut() = nuovo_stato;

        assert_eq!(conto.stato.borrow().nome(), "Oro");
    }

    #[test]
    fn test_oro_preleva() {
        let mut conto = conto_di_test();
        *conto.saldo.borrow_mut() = 1200.0;
        *conto.stato.borrow_mut() = Box::new(Oro);

        let nuovo_stato = conto.stato.replace(Box::new(Oro)).preleva(&mut conto, 1150.0);
        *conto.stato.borrow_mut() = nuovo_stato;

        assert_eq!(conto.stato.borrow().nome(), "Rosso");
    }

    #[test]
    fn test_oro_paga_interessi() {
        let mut conto = conto_di_test();
        *conto.saldo.borrow_mut() = 1000.0;
        *conto.stato.borrow_mut() = Box::new(Oro);

        let nuovo_stato = conto.stato.replace(Box::new(Oro)).paga_interessi(&mut conto, 100.0);
        *conto.stato.borrow_mut() = nuovo_stato;

        assert_eq!(conto.stato.borrow().nome(), "Oro");
    }
}

        