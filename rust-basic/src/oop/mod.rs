trait Etudie {
    fn etudie_hard() -> i32;

    fn etudie_tous_les_jours() -> u32;
}

// trait Print<slf = Self> {
//     fn print_inherit(self) {
//         println!("Je suis {}, J'ai {} ans.", &self.nom, &self.age);
//     }
// }

struct Personne {
    pub age: u32,
    pub nom: String,
}

impl Personne {
    fn print(&self) {
        println!("Je suis {}, J'ai {} ans.", &self.nom, &self.age);
    }
}

struct Etudiant {
    pub age: u32,
    pub mon: String,
}

impl Etudie for Personne {
    fn etudie_hard() -> i32 {
        println!("J'etudie beaucoup!");
        100
    }

    fn etudie_tous_les_jours() -> u32 {
        println!("J'etudie tous les jours. Et toi?");
        200
    }
}

pub fn run() {
    let personne_un = Personne {
        age: 20,
        nom: String::from("Xavier"),
    };

    let personne_deux = Personne {
        age: 34,
        nom: String::from("David"),
    };

    personne_un.print();
    personne_deux.print();
}
