# Arquitetura VAZ 1.0 Alpha

## Fluxo nativo
1. O endpoint resolve a identidade pública do destinatário.
2. O remetente serializa a mensagem localmente.
3. Um segredo efêmero X25519 é combinado com a chave pública de troca do destinatário.
4. HKDF-SHA-256 deriva uma chave por objeto.
5. XChaCha20-Poly1305 cifra o conteúdo; Ed25519 autentica o envelope.
6. Somente o envelope cifrado entra na camada de storage.
7. Reed-Solomon cria data/parity shards; cada shard recebe hash BLAKE3.
8. Nós armazenam bytes opacos endereçados por hash.
9. O destinatário reúne shards suficientes, reconstrói o envelope, verifica assinatura e decifra localmente.

## Ledger
O ledger v1 é deliberadamente mínimo: endereço, chave pública de assinatura, chave pública de troca e contador de rotação. A implementação atual é local/in-memory; consenso distribuído é trabalho futuro. **Não é uma blockchain de produção.**

## Nó
O nó alpha expõe armazenamento content-addressed apenas em localhost. Descoberta P2P, autenticação entre peers, quotas e persistência em disco ainda precisam ser implementadas.

## SMTP
SMTP é somente borda de interoperabilidade. Mensagens VAZ→VAZ não dependem dele. O gateway permanece fora do caminho crítico até existir autenticação, reputação, antispam e política operacional.