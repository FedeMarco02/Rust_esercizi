se std::sync::{Arc, Mutex, mpsc::{self, Sender, Receiver}};//Arc lo usiamo per condividere un dato tra più thread
//Mutex permette di modificare un dato condiviso tra più thread in sicurezza
//mpsc invece permette la comunicazione tra i thread: sender per inviare messaggi e receiver per riceverli.
use std::thread;

//Questa struttura definisce l'oggetto messo all'asta:debug e clone permettono di stamparlo e clonarlo facilmente
#[derive(Debug, Clone)]
struct Prodotto {
    nome: String,
    prezzo_partenza: u32,
    prezzo_riserva: u32,
}
//Questo enumerato definisce i messaggi che i partecipanti inviano al banditore(o fanno un offerta o abbandonano)
#[derive(Debug)]
enum MessaggioPartecipante {
    Offerta(u32),
    Abbandona,
}
//Questo enumerato definisce i messaggi che il banditore invia ai partecipanti(descrive il prodotto e annuncia una nuova offerta da rilanciare o abbandonare)
#[derive(Debug)]
enum MessaggioBanditore {
    InfoProdotto(Prodotto),
    NuovaOfferta(u32),
}
//Queesta struttura definisce il risultato dell'asta, quindi il prodotto, il prezzo di vendita e l'ID del partecipante vincente
#[derive(Debug, Clone)]
struct RisultatoAsta {
    nome_prodotto: String,
    prezzo_vincente: Option<u32>,
    id_vincitore: Option<usize>,
}
//Qui definiamo la funzione con parametro numero partecipanti 
pub fn esegui_asta_inglese(numero_partecipanti: usize) -> RisultatoAsta {
    let prodotto = Prodotto {
        nome: "Vaso Antico".to_string(),
        prezzo_partenza: 100,
        prezzo_riserva: 150,
    };
//la struttura risultato viene racchiusa in Arc<<Mutex< così più threads possono modificarla in sicurezza
    let risultato = Arc::new(Mutex::new(RisultatoAsta {
        nome_prodotto: prodotto.nome.clone(),
        prezzo_vincente: None,
        id_vincitore: None,
    }));
//Qui ora creiamo il canale di comunicazione da partecipante a banditore.Tx_banditore è il canale per mandare messaggi al banditore. Tx_ai_partecipanti è il vettore di sender per comuicare dal banditore ai partecipanti
    let (tx_banditore, rx_banditore): (Sender<(usize, MessaggioPartecipante)>, Receiver<(usize, MessaggioPartecipante)>) = mpsc::channel();
    let mut tx_ai_partecipanti = Vec::new();
//Qui creiamo il canale di comunicazione da banditore a partecipante. Cloniamo il canale creato in precedenza e infine aggiungiamo il canale da banditore a partecipante alla lista che il banditore userà(con push)
    for id in 0..numero_partecipanti {
        let tx_banditore_clonato = tx_banditore.clone();
        let (tx_banditore_partecipante, rx_partecipante): (Sender<MessaggioBanditore>, Receiver<MessaggioBanditore>) = mpsc::channel();
        tx_ai_partecipanti.push(tx_banditore_partecipante);
//Qui definiamo la logica di ogni partecipante:quando un partecipante riceve un messaggio,si sceglie in maniera randomica tra true(fa un'offerta) e false(abbandona l'asta)
        thread::spawn(move || {
            let mut prezzo_corrente = prodotto.prezzo_partenza;
            loop {
                if let Ok(msg) = rx_partecipante.recv() {
                    match msg {
                        MessaggioBanditore::InfoProdotto(p) => {
                            let scelta:bool=rand::random();
                            if scelta{
                                tx_banditore_clonato.send((id,MessaggioPartecipante::Offerta(p.prezzo_partenza))).unwrap();
                            } else{
                                tx_banditore_clonato.send((id,MessaggioPartecipante::Abbandona)).unwrap();
                                break;
                            }
                        }//Se scelta è false allora esce dal loop, sennò propone una nuova offerta

                        //Qui il banditore invia una nuova offerta dopo aver ricevuto il messaggio di offerta del partecipante.Anche qui randomizza la scelta:se true, rilancia di +10 sennò abbandona l'asta. 
                        MessaggioBanditore::NuovaOfferta(prezzo) => {
                            let scelta:bool=rand::random();
                            if scelta {
                                prezzo_corrente=prezzo+10;
                                tx_banditore_clonato.send((id,MessaggioPartecipante::Offerta(prezzo_corrente))).unwrap();
                            }else{
                                tx_banditore_clonato.send((id,MessaggioPartecipante::Abbandona)).unwrap();
                                break;
                            }
                        }
                    }
                } else {
                    break;//Questo break lo abbiamo nel caso succedesse una rottura del canale e quindi esso viene chiuso rompendo il loop
                }
            }
        });
    }
//Questa invece è la logica del banditore:clona il risultato condiviso, tiene una lista dei partecipanti attivi, del prezzo corrente e del miglior offerente
    let risultato_clonato = Arc::clone(&risultato);
    thread::spawn(move || {
        let mut partecipanti_attivi: Vec<usize> = (0..numero_partecipanti).collect();
        let mut prezzo_corrente = prodotto.prezzo_partenza;
        let mut miglior_offerente = None;
//Qui avviene l'invio iniziale delle info sul prodotto a tutti i partecipanti
        for tx in &tx_ai_partecipanti {
            tx.send(MessaggioBanditore::InfoProdotto(prodotto.clone())).unwrap();
        }
//Qui c'è il ciclo in cui riceve i messaggi: se riceve abbandona, elimina il partecipante da quelli attivi.
        while !partecipanti_attivi.is_empty() {
            if let Ok((id, messaggio)) = rx_banditore.recv() {
                match messaggio {
                    MessaggioPartecipante::Abbandona => {
                        partecipanti_attivi.retain(|&x| x != id);
                    }
                    //Qui invece è il caso in cui riceve un'offerta: se il prezzo è > di prezzo_corrente,aggiorna prezzo_corrente all'offerta ricevuta e miglior offerente diventa il partcipante dell'offerta
                    MessaggioPartecipante::Offerta(offerta) => {
                        if offerta > prezzo_corrente {
                            prezzo_corrente = offerta;
                            miglior_offerente = Some(id);
                            //Qui invece aggiorna tutti i partecipanti attivi della nuova offerta ricevuta e quindi del nuvo prezzo_corrente
                            for pid in &partecipanti_attivi {
                                tx_ai_partecipanti[*pid].send(MessaggioBanditore::NuovaOfferta(prezzo_corrente)).unwrap();
                            }
                        }
                    }
                }
            }
        }
//Quando non restano più partecipanti attivi l'asta viene conclusa e se il prezzo corrente è maggiore di quello di riserva allora viene salvato il vincitore e il prezzo
        let mut res = risultato_clonato.lock().unwrap();
        if prezzo_corrente >= prodotto.prezzo_riserva {
            res.prezzo_vincente = Some(prezzo_corrente);
            res.id_vincitore = miglior_offerente;
        }
    }).join().unwrap();//Qui aspetta la fine del thread Banditore

    let risultato_finale = risultato.lock().unwrap().clone();//Qui legge il risultato dal mutex e poi lo ritorna come risultato finale 
    risultato_finale
}



#[test]
fn test_concurrent_message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Test message").unwrap();
    });

    thread::sleep(Duration::from_millis(100));
    assert_eq!(rx.recv().unwrap(), "Test message");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_asta_basic_comunicazione() {
        
        let risultato = esegui_asta_inglese(2);        
        assert_eq!(risultato.nome_prodotto, "Vaso Antico");
        println!("Risultato asta: {:?}", risultato);
    }

    #[test]
    fn test_participanti_abbandonano_subito() {
        let risultato = esegui_asta_inglese(1);
        assert!(risultato.prezzo_vincente.is_none() || risultato.id_vincitore.is_some());
    }
}
