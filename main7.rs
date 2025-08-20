use std::sync::{Arc, Mutex, mpsc::{self, Sender, Receiver}};
use std::thread;

#[derive(Debug, Clone)]
struct Prodotto {
    nome: String,
    prezzo_partenza: u32,
    prezzo_riserva: u32,
}

#[derive(Debug)]
enum MessaggioPartecipante {
    Offerta(u32),
    Abbandona,
}

#[derive(Debug)]
enum MessaggioBanditore {
    InfoProdotto(Prodotto),
    NuovaOfferta(u32),
}

#[derive(Debug, Clone)]
struct RisultatoAsta {
    nome_prodotto: String,
    prezzo_vincente: Option<u32>,
    id_vincitore: Option<usize>,
}

pub fn esegui_asta_inglese(numero_partecipanti: usize) -> RisultatoAsta {
    let prodotto = Prodotto {
        nome: "Vaso Antico".to_string(),
        prezzo_partenza: 100,
        prezzo_riserva: 150,
    };

    let risultato = Arc::new(Mutex::new(RisultatoAsta {
        nome_prodotto: prodotto.nome.clone(),
        prezzo_vincente: None,
        id_vincitore: None,
    }));

    let (tx_banditore, rx_banditore): (Sender<(usize, MessaggioPartecipante)>, Receiver<(usize, MessaggioPartecipante)>) = mpsc::channel();
    let mut tx_ai_partecipanti = Vec::new();

    for id in 0..numero_partecipanti {
        let tx_banditore_clonato = tx_banditore.clone();
        let (tx_banditore_partecipante, rx_partecipante): (Sender<MessaggioBanditore>, Receiver<MessaggioBanditore>) = mpsc::channel();
        tx_ai_partecipanti.push(tx_banditore_partecipante);

        thread::spawn(move || {
            let mut prezzo_corrente = prodotto.prezzo_partenza;
            loop {
                if let Ok(msg) = rx_partecipante.recv() {
                    match msg {
                        MessaggioBanditore::InfoProdotto(p) => {
                            if id % 2 == 0 {
                                tx_banditore_clonato.send((id, MessaggioPartecipante::Offerta(p.prezzo_partenza))).unwrap();
                            } else {
                                tx_banditore_clonato.send((id, MessaggioPartecipante::Abbandona)).unwrap();
                                break;
                            }
                        }
                        MessaggioBanditore::NuovaOfferta(prezzo) => {
                            prezzo_corrente = prezzo + 10;
                            if prezzo_corrente < 180 {
                                tx_banditore_clonato.send((id, MessaggioPartecipante::Offerta(prezzo_corrente))).unwrap();
                            } else {
                                tx_banditore_clonato.send((id, MessaggioPartecipante::Abbandona)).unwrap();
                                break;
                            }
                        }
                    }
                } else {
                    break;
                }
            }
        });
    }

    let risultato_clonato = Arc::clone(&risultato);
    thread::spawn(move || {
        let mut partecipanti_attivi: Vec<usize> = (0..numero_partecipanti).collect();
        let mut prezzo_corrente = prodotto.prezzo_partenza;
        let mut miglior_offerente = None;

        for tx in &tx_ai_partecipanti {
            tx.send(MessaggioBanditore::InfoProdotto(prodotto.clone())).unwrap();
        }

        while !partecipanti_attivi.is_empty() {
            if let Ok((id, messaggio)) = rx_banditore.recv() {
                match messaggio {
                    MessaggioPartecipante::Abbandona => {
                        partecipanti_attivi.retain(|&x| x != id);
                    }
                    MessaggioPartecipante::Offerta(offerta) => {
                        if offerta > prezzo_corrente {
                            prezzo_corrente = offerta;
                            miglior_offerente = Some(id);
                            for pid in &partecipanti_attivi {
                                tx_ai_partecipanti[*pid].send(MessaggioBanditore::NuovaOfferta(prezzo_corrente)).unwrap();
                            }
                        }
                    }
                }
            }
        }

        let mut res = risultato_clonato.lock().unwrap();
        if prezzo_corrente >= prodotto.prezzo_riserva {
            res.prezzo_vincente = Some(prezzo_corrente);
            res.id_vincitore = miglior_offerente;
        }
    }).join().unwrap();

    let risultato_finale = risultato.lock().unwrap().clone();
    risultato_finale
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risultato_asta() {
        let risultato = esegui_asta_inglese(4);
        println!("Risultato Asta: {:?}", risultato);

        assert_eq!(risultato.nome_prodotto, "Vaso Antico");
        assert!(risultato.prezzo_vincente.is_some() || risultato.id_vincitore.is_none());
    }
}
