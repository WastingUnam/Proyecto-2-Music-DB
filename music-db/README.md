# Proyecto 2 Base de datos musical

Minar los metadatos de las canciones dado un directorio y guardarlos en una base de datos. Y poder hacer busquedas en esta misma.

## Compilar y correr con cargo

```bash
cargo build
cargo run
```

## Botones y que hace cada uno

| Boton | Que hace |
|---------|----------|
| `Buscar` | Dada una busqueda valida genera el query que te da lo que pides |
| `Limpiar` | Limpia la busqueda y te da la base de datos completa |
| `Editar Performer` | Dice si ese performer es una persona o un grupo |
| `Minar carpeta` | Abre un menu para elegir que carpeta a minar |
| `/adentro de editar performer`| Adentro del menu |
| `Persona` | Modificar nombre artistico, real, fecha de nacimiento y muerte |
| `Grupo` | Fecha de inicio y fecha de fin |
| `Editar miembros` | De las personas que estan en la tabla de persons, puedes poner cuales si estan ese grupo |

## Pruebas

Gracias a una amplia cantidad de pruebas empiricas. Podemos concluir con certeza que si funciona.

## Docker

No hay, perdon Edgar :(

## Como funciona

Con la gracia de Linus Torvals. Y con mucha fe.
