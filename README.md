# busca_megastore
<h1>Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore </h1>
Descrição do Projeto
Este projeto consiste na implementação de um sistema de busca otimizado para o catálogo de produtos da "MegaStore", utilizando a linguagem de programação Rust. O objetivo do sistema é:
- Permitir buscas rápidas e precisas em um catálogo extenso de produtos.
- Oferecer recomendações personalizadas com base em um grafo de relacionamentos entre os produtos.
- Melhorar a experiência do cliente, aumentando as taxas de conversão e fidelização.


<h4>Tecnologias Utilizadas</h4>
- Rust: Linguagem de programação de alto desempenho e segurança de memória.
- HashMap: Estrutura de dados para indexação e buscas rápidas.
- Crate std::collections: Para utilização de tabelas hash e grafos.
- Cargo: Gerenciador de pacotes e build para projetos Rust.


<h4>Funcionalidades Principais</h4>
- Busca com Filtros e Cache:- Filtra produtos por categoria e marca.
- Utiliza cache para armazenar resultados frequentes, melhorando o desempenho.

- Recomendações de Produtos:- Sistema de recomendação baseado em grafos para sugerir produtos relacionados.

- Escalabilidade e Eficiência:- Projetado para lidar com um grande número de produtos e consultas simultâneas.



<h4>Como Executar o Sistema de Busca</h4>
- Certifique-se de que o Rust está instalado. Caso contrário, instale-o seguindo as instruções em rust-lang.org.
- Clone o repositório do projeto:git clone <URL_DO_REPOSITORIO>
cd <NOME_DA_PASTA_DO_PROJETO>

- Compile o projeto:cargo build

- Execute o sistema:cargo run



<h4>Como Executar os Testes</h4>
- Certifique-se de que o projeto foi clonado e que você está na raiz do diretório.
- Execute todos os testes com:cargo test

- Verifique o terminal para os resultados dos testes. Todos devem passar!


Exemplos de Uso
Busca com Filtros
- Consulta: Produtos na categoria "Acessorios" da marca "AccessoryCo".
- Exemplo de execução:Produtos encontrados na categoria 'Acessorios': 
[Product { id: 102, name: "Capas de Celular", category: "Acessorios", brand: "AccessoryCo" }]

<h4>Recomendações de Produtos</h4>
- Consulta: Recomendações para o produto "Smartphone".
- Exemplo de execução:Recomendações para 'Smartphone':
- ID: 102, Nome: Capas de Celular
- ID: 103, Nome: Carregador



<h4>Arquitetura do Sistema </h4>
- Módulo Principal:- Contém a implementação das funções de busca e recomendação.

- Cache:- Armazenamento de resultados frequentes para melhorar o desempenho.

- Grafo de Recomendações:- Baseado em HashMap, conectando produtos relacionados.

- Testes:- Cobrem busca com filtros, uso de cache e recomendações.



<h4>Algoritmos e Estruturas de Dados</h4>
- Tabelas Hash: Usadas para indexação de produtos com acesso em tempo constante.
- Grafo de Recomendação: Modelado como um HashMap onde cada chave é um produto e o valor é uma lista de produtos relacionados.


<h4>Desempenho e Escalabilidade</h4>
- Otimização:- As tabelas hash garantem tempo de busca constante, mesmo com milhões de produtos.

- Cache:- Reduz o tempo de resposta para buscas repetidas.

- Teste de Escalabilidade:- Testado com diferentes volumes de dados para garantir eficiência.



Contribuições
Este projeto é parte de um estudo de caso acadêmico da universidade UNIFECAF.
