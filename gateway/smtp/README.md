# VAZ SMTP Gateway

Fronteira planejada para `@vaz.br` ↔ SMTP. **Não implementada na alpha por segurança operacional.**

Antes de ativar publicamente serão necessários SPF, DKIM, DMARC, MTA-STS/TLS-RPT, filas, reputação, rate limiting, antispam, tratamento de bounce, observabilidade e política de abuso.

O gateway nunca deve receber chaves privadas VAZ e não faz parte do transporte VAZ→VAZ.