# Threat Model

## Objetivos atuais
- Nó de armazenamento curioso recebe apenas shards do envelope cifrado.
- Alteração de shard é detectada por hash.
- Alteração do envelope deve falhar na assinatura/AEAD.
- Perda limitada de nós pode ser tolerada pelos parity shards.

## Não resolvido na alpha
- anonimato e ocultação de metadados/IP;
- Sybil/eclipse attacks e descoberta P2P adversarial;
- consenso Byzantine para identidade;
- spam/abuso e interoperabilidade SMTP segura;
- recuperação de conta e sincronização multi-device;
- comprometimento do endpoint e supply chain;
- análise de tráfego, censura e disponibilidade em escala.

Nenhuma alegação de “segurança extrema”, anonimato ou invulnerabilidade deve ser feita antes de auditoria independente, fuzzing e testes adversariais.