

# Sistema de Votación ZK

Un sistema de votación que preserva la privacidad, construido en Solana utilizando pruebas de conocimiento cero. El programa está desplegado en Solana Devnet con el id del programa: `EoUv2UHa4y25Vm3opdDZGyfMbgEZ5DF7PaUTdHB2PLrM`

## Tabla de Contenidos
- [Características](#features)
- [Requisitos](#prerequisites)
- [Instalación](#installation)
- [Uso](#usage)
- [Pruebas](#testing)
- [Contribución](#contributing)
- [Licencia](#license)

## Características

- Crear y gestionar DAOs
- Crear propuestas
- Emitir votos cifrados
- Realizar el recuento de votos preservando la privacidad
- Recompensar a los participantes

## Requisitos

Antes de comenzar, asegúrate de cumplir con los siguientes requisitos:

- Rust (última versión estable)
- Herramientas CLI de Solana (v1.10.0 o posterior)
- Node.js (v14 o posterior)
- Yarn

## Instalación

1. Clona el repositorio:
   ```
   git clone https://github.com/akshatcoder-hash/zk-voting.git
   cd zk-voting
   ```

2. Instala las dependencias:
   ```
   yarn install
   ```

3. Compila el programa:
   ```
   anchor build
   ```

## Uso

1. Inicia un clúster local de Solana:
   ```
   solana-test-validator
   ```

2. Despliega el programa:
   ```
   anchor deploy
   ```

3. Ejecuta el cliente:
   ```
   anchor run client
   ```

## Pruebas

Para ejecutar el conjunto de pruebas:

```
anchor test
```

Esto ejecutará todos los casos de prueba, incluida la inicialización de una DAO, la creación de una propuesta, la emisión de votos y el recuento de resultados.

## Contribución

¡Damos la bienvenida a las contribuciones al Sistema de Votación ZK! Consulta nuestra [Guía de Contribución](CONTRIBUTING.md) para más detalles.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - consulta el archivo [LICENSE](LICENSE) para más detalles.
