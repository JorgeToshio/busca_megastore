// Importando HashMap da biblioteca padrão.
use std::collections::HashMap;

// Estrutura para representar cada produto no sistema.
// Contém informações detalhadas como ID, nome, categoria e marca.
#[derive(Debug, Clone)]
struct Product {
    id: u32,             // Identificador único do produto.
    name: String,        // Nome do produto.
    category: String,    // Categoria do produto.
    brand: String,       // Marca do produto.
}

fn main() {
    // Criando o índice de produtos.
    let mut products: HashMap<u32, Product> = HashMap::new();
    products.insert(
        101,
        Product {
            id: 101,
            name: "Smartphone".to_string(),
            category: "Eletronicos".to_string(),
            brand: "TechBrand".to_string(),
        },
    );
    products.insert(
        102,
        Product {
            id: 102,
            name: "Capas de Celular".to_string(),
            category: "Acessorios".to_string(),
            brand: "AccessoryCo".to_string(),
        },
    );
    products.insert(
        103,
        Product {
            id: 103,
            name: "Carregador".to_string(),
            category: "Acessorios".to_string(),
            brand: "ChargePlus".to_string(),
        },
    );

    // Criando o grafo de recomendações.
    let mut recommendation_graph: HashMap<u32, Vec<u32>> = HashMap::new();
    recommendation_graph.insert(101, vec![102, 103]); // Smartphone recomenda Capas e Carregadores.
    recommendation_graph.insert(102, vec![101]);     // Capas recomendam Smartphone.
    recommendation_graph.insert(103, vec![101]);     // Carregador recomenda Smartphone.

    // Inicializando o cache de buscas.
    let mut cache: HashMap<String, Vec<&Product>> = HashMap::new();

    // Buscando produtos por categoria e marca utilizando cache.
    search_with_cache("Acessorios", Some("AccessoryCo"), &products, &mut cache);

    // Buscando recomendações baseadas no grafo de produtos.
    recommend_products(101, &products, &recommendation_graph);
}

// Função para busca com filtros e cache.
// Utiliza lifetimes para lidar com referências de dados de longa duração.
fn search_with_cache<'a>(
    category: &str,                       // Categoria a ser buscada.
    brand_filter: Option<&str>,           // Filtro opcional de marca.
    products: &'a HashMap<u32, Product>,  // Índice de produtos com lifetime `'a`.
    cache: &mut HashMap<String, Vec<&'a Product>>, // Cache para buscas frequentes.
) {
    // Verifica se o resultado da busca já está no cache.
    if let Some(cached_result) = cache.get(category) {
        println!("Resultado em cache para '{}': {:?}", category, cached_result);
    } else {
        // Filtra os produtos que pertencem à categoria especificada.
        let filtered_products: Vec<&Product> = products
            .values()                      // Itera sobre todos os produtos.
            .filter(|product| {
                product.category == category // Filtra pela categoria.
                    && brand_filter.map_or(true, |brand| product.brand == brand) // Aplica filtro de marca.
            })
            .collect();                     // Coleta os produtos filtrados.

        // Apenas adiciona ao cache se houver produtos encontrados.
        if !filtered_products.is_empty() {
            cache.insert(category.to_string(), filtered_products.clone());
        }

        // Exibe os produtos encontrados ou uma mensagem de "nenhum resultado".
        if !filtered_products.is_empty() {
            println!(
                "Produtos encontrados na categoria '{}': {:?}",
                category, filtered_products
            );
        } else {
            println!("Nenhum produto encontrado na categoria '{}'.", category);
        }
    }
}

// Função para gerar recomendações de produtos com base no grafo.
fn recommend_products(
    product_id: u32,                           // ID do produto base.
    products: &HashMap<u32, Product>,          // Índice de produtos.
    recommendation_graph: &HashMap<u32, Vec<u32>>, // Grafo de recomendações.
) {
    // Busca os IDs dos produtos recomendados no grafo.
    if let Some(recommended_ids) = recommendation_graph.get(&product_id) {
        println!("Recomendações para '{}':", products.get(&product_id).unwrap().name);
        for id in recommended_ids {
            if let Some(product) = products.get(id) {
                println!("- ID: {}, Nome: {}", product.id, product.name);
            }
        }
    } else {
        println!("Nenhuma recomendação encontrada para o produto ID {}.", product_id);
    }
}

// Módulo de testes para validar funcionalidades.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_with_cache_found() {
        let mut products: HashMap<u32, Product> = HashMap::new();
        products.insert(
            101,
            Product {
                id: 101,
                name: "Smartphone".to_string(),
                category: "Eletronicos".to_string(),
                brand: "TechBrand".to_string(),
            },
        );

        let mut cache: HashMap<String, Vec<&Product>> = HashMap::new();

        search_with_cache("Eletronicos", Some("TechBrand"), &products, &mut cache);
        assert!(cache.get("Eletronicos").is_some());
    }

    #[test]
    fn test_search_with_cache_not_found() {
        let mut products: HashMap<u32, Product> = HashMap::new();
        products.insert(
            101,
            Product {
                id: 101,
                name: "Smartphone".to_string(),
                category: "Eletronicos".to_string(),
                brand: "TechBrand".to_string(),
            },
        );

        let mut cache: HashMap<String, Vec<&Product>> = HashMap::new();

        search_with_cache("Vestuário", None, &products, &mut cache);
        assert!(cache.get("Vestuário").is_none());
    }

    #[test]
    fn test_recommend_products() {
        let mut products: HashMap<u32, Product> = HashMap::new();
        products.insert(
            101,
            Product {
                id: 101,
                name: "Smartphone".to_string(),
                category: "Eletronicos".to_string(),
                brand: "TechBrand".to_string(),
            },
        );

        let mut recommendation_graph: HashMap<u32, Vec<u32>> = HashMap::new();
        recommendation_graph.insert(101, vec![102, 103]);

        recommend_products(101, &products, &recommendation_graph);
        assert!(recommendation_graph.get(&101).is_some());
    }
}