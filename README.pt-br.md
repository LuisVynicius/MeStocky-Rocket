# Mestocky

Este repositório contém o backend do projeto Mestocky, desenvolvido para servir como a API principal do sistema de estoque utilizado pelo frontend em Angular (presente neste mesmo perfil).

## Variáveis de ambiente

- DATABASE_URL: URL de conexão com o banco de dados.
- JWT_SECRET: Chave usada para geração e validação dos tokens JWT.
- ENCRYPT_COST:Custo de encriptação de senhas (entre 4 e 31).
// Valores maiores aumentam a segurança, mas tornam o processo de hash mais lento.

## Rotas
A seguir, uma visão geral das rotas disponíveis, separadas por entidade.

- Categorias:
 - GET:
  - /category -> Retorna todas as categorias.
  - /category/admin -> Retorna categorias com informações adicionais (somente administradores).
 - POST:
  - /category -> Cria uma nova categoria.
 - PUT:
  - /category -> Atualiza uma categoria existente.
 - DELETE:
  - /category/<id> -> Remove uma categoria pelo ID.

- Produtos:
 - GET:
  - /product -> Retorna todos os produtos.
  - /product/informations -> Retorna dados gerais de estoque dos produtos.
  - /product/<id> -> Retorna um produto específico.
 - POST:
  - /product -> Cria um novo produto.
 - PUT:
  - /product -> Atualiza um produto.
  - /product/quantity -> Altera a quantidade de um produto, registrando o motivo.
 - DELETE:
  - /product/<id> -> Remove um produto pelo ID.

- Motivos:
 - GET:
  - /reason -> Retorna todos os motivos.
 - POST:
  - /reason -> Cria um motivo.
 - PUT:
  - /reason -> Atualiza um motivo.
 - DELETE:
  - /reason/<id> -> Remove um motivo pelo ID.

- Relatórios:
 - GET:
  - /report -> Retorna todos os relatórios de movimentação.
 - PUT:
  - /report - Atualiza um relatório.
  
- Usuários:
 - GET:
  - /user -> Retorna todos os usuários.
  - /login/valid -> Verifica se o token atual é válido.
 - POST:
  - /user -> Cria um novo usuário.
  - /login -> Gera token de autenticação.
 - PUT:
  - /user/informations -> Atualiza informações básicas do usuário logado.
  - /user/credentials -> Atualiza as credenciais do usuário logado.
 - DELETE:
  - /user/<id> -> Remove um usuário pelo ID.