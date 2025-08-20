use rand::Rng;



pub fn new_array(n:i32,min:i32,max:i32)->Vec<i32>{
    let mut arr=rand::thread_rng();
    (0..n).map(|_| arr.gen_range(min..=max)).collect()
}

pub fn slice(&arr:Vec<i32>)->Vec<i32>{
    #[derive(Debug, Clone, Copy)]
    struct configurazione{
        start:usize,
        best_len:usize,
        current_start:Option<usize>,
        current_len:usize,
    }

    let configurazione_finale=arr.iter().enumerate().fold(
        configurazione{
        start:0,
        best_len:0,
        current_start:None,
        current_len:0
        },
        |mut configurazione,(i,&elem)|{
            if elem>=0{
                if configurazione.current_start.is_none(){
                    configurazione.current_start=Some(i);
                    configurazione.current_len=1;
                }else{
                    configurazione.current_len+=1;
                }
                if configurazione.current_len>=configurazione.best_len{
                    configurazione.best_len=configurazione.current_len;
                    configurazione.start=configurazione.current_start.unwrap();
                }
            }else{
                configurazione.current_start=None;
                    configurazione.current_len=0;
                }
            }
            configurazione
        },
    );

    &arr[configurazione_finale.start..configurazione_finale.start+configurazione_finale.best_len]
}   
            


    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_array_and_slice() {
        
        let n = 10;
        let min = -3;
        let max = 3;
        let arr = new_array(n, min, max);
        assert_eq!(arr.len(), n as usize, "La lunghezza dell'array è errata");
        assert!(
            arr.iter().all(|&x| x >= min && x <= max),
            
        );

        
        let test_array = vec![-1, 2, 3, -2, 4, 5, 6, -1, 0];
        let result = slice(&test_array);
        assert_eq!(result, vec![4, 5, 6], "La sottosequenza più lunga di numeri >= 0 è errata");
    }