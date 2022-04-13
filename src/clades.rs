use std::{
    boxed::Box,
    fmt::{self, Display},
};
use tabled::{Disable, Footer, MaxWidth, Modify, Rows, Table, Tabled};

#[derive(Debug, Clone)]
pub struct Seq<'a>(pub Box<&'a [&'a str]>);

impl Seq<'_> {
    pub fn get(&self, index: usize) -> Option<&&str> {
        self.0.get(index)
    }
}

impl Display for Seq<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.0.join(", ");
        write!(f, "{}", inner)
    }
}
#[derive(Debug, Clone, Tabled)]
pub struct TelomereSeq<'a> {
    #[tabled(rename = "Clade")]
    pub clade: &'a str,
    #[tabled(rename = "Telomeric repeat units")]
    pub seq: Seq<'a>,
    pub length: usize,
}

// automated input start

pub const CLADES: &[&str] = &[
    "Accipitriformes",
    "Actiniaria",
    "Agaricales",
    "Alismatales",
    "Amphilepidida",
    "Anura",
    "Apiales",
    "Aplousobranchia",
    "Aquifoliales",
    "Araneae",
    "Artiodactyla",
    "Asparagales",
    "Asterales",
    "Atheriniformes",
    "Balanomorpha",
    "Boraginales",
    "Brassicales",
    "Buxales",
    "Camarodonta",
    "Caprimulgiformes",
    "Carcharhiniformes",
    "Cardiida",
    "Carnivora",
    "Caryophyllales",
    "Celastrales",
    "Chaetocerotales",
    "Cheilostomatida",
    "Chiroptera",
    "Chitonida",
    "Chlamydomonadales",
    "Coleoptera",
    "Comatulida",
    "Crassiclitellata",
    "Cucurbitales",
    "Cypriniformes",
    "Decapoda",
    "Dioctophymatida",
    "Dipsacales",
    "Ericales",
    "Eucoccidiorida",
    "Euglenales",
    "Eulipotyphla",
    "Fabales",
    "Fagales",
    "Forcipulatida",
    "Fucales",
    "Gentianales",
    "Geophilomorpha",
    "Geraniales",
    "Gigartinales",
    "Glomerida",
    "Hemiptera",
    "Heteronemertea",
    "Hirudinida",
    "Hymenoptera",
    "Hypnales",
    "Isochrysidales",
    "Lamiales",
    "Lepidoptera",
    "Liliales",
    "Lithobiomorpha",
    "Littorinimorpha",
    "Lunulariales",
    "Lycopodiales",
    "Malpighiales",
    "Malvales",
    "Megaloptera",
    "Myrtales",
    "Neuroptera",
    "Nudibranchia",
    "Odonata",
    "Opiliones",
    "Orthoptera",
    "Ostreida",
    "Palmariales",
    "Pectinida",
    "Pelecaniformes",
    "Perciformes",
    "Phlebobranchia",
    "Phyllodocida",
    "Plecoptera",
    "Poales",
    "Polytrichales",
    "Primates",
    "Procellariiformes",
    "Pyrenomonadales",
    "Ranunculales",
    "Rhabditida",
    "Rodentia",
    "Rosales",
    "Sabellida",
    "Salmoniformes",
    "Sapindales",
    "Scombriformes",
    "Scorpiones",
    "Solanales",
    "Sphagnales",
    "Stolidobranchia",
    "Trichoptera",
    "Trochida",
    "Venerida",
];

