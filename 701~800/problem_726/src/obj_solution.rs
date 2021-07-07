use std::collections::HashMap;

use crate::is_digit;

#[derive(Debug)]
struct Atom {
    pub name: String, 
    pub count: usize,
}

impl Atom {
    fn new(name: String, count: usize) -> Self {
        Atom {
            name,
            count
        }
    }
}


/// 
///
///
pub fn count_of_atoms(formula: String) -> String {
    let mut atoms_stack: Vec<Atom> = Vec::new();
    let cs: Vec<char> = formula.chars().collect();
    let mut atom = String::new();
    let mut count = String::new();
    for i in 0..cs.len() {
        let c = cs[i];
        if c >= 'A' &&  c <= 'Z' {
            if atom.len() > 0 {
                atoms_stack.push(Atom::new(atom.clone(), 1));
                atom = String::new();
            } 
            atom.push(c);
        } else if c >= 'a' && c <= 'z' {
            atom.push(c);
        } else if is_digit(&c) {
            count.push(c);
            if i + 1 == cs.len() || !is_digit(&cs[i + 1]) {
                let ct = count.parse::<usize>().unwrap();
                if let Some(v) = atoms_stack.last() {
                    if v.name == ")" {
                        for idx in (0..atoms_stack.len()).rev() {
                            if atoms_stack[idx].name == ")" {
                                atoms_stack.remove(idx);
                            } else if atoms_stack[idx].name == "(" {
                                atoms_stack.remove(idx);
                                break;
                            } else {
                                atoms_stack[idx].count = atoms_stack[idx].count * ct;
                            }
                        }
                    }
                }
                if atom.len() > 0 {
                    atoms_stack.push(Atom::new(atom.clone(), ct));
                    atom = String::new();
                }
                count = String::new();
            }
        } else if c == '(' || c == ')' {
            if atom.len() > 0 {
                atoms_stack.push(Atom::new(atom.clone(), 1));
                atom = String::new();
            } 
            atoms_stack.push(Atom::new(c.to_string(), 0));
        }
    }
    if atom.len() > 0 {
        atoms_stack.push(Atom::new(atom.clone(), 1));
    }
    println!("{:?}", atoms_stack);
    let mut map: HashMap<String, usize> = HashMap::new();
    for a in atoms_stack.iter() {
        *map.entry(a.name.clone()).or_insert(0) += a.count;
    }

    let mut vec: Vec<(String, usize)> = map.into_iter().collect();
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    let mut rst = String::new();
    for o in vec {
        if o.1 > 1 {
            rst = rst + &o.0 + &o.1.to_string();
        } else if o.1 == 1 {
            rst.push_str(&o.0);
        }
    }
    rst
}

