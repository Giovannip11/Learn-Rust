use std::cell::RefCell;
use std::rc::{Rc, Weak};

//Definição das possíveis cores para um nó
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cor {
    Rubro,
    Negra,
}

type NoRef<T> = Rc<RefCell<No<T>>>;
#[derive(Debug)]
pub struct No<T> {
    pub chave: T,
    pub cor: Cor,
    pub pai: Option<Weak<RefCell<No<T>>>>, //vou utilizar o weak para evitar ciclos e vazamentos de memória
    pub esq: Option<NoRef<T>>, //Como um nó pode ou não ter filhos(dir,esq) usamos o option para representar essa ausência.
    pub dir: Option<NoRef<T>>, //O box ta servido para saber o tamanho exato do struct em tempo de compilação.
}
impl<T> No<T> {
    pub fn new(chave: T) -> NoRef<T> {
        Rc::new(RefCell::new(No {
            chave,
            cor: Cor::Rubro, //É aquilo todo nó inserido entra como rubro.
            pai: None,
            esq: None,
            dir: None,
        }))
    }
}
//Estrutura para represnetar a arvore
#[derive(Debug)]
pub struct RubroNegraArv<T> {
    pub root: Option<NoRef<T>>,
}
impl<T> RubroNegraArv<T> {
    pub fn new() -> Self {
        RubroNegraArv { root: None }
    }
}

fn eh_rubro<T>(no: &Option<NoRef<T>>) -> bool {
    match no {
        Some(n) => n.borrow().cor == Cor::Rubro,
        None => false,
    }
}
fn rotacao_esq<T>(no: Option<NoRef<T>>) -> Option<NoRef<T>> {
    let mut nova_raiz = no.dir.take().expect("Filho direito necessario");
    no.dir = nova_raiz.esq.take();
    no.cor = Cor::Rubro;
    nova_raiz.esq = Some(no);
    nova_raiz
}

fn main() {
    println!("Hello World");
}
