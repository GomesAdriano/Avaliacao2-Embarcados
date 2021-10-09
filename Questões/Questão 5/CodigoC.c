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