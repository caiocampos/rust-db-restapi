# rust-db-restapi

[![DepShield Badge](https://depshield.sonatype.org/badges/caiocampos/rust-db-restapi/depshield.svg)](https://depshield.github.io)
[![License](https://img.shields.io/github/license/caiocampos/rust-db-restapi.svg)](LICENSE)

Servidor Rust simples com conexão ao PostgresSQL

## Executando:

Para executar o projeto é necessário o Rust instalado e configurado, siga as instruções do site a seguir para configurar:

https://www.rust-lang.org/tools/install

### Atenção

`Instale a versão nightly, atualmente o framework rocket depende desta versão. No caso de ser uma instalação em Windows, é recomendado instalar a versão MSVC.`

`Pode ser necessário instalar dependências do banco de dados, para isso confira as instruções no site:`

https://github.com/diesel-rs/diesel/blob/master/guide_drafts/backend_installation.md


Antes de executar modifique o arquivo config.toml para apontar para o PostgreSQL instalado.

Após configurar o ambiente e configurar o arquivo config.toml compile o código, utilize o seguinte comando para isso:

> cargo build

E depois, para executar:

> cargo run
