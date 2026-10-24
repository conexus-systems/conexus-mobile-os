---
title: "BAMBA: jogo de luta 2D compatível com MUGEN para Android e iOS"
description: "BAMBA é um jogo de luta 2D para Android e iOS com engine própria compatível com o formato de personagens do MUGEN, luta online com rollback e personagens baixáveis."
date: 2026-10-24
categories: [jogos]
tags: [bamba, mugen, jogo de luta, android, ios, rollback]
---

O **BAMBA** é um jogo de luta 2D para **Android e iOS** (Expo / React Native) com engine própria compatível com o formato de personagens do **MUGEN**, luta online com **rollback** e catálogo de personagens baixáveis.

## O que ele faz

- **Engine em TypeScript** que lê os arquivos `.def`, `.air`, `.cmd` e `.cns`: estados, triggers com expressões, `HitDef`, projéteis, caixas de colisão, comandos, rounds, KO e tempo. É determinística, requisito do rollback.
- **A mesma engine em Rust** (desde a versão 0.6.0), com netcode de rollback e build WASM.
- Integração com os serviços de conta e API da Conexus.

O MUGEN original é um programa fechado que roda só no Windows; o BAMBA **reimplementa o formato**, sem usar o código original.

## Modelo de compra

Jogo pago, com compra única de R$ 49,99 na Google Play, sem anúncios, sem compras no app e sem nada bloqueado. O modo online exige a cópia comprada; o offline nunca depende disso.

Leia também a [política de privacidade do BAMBA]({{ '/privacidade-bamba/' | relative_url }}).
