// primeiramente gostaria de falar sobre a estrutura de um compilador, como que ele funciona e tudo
// mais. Primeiramente vamos considerar o que é um compilador, isso basicamente é um programa que
// pega uma linguagem fonte(a linguagem de entrada) e transforma em uma linguagem objeto(programa
// de saida, normalmente sendo em linguagem de máquina) com semantica equivalente. Dentro dessa
// caixa preta, que é o compilador, tem-se duas partes para o mapeamento, sendo: análise(front-end
// do compilador) e síntese(back-end do compilador).
// de uma forma geral, quando analisamos um compilador vemos que ele é uma sequencia de fases,
// sendo cada uma delas uma transformação de uma representação de um programa fonte em um outro
// programa até chegar no programa objeto.
//
// Fases de um compilador:
//  -fluxo de caracteres -> [analisador léxico]=> fluxo de tokens -> [analisador sintático]=>
//  árvore de sintaxe -> [analisador semantico]=> árvore de sintaxe -> [geradir de codigo
//  intermediario] => representacao intermediária ->  [otimizador de codigo dependente de
//  maquina]=> representacao intermediaria -> [gerador de codigo]=> codigo de maquina alvo ->
//  [otimizador de codigo independente de maquina]=> codigo da maquina alvo
//
// ANALISE E SINTESE:
// - análise lexica:
//  e o fluxo de caracteres que compoe o programa fonte e agrupa sequencias significativas(analise
//  lexica),
//  chamadas lexemas. Para cada lexema, o analdor produz um token no formato: 
//      (nome-token, valor-atribuido)
//      exemplo para token:
//          position = initial + rate * 60
//              vira:
//          (id, 1) (=) (id, 2) (+) (id, 3) (*) (60)
//
//  que passa para proxima fase "analise sintatica". O nome-token é um valor abstrato, já o
//  valor-atribuido aponta para uma entrada na tabela de simbolos referente a esse token.
//
// - análise sintática:
//  esse analisador será responsável para cirar uma representaçzào tipo árvore da resposta a
//  analise lexica. Tem como objetivo mostrar a estrutura gramatical da sequencia de tokens. é
//  formado como uma "árvore de sintaxe", mostrando a ordem em que as operações devem acontecer.
//
//      exemplo para a árvore dado ultimo exemplo:
//
//         =
//        / \
//  (id, 1)  +
//          / \
//   (id, 2)   *
//            / \
//     (id, 3)   60
//
// - analise semantica:
//   responsável pela verificação de tipos. fazendo o seguinte com nossa arvore de sintaxe:
//         =
//        / \
//  (id, 1)  +
//          / \
//   (id, 2)   *
//            / \
//     (id, 3)   inttofloat #coment: isso se chama coerção, transforma int para float
//                   |               caso seja necessário e possível.
//                  60
//
fn main() {
    println!("Hello, world!");
}
