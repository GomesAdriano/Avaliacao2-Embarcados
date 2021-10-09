fn main(){
    
    //entradas
    let a = vec![false,false,true,true];
    let b = vec![false,true,false,true];
    let mut x = Vec::<bool>::new();
    let mut pos = 0;
    
    println!("Tabela Verdade\n  A | B | X");
    
    loop{
        //X <= not(not(B) nand (A or B))     - circuito
        x.push(!(!((!b[pos]) && (a[pos] || b[pos]))));
        println!("  {} | {} | {}", match a[pos] { true => 1, false => 0},
                                   match b[pos] { true => 1, false => 0},
                                   match x[pos] { true => 1, false => 0});
        pos+=1;
        
        if pos >= 4{
            break;
        }
    }
    
    pos = 0;
    println!("\nPasso a Passo\n");
    
    loop{
        println!("A = {}", match a[pos] { true => 1, false => 0});
        println!("B = {}", match b[pos] { true => 1, false => 0});
        println!("X = !(!(!({}) && ({} || {}))) ", match b[pos] { true => 1, false => 0},
                                                   match a[pos] { true => 1, false => 0},
                                                   match b[pos] { true => 1, false => 0});
        println!("X = !(!({} && {})) ", match !b[pos] { true => 1, false => 0},
                                        match a[pos] || b[pos] { true => 1, false => 0});
        println!("X = !(!({})) ", match (!b[pos]) && (a[pos] || b[pos]) { true => 1, false => 0});
        println!("X = !({}) ", match !((!b[pos]) && (a[pos] || b[pos])) { true => 1, false => 0});
        println!("X = {}", match x[pos] { true => 1, false => 0});
        println!("\n");
                                 
        pos+=1;
        
        if pos >= 4{
            break;
        }
    }
}