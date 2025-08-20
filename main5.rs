
use std::io;
use std::str::FromStr;
pub type LinkedList<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub next: LinkedList<T>,
    pub data: T,
}

impl<T> Node<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    pub fn new(data: T) -> Self {
        Node { data, next: None }
    }

    pub fn lunghezza_lista(&self) -> usize {
        let mut count: usize = 1;
        let mut current = &self.next;

        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        println!("La lunghezza della lista è: {}", count);
        count
    }

    pub fn inserisci_elemento(&mut self) {
        let lunghezza = self.lunghezza_lista();

        let posizione = loop {
            let mut input = String::new();
            println!(
                "Inserisci la posizione in cui vuoi inserire il nuovo elemento (1-{}): ",
                lunghezza
            );
            io::stdin().read_line(&mut input).expect("Errore input");

            if let Ok(pos) = input.trim().parse::<usize>() {
                if pos >= 1 && pos <= lunghezza {
                    break pos;
                } else {
                    println!("Posizione fuori dai limiti");
                }
            } else {
                println!("Inserisci un numero valido");
            }
        };

        let valore = loop {
            let mut input = String::new();
            println!("Inserisci il valore che vuoi inserire:");
            io::stdin().read_line(&mut input).expect("Errore input");

            if let Ok(valore) = input.trim().parse::<T>() {
                break valore;
            } else {
                println!("Il valore non è valido");
            }
        };

        let mut current = self;
        for _ in 1..posizione {
            if let Some(ref mut next_node) = current.next {
                current = next_node;
            }
        }
        let vecchio_next = current.next.take();
        let nuovo_next = Box::new(Node {
            data: valore,
            next: vecchio_next,
        });
        current.next = Some(nuovo_next);
        println!("Elemento inserito !");
    }

    pub fn rimuovi_elemento(&mut self) {
        let lunghezza = self.lunghezza_lista();

        let posizione = loop {
            let mut input = String::new();
            println!(
                "Inserisci la posizione da cui vuoi rimuovere l'elemento (2-{}): ",
                lunghezza
            );
            io::stdin().read_line(&mut input).expect("Errore input");

            if let Ok(pos) = input.trim().parse::<usize>() {
                if pos >= 2 && pos <= lunghezza {
                    break pos;
                } else {
                    println!("Posizione fuori dai limiti");
                }
            } else {
                println!("Inserisci un numero valido");
            }
        };

        let mut current = self;
        for _ in 1..(posizione - 1) {
            if let Some(ref mut next_node) = current.next {
                current = next_node;
            }
        }
        if let Some(ref mut to_delete) = current.next {
            current.next = to_delete.next.take();
            println!("Elemento rimosso");
        } else {
            println!("Errore nella rimozione");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut lista = Node::new(10);

        assert_eq!(lista.lunghezza_lista(), 1);
        println!(" Test lunghezza iniziale superato.");

        println!(" Ora test manuale di inserimento:");
        lista.inserisci_elemento();
        println!(" Lista dopo inserimento. Lunghezza attuale: {}", lista.lunghezza_lista());

        println!(" Ora test manuale di rimozione:");
        lista.rimuovi_elemento();
        println!(" Lista dopo rimozione. Lunghezza attuale: {}", lista.lunghezza_lista());

        println!(" Fine test manuali.");
    }
}
