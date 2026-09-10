# Política de Segurança

VAZ está em **alpha experimental e não auditada**. Não use para segredos, dados pessoais reais ou comunicação cuja exposição cause dano.

## Regras
- Nunca publique chaves privadas, recovery seeds, tokens, senhas, mensagens reais ou dumps de usuários em issues/PRs.
- Falhas que permitam leitura de mensagens, falsificação de identidade, execução remota, bypass criptográfico ou comprometimento de releases devem ser relatadas privadamente pelo recurso Private Vulnerability Reporting do GitHub, quando habilitado.
- Não existe bug bounty nesta fase.

## Criptografia
A implementação usa primitivas/bibliotecas existentes, mas a composição atual **não é considerada auditada**. O ledger jamais deve conter conteúdo privado ou chaves privadas.