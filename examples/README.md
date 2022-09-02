# Examples

## Create an image

```sh
cargo run --release --example gen_image knots 30 20
```

Create a png image using the provided tile set

![](/examples/output.png)
> [source](/examples/gen_image.rs)

## Create an animated gif

```sh
cargo run --release --example gif basic 30 20
```

Create a gif showing each step of the collapsing wave.

![](/examples/output.gif)
> [source](/examples/gif.rs)

## Calculate connectors

```sh
cargo run --release --example print_connectors
```

Calculate and print the connectorID of each tile

| Image     | Direction | ConnectorID |
|-----------|-----------|-------------|
| blank.png | up        | A0DF4247    |
| blank.png | right     | A0DF4247    |
| blank.png | down      | A0DF4247    |
| blank.png | left      | A0DF4247    |
|           |           |             |
| down.png  | up        | A0DF4247    |
| down.png  | right     | E3820096    |
| down.png  | down      | E3820096    |
| down.png  | left      | E3820096    |
|           |           |             |
| left.png  | up        | E3820096    |
| left.png  | right     | A0DF4247    |
| left.png  | down      | E3820096    |
| left.png  | left      | E3820096    |
|           |           |             |
| right.png | up        | E3820096    |
| right.png | right     | E3820096    |
| right.png | down      | E3820096    |
| right.png | left      | A0DF4247    |
|           |           |             |
| up.png    | up        | E3820096    |
| up.png    | right     | E3820096    |
| up.png    | down      | A0DF4247    |
| up.png    | left      | E3820096    |
|           |           |             |
> [source](/examples/print_connectors.rs)