pub fn return_telomere_sequence(clade: &str) -> TelomereSeq {
    let result = match clade {
        "Accipitriformes" => TelomereSeq {
            clade: "Accipitriformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Actiniaria" => TelomereSeq {
            clade: "Actiniaria",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Agaricales" => TelomereSeq {
            clade: "Agaricales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Alismatales" => TelomereSeq {
            clade: "Alismatales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Amphilepidida" => TelomereSeq {
            clade: "Amphilepidida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Anura" => TelomereSeq {
            clade: "Anura",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Apiales" => TelomereSeq {
            clade: "Apiales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Aplousobranchia" => TelomereSeq {
            clade: "Aplousobranchia",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Aquifoliales" => TelomereSeq {
            clade: "Aquifoliales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Araneae" => TelomereSeq {
            clade: "Araneae",
            seq: Seq(Box::new(&[
                "AATAT", "ACTAT", "AACAT", "AAAGC", "ACATG", "AACTTGT",
            ])),
            length: 6,
        },

        "Artiodactyla" => TelomereSeq {
            clade: "Artiodactyla",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Asparagales" => TelomereSeq {
            clade: "Asparagales",
            seq: Seq(Box::new(&["AACCGAGCCCAT", "AACCCT"])),
            length: 2,
        },

        "Asterales" => TelomereSeq {
            clade: "Asterales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Atheriniformes" => TelomereSeq {
            clade: "Atheriniformes",
            seq: Seq(Box::new(&["ACCAG"])),
            length: 1,
        },

        "Balanomorpha" => TelomereSeq {
            clade: "Balanomorpha",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Boraginales" => TelomereSeq {
            clade: "Boraginales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Brassicales" => TelomereSeq {
            clade: "Brassicales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Buxales" => TelomereSeq {
            clade: "Buxales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Camarodonta" => TelomereSeq {
            clade: "Camarodonta",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Caprimulgiformes" => TelomereSeq {
            clade: "Caprimulgiformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Carcharhiniformes" => TelomereSeq {
            clade: "Carcharhiniformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Cardiida" => TelomereSeq {
            clade: "Cardiida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Carnivora" => TelomereSeq {
            clade: "Carnivora",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Caryophyllales" => TelomereSeq {
            clade: "Caryophyllales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Celastrales" => TelomereSeq {
            clade: "Celastrales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Chaetocerotales" => TelomereSeq {
            clade: "Chaetocerotales",
            seq: Seq(Box::new(&["ACCCT"])),
            length: 1,
        },

        "Cheilostomatida" => TelomereSeq {
            clade: "Cheilostomatida",
            seq: Seq(Box::new(&["AAACCCC", "ACAGT", "AAGTCT"])),
            length: 3,
        },

        "Chiroptera" => TelomereSeq {
            clade: "Chiroptera",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Chitonida" => TelomereSeq {
            clade: "Chitonida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Chlamydomonadales" => TelomereSeq {
            clade: "Chlamydomonadales",
            seq: Seq(Box::new(&["AAGGATGGAC", "AACCCT"])),
            length: 2,
        },

        "Coleoptera" => TelomereSeq {
            clade: "Coleoptera",
            seq: Seq(Box::new(&[
                "AGATATAT",
                "AAAAATTC",
                "AAGTC",
                "AAGTCG",
                "AAAAATAC",
                "AAAGGTCACC",
                "AAAGGAC",
                "AACAGACCCG",
                "AAATAACT",
                "ACAGACTG",
                "AACAT",
                "AACTCC",
                "ACTATG",
                "ACCTG",
                "AAATAATT",
                "AACCT",
                "ACTCTG",
                "AAGACAGAC",
                "AAGTAATC",
                "AACCC",
            ])),
            length: 20,
        },

        "Comatulida" => TelomereSeq {
            clade: "Comatulida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Crassiclitellata" => TelomereSeq {
            clade: "Crassiclitellata",
            seq: Seq(Box::new(&["AAGGAC", "AACTC", "AACCCT"])),
            length: 3,
        },

        "Cucurbitales" => TelomereSeq {
            clade: "Cucurbitales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Cypriniformes" => TelomereSeq {
            clade: "Cypriniformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Decapoda" => TelomereSeq {
            clade: "Decapoda",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Dioctophymatida" => TelomereSeq {
            clade: "Dioctophymatida",
            seq: Seq(Box::new(&["ACGATG"])),
            length: 1,
        },

        "Dipsacales" => TelomereSeq {
            clade: "Dipsacales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Ericales" => TelomereSeq {
            clade: "Ericales",
            seq: Seq(Box::new(&["AAGCATT", "AAACCCT", "AAGCATC"])),
            length: 3,
        },

        "Eucoccidiorida" => TelomereSeq {
            clade: "Eucoccidiorida",
            seq: Seq(Box::new(&["AAACCCT", "AAGGAGGAGACAAT"])),
            length: 2,
        },

        "Euglenales" => TelomereSeq {
            clade: "Euglenales",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Eulipotyphla" => TelomereSeq {
            clade: "Eulipotyphla",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Fabales" => TelomereSeq {
            clade: "Fabales",
            seq: Seq(Box::new(&["AAACCCT", "AACCT"])),
            length: 2,
        },

        "Fagales" => TelomereSeq {
            clade: "Fagales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Forcipulatida" => TelomereSeq {
            clade: "Forcipulatida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Fucales" => TelomereSeq {
            clade: "Fucales",
            seq: Seq(Box::new(&["ACACT", "AACCCT"])),
            length: 2,
        },

        "Gentianales" => TelomereSeq {
            clade: "Gentianales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Geophilomorpha" => TelomereSeq {
            clade: "Geophilomorpha",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Geraniales" => TelomereSeq {
            clade: "Geraniales",
            seq: Seq(Box::new(&["AAACCCT", "AACCCT"])),
            length: 2,
        },

        "Gigartinales" => TelomereSeq {
            clade: "Gigartinales",
            seq: Seq(Box::new(&["ACAGGCGTGCCC"])),
            length: 1,
        },

        "Glomerida" => TelomereSeq {
            clade: "Glomerida",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Hemiptera" => TelomereSeq {
            clade: "Hemiptera",
            seq: Seq(Box::new(&[
                "ACAGAGAGGC",
                "ACCGAG",
                "AACCT",
                "AATATAG",
                "AATAC",
                "AACACTCCCT",
                "AAACCTATCC",
                "AACCTACCT",
                "AAAATTGTTGATGGAGATCATAC",
                "AACCATCCCT",
                "AAACCACCCT",
                "AAGAAT",
                "AAGAATATAGAAT",
            ])),
            length: 13,
        },

        "Heteronemertea" => TelomereSeq {
            clade: "Heteronemertea",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Hirudinida" => TelomereSeq {
            clade: "Hirudinida",
            seq: Seq(Box::new(&["AACACGAGATG"])),
            length: 1,
        },

        "Hymenoptera" => TelomereSeq {
            clade: "Hymenoptera",
            seq: Seq(Box::new(&[
                "ACGATG",
                "AATAT",
                "AACGAGTCG",
                "AAACGAGTC",
                "AGAGAT",
                "AAAACG",
                "AACCCGAACCT",
                "AACCCAGACCT",
                "AACGTAT",
                "AAACAC",
                "AGGGATATC",
                "ACAGAG",
                "AAAAAC",
                "AACGAC",
                "ACCTG",
                "AACCCAGACCC",
                "AACCCAGACGC",
                "AACCCTGACGC",
                "ACTCT",
                "AACCT",
                "AACCCCAACCT",
                "ACATCGT",
                "AACCCT",
                "AACCC",
            ])),
            length: 24,
        },

        "Hypnales" => TelomereSeq {
            clade: "Hypnales",
            seq: Seq(Box::new(&["AAACCCT", "AACAG"])),
            length: 2,
        },

        "Isochrysidales" => TelomereSeq {
            clade: "Isochrysidales",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Lamiales" => TelomereSeq {
            clade: "Lamiales",
            seq: Seq(Box::new(&["AAACCCT", "AACCCTAAT"])),
            length: 2,
        },

        "Lepidoptera" => TelomereSeq {
            clade: "Lepidoptera",
            seq: Seq(Box::new(&[
                "ACTCTG",
                "AACCT",
                "AACTCG",
                "ACATC",
                "AACCATCCCT",
                "AAGACGGTAAGTGTGTATGTATGT",
                "AACGTGAT",
            ])),
            length: 7,
        },

        "Liliales" => TelomereSeq {
            clade: "Liliales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Lithobiomorpha" => TelomereSeq {
            clade: "Lithobiomorpha",
            seq: Seq(Box::new(&["AACCT", "AAAGTCG"])),
            length: 2,
        },

        "Littorinimorpha" => TelomereSeq {
            clade: "Littorinimorpha",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Lunulariales" => TelomereSeq {
            clade: "Lunulariales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Lycopodiales" => TelomereSeq {
            clade: "Lycopodiales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Malpighiales" => TelomereSeq {
            clade: "Malpighiales",
            seq: Seq(Box::new(&["AAACCCT", "AACCCT"])),
            length: 2,
        },

        "Malvales" => TelomereSeq {
            clade: "Malvales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Megaloptera" => TelomereSeq {
            clade: "Megaloptera",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Myrtales" => TelomereSeq {
            clade: "Myrtales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Neuroptera" => TelomereSeq {
            clade: "Neuroptera",
            seq: Seq(Box::new(&["AACCC"])),
            length: 1,
        },

        "Nudibranchia" => TelomereSeq {
            clade: "Nudibranchia",
            seq: Seq(Box::new(&["ACAGTAGC", "AAACAC"])),
            length: 2,
        },

        "Odonata" => TelomereSeq {
            clade: "Odonata",
            seq: Seq(Box::new(&["AGATC", "AGCCATCGCCAT", "AACCC"])),
            length: 3,
        },

        "Opiliones" => TelomereSeq {
            clade: "Opiliones",
            seq: Seq(Box::new(&["ACGAG"])),
            length: 1,
        },

        "Orthoptera" => TelomereSeq {
            clade: "Orthoptera",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Ostreida" => TelomereSeq {
            clade: "Ostreida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Palmariales" => TelomereSeq {
            clade: "Palmariales",
            seq: Seq(Box::new(&["ACACTGAGT"])),
            length: 1,
        },

        "Pectinida" => TelomereSeq {
            clade: "Pectinida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Pelecaniformes" => TelomereSeq {
            clade: "Pelecaniformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Perciformes" => TelomereSeq {
            clade: "Perciformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Phlebobranchia" => TelomereSeq {
            clade: "Phlebobranchia",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Phyllodocida" => TelomereSeq {
            clade: "Phyllodocida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Plecoptera" => TelomereSeq {
            clade: "Plecoptera",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Poales" => TelomereSeq {
            clade: "Poales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Polytrichales" => TelomereSeq {
            clade: "Polytrichales",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Primates" => TelomereSeq {
            clade: "Primates",
            seq: Seq(Box::new(&["AATGG"])),
            length: 1,
        },

        "Procellariiformes" => TelomereSeq {
            clade: "Procellariiformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Pyrenomonadales" => TelomereSeq {
            clade: "Pyrenomonadales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Ranunculales" => TelomereSeq {
            clade: "Ranunculales",
            seq: Seq(Box::new(&["AAACCCT", "AACCCTG", "AACCCCG", "AAACCG"])),
            length: 4,
        },

        "Rhabditida" => TelomereSeq {
            clade: "Rhabditida",
            seq: Seq(Box::new(&["AAGCCT"])),
            length: 1,
        },

        "Rodentia" => TelomereSeq {
            clade: "Rodentia",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Rosales" => TelomereSeq {
            clade: "Rosales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Sabellida" => TelomereSeq {
            clade: "Sabellida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Salmoniformes" => TelomereSeq {
            clade: "Salmoniformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Sapindales" => TelomereSeq {
            clade: "Sapindales",
            seq: Seq(Box::new(&["AAACCCT"])),
            length: 1,
        },

        "Scombriformes" => TelomereSeq {
            clade: "Scombriformes",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Scorpiones" => TelomereSeq {
            clade: "Scorpiones",
            seq: Seq(Box::new(&["AACCT"])),
            length: 1,
        },

        "Solanales" => TelomereSeq {
            clade: "Solanales",
            seq: Seq(Box::new(&["AAACCCT", "AACCCTG"])),
            length: 2,
        },

        "Sphagnales" => TelomereSeq {
            clade: "Sphagnales",
            seq: Seq(Box::new(&["AAACCT"])),
            length: 1,
        },

        "Stolidobranchia" => TelomereSeq {
            clade: "Stolidobranchia",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        "Trichoptera" => TelomereSeq {
            clade: "Trichoptera",
            seq: Seq(Box::new(&["AACCT", "AATGACAGCG"])),
            length: 2,
        },

        "Trochida" => TelomereSeq {
            clade: "Trochida",
            seq: Seq(Box::new(&["AACATG", "AACCCT"])),
            length: 2,
        },

        "Venerida" => TelomereSeq {
            clade: "Venerida",
            seq: Seq(Box::new(&["AACCCT"])),
            length: 1,
        },

        _ => panic!("{} is not yet accounted for in this pipeline.", clade),
    };
    result
}
// automated input end

pub fn print_table() {
    let mut clade_vec = Vec::new();

    for clade in CLADES {
        clade_vec.push(return_telomere_sequence(clade));
    }

    eprintln!(
        "{}",
        Table::new(&clade_vec)
            .with(
                Modify::new(Rows::new(1..clade_vec.len() - 1))
                    .with(MaxWidth::wrapping(30).keep_words()),
            )
            .with(Disable::Column(2..3))
            .with(Footer(
                "This table is modified from \"A telomeric repeat database\"\nhttps://github.com/tolkit/a-telomeric-repeat-database"
            ))
            .to_string()
    );
}
