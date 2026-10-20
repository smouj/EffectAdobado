# EffectAdobado

<p align="center"><img src="assets/adobado/icon.png" width="104" alt="EffectAdobado"></p>

**Aplica efectos visuales a tus proyectos.**

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-6cb6ff)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/Rust-1.95%2B-ff7a59)](Cargo.toml)
[![Windows build](https://github.com/smouj/EffectAdobado/actions/workflows/windows-build.yml/badge.svg?branch=main)](https://github.com/smouj/EffectAdobado/actions/workflows/windows-build.yml)
[![Prueba](https://img.shields.io/badge/prueba-inicio%20comprobado-6cb6ff)](#estado)
[![Estrellas](https://img.shields.io/github/stars/smouj/EffectAdobado?style=flat&color=B8F34A)](https://github.com/smouj/EffectAdobado/stargazers)
[![Adobado Kit](https://img.shields.io/badge/Adobado%20Kit-hub-b8f34a)](https://github.com/smouj/Adobado_Kit)

> **Pruebas en Windows (21/10/2026):** la compilación, la instalación y el inicio están comprobados. Falta validar el flujo con un archivo real: abrir, editar y exportar. La imagen es una tarjeta de marca; no muestra la interfaz.
>
> Proyecto independiente. No afiliado a ArtCraft ni a Adobe.

![EffectAdobado — tarjeta de marca; interfaz pendiente](docs/preview.png)

<sub>Tarjeta de marca; no es una captura de la interfaz.</sub>

## Estado

- **Build de Windows:** correcto en GitHub Actions; consulta el [workflow actual](https://github.com/smouj/EffectAdobado/actions/workflows/windows-build.yml).
- **Instalación e inicio:** comprobados en Windows el 21/10/2026; en la primera prueba, la ventana tardó aproximadamente 25 segundos en responder.
- **Prueba con un archivo:** falta completar el flujo de abrir, editar y exportar.
- **Estado:** alpha; falta validar una tarea completa con un archivo real y revisar la demora inicial.
- **Descarga pública:** todavía no hay un paquete disponible.
- **Commit de origen:** ef74d82902ba486891b02afa87e42963cc9b84cc.

## Build

Requisitos: Rust 1.95 o superior.

    cargo build --release -p effectcraft

El nombre interno del crate se conserva para no romper el workspace; el nombre de producto visible es EffectAdobado.

## Procedencia y licencias

- Upstream: https://github.com/storytold/effectcraft
- Commit base: ef74d82902ba486891b02afa87e42963cc9b84cc
- Licencia: MIT OR Apache-2.0. Se conservan los archivos de licencia, NOTICE y ATTRIBUTION del upstream.
- Cambios: docs/ADOBADO.md.
- La marca, el wordmark y los logos de ArtCraft no estan cubiertos por la licencia y se retiran de la distribucion.

## Adobado Kit

Este repo forma parte de Adobado Kit: https://github.com/smouj/Adobado_Kit

Hecho por @smouj.
