use rand::prelude::SliceRandom;
use std::fmt;
use rand::Rng;
#[warn(unused_mut)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direzione {
    Su,
    Giu,
    Sinistra,
    Destra,
}

impl Direzione {
    fn opposta(&self) -> Direzione {
        match self {
            Direzione::Su => Direzione::Giu,
            Direzione::Giu => Direzione::Su,
            Direzione::Sinistra => Direzione::Destra,
            Direzione::Destra => Direzione::Sinistra,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ContenutoCella {
    Vuoto,
    Cibo(i32),
    Veleno(i32),
    Muro,
}

struct Campo {
    n: usize,
    matrice: Vec<Vec<ContenutoCella>>,
}

impl Campo {
    fn nuovo(n: usize, m: usize) -> Campo {
        let mut matrice = vec![vec![ContenutoCella::Vuoto; n]; n];

        
        for i in 0..n {
            matrice[0][i] = ContenutoCella::Muro;
            matrice[n - 1][i] = ContenutoCella::Muro;
            matrice[i][0] = ContenutoCella::Muro;
            matrice[i][n - 1] = ContenutoCella::Muro;
        }

        let mut rng = rand::thread_rng();

        
        let mut pos_libere = Vec::new();
        for r in 1..n - 1 {
            for c in 1..n - 1 {
                pos_libere.push((r, c));
            }
        }
        pos_libere.shuffle(&mut rng);

        for i in 0..m {
            let (r, c) = pos_libere[i];
            matrice[r][c] = ContenutoCella::Cibo(rng.gen_range(1..10));
        }

        
        for i in m..2 * m {
            let (r, c) = pos_libere[i];
            matrice[r][c] = ContenutoCella::Veleno(rng.gen_range(1..10));
        }

        Campo { n, matrice }
    }

    fn contenuto(&self, r: usize, c: usize) -> ContenutoCella {
        self.matrice[r][c]
    }

    fn set_contenuto(&mut self, r: usize, c: usize, val: ContenutoCella) {
        self.matrice[r][c] = val;
    }
}

struct Giocatore {
    r: usize,
    c: usize,
    direzione: Direzione,
    forza: i32,
}

impl Giocatore {
    fn nuovo(campo: &Campo, forza: i32) -> Giocatore {
        let mut rng = rand::thread_rng();
        let mut pos_libere = Vec::new();
        for r in 1..campo.n - 1 {
            for c in 1..campo.n - 1 {
                if campo.contenuto(r, c) == ContenutoCella::Vuoto {
                    pos_libere.push((r, c));
                }
            }
        }
        let &(r, c) = pos_libere.choose(&mut rng).expect("Deve esserci almeno una cella libera");
        let direzione = match rng.gen_range(0..4) {
            0 => Direzione::Su,
            1 => Direzione::Giu,
            2 => Direzione::Sinistra,
            _ => Direzione::Destra,
        };
        Giocatore { r, c, direzione, forza }
    }

    fn muovi(&mut self, campo: &mut Campo) {
        let (dr, dc) = match self.direzione {
            Direzione::Su => (-1i32, 0),
            Direzione::Giu => (1, 0),
            Direzione::Sinistra => (0, -1),
            Direzione::Destra => (0, 1),
        };
        let nr = (self.r as i32 + dr) as usize;
        let nc = (self.c as i32 + dc) as usize;

        match campo.contenuto(nr, nc) {
            ContenutoCella::Muro => {
                
                self.direzione = self.direzione.opposta();
            }
            ContenutoCella::Cibo(q) => {
                self.forza += q;
                campo.set_contenuto(nr, nc, ContenutoCella::Vuoto);
                self.r = nr;
                self.c = nc;
            }
            ContenutoCella::Veleno(q) => {
                self.forza -= q;
                campo.set_contenuto(nr, nc, ContenutoCella::Vuoto);
                self.r = nr;
                self.c = nc;
            }
            ContenutoCella::Vuoto => {
                self.r = nr;
                self.c = nc;
            }
        }
    }

    fn cambia_direzione_casuale(&mut self) {
        let mut rng = rand::thread_rng();
        let d = match rng.gen_range(0..4) {
            0 => Direzione::Su,
            1 => Direzione::Giu,
            2 => Direzione::Sinistra,
            _ => Direzione::Destra,
        };
        self.direzione = d;
    }
}

struct Configurazione {
    campo: Campo,
    giocatore: Giocatore,
    max_mosse: usize,
    mosse_fatte: usize,
}

impl Configurazione {
    fn nuova(n: usize, m: usize, forza_iniziale: i32, max_mosse: usize) -> Configurazione {
        let mut campo = Campo::nuovo(n, m);
        let giocatore = Giocatore::nuovo(&campo, forza_iniziale);
        Configurazione {
            campo,
            giocatore,
            max_mosse,
            mosse_fatte: 0,
        }
    }

    fn step(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let testa = rng.gen_bool(0.5);
        if testa {
            self.giocatore.muovi(&mut self.campo);
        } else {
            self.giocatore.cambia_direzione_casuale();
            self.giocatore.muovi(&mut self.campo);
        }
        self.mosse_fatte += 1;
    }

    fn finito(&self) -> Option<&'static str> {
        if self.giocatore.forza <= 0 {
            Some("PERDE")
        } else if self.mosse_fatte >= self.max_mosse {
            Some("VINCE")
        } else {
            None
        }
    }
}

impl fmt::Display for Configurazione {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.campo.n {
            for c in 0..self.campo.n {
                if self.giocatore.r == r && self.giocatore.c == c {
                    write!(f, "P ")?; // giocatore
                } else {
                    let simbolo = match self.campo.contenuto(r, c) {
                        ContenutoCella::Vuoto => ". ",
                        ContenutoCella::Muro => "# ",
                        ContenutoCella::Cibo(_) => "C ",
                        ContenutoCella::Veleno(_) => "V ",
                    };
                    write!(f, "{}", simbolo)?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "Forza: {}, Mosse: {}/{}", self.giocatore.forza, self.mosse_fatte, self.max_mosse)
    }
}

fn main() {
    let n = 7;
    let m = 3;
    let forza_iniziale = 10;
    let max_mosse = 30;

    let mut gioco = Configurazione::nuova(n, m, forza_iniziale, max_mosse);

    println!("Inizio gioco:\n{}", gioco);

    while gioco.finito().is_none() {
        gioco.step();
        println!("{}", gioco);
    }

    println!("Risultato finale: {}", gioco.finito().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rimbalzo() {
        
        let mut campo = Campo::nuovo(3, 0);
        let mut giocatore = Giocatore { r: 1, c: 1, direzione: Direzione::Su, forza: 10 };

        
        giocatore.muovi(&mut campo);
        
        assert_eq!(giocatore.r, 1);
        assert_eq!(giocatore.c, 1);
        assert_eq!(giocatore.direzione, Direzione::Giu);
    }

    #[test]
    fn test_cibo_e_veneno() {
        let mut campo = Campo::nuovo(5, 0);
        campo.set_contenuto(2, 2, ContenutoCella::Cibo(5));
        campo.set_contenuto(2, 3, ContenutoCella::Veleno(3));

        let mut giocatore = Giocatore { r: 2, c: 1, direzione: Direzione::Destra, forza: 10 };
    }
}









