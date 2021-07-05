use std::{collections::HashMap, usize};

/// K4(ON(SO3)2)2
/// H2O
/// Mg(OH)2

fn main() {
    println!("{}", is_digit(&'a'));
    println!("{}", is_digit(&'2'));

    let s = String::from("K4(ON(SO3)2)2");
    println!("{}", count_of_atoms(s));
}
/// https://leetcode-cn.com/problems/number-of-atoms/
/// 726. 原子的数量
pub fn count_of_atoms(formula: String) -> String {
    let chars: Vec<char> = formula.chars().collect();
    let mut atoms_stack: Vec<String> = Vec::new();
    let mut count_stack: Vec<usize> = Vec::new();
    let mut atom = String::new();
    let mut count = String::new();
    let len = chars.len();
    for i in 0..len {
        let c = chars[i];
        if c >= 'A' && c <= 'Z' {
            if atom.len() > 0 {
                atoms_stack.push(atom.clone());
                atom = String::new();
                if count.len() == 0 {
                    count_stack.push(1);
                }
            } 
            atom.push(c);
        } else if c >= 'a' && c <= 'z' {
            atom.push(c);
        } else if is_digit(&c) {
            if atom.len() > 0 {
                atoms_stack.push(atom.clone());
                atom = String::new();
            }
            count.push(c);
            if i + 1 == len || !is_digit(&chars[i + 1]) {
                let count_num = count.parse::<usize>().unwrap();
                if let Some(pre_atom) = atoms_stack.last() {
                    if pre_atom == ")" {
                        atoms_stack.pop();
                        let mut move_idx = atoms_stack.len() - 1;
                        let mut count_idx = count_stack.len() - 1;
                        while move_idx >= 0 {
                            if atoms_stack[move_idx] == ")" {
                                atoms_stack.remove(move_idx);
                            } else if atoms_stack[move_idx] == "(" {
                                atoms_stack.remove(move_idx);
                                break;
                            } else {
                                count_stack[count_idx] = count_stack[count_idx] * count_num;
                                if count_idx >= 1 {
                                    count_idx -= 1;    
                                }
                            }
                            if move_idx == 0 {
                                break;
                            }
                            move_idx -= 1;
                        }
                        count = String::new();
                    } else {
                        
                        count_stack.push(count_num);
                        
                    }
                }
            }
        } else if c == '(' ||  c == ')' {
            if atom.len() > 0 {
                atoms_stack.push(atom.clone());
                count_stack.push(1);
                atom = String::new();
            }
            atoms_stack.push(c.to_string());
        } 
    }
    if atom.len() > 0 {
        atoms_stack.push(atom.clone());
        count_stack.push(1);
    }
    let mut map = HashMap::new();
    println!("{:?}", atoms_stack);
    println!("{:?}", &count_stack);
    for i in 0..atoms_stack.len() {
        *map.entry(atoms_stack.get(i).unwrap()).or_insert(0) += count_stack[i];
    }
    let mut vec: Vec<(&String, usize)> = map.into_iter().collect();
    vec.sort_by(|a, b| a.0.cmp(b.0));
    let mut rst = String::new();
    for (s, v) in vec {
        rst.push_str(&*s);
        if v != 1 {
            rst.push_str(&v.to_string());
        }
        
    }
    rst
}

fn is_digit(c: &char) -> bool{
    *c >= '0' && *c <= '9'
}