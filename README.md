# TogglReports - UI

![Latest Release](https://img.shields.io/github/v/release/ro-56/togglReports-UI)
![License, MIT](https://img.shields.io/badge/license-MIT-green)

---

TogglReports-UI é um aplicação criada para gerar relatórios a partir das informações do Toggl.

## Utilização

### Entradas de horas

É necessário que as entradas presentes no Toggle estejam de acordo com o padrão esperado para que sejam traduzidas para o formato SGU.

Abaixo está a relação entre campos:

| Toggl       	| SGU       	|
|-------------	|-----------	|
| Description 	| Nome      	|
| Project     	| Projeto   	|
| Tag         	| Categoria 	|

### Configurando a ferramenta

Antes do primeiro uso, é preciso configurar a ferramenta para conseguir se conectar com o Toggl. As configurações se encontram no botão superior direito (engrenagem).

- **API Token:** Token de usuário do Toggl. Para mais informações sobre onde encontrar esta informação, [clique aqui](https://support.toggl.com/en/articles/3116844-where-is-my-api-key-located)
- **Workspace ID:** _Workspace_ do Toggl que contém as entradas. Só pode ser escolhido depois que o API Token estiver preenchido.
- **Save output to:** Diretório local onde os relatórios criados serão salvos
- **SGU Name:** Nome do SGU na qual as entradas devem ser atribuídas
- **Tag to ignore:** Caso esta _tag_ esteja presente, a entrada será ignorada no momento de criação do relatório final
- **Default tag:** caso nenhuma _tag_ seja encontrada, esta será utilizada.

**É necessário salvar após preencher as informações.**

### Gerando um relatório

Após configurar a ferramenta, será possível gerar relatórios de horas para o SGU a partir da tela inicial. Para retornar, clique no botão superior esquerdo (casa).

1. Selecione um intervalo de tempo para obter as entradas.
    - Caso seja escolhido o intervalo "customizado", preencha as datas de início e fim
2. Clique em confirmar.

O relatório gerado será salvo no local configurado anteriormente e poderá ser adicionado ao SGU.

## Desenvolvimento

Abaixo estão algumas informações importantes para o desenvolvimento.

### IDE - Configuração Recomendada

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

### Executando localmente

1. Instale as dependências: `npm install`
2. Execute em modo de desenvolvimento: `npm run tauri dev`

### Versionamento

O repositório está configurado para lançar uma nova versão quando for criada uma tag no padrão `v*`.
