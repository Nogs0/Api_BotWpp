# api_bot_wpp

Desenvolvido por João Guilherme Saraiva // N0gs

> Projeto pessoal que objetiva criar uma api que receberá os dados de um bot de Whats App, tratá-los e criar uma mensagem automática dinâmica, de acordo com os dados enviados para o aplicativo, para ser enviada aos clientes necessitados do serviço. As mensagens automáticas serão enviadas aos clientes que enviarem mensagem para àqueles motoristas OFFLINE

Funcionará da seguinte maneira:
- utilizando o recurso de regras do aplicativo de resposta automática *autoresponder wa*, está API receberá os dados de um determinado grupo denominado *Central_motoristas*, onde os usuários apenas enviarão mensagens simples, como: 'OFFLINE' ou 'ONLINE'
- De acordo com os envios, o aplicativo redicionará para a rota ideal, seja para dizer que o motorista está online, ou offline

>
    ONLINE
        Caso o motorista opte por esta opção, a API irá inserir seu nome e telefone na mensagem automática. Deixando claro sua disponibilidade.

    OFFLINE
        Neste caso o motorista será removido da mensagem

Primeiro projeto em RUST a ser realizado por este desenvolvedor.