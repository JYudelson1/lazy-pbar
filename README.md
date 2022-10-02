# lazy-pbar

A very lazy wrapper for any ProgressIterator with known length
Looks nice enough as a default
Style heavily borrowing from [tqdm](https://github.com/tqdm/tqdm).


Example:
```
use lazy-pbar::pbar;

let mut x = 0;
for i in pbar(0..1_000_000, 1_000_000) {
  x = i;
}
```
