# Temporary

Should move extra details from Rust API documentation to book somewhere about scaling/translation/numbering since most people will not be reading the Rust API documentation.

"The voxel data can be scaled and translated (in that order)."

$$
x \mapsto s_x x + t_x\qquad y \mapsto s_y y + t_y\qquad z \mapsto s_z z + t_z
$$

```bob
                    8       7
                     *-------*
                  5 /|    6 /|
 z                 *-+-----* |
  ^  y             | |4    | |3
  | ^              | *-----|-*
  |/               |/      |/
  +-----> x        *-------*
                   1       2
```
