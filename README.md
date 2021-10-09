# Avaliaçao 2 - Sistemas Embarcados
 
 ## Sobre este repositório
 <p>Projeto destinado a resolução das questões 1, 2 e 5 da 2ª avaliação da disciplina de **Sistemas Embarcados** ministrada pelo professor **Vandermi João da Silva**.</p>
 
 ### Identificação
 - Aluno: Adriano dos Santos Gomes
 - Matrícula: 21751229

 ### Resolução
 **Questão 1 - Construa um circuito com portas lógicas que permitam modificar as saídas quando processadas. O circuito deverá receber como entrada um registrador A e B e a resposta deverá ser armazenada em X. A Saída deverá inverter o resultado do processamento. Após a construção do circuito, codifique-o usando VHDL.**
 
 ![circuito](https://github.com/GomesAdriano/Avaliacao2-Embarcados/blob/main/circuito.jpg?raw=true)
 
 ~~~VHDL
 library ieee;
 use ieee.std_logic_1164.all;

 entity questao is
	port(
		A, B : in std_logic;
		X    : out std_logic
	);
 end questao;

 architecture arq_questao of questao is
 begin
	X <= not(not(B) nand (A or B))
 end arq_questao;
 ~~~

 **Questão 2 - Codifique o circuito desenvolvido na questão 1 em linguagem de programação Rust.**
~~~rust
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
~~~

**Questão 5 - Codifique o circuito desenvolvido na questão 1 em linguagem de programação C.**
~~~c
#include <stdio.h>
#define PADRAO 4

int main(){
    
    //entradas
    int a[PADRAO] = {0, 0, 1, 1};
    int b[PADRAO] = {0, 1, 0, 1};
    int x[PADRAO];
    
    printf("Tabela Verdade\n  A | B | X\n");
    for(int posicao = 0; posicao < PADRAO; posicao++){
              //X <= not(not(B) nand (A or B))     - circuito
        x[posicao] = !(!((!b[posicao]) && (a[posicao] || b[posicao])));
        printf("  %d | %d | %d \n", a[posicao], b[posicao], x[posicao]);
    }
    
    printf("\nPasso a Passo\n");
    for(int posicao = 0; posicao < PADRAO; posicao++){
        printf("A = %d\n", a[posicao]);
        printf("B = %d\n", b[posicao]);
        printf("X = !(!(!(%d) && (%d || %d)))\n", b[posicao], a[posicao], b[posicao]);
        printf("X = !(!(%d && %d)\n", !b[posicao], a[posicao] || b[posicao]);
        printf("X = !(!(%d))\n", (!b[posicao]) && (a[posicao] || b[posicao]));
        printf("X = !(%d)\n", !((!b[posicao]) && (a[posicao] || b[posicao])));
        printf("X = %d\n", x[posicao]);
        printf("\n");
    }
    return 0;
}
~~~


