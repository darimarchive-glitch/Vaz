# Contribuindo

1. Abra uma issue antes de mudanças de protocolo ou criptografia.
2. Mudanças de segurança precisam de testes e revisão independente.
3. Nunca inclua dados reais ou secrets.
4. `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace` devem passar.
5. Mudanças de protocolo devem atualizar a documentação em `docs/`.
6. Dependências novas precisam de justificativa, manutenção ativa e licença compatível.