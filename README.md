# VAZ

> Rede brasileira aberta e descentralizada de comunicação — **v1.0.0-alpha.1**

VAZ é um projeto experimental open source para comunicação P2P com identidade criptográfica, armazenamento distribuído de objetos cifrados e interoperabilidade futura com e-mail tradicional.

## Estado de segurança

**ALPHA / NÃO USE PARA DADOS REAIS.** Esta versão não foi auditada. O projeto não promete anonimato, invulnerabilidade ou segurança absoluta. Chaves privadas, mensagens, anexos e dados pessoais nunca devem ser gravados em ledger público.

## Objetivo da v1

Demonstrar o núcleo: Alice cria uma identidade, cifra e assina uma mensagem para Bob, o objeto cifrado é dividido e distribuído entre nós, alguns nós podem desaparecer e Bob ainda consegue recuperar, verificar e decifrar a mensagem.

## Arquitetura

- `crates/vaz-crypto`: envelope criptográfico.
- `crates/vaz-protocol`: tipos canônicos do VMP.
- `crates/vaz-storage`: fragmentação, integridade e shards.
- `crates/vaz-ledger`: ledger mínimo de identidade/chaves públicas; zero conteúdo privado on-chain.
- `crates/vaz-node`: nó de armazenamento experimental.
- `apps/desktop`: cliente de mensagens/e-mail em desenvolvimento.
- `gateway/smtp`: fronteira SMTP, deliberadamente desativada nesta alpha.
- `docs/`: arquitetura e threat model.

## Executar

Requer Rust estável:

```bash
cargo test --workspace
cargo run -p vaz-node
```

O nó experimental escuta somente `127.0.0.1:8787` por padrão.

## Princípios

1. Criptografia antes de transmitir ou fragmentar.
2. Chave privada permanece no endpoint.
3. Ledger contém somente material público mínimo e eventos autorizados.
4. Segurança não depende de código secreto.
5. Formatos têm versão e limites explícitos.
6. Rede trata peers, pacotes e armazenamento como não confiáveis.
7. SMTP é ponte externa, não transporte interno da VAZ.

Leia `SECURITY.md` e `docs/THREAT_MODEL.md` antes de testar ou contribuir.

## Licença

Apache-2.0.